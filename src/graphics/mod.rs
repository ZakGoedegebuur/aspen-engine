use std::{error::Error, sync::Arc};

use vulkano::{instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, swapchain::Surface, VulkanLibrary};
use winit::event_loop::EventLoop;

use crate::logging::AspenLogger;

use self::window::AspenWindow;

pub mod window;

#[allow(unused)]
pub struct Graphics {
    pub windows: Vec<window::AspenWindow>,
    vk_lib: Arc<VulkanLibrary>,
    vk_instance: Arc<Instance>
}

impl Graphics {
    pub fn new(logger: &mut AspenLogger, event_loop: &EventLoop<()>) -> Result<Graphics, Box<dyn Error>> {
        let vk_lib = VulkanLibrary::new()
            .map_err(|err| { 
                let error = Box::new(GraphicsError {
                    error_type: GraphicsErrorType::FailedToGetVKLibrary,
                    message: err.to_string(),
                });
                logger.log(error.to_string());
                error
            })?;

        let vk_instance = Instance::new(
            vk_lib.clone(), 
            InstanceCreateInfo {
                enabled_extensions: Surface::required_extensions(event_loop),
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            }
        ).map_err(|err| {
            let error = Box::new(GraphicsError {
                error_type: GraphicsErrorType::FailedToCreateVKInstance,
                message: err.to_string()
            });
            logger.log(error.to_string());
            error
        })?;

        let main_window = AspenWindow::new(event_loop, &vk_instance).map_err(|err| {
            let error = Box::new(GraphicsError {
                error_type: GraphicsErrorType::FailedToCreateMainWindow,
                message: err.to_string()
            });
            logger.log(error.to_string());
            error
        })?;

        Ok(Graphics {
            windows: vec![main_window],
            vk_lib,
            vk_instance,
        })
    }

    pub fn add_window(&mut self, event_loop: &EventLoop<()>) -> Result<(), Box<dyn Error>> {
        self.windows.push(window::AspenWindow::new(event_loop, &self.vk_instance)?);
        Ok(())
    }
}

#[derive(Debug)]
pub struct GraphicsError {
    pub error_type: GraphicsErrorType,
    pub message: String,
}

impl std::fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)       
    }
}

impl Error for GraphicsError {}

#[derive(Debug)]
pub enum GraphicsErrorType {
    FailedToGetVKLibrary,
    FailedToCreateVKInstance,
    FailedToCreateMainWindow,
}