extern crate sdl2;
extern crate gl;

use sdl2::video::WindowBuilder;
use sdl2::event::Event;

use std::ffi::CString;

use r_engine::window_fn;
use r_engine::event::GameEvent;
use r_engine::render;

pub mod render_gl;

fn main() {
    /*
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

    let _gl_context = window.gl_create_context().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    */
    let mut sdl = window_fn::GameWindow::create_window();
    
    let _gl_context = sdl.window.gl_create_context().unwrap();

    
    let _gl = gl::load_with(|s| sdl.video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 500, 500);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    };
    let vert_shader = render::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();
    let frag_shader = render::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let shader_program = render::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }
    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    shader_program.set_used();
    
    //set up loop with life time parameter inorder to break specific loop.
    'game: loop {
        match sdl.event_loop() {
            GameEvent::Quit => {
                break 'game
            }
            _ => {},
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                3
            );
        }

        sdl.window.gl_swap_window();
    }
}