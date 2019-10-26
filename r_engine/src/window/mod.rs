
use winit::event::WindowEvent;
use winit::window::WindowBuilder;
use winit::window::Window;
use winit::event_loop::EventLoop;
use winit::event::Event;

use crate::event::ManageEvents;




pub struct EWindow {
    screen: Window,
}

impl EWindow {
    //apply the screen settings to this new funcition as a parameter
    pub fn new(event_manager: &ManageEvents) -> EWindow { //plan to add any extra settings like fullscreen to the struct later on
        EWindow {
            screen: WindowBuilder::new().build(&event_manager.event_loop).unwrap(),
        }
    }
}