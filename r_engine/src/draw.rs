use crate::render;
use std::ffi::CString;

pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    };
}
pub fn rectangle(w: f32, h: f32, x: f32, y: f32) {
    let vert_shader = render::Shader::from_vert_source(&CString::new(include_str!("rectangle.vert")).unwrap()).unwrap();
    let frag_shader = render::Shader::from_frag_source(&CString::new(include_str!("rectangle.frag")).unwrap()).unwrap();

    let shader_program = render::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let vertices: Vec<f32> = vec![
        x, y, 0.0,
        x + w, y, 0.0,
        x + w, y - h, 0.0,
        x, y - h, 0.0,
        
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

    unsafe {
        gl::BindVertexArray(vao);
        gl::DrawArrays(
            gl::TRIANGLES,
            0,
            3,
        );
    }
}

pub fn triangle(color: [f32; 3]) {
    let vert_shader = render::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();
    let frag_shader = render::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let shader_program = render::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let color_location = unsafe {
        gl::GetUniformLocation(shader_program.id, CString::new("fragColor").unwrap().as_ptr() as *const gl::types::GLchar)
    };
    unsafe {
        gl::UseProgram(shader_program.id);
        gl::Uniform4f(color_location, color[0], color[1], color[2], 1.0);
    }

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

    unsafe {
        gl::BindVertexArray(vao);
        gl::DrawArrays(
            gl::TRIANGLES,
            0,
            3
        );
    }
}

pub fn rect() {
}