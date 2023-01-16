#[derive(Clone, Debug, Copy)]
struct Vertex {
    pos: [f32; 4],
    color: [f32; 4],
}

use std::sync::{Arc};

use ash::{vk::{self}};

use xrrs::{
    graphics::{
        framebuffers::Framebuffers,
        pipeline::Pipeline,
        render_pass::RenderPass,
        vk_base::VkBase
    },
    Renderer,
    xr::{swapchain::Swapchain}
};

pub struct TriangleRenderer {
    renderpass: vk::RenderPass
}

impl Renderer for TriangleRenderer {
    fn new(vk_base: Arc<VkBase>, swapchain: &Swapchain) -> Self {
        unsafe {
            let renderpass_attachments = [
                vk::AttachmentDescription {
                    format: vk::Format::R8G8B8A8_SRGB,
                    samples: vk::SampleCountFlags::TYPE_1,
                    load_op: vk::AttachmentLoadOp::CLEAR,
                    store_op: vk::AttachmentStoreOp::STORE,
                    final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                    ..Default::default()
                },
                vk::AttachmentDescription {
                    format: vk::Format::D16_UNORM,
                    samples: vk::SampleCountFlags::TYPE_1,
                    load_op: vk::AttachmentLoadOp::CLEAR,
                    initial_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                    final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                    ..Default::default()
                },
            ];

            let color_attachment_references = [vk::AttachmentReference {
                attachment: 0,
                layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            }];
            let depth_attachment_reference = vk::AttachmentReference {
                attachment: 1,
                layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            };

            let subpass = [vk::SubpassDescription::builder()
                .color_attachments(&color_attachment_references)
                .depth_stencil_attachment(&depth_attachment_reference)
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .build()];

            let subpass_dependencies = [vk::SubpassDependency {
                src_subpass: vk::SUBPASS_EXTERNAL,
                src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
                    | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                ..Default::default()
            }];

            let renderpass_create_info = vk::RenderPassCreateInfo::builder()
                .attachments(&renderpass_attachments)
                .subpasses(&subpass)
                .dependencies(&subpass_dependencies);

            let renderpass = vk_base
                .device
                .handle
                .create_render_pass(&renderpass_create_info, None)
                .unwrap();

            TriangleRenderer {
                renderpass
            }
        }
    }

    fn draw(&mut self, swapchain: &mut Swapchain) {

    }
}

fn main() {
    println!("Hello world");
}
