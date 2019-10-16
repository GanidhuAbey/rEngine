//rEngine: a 2D game engine for rust created with "ash", a lightweight wrapper around vulkan.
use std::sync::Arc;

use vulkano::instance::{Instance};
use vulkano_win::required_extensions;

mod window;

use crate::window::EWindow;

struct Engine {
    instance: Arc<Instance>,
}
impl Engine {
    fn new() -> Engine {
        Engine {
            instance: {
                let extensions = required_extensions();

                Instance::new(None, &extensions, None).unwrap()
            }
        }
    }
}

fn main() {
    let engine = Engine::new();
    let surface = EWindow::new(engine.instance);
    let _window = surface.window();
}