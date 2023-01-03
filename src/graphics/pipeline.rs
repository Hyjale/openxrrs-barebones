use ash::{vk::{self}};
use std::sync::{Arc};

use crate::graphics::{
    device::Device,
    render_pass::RenderPass,
    shader_module::ShaderModule
};

pub struct Pipeline {
    pipeline: ash::vk::Pipeline,
    pipeline_layout: ash::vk::PipelineLayout,
}

impl Pipeline {
    pub fn new(device: &Arc<Device>,
               render_pass: &Arc<RenderPass>
    ) -> Arc<Pipeline> {
        unsafe {
            let pipeline_layout = device
                .get()
                .create_pipeline_layout(
                    &vk::PipelineLayoutCreateInfo::builder().set_layouts(&[]),
                    None,
                )
                .unwrap();

            let noop_stencil_state = vk::StencilOpState {
                fail_op: vk::StencilOp::KEEP,
                pass_op: vk::StencilOp::KEEP,
                depth_fail_op: vk::StencilOp::KEEP,
                compare_op: vk::CompareOp::ALWAYS,
                compare_mask: 0,
                write_mask: 0,
                reference: 0,
            };

            let vert_module = ShaderModule::new(device, include_bytes!("triangle.vert.spv"));
            let frag_module = ShaderModule::new(device, include_bytes!("triangle.frag.spv"));

            let pipeline = device
                .get()
                .create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    &[vk::GraphicsPipelineCreateInfo::builder()
                        .stages(&[
                            vk::PipelineShaderStageCreateInfo {
                                stage: vk::ShaderStageFlags::VERTEX,
                                module: vert_module,
                                p_name: b"main\0".as_ptr() as _,
                                ..Default::default()
                            },
                            vk::PipelineShaderStageCreateInfo {
                                stage: vk::ShaderStageFlags::FRAGMENT,
                                module: frag_module,
                                p_name: b"main\0".as_ptr() as _,
                                ..Default::default()
                            },
                        ])
                        .vertex_input_state(&vk::PipelineVertexInputStateCreateInfo::default())
                        .input_assembly_state(
                            &vk::PipelineInputAssemblyStateCreateInfo::builder()
                                .topology(vk::PrimitiveTopology::TRIANGLE_LIST),
                        )
                        .viewport_state(
                            &vk::PipelineViewportStateCreateInfo::builder()
                                .scissor_count(1)
                                .viewport_count(1),
                        )
                        .rasterization_state(
                            &vk::PipelineRasterizationStateCreateInfo::builder()
                                .cull_mode(vk::CullModeFlags::NONE)
                                .polygon_mode(vk::PolygonMode::FILL)
                                .line_width(1.0),
                        )
                        .multisample_state(
                            &vk::PipelineMultisampleStateCreateInfo::builder()
                                .rasterization_samples(vk::SampleCountFlags::TYPE_1),
                        )
                        .depth_stencil_state(
                            &vk::PipelineDepthStencilStateCreateInfo::builder()
                                .depth_test_enable(false)
                                .depth_write_enable(false)
                                .front(noop_stencil_state)
                                .back(noop_stencil_state),
                        )
                        .color_blend_state(
                            &vk::PipelineColorBlendStateCreateInfo::builder().attachments(&[
                                vk::PipelineColorBlendAttachmentState {
                                    blend_enable: vk::TRUE,
                                    src_color_blend_factor: vk::BlendFactor::ONE,
                                    dst_color_blend_factor: vk::BlendFactor::ZERO,
                                    color_blend_op: vk::BlendOp::ADD,
                                    color_write_mask: vk::ColorComponentFlags::R
                                        | vk::ColorComponentFlags::G
                                        | vk::ColorComponentFlags::B,
                                    ..Default::default()
                                },
                            ]),
                        )
                        .dynamic_state(
                            &vk::PipelineDynamicStateCreateInfo::builder().dynamic_states(&[
                                vk::DynamicState::VIEWPORT,
                                vk::DynamicState::SCISSOR,
                            ]),
                        )
                        .layout(pipeline_layout)
                        .render_pass(render_pass.get())
                        .subpass(0)
                        .build()],
                    None,
                )
                .unwrap()[0];

            device.get().destroy_shader_module(vert_module, None);
            device.get().destroy_shader_module(frag_module, None);

            Arc::new(Pipeline {
                pipeline: pipeline,
                pipeline_layout: pipeline_layout,
            })
        }
    }

    pub fn get(&self) -> ash::vk::Pipeline {
        self.pipeline
    }

    pub fn pipeline_layout(&self) -> ash::vk::PipelineLayout {
        self.pipeline_layout
    }
}
