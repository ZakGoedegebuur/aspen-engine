use std::sync::Arc;

use winit::{window::{Window, WindowBuilder, WindowId}, event_loop::EventLoop};

use crate::error::AspenError;

#[derive(Debug)]
pub struct AspenWindow {
    window: Arc<Window>
}

type AspenWindowID = WindowId;

impl AspenWindow {
    pub fn new(event_loop: &EventLoop<()>) -> Result<AspenWindow, AspenError> {
        let window = Arc::new(
            WindowBuilder::new().build(event_loop)?
        );

        Ok(AspenWindow {
            window
        })
    }

    pub fn id(&self) -> AspenWindowID {
        self.window.id()
    }
}