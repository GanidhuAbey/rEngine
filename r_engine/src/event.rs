use winit::event_loop::EventLoop;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;

pub enum GameEvent {
    KeyEvent,
    //basically an error value that doesn't crash the program
    NoEvent,
}

pub struct ManageEvents {
    pub event_loop: EventLoop<()>,
    pub current_event: GameEvent,
}

impl ManageEvents {
    pub fn new() -> ManageEvents {
        ManageEvents {
            event_loop: EventLoop::new(),
            current_event: GameEvent::NoEvent,
        }
    }
    pub fn request_event(event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow|
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {*control_flow = ControlFlow::Exit},
                _ => *control_flow = ControlFlow::Wait,
            }
        );
    }
}

