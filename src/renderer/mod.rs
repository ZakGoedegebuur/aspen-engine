use std::sync::Arc;

use vulkano::{
    VulkanLibrary,
    instance::{Instance, InstanceCreateInfo},
    swapchain::Surface
};

use winit::event_loop::EventLoop;

use crate::error::{
    Error,
    ErrorType
};

#[derive(Debug)]
pub struct Renderer {
    winit_event_loop: EventLoop<()>,
    vk_instance: Arc<Instance>,
}

impl Renderer {
    pub fn new() -> Result<Renderer, Error> {
        let winit_event_loop = match EventLoop::new() {
            Ok(eloop) => eloop,
            Err(err) => return Err(Error::new(
                ErrorType::WinitEventLoopCreationFailed, 
                "Winit event loop creation failed".to_owned(),
                err.to_string()
            ))
        };

        let library = match VulkanLibrary::new() {
            Ok(lib) => lib,
            Err(err) => return Err(Error::new(
                ErrorType::VulkanMissing, 
                "System could not find vulkan. Make sure your system supports vulkan and your drivers are up-to-date".to_owned(),
                err.to_string()
            ))
        };

        let vk_instance = match Instance::new(library, InstanceCreateInfo {
            enabled_extensions: Surface::required_extensions(&winit_event_loop),
            ..Default::default()
        }) {
            Ok(instance) => instance,
            Err(err) => return Err(Error::new(
                ErrorType::FailedToCreateVulkanInstance, 
                "Vulkan instance creation failed".to_owned(),
                err.to_string()
            ))
        };

        Ok(Renderer {
            winit_event_loop,
            vk_instance
        })
    }
}