//rEngine: a 2D game engine for rust created with "ash", a lightweight wrapper around vulkan.
use winit::EventsLoop;
use winit::WindowBuilder;
use winit::Window;

use std::sync::Arc;

use vulkano::instance::{Instance};
use vulkano::swapchain::Surface;
use vulkano_win::VkSurfaceBuild;
use vulkano_win::required_extensions;

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
    fn create_window(self) -> Arc<Surface<Window>> {
        let event_loop = EventsLoop::new();
        WindowBuilder::new().build_vk_surface(&event_loop, self.instance.clone()).unwrap()
    }
}

fn main() {
    //Initialize Engine
    let engine = Engine::new();

    //create window
    let surface = engine.create_window();
    let window = surface.window();
}