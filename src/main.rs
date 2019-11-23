
use r_engine::window_fn;
use r_engine::event::GameEvent;
use r_engine::draw;

pub mod render_gl;

fn main() {
    let mut sdl = window_fn::GameWindow::create_window();
    
    //set up loop with life time parameter inorder to break specific loop.
    'game: loop {
        match sdl.event_loop() {
            GameEvent::Quit => {
                break 'game
            }
            _ => {},
        }
        draw::triangle();

        sdl.window.gl_swap_window();
    }
}