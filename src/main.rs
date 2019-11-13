use r_engine::window;
use r_engine::event::GameEvent;
use r_engine::event;

fn main() {
    let event_manager = event::ManageEvents::new();
    let window = window::EWindow::new(&event_manager);

    let current_event = event::ManageEvents::request_event(event_manager.event_loop);
    println!("this line should be unreadable");
}