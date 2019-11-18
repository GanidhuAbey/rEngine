use crate::render;
use crate::event;

use sdl2::video::Window;
use sdl2::video::WindowBuilder;
use sdl2::VideoSubsystem;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::event::Event;

use event::GameEvent;

pub struct GameWindow {
    pub window: Window,
    pub video_subsystem: VideoSubsystem,
    pub event_pump: EventPump,
    pub sdl: Sdl,
}

impl GameWindow {
    pub fn create_window() -> GameWindow {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
    
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);
    
        let window = WindowBuilder::new(
            &video_subsystem,
            "rEngine",
            500,
            500,
        ).opengl().build().unwrap();
    
        let mut event_pump = sdl.event_pump().unwrap();
    
        GameWindow {
            window,
            video_subsystem,
            event_pump,
            sdl
        }
    
    }
    pub fn event_loop(&mut self) -> GameEvent {
        for events in self.event_pump.poll_iter() {
            match events {
                Event::Quit {..} => {
                    return GameEvent::Quit;
                },
                _ => {},
            }
        }
        GameEvent::Blank
    }
}
