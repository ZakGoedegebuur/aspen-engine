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

use crate::error::{
    Error,
    ErrorType
};

#[derive(Debug)]
pub struct Aspen {
    event_loop: EventLoop<()>,
    instance: Arc<Instance>,
    main_window: Arc<Window>,
    main_window_surface: Arc<Surface>
}

impl Aspen {
    pub fn new() -> Result<Aspen, Error> {
        let event_loop = match EventLoop::new() {
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

        let instance = match Instance::new(library, InstanceCreateInfo {
            enabled_extensions: Surface::required_extensions(&event_loop),
            ..Default::default()
        }) {
            Ok(instance) => instance,
            Err(err) => return Err(Error::new(
                ErrorType::FailedToCreateVulkanInstance, 
                "Vulkan instance creation failed".to_owned(),
                err.to_string(),
            ))
        };

        //return Err(Error::new(
        //    ErrorType::VulkanMissing, 
        //    "Test error".to_owned(),
        //    "Generic failure".to_owned(),
        //));

        let main_window = Arc::new(match WindowBuilder::new().build(&event_loop) {
            Ok(val) => val,
            Err(err) => return Err(Error::new(
                ErrorType::WinitWindowCreationFailed,
                "Winit main window creation failed".to_owned(),
                err.to_string(),
            ))
        });

        let main_window_surface = match Surface::from_window(instance.clone(), main_window.clone()) {
            Ok(val) => val,
            Err(err) => return Err(Error::new(
                ErrorType::SurfaceCreationFailed, 
                "Surface creation failed".to_owned(), 
                err.to_string(),
            ))
        };

        Ok(Aspen {
            event_loop,
            instance,
            main_window,
            main_window_surface,
        })
    }

    pub fn run(self) {
        let mut close_requested = false;
        
        self.event_loop.run(|event, elwt| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        close_requested = true;
                    },
                    _ => {}
                },
                _ => {}
            }

            if close_requested {
                elwt.exit();
            }
        });
    }
}