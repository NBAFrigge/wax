mod state;
use std::os::fd::AsFd;

use state::State;
use wayland_client::{Connection, EventQueue};

fn main() {
    let connection = Connection::connect_to_env().unwrap();
    let mut event_queue: EventQueue<State> = connection.new_event_queue();
    let qh = event_queue.handle();
    let display = connection.display();

    let _registry = display.get_registry(&qh, ());

    let mut state = State::new();

    event_queue.roundtrip(&mut state).unwrap();
    event_queue.roundtrip(&mut state).unwrap();

    let manager = state.manager.as_ref().unwrap();
    let seat = state.seat.as_ref().unwrap();
    let device = manager.get_data_device(seat, &qh, ());
    state.device = Some(device);

    loop {
        event_queue.blocking_dispatch(&mut state).unwrap();

        if let Some(offer) = &state.current_offer {
            let (fd_read, fd_write) = rustix::pipe::pipe().unwrap();

            let mime = if state.mime_types.contains(&"text/plain".to_string()) {
                "text/plain"
            } else {
                continue;
            };

            offer.receive(mime.to_string(), fd_write.as_fd());
            drop(fd_write);

            event_queue.flush().unwrap();

            let mut file = std::fs::File::from(fd_read);
            let mut buffer = Vec::new();
            std::io::Read::read_to_end(&mut file, &mut buffer).unwrap();

            println!("Content: {}", String::from_utf8_lossy(&buffer));
            state.current_offer = None;
        }
    }
}
