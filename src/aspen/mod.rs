pub mod error;

use error::{
    Error,
    ErrorType
};

use std::sync::Arc;

use vulkano::{
    VulkanLibrary,
    instance::{
        Instance, 
        InstanceCreateInfo, InstanceCreateFlags
    },
    swapchain::{
        Surface, 
        Swapchain, 
        SwapchainCreateInfo, acquire_next_image, SwapchainPresentInfo
    }, 
    device::{
        DeviceExtensions, 
        QueueFlags, 
        physical::{PhysicalDeviceType, PhysicalDevice}, 
        Device, 
        DeviceCreateInfo, 
        QueueCreateInfo, 
        Features, Queue
    }, 
    Version, 
    image::{ImageUsage, Image, view::ImageView}, 
    memory::allocator::{
        StandardMemoryAllocator, 
        AllocationCreateInfo, 
        MemoryTypeFilter
    }, 
    buffer::{
        BufferContents, 
        Buffer, 
        BufferCreateInfo, 
        BufferUsage, Subbuffer
    }, 
    pipeline::{
        graphics::{
            vertex_input::{
                Vertex,
                VertexDefinition
            }, 
            subpass::PipelineRenderingCreateInfo, 
            GraphicsPipelineCreateInfo, 
            input_assembly::InputAssemblyState, 
            viewport::{
                ViewportState, 
                Viewport
            }, 
            rasterization::RasterizationState, 
            multisample::MultisampleState, 
            color_blend::{
                ColorBlendState, 
                ColorBlendAttachmentState
            }
        }, 
        PipelineShaderStageCreateInfo, 
        PipelineLayout, 
        layout::PipelineDescriptorSetLayoutCreateInfo, 
        GraphicsPipeline, 
        DynamicState
    }, 
    command_buffer::{allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage, RenderingInfo, RenderingAttachmentInfo}, 
    sync,
    sync::GpuFuture, Validated, VulkanError, render_pass::{AttachmentLoadOp, AttachmentStoreOp}
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
    vk_physical_device: Arc<PhysicalDevice>,
    vk_device: Arc<Device>,
    vk_command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    vk_graphics_queue: Arc<Queue>,

    // Index '0' should always be the main window if the app is not windowless
    window: WindowWrapper, 
    graphics_pipelines: Vec<Arc<GraphicsPipeline>>,
    vertex_buffers: Vec<Subbuffer<[Vertex2D]>>,
}

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct Vertex2D {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}

struct WindowWrapper {
    window: Arc<Window>,
    surface: Arc<Surface>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<Image>>,
    image_views: Vec<Arc<ImageView>>,
    recreate_swapchain: bool,
    prev_frame_end: Option<Box<dyn GpuFuture>>,
    viewport: Viewport,
}

impl Framework {
    pub fn new() -> Result<Framework, error::Error> {
        let event_loop = match EventLoop::new() {
            Ok(eloop) => eloop,
            Err(err) => return Err(Error::new(
                ErrorType::EventLoopCreationFailed, 
                err.to_string()
            ))
        };

        let vk_library = match VulkanLibrary::new() {
            Ok(lib) => lib,
            Err(_) => return Err(Error::new(
                ErrorType::VulkanMissing,
                "System could not find Vulkan. Make sure your system supports Vulkan and your drivers are up-to-date.".to_owned(),
            ))
        };

        let vk_instance = match Instance::new(
            vk_library.clone(), 
            InstanceCreateInfo {
            enabled_extensions: Surface::required_extensions(&event_loop),
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        }) {
            Ok(instance) => instance,
            Err(err) => return Err(Error::new(
                ErrorType::VulkanInstanceCreationFailed, 
                err.to_string(),
            ))
        };

        let main_window = Arc::new(match WindowBuilder::new().build(&event_loop) {
            Ok(val) => val,
            Err(err) => return Err(Error::new(
                ErrorType::WindowCreationFailed,
                err.to_string(),
            ))
        });

        let main_window_surface = match Surface::from_window(vk_instance.clone(), main_window.clone()) {
            Ok(val) => val,
            Err(err) => return Err(Error::new(
                ErrorType::VulkanSurfaceCreationFailed, 
                err.to_string(),
            ))
        };

        let mut vk_device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..Default::default()
        };

        let pdevices = vk_instance.enumerate_physical_devices();
        let (vk_physical_device, vk_queue_family_index) = match pdevices {
            Err(err) => return Err(Error::new(
                ErrorType::VulkanPhysicalDeviceEnumerationFailed, 
                err.to_string()
            )),
            Ok(pdevices) => {
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
                            // We select a queue family that supports graphics operations. When drawing to
                            // a window surface, as we do in this example, we also need to check that
                            // queues in this queue family are capable of presenting images to the surface.
                            q.queue_flags.intersects(QueueFlags::GRAPHICS)
                                && p.surface_support(i as u32, &main_window_surface).unwrap_or(false)
                        })
                        // The code here searches for the first queue family that is suitable. If none is
                        // found, `None` is returned to `filter_map`, which disqualifies this physical
                        // device.
                        .map(|i| (p, i as u32))
                })
                .min_by_key(|(p, _)| {
                    // We assign a lower score to device types that are likely to be faster/better.
                    match p.properties().device_type {
                        PhysicalDeviceType::DiscreteGpu => 0,
                        PhysicalDeviceType::IntegratedGpu => 1,
                        PhysicalDeviceType::VirtualGpu => 2,
                        PhysicalDeviceType::Cpu => 3,
                        PhysicalDeviceType::Other => 4,
                        _ => 5,
                    }
                });

                match p_opt {
                    None => return Err(Error::new(
                        ErrorType::NoSuitableVulkanPhysicalDevices,
                        "No suitable GPU found. Try updating your graphics drivers".to_owned()
                    )),
                    Some(p) => p
                }
            }
        };

        if vk_physical_device.api_version() < Version::V1_3 {
            vk_device_extensions.khr_dynamic_rendering = true;
        }

        //println!(
        //    "Using device: {} (type: {:?})",
        //    vk_physical_device.properties().device_name,
        //    vk_physical_device.properties().device_type,
        //);

        let (vk_device, mut vk_queues) = match Device::new(
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
        ) {
            Ok(d) => d,
            Err(err) => return Err(Error::new(
                ErrorType::VulkanDeviceCreationFailed, 
                err.to_string()
            ))
        };

        let vk_graphics_queue = vk_queues.next().expect("this should never panic");
        
        let (vk_swapchain, vk_images) = {
            let surface_capabilities = match vk_device
                .physical_device()
                .surface_capabilities(&main_window_surface, Default::default()) {
                    Ok(sc) => sc,
                    Err(err) => return Err(Error::new(
                        ErrorType::GetSurfaceCapabilitiesFailed,
                        err.to_string()
                    ))
            };

            let image_format = match vk_device 
                .physical_device()
                .surface_formats(&main_window_surface, Default::default()) {
                    Ok(f) => f,
                    Err(err) => return Err(Error::new(
                        ErrorType::GetSurfaceFormatFailed, 
                        err.to_string()
                    ))
            }[0].0;

            match Swapchain::new(
                vk_device.clone(), 
                main_window_surface.clone(), 
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count.max(2),
                    image_format,
                    image_extent: main_window.inner_size().into(),
                    image_usage: ImageUsage::COLOR_ATTACHMENT,
                    composite_alpha: match surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next() {
                            Some(sca) => sca,
                            None => return Err(Error::new(
                                ErrorType::GetSurfaceCompositeAlphaFailed, 
                                "Failed to get the surface's supported composite alpha, whatever that means".to_owned()
                        ))  
                        },
                    ..Default::default()
                }
            ) {
                Ok(sc) => sc,
                Err(err) => return Err(Error::new(
                    ErrorType::VulkanSwapchainCreationFailed, 
                err.to_string()
            ))
            }
        };

        let vk_memory_allocator = Arc::new(StandardMemoryAllocator::new_default(vk_device.clone()));

        let vertices = [
            Vertex2D {
                position: [-0.5, -0.5],
            },
            Vertex2D {
                position: [0.5, -0.5],
            },
            Vertex2D {
                position: [0.0, 0.5],
            },
        ];

        let vertex_buffer = Buffer::from_iter(
            vk_memory_allocator,
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            vertices,
        )
        .expect("abstract this later");

        mod vs {
            vulkano_shaders::shader! {
                ty: "vertex",
                src: r"
                    #version 450
    
                    layout(location = 0) in vec2 position;
    
                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                    }
                ",
            }
        }
    
        mod fs {
            vulkano_shaders::shader! {
                ty: "fragment",
                src: r"
                    #version 450
    
                    layout(location = 0) out vec4 f_color;
    
                    void main() {
                        f_color = vec4(1.0, 0.0, 0.0, 1.0);
                    }
                ",
            }
        }

        let vk_def_pipeline = {
            let vs = vs::load(vk_device.clone())
                .unwrap()
                .entry_point("main")
                .unwrap();
            let fs = fs::load(vk_device.clone())
                .unwrap()
                .entry_point("main")
                .unwrap();

            let vertex_input_state = Vertex2D::per_vertex()
                .definition(&vs.info().input_interface)
                .unwrap();

            let stages = [
                PipelineShaderStageCreateInfo::new(vs),
                PipelineShaderStageCreateInfo::new(fs),
            ];

            let layout = PipelineLayout::new(
                vk_device.clone(),
                PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                    .into_pipeline_layout_create_info(vk_device.clone())
                    .unwrap(),
            )
            .unwrap();

            let subpass = PipelineRenderingCreateInfo {
                color_attachment_formats: vec![Some(vk_swapchain.image_format())],
                ..Default::default()
            };

            GraphicsPipeline::new(
                vk_device.clone(), 
                None, 
                GraphicsPipelineCreateInfo {
                    stages: stages.into_iter().collect(),
                    vertex_input_state: Some(vertex_input_state),
                    input_assembly_state: Some(InputAssemblyState::default()),
                    viewport_state: Some(ViewportState::default()),
                    rasterization_state: Some(RasterizationState::default()),
                    multisample_state: Some(MultisampleState::default()),
                    color_blend_state: Some(ColorBlendState::with_attachment_states(
                        subpass.color_attachment_formats.len() as u32,
                        ColorBlendAttachmentState::default()
                    )),
                    dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                    subpass: Some(subpass.into()),
                    ..GraphicsPipelineCreateInfo::layout(layout)
                }
            ).expect("pipeline creation failed")
        };

        let mut viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [0.0, 0.0],
            depth_range: 0.0..=1.0,
        };

        let attachment_image_views = window_size_dependent_setup(&vk_images, &mut viewport);

        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            vk_device.clone(),
            Default::default(),
        ));

        Ok(Framework {
            event_loop,
            vk_library: vk_library.clone(),
            vk_instance: vk_instance.clone(),
            vk_physical_device: vk_physical_device.clone(),
            vk_device: vk_device.clone(),
            vk_command_buffer_allocator: command_buffer_allocator.clone(),
            vk_graphics_queue: vk_graphics_queue.clone(),
            window: WindowWrapper {
                window: main_window,
                surface: main_window_surface,
                swapchain: vk_swapchain,
                images: vk_images,
                image_views: attachment_image_views,
                recreate_swapchain: false,
                prev_frame_end: Some(sync::now(vk_device.clone()).boxed()),
                viewport,
            },
            graphics_pipelines: vec![vk_def_pipeline],
            vertex_buffers: vec![vertex_buffer],
        })
    }

    pub fn run(mut self) -> Result<(), ()> {
        let _ = self.event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent { 
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    elwt.exit();
                },
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    self.window.recreate_swapchain = true;
                }
                Event::WindowEvent { 
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    println!("redraw requested!");
                    let image_extent: [u32; 2] = self.window.window.inner_size().into();

                    if image_extent.contains(&0) {
                        return;
                    }

                    self.window.prev_frame_end.as_mut().unwrap().cleanup_finished();

                    if self.window.recreate_swapchain {
                        let (new_swapchain, new_images) = self.window.swapchain
                            .recreate(SwapchainCreateInfo {
                                image_extent,
                                ..self.window.swapchain.create_info()
                            })
                            .expect("failed to recreate swapchain");
    
                        self.window.swapchain = new_swapchain;

                        self.window.image_views =
                            window_size_dependent_setup(&new_images, &mut self.window.viewport);
                        
                        self.window.recreate_swapchain = false;
                    }

                    let (image_index, suboptimal, acquire_future) =
                    match acquire_next_image(self.window.swapchain.clone(), None).map_err(Validated::unwrap) {
                        Ok(r) => r,
                        Err(VulkanError::OutOfDate) => {
                            self.window.recreate_swapchain = true;
                            return;
                        }
                        Err(e) => panic!("failed to acquire next image: {e}"),
                    };

                    if suboptimal {
                        self.window.recreate_swapchain = true;
                    }

                    let mut builder = AutoCommandBufferBuilder::primary(
                        &self.vk_command_buffer_allocator, 
                        self.vk_graphics_queue.queue_family_index(), 
                        CommandBufferUsage::OneTimeSubmit,
                    ).unwrap();

                    builder
                        .begin_rendering(
                            RenderingInfo {
                                color_attachments: vec![
                                    Some(RenderingAttachmentInfo {
                                        load_op: AttachmentLoadOp::Clear,
                                        store_op: AttachmentStoreOp::Store,
                                        clear_value: Some([0.0, 0.0, 1.0, 1.0].into()),
                                        ..RenderingAttachmentInfo::image_view(
                                            self.window.image_views[image_index as usize].clone()
                                        )
                                    })
                                ],
                                ..Default::default()
                            }
                        ).unwrap()
                        .set_viewport(0, [self.window.viewport.clone()].into_iter().collect())
                        .unwrap()
                        .bind_pipeline_graphics(self.graphics_pipelines[0].clone())
                        .unwrap()
                        .bind_vertex_buffers(0, self.vertex_buffers[0].clone())
                        .unwrap()
                        .draw(self.vertex_buffers[0].len() as u32, 1, 0, 0)
                        .unwrap()
                        .end_rendering()
                        .unwrap();
                    
                    let command_buffer = builder.build().unwrap();

                    let future = self.window.prev_frame_end
                        .take()
                        .unwrap()
                        .join(acquire_future)
                        .then_execute(self.vk_graphics_queue.clone(), command_buffer)
                        .unwrap()
                        // The color output is now expected to contain our triangle. But in order to
                        // show it on the screen, we have to *present* the image by calling
                        // `then_swapchain_present`.
                        //
                        // This function does not actually present the image immediately. Instead it
                        // submits a present command at the end of the queue. This means that it will
                        // only be presented once the GPU has finished executing the command buffer
                        // that draws the triangle.
                        .then_swapchain_present(
                            self.vk_graphics_queue.clone(),
                            SwapchainPresentInfo::swapchain_image_index(self.window.swapchain.clone(), image_index),
                        )
                        .then_signal_fence_and_flush();

                    match future.map_err(Validated::unwrap) {
                        Ok(future) => {
                            self.window.prev_frame_end = Some(future.boxed());
                        }
                        Err(VulkanError::OutOfDate) => {
                            self.window.recreate_swapchain = true;
                            self.window.prev_frame_end = Some(sync::now(self.vk_device.clone()).boxed());
                        }
                        Err(e) => {
                            println!("failed to flush future: {e}");
                            self.window.prev_frame_end = Some(sync::now(self.vk_device.clone()).boxed());
                        }
                    }
                }
                //Event::AboutToWait => self.window.window.request_redraw(),
                _ => (),
            }
        });

        Ok(())
    }
}

fn window_size_dependent_setup(
    images: &[Arc<Image>],
    viewport: &mut Viewport,
) -> Vec<Arc<ImageView>> {
    let extent = images[0].extent();
    viewport.extent = [extent[0] as f32, extent[1] as f32];

    images
        .iter()
        .map(|image| ImageView::new_default(image.clone()).unwrap())
        .collect::<Vec<_>>()
}