pub mod error;

use std::sync::Arc;

use vulkano::{
    VulkanLibrary,
    instance::{
        Instance, 
        InstanceCreateInfo
    },
    swapchain::Surface
};

use winit::{
    event_loop::{
        EventLoop,
        ControlFlow,
    },
    event::{
        Event,
        WindowEvent,
    },
    window::{
        Window,
        WindowBuilder
    }
};

pub struct Framework {
    event_loop: EventLoop<()>,
    vk_library: Arc<VulkanLibrary>,
    vk_instance: Arc<Instance>,
    // Index '0' should always be the main window if the app is not windowless
    windows: Vec<WindowSurfacePair>, 
}

struct WindowSurfacePair {
    window: Arc<Window>,
    surface: Arc<Surface>
}

impl Framework {
    pub fn new() -> Result<Framework, error::Error> {
        let event_loop = match EventLoop::new() {
            Ok(eloop) => eloop,
            Err(err) => return Err(error::Error::new(
                error::ErrorType::EventLoopCreationFailed, 
                err.to_string()
            ))
        };

        let vk_library = match VulkanLibrary::new() {
            Ok(lib) => lib,
            Err(_) => return Err(error::Error::new(
                error::ErrorType::VulkanMissing,
                "System could not find vulkan. Make sure your system supports vulkan and your drivers are up-to-date".to_owned(),
            ))
        };

        let vk_instance = match Instance::new(vk_library.clone(), InstanceCreateInfo {
            enabled_extensions: Surface::required_extensions(&event_loop),
            ..Default::default()
        }) {
            Ok(instance) => instance,
            Err(err) => return Err(error::Error::new(
                error::ErrorType::VulkanInstanceCreationFailed, 
                err.to_string(),
            ))
        };

        let main_window = Arc::new(match WindowBuilder::new().build(&event_loop) {
            Ok(val) => val,
            Err(err) => return Err(error::Error::new(
                error::ErrorType::WindowCreationFailed,
                err.to_string(),
            ))
        });

        let main_window_surface = match Surface::from_window(vk_instance.clone(), main_window.clone()) {
            Ok(val) => val,
            Err(err) => return Err(error::Error::new(
                error::ErrorType::VulkanSurfaceCreationFailed, 
                err.to_string(),
            ))
        };

        Ok(Framework {
            event_loop,
            vk_library: vk_library.clone(),
            vk_instance,
            windows: vec![ WindowSurfacePair {
                window: main_window,
                surface: main_window_surface
            } ]
        })
    }

    pub fn run(&self) -> Result<(), ()> {


        Ok(())
    }
}