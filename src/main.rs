extern crate sdl2;
extern crate gl;

use sdl2::video::WindowBuilder;
use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::ffi::CStr;

struct Player {
    x: i32,
    y: i32,
}

impl Player {
    fn new(x: i32, y: i32) -> Player {
        Player {
            x: x,
            y: y,
        }
    }
}

fn shader_from_source(source: &str, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id  = unsafe{gl::CreateShader(kind);}
    
}

fn main() {
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

    let gl_context = window.gl_create_context().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut player = Player::new(0, 50);

    unsafe {
        gl::Viewport(0, 0, 500, 500);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }
    //set up loop with life time parameter inorder to break specific loop.
    'game: loop {
        for events in event_pump.poll_iter() {
            match events {
                Event::Quit {..} => {
                    break 'game;
                },
                _ => {},
            }
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}