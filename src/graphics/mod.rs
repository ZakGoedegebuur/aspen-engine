use std::{
    error::Error, 
    sync::{Arc, Mutex}
};

use vulkano::{
    command_buffer::allocator::StandardCommandBufferAllocator, 
    device::{
        physical::{
            PhysicalDevice, 
            PhysicalDeviceType
        }, Device, DeviceCreateInfo, DeviceExtensions, Features, Queue, QueueCreateInfo, QueueFlags
    }, instance::{
        Instance, 
        InstanceCreateFlags, 
        InstanceCreateInfo
    }, 
    memory::allocator::{
        FreeListAllocator, 
        GenericMemoryAllocator, 
        StandardMemoryAllocator
    }, 
    swapchain::Surface, 
    Version, 
    VulkanLibrary
};

use winit::event_loop::EventLoop;

use crate::logging::AspenLogger;

use self::window::AspenWindow;

pub mod window;
pub mod shader;

#[allow(unused)]
pub struct Graphics {
    pub windows: Vec<Arc<Mutex<AspenWindow>>>,
    vk_lib: Arc<VulkanLibrary>,
    vk_instance: Arc<Instance>,
    vk_physical_device: Arc<PhysicalDevice>,
    pub vk_device: Arc<Device>,
    vk_graphics_queue: Arc<Queue>,
    vk_memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>
}

impl Graphics {
    pub fn new<T>(logger: &mut AspenLogger, event_loop: &EventLoop<T>) -> Result<Graphics, Box<dyn Error>> {
        let vk_lib = VulkanLibrary::new()
            .map_err(|err| { 
                let error = GraphicsError::new(
                    GraphicsErrorType::FailedToGetVKLibrary,
                    err,
                );
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
            let error = GraphicsError::new(
                GraphicsErrorType::FailedToCreateVKInstance,
                err
            );
            logger.log(error.to_string());
            error
        })?;

        let main_window = Arc::new(Mutex::new(AspenWindow::new(event_loop, &vk_instance).map_err(|err| {
            let error = GraphicsError::new(
                GraphicsErrorType::FailedToCreateMainWindow,
                err
            );
            logger.log(error.to_string());
            error
        })?));

        let mut vk_device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..Default::default()
        };

        let pdevices = vk_instance.enumerate_physical_devices().map_err(|err| {
            let error = GraphicsError::new(
                GraphicsErrorType::CouldNotEnumerateGraphicsDevices,
                err
            );
            logger.log(error.to_string());
            error
        })?;
        
        let (vk_physical_device, vk_queue_family_index) = {
            let p_opt = pdevices.filter(|p| { 
                p.api_version() >= Version::V1_3 || p.supported_extensions().khr_dynamic_rendering
            })
            .filter(|p| {
                p.supported_extensions().contains(&vk_device_extensions)
            })
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &main_window.lock().unwrap().surface()).unwrap_or(false)
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| {
                match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                    _ => 5,
                }
            });

            p_opt.ok_or(GraphicsError::new(
                GraphicsErrorType::NoSuitableGPU,
                "Could not find GPU with support for Vulkan dynamic rendering. Try updating your graphics drivers"
            ))
        }?;

        if vk_physical_device.api_version() < Version::V1_3 {
            vk_device_extensions.khr_dynamic_rendering = true;
        }

        println!(
            "Using device: {} (type: {:?})",
            vk_physical_device.properties().device_name,
            vk_physical_device.properties().device_type,
        );

        let (vk_device, mut vk_queues) = Device::new(
            vk_physical_device.clone(),
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index: vk_queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: vk_device_extensions,
                enabled_features: Features {
                    dynamic_rendering: true,
                    ..Features::empty()
                },
                ..Default::default()
            }
        ).map_err(|err| {
            GraphicsError::new(
                GraphicsErrorType::DeviceOrQueueCreationFailed,
                err
            )
        })?;

        let vk_graphics_queue = vk_queues.next().ok_or(GraphicsError::new(
            GraphicsErrorType::NoGraphicsQueueAvailable,
            "No vulkan queues available"
        ))?;

        let vk_memory_allocator = Arc::new(
            StandardMemoryAllocator::new_default(vk_device.clone())
        );

        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            vk_device.clone(),
            Default::default(),
        ));

        Ok(Graphics {
            windows: vec![main_window],
            vk_lib,
            vk_instance,
            vk_physical_device,
            vk_device,
            vk_graphics_queue,
            vk_memory_allocator,
            command_buffer_allocator,
        })
    }

    pub fn add_window<T>(&mut self, event_loop: &EventLoop<T>) -> Result<(), Box<dyn Error>> {
        self.windows.push(Arc::new(Mutex::new(AspenWindow::new(event_loop, &self.vk_instance)?)));
        Ok(())
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct GraphicsError {
    error_type: GraphicsErrorType,
    message: String,
}

impl GraphicsError {
    pub fn new(error_type: GraphicsErrorType, message: impl std::string::ToString) -> GraphicsError {
        GraphicsError {
            error_type,
            message: message.to_string(),
        }
    }
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
    CouldNotEnumerateGraphicsDevices,
    NoSuitableGPU,
    DeviceOrQueueCreationFailed,
    NoGraphicsQueueAvailable,
    GetSurfaceCapabilitiesFailed,
    GetSurfaceFormatsFailed
}