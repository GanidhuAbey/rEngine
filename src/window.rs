use winit::EventsLoop;
use winit::WindowBuilder;
use winit::Window;

use std::sync::Arc;

use vulkano::instance::{Instance};
use vulkano::swapchain::Surface;
use vulkano_win::VkSurfaceBuild;

pub struct EWindow {
    //not much here yet
    //filler: u32
}

impl EWindow {
    //returns a surface which can be converted into a window.
    pub fn new(instance: Arc<Instance>) -> Arc<Surface<Window>> { //plan to add any extra settings like fullscreen to the struct later on
        let event_loop = EventsLoop::new();
        return WindowBuilder::new().build_vk_surface(&event_loop, instance.clone()).unwrap()
    }
}