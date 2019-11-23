use crate::event;

use sdl2::video::Window;
use sdl2::video::WindowBuilder;
use sdl2::video::GLContext;
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
    pub gl_context: GLContext,
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
    
        let event_pump = sdl.event_pump().unwrap();

        let gl_context = window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        
        unsafe {
            gl::Viewport(0, 0, 500, 500);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        };
    
        GameWindow {
            window,
            video_subsystem,
            event_pump,
            sdl,
            gl_context
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
