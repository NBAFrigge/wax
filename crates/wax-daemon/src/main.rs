mod config;
mod state;

use regex::RegexSet;
use state::State;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::fd::AsFd;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use wax_ipc::{Request, Response};
use wax_store::{ClipContent, ClipStore, Limits};
use wayland_client::{Connection, EventQueue};
use wayland_protocols_wlr::data_control::v1::client::zwlr_data_control_offer_v1::ZwlrDataControlOfferV1;

/// How many times to try opening the database before giving up
const DB_OPEN_ATTEMPTS: u32 = 10;

const DB_OPEN_RETRY_DELAY: std::time::Duration = std::time::Duration::from_millis(500);

/// Upper bound on a single clipboard offer, so a source app offering an
/// enormous `text/plain` cannot make the daemon read it all into memory.
const MAX_OFFER_BYTES: u64 = 64 * 1024 * 1024;

// Owner read write
const FILE_PERMISSION: u32 = 0o600;

fn open_store_with_retry(
    path: &std::path::Path,
    limits: &Limits,
) -> Result<ClipStore, Box<dyn std::error::Error>> {
    for attempt in 0..DB_OPEN_ATTEMPTS {
        match ClipStore::open(path, limits.clone()) {
            Ok(store) => return Ok(store),
            Err(e) => {
                if attempt == 0 {
                    eprintln!("db locked, retrying: {}", e);
                }
                std::thread::sleep(DB_OPEN_RETRY_DELAY);
            }
        }
    }
    Err(format!(
        "could not open database after {} attempts",
        DB_OPEN_ATTEMPTS
    )
    .into())
}

/// Bind the IPC socket, owner-only.
///
/// A freshly created socket gets `0777 & ~umask`, which under a typical umask
/// of 022 is `0755`. Write permission is what grants `connect()`, so at that
/// mode any local user could run `wax list` and read the entire clipboard
/// history. The permission error is propagated rather than ignored: starting up
/// with a world-readable socket is worse than not starting.
fn bind_socket(path: &std::path::Path) -> Result<UnixListener, Box<dyn std::error::Error>> {
    std::fs::remove_file(path).ok();
    let listener = UnixListener::bind(path)?;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(FILE_PERMISSION))?;
    Ok(listener)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::connect_to_env()?;
    let mut event_queue: EventQueue<State> = connection.new_event_queue();
    let qh = event_queue.handle();
    let display = connection.display();
    let _registry = display.get_registry(&qh, ());
    let mut state = State::new();
    event_queue.roundtrip(&mut state)?;
    event_queue.roundtrip(&mut state)?;

    let manager = state.manager.as_ref().ok_or("no data device manager")?;
    let seat = state.seat.as_ref().ok_or("no seat")?;
    let device = manager.get_data_device(seat, &qh, ());
    state.device = Some(device);

    let config = config::Config::load();
    let limits = wax_store::Limits {
        max_entries: config.max_entries,
        ttl_secs: config.ttl_secs,
    };

    let regex_exclude = regex::RegexSet::new(config.excluded_pattern)?;

    let db_path = wax_store::default_db_path();
    let store = Arc::new(open_store_with_retry(&db_path, &limits)?);
    eprintln!("wax daemon started, db: {}", db_path.display());

    let socket_path = wax_ipc::socket_path();
    let store_ipc = Arc::clone(&store);
    let running = Arc::new(AtomicBool::new(true));
    let running_ipc = Arc::clone(&running);

    let listener = bind_socket(&socket_path)?;
    eprintln!("listening on {}", socket_path.display());

    std::thread::spawn(move || {
        for stream in listener.incoming() {
            if !running_ipc.load(Ordering::Relaxed) {
                break;
            }
            let Ok(stream) = stream else { continue };
            let store = Arc::clone(&store_ipc);

            std::thread::spawn(move || {
                let mut reader = BufReader::new(&stream);
                let mut writer = &stream;
                let mut line = String::new();

                if reader.read_line(&mut line).is_err() {
                    return;
                }

                let clips_to_strings = |clips: Vec<wax_store::Clip>| {
                    clips
                        .into_iter()
                        .map(|c| match c.content {
                            ClipContent::Text(t) => t,
                            ClipContent::Image(p) => format!("[img] {}", p),
                        })
                        .collect()
                };

                let response = match serde_json::from_str::<Request>(line.trim()) {
                    Ok(Request::Get { n }) => match store.get(n) {
                        Ok(clips) => Response::Clips(clips_to_strings(clips)),
                        Err(e) => Response::Error(e.to_string()),
                    },
                    Ok(Request::GetPinned) => match store.get_pinned_clips() {
                        Ok(clips) => Response::Clips(clips_to_strings(clips)),
                        Err(e) => Response::Error(e.to_string()),
                    },
                    Ok(Request::Delete { text }) => {
                        let result = if let Some(path) = text.strip_prefix("[img] ") {
                            store.delete_image(path)
                        } else {
                            store.delete_text(&text)
                        };
                        match result {
                            Ok(_) => Response::Ok,
                            Err(e) => Response::Error(e.to_string()),
                        }
                    }
                    Ok(Request::Pin { text }) => match store.pin_text(&text) {
                        Ok(_) => Response::Ok,
                        Err(e) => Response::Error(e.to_string()),
                    },
                    Ok(Request::Unpin { text }) => match store.unpin_text(&text) {
                        Ok(_) => Response::Ok,
                        Err(e) => Response::Error(e.to_string()),
                    },
                    Ok(Request::Clear) => match store.clear() {
                        Ok(_) => Response::Ok,
                        Err(e) => Response::Error(e.to_string()),
                    },
                    Err(e) => Response::Error(format!("invalid request: {}", e)),
                };

                let Ok(mut json) = serde_json::to_string(&response) else {
                    return;
                };
                json.push('\n');
                writer.write_all(json.as_bytes()).ok();
            });
        }
    });

    loop {
        if let Err(e) = event_queue.blocking_dispatch(&mut state) {
            eprintln!("wayland dispatch error: {}", e);
            break;
        }

        if let Some(offer) = &state.current_offer {
            let skip = if state.is_primary {
                !config.primary_selection
            } else {
                !config.clipboard
            };

            if skip {
                state.current_offer = None;
                state.mime_types.clear();
                continue;
            }

            let mime = state
                .mime_types
                .iter()
                .find(|m| *m == "text/plain")
                .or_else(|| {
                    state
                        .mime_types
                        .iter()
                        .find(|m| m.starts_with("text/plain"))
                })
                .or_else(|| state.mime_types.iter().find(|m| *m == "UTF8_STRING"))
                .or_else(|| state.mime_types.iter().find(|m| *m == "STRING"))
                .or_else(|| state.mime_types.iter().find(|m| *m == "image/png"))
                .cloned();

            let Some(mime) = mime else {
                eprintln!("wax: no supported format offered: {:?}", state.mime_types);
                state.current_offer = None;
                state.mime_types.clear();
                continue;
            };

            if let Err(e) = handle_offer(offer, &mime, &mut event_queue, &store, &regex_exclude) {
                eprintln!("failed to handle clipboard offer: {}", e);
            }

            state.current_offer = None;
            state.mime_types.clear();
        }
    }

    running.store(false, Ordering::Relaxed);
    std::fs::remove_file(&socket_path).ok();
    eprintln!("wax daemon stopped");
    Ok(())
}

#[cfg(test)]
mod tests;

fn handle_offer(
    offer: &ZwlrDataControlOfferV1,
    mime: &str,
    event_queue: &mut EventQueue<State>,
    store: &ClipStore,
    regex_set: &RegexSet,
) -> Result<(), Box<dyn std::error::Error>> {
    let (fd_read, fd_write) = rustix::pipe::pipe()?;
    offer.receive(mime.to_string(), fd_write.as_fd());
    drop(fd_write);
    event_queue.flush()?;

    let mut buffer = Vec::new();
    std::fs::File::from(fd_read)
        .take(MAX_OFFER_BYTES)
        .read_to_end(&mut buffer)?;

    if is_text_mime(mime) {
        let text = String::from_utf8_lossy(&buffer);
        if accept_text(&text, regex_set) {
            store.push_text(&text)?;
        }
    } else if mime == "image/png" {
        store.push_image(&buffer)?;
    } else {
        eprintln!("wax: unsupported clipboard type {mime}, not stored");
    }

    Ok(())
}

fn accept_text(text: &str, regex_set: &RegexSet) -> bool {
    !text.trim().is_empty() && !regex_set.is_match(text)
}

/// Whether a MIME type carries text that should be stored as a text clip.
///
/// Apps advertise the same clipboard data under several names: the bare
/// `text/plain` most often, but also `text/plain;charset=utf-8` and the
/// X11-era `UTF8_STRING` / `STRING`. Comparing against the bare literal alone
/// drops every copy offered only under one of the other names.
///
/// Deliberately narrow: only `text/plain` and its parameterised forms count.
/// Browsers offer `text/html` and `text/rtf` alongside the plain text, and a
/// prefix match on `text/` would paste raw markup into the target app.
fn is_text_mime(mime: &str) -> bool {
    mime == "text/plain"
        || mime.starts_with("text/plain;")
        || matches!(mime, "UTF8_STRING" | "STRING" | "TEXT")
}
