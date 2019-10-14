//rEngine: a 2D game engine for rust created with "ash", a lightweight wrapper around vulkan.
use winit::EventsLoop;
use winit::Window;

fn window() {
    let event_loop = EventsLoop::new();
    let mut window = Window::new(&event_loop).unwrap();
}

fn main() {
    let window = window();
}