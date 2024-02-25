use std::{error::Error, sync::Arc};

use vulkano::{
    device::Device, 
    pipeline::{
        graphics::{
            color_blend::{
                ColorBlendAttachmentState, 
                ColorBlendState
            }, 
            input_assembly::InputAssemblyState, 
            multisample::MultisampleState, 
            rasterization::RasterizationState, 
            subpass::PipelineRenderingCreateInfo, 
            vertex_input::{
                self, 
                VertexDefinition
            }, 
            viewport::ViewportState, 
            GraphicsPipelineCreateInfo
        }, 
        layout::PipelineDescriptorSetLayoutCreateInfo, 
        DynamicState, 
        GraphicsPipeline, 
        PipelineLayout, 
        PipelineShaderStageCreateInfo
    }, 
    swapchain::Swapchain
};

#[derive(Debug)]
pub struct ShaderProgram {
    pipeline: Arc<GraphicsPipeline>
}

impl ShaderProgram {
    pub fn new<Vertex>(device: &Arc<Device>, swapchain: &Arc<Swapchain>) -> Result<ShaderProgram, Box<dyn Error>> 
    where
        Vertex: vertex_input::Vertex
    {
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

        let pipeline = {
            let vs = vs::load(device.clone())
                .unwrap()
                .entry_point("main")
                .unwrap();
            let fs = fs::load(device.clone())
                .unwrap()
                .entry_point("main")
                .unwrap();

            let vertex_input_state = Vertex::per_vertex()
                .definition(&vs.info().input_interface)
                .unwrap();

            let stages = [
                PipelineShaderStageCreateInfo::new(vs),
                PipelineShaderStageCreateInfo::new(fs),
            ];

            let layout = PipelineLayout::new(
                device.clone(),
                PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                    .into_pipeline_layout_create_info(device.clone())
                    .unwrap(),
            )
            .unwrap();

            let subpass = PipelineRenderingCreateInfo {
                color_attachment_formats: vec![Some(swapchain.image_format())],
                ..Default::default()
            };

            GraphicsPipeline::new(
                device.clone(), 
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

        Ok(ShaderProgram {
            pipeline
        })
    }
}