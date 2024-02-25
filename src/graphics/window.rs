use std::{error::Error, sync::Arc};

use vulkano::{device::Device, image::{view::ImageView, Image, ImageUsage}, instance::Instance, pipeline::graphics::viewport::Viewport, swapchain::{Surface, Swapchain, SwapchainCreateInfo}};
use winit::event_loop::EventLoop;

#[derive(Debug)]
pub struct AspenWindow {
    window: Arc<winit::window::Window>,
    surface: Arc<Surface>,
    present: Option<Present>,
    pub should_recreate_swapchain: bool,
}

#[derive(Debug)]
struct Present {
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<Image>>,
    image_views: Vec<Arc<ImageView>>,
    viewport: Viewport,
}

impl AspenWindow {
    pub fn new<T>(event_loop: &EventLoop<T>, instance: &Arc<Instance>) -> Result<AspenWindow, Box<dyn Error>> {
        let window = Arc::new(
            winit::window::WindowBuilder::new().build(event_loop)?
        );

        let surface = Surface::from_window(instance.clone(), window.clone())?;
        
        Ok(AspenWindow {
            window,
            surface,
            present: None,
            should_recreate_swapchain: true,
        })
    }

    pub fn set_recreate_swapchain(&mut self) {
        self.should_recreate_swapchain = true
    }

    pub fn recreate_swapchain(&mut self, device: &Arc<Device>) -> Result<(), Box<dyn Error>> {
        match self.present {
            None => {
                let (swapchain, images) = {
                    let surface_capabilities = device
                        .physical_device()
                        .surface_capabilities(&self.surface, Default::default())
                        .map_err(|err| {
                            WindowError::new(
                                WindowErrorType::GetSurfaceCapabilitiesFailed,
                                err
                            )
                        })?;
        
                    let image_format = device 
                        .physical_device()
                        .surface_formats(&self.surface, Default::default())
                        .map_err(|err| {
                            WindowError::new(
                                WindowErrorType::GetSurfaceFormatsFailed,
                                err
                            )
                        })?[0].0;
                        
                    Swapchain::new(
                        device.clone(), 
                        self.surface.clone(), 
                        SwapchainCreateInfo {
                            min_image_count: surface_capabilities.min_image_count.max(2),
                            image_format,
                            image_extent: self.window.inner_size().into(),
                            image_usage: ImageUsage::COLOR_ATTACHMENT,
                            composite_alpha: surface_capabilities
                                .supported_composite_alpha
                                .into_iter()
                                .next()
                                .ok_or(
                                    WindowError::new(
                                        WindowErrorType::GetWindowCompositeSurfaceFailed,
                                        "Failed to get the surface's supported composite alpha, whatever that means"
                                    )
                                )?,
                            ..Default::default()
                        }
                    ).map_err(|err| {
                        WindowError::new(
                            WindowErrorType::CreateSwapChainFailed,
                            err
                        )
                    })?
                };

                let (image_views, viewport) = AspenWindow::window_size_dependent_setup(&images);
    
                self.present = Some(Present {
                    swapchain, 
                    images,
                    image_views,
                    viewport,
                });
            },
            Some(ref mut present) => {
                let (image_views, viewport) = AspenWindow::window_size_dependent_setup(&present.images);

                present.image_views = image_views;
                present.viewport = viewport;
            }
        }

        Ok(())
    }

    fn window_size_dependent_setup(
        images: &[Arc<Image>],
    ) -> (Vec<Arc<ImageView>>, Viewport) {
        let extent = images[0].extent();
        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [extent[0] as f32, extent[1] as f32],
            depth_range: 0.0..=1.0,
        };
    
        (
            images
            .iter()
            .map(|image| ImageView::new_default(image.clone()).unwrap())
            .collect::<Vec<_>>(),
            viewport
        )
    }
    
    pub fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }

    pub fn winit_window(&self) -> &Arc<winit::window::Window> {
        &self.window
    }

    pub fn surface(&self) -> &Arc<Surface> {
        &self.surface
    }
}

#[derive(Debug)]
struct WindowError {
    error_type: WindowErrorType,
    message: String,
}

impl WindowError {
    fn new(error_type: WindowErrorType, message: impl std::string::ToString) -> WindowError {
        WindowError {
            error_type,
            message: message.to_string()
        }
    }
}

impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for WindowError {}

#[derive(Debug)]
enum WindowErrorType {
    GetSurfaceCapabilitiesFailed,
    GetSurfaceFormatsFailed,
    GetWindowCompositeSurfaceFailed,
    CreateSwapChainFailed,
}