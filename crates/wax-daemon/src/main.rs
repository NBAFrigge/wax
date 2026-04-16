mod config;
mod state;

use regex::RegexSet;
use state::State;
use std::io::{BufRead, BufReader, Write};
use std::os::fd::AsFd;
use std::os::unix::net::UnixListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{error, info, warn};
use wax_ipc::{Request, Response};
use wax_store::{ClipContent, ClipStore, Limits};
use wayland_client::{Connection, EventQueue};
use wayland_protocols_wlr::data_control::v1::client::zwlr_data_control_offer_v1::ZwlrDataControlOfferV1;

fn open_store_with_retry(
    path: &std::path::Path,
    limits: &Limits,
) -> Result<ClipStore, Box<dyn std::error::Error>> {
    for attempt in 0..10 {
        match ClipStore::open(path, limits.clone()) {
            Ok(store) => return Ok(store),
            Err(e) => {
                if attempt == 0 {
                    warn!("db locked, retrying: {}", e);
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }
    Err("could not open database after 10 attempts".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

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
        max_db_bytes: config.max_db_mb * 1024 * 1024,
        max_images_bytes: config.max_images_mb * 1024 * 1024,
        ttl_secs: config.ttl_secs,
    };

    let regex_exclude = regex::RegexSet::new(config.excluded_pattern)?;

    let db_path = wax_store::default_db_path();
    let store = Arc::new(open_store_with_retry(&db_path, &limits)?);
    info!("wax daemon started, db: {}", db_path.display());

    let socket_path = wax_ipc::socket_path();
    let store_ipc = Arc::clone(&store);
    let running = Arc::new(AtomicBool::new(true));
    let running_ipc = Arc::clone(&running);

    std::fs::remove_file(&socket_path).ok();
    let listener = UnixListener::bind(&socket_path)?;
    info!("listening on {}", socket_path.display());

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

                let response = match serde_json::from_str::<Request>(line.trim()) {
                    Ok(Request::Get { n }) => match store.get(n) {
                        Ok(clips) => {
                            let strings: Vec<String> = clips
                                .into_iter()
                                .map(|c| match c.content {
                                    ClipContent::Text(t) => t,
                                    ClipContent::Image(p) => format!("[img] {}", p),
                                })
                                .collect();
                            Response::Clips(strings)
                        }
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
            error!("wayland dispatch error: {}", e);
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

            let mime = if state.mime_types.iter().any(|m| m == "text/plain") {
                "text/plain"
            } else if state.mime_types.iter().any(|m| m == "image/png") {
                "image/png"
            } else {
                state.current_offer = None;
                state.mime_types.clear();
                continue;
            };

            if let Err(e) = handle_offer(offer, mime, &mut event_queue, &store, &regex_exclude) {
                warn!("failed to handle clipboard offer: {}", e);
            }

            state.current_offer = None;
            state.mime_types.clear();
        }
    }

    running.store(false, Ordering::Relaxed);
    std::fs::remove_file(&socket_path).ok();
    info!("wax daemon stopped");
    Ok(())
}

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

    let mut file = std::fs::File::from(fd_read);
    let mut buffer = Vec::new();
    std::io::Read::read_to_end(&mut file, &mut buffer)?;

    match mime {
        "text/plain" => {
            let text = String::from_utf8_lossy(&buffer);
            if !regex_set.is_match(&text) {
                store.push_text(text.trim())?;
                info!("saved text: {} bytes", buffer.len());
            } else {
                info!("text skipped")
            }
        }
        "image/png" => {
            store.push_image(&buffer)?;
            info!("saved image: {} bytes", buffer.len());
        }
        _ => {}
    }

    Ok(())
}
