use std::{error::Error, sync::Arc};

use vulkano::{instance::Instance, swapchain::Surface};
use winit::{window::{Window, WindowBuilder, WindowId}, event_loop::EventLoop};

#[derive(Debug)]
pub struct AspenWindow {
    window: Arc<Window>,
    surface: Arc<Surface>,
}

impl AspenWindow {
    pub fn new(event_loop: &EventLoop<()>, instance: &Arc<Instance>) -> Result<AspenWindow, Box<dyn Error>> {
        let window = Arc::new(
            WindowBuilder::new().build(event_loop)?
        );

        let surface = Surface::from_window(instance.clone(), window.clone())?;

        Ok(AspenWindow {
            window,
            surface,
        })
    }
    
        pub fn id(&self) -> WindowId {
            self.window.id()
        }

    pub fn winit(&self) -> Arc<Window> {
        self.window.clone()
    }

    pub fn surface(&self) -> Arc<Surface> {
        self.surface.clone()
    }
}