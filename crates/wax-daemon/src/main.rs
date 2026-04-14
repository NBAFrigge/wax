mod state;

use state::State;
use std::io::{BufRead, BufReader, Write};
use std::os::fd::AsFd;
use std::os::unix::net::UnixListener;
use std::sync::Arc;
use tracing::{error, info, warn};
use wax_ipc::{Request, Response, SOCKET_PATH};
use wax_store::{ClipContent, ClipStore};
use wayland_client::{Connection, EventQueue};
use wayland_protocols_wlr::data_control::v1::client::zwlr_data_control_offer_v1::ZwlrDataControlOfferV1;

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

    let store = Arc::new(ClipStore::open("db.redb")?);
    info!("wax daemon started");

    // IPC thread
    let store_ipc = Arc::clone(&store);
    std::fs::remove_file(SOCKET_PATH).ok();
    let listener = UnixListener::bind(SOCKET_PATH)?;

    std::thread::spawn(move || {
        for stream in listener.incoming() {
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
                    Ok(Request::Clear) => match store.clear() {
                        Ok(_) => Response::Ok,
                        Err(e) => Response::Error(e.to_string()),
                    },
                    Err(e) => Response::Error(format!("invalid request: {}", e)),
                };

                let mut json = serde_json::to_string(&response).unwrap();
                json.push('\n');
                writer.write_all(json.as_bytes()).ok();
            });
        }
    });

    // Wayland loop
    loop {
        if let Err(e) = event_queue.blocking_dispatch(&mut state) {
            error!("wayland dispatch error: {}", e);
            break;
        }

        if let Some(offer) = &state.current_offer {
            let mime = if state.mime_types.contains(&"text/plain".to_string()) {
                "text/plain"
            } else if state.mime_types.contains(&"image/png".to_string()) {
                "image/png"
            } else {
                state.current_offer = None;
                continue;
            };

            if let Err(e) = handle_offer(offer, mime, &mut event_queue, &store) {
                warn!("failed to handle clipboard offer: {}", e);
            }

            state.current_offer = None;
        }
    }

    Ok(())
}

fn handle_offer(
    offer: &ZwlrDataControlOfferV1,
    mime: &str,
    event_queue: &mut EventQueue<State>,
    store: &ClipStore,
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
            store.push_text(text.trim())?;
            info!("saved text: {} bytes", buffer.len());
        }
        "image/png" => {
            store.push_image(&buffer)?;
            info!("saved image: {} bytes", buffer.len());
        }
        _ => {}
    }

    Ok(())
}

