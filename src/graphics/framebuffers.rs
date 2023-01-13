use std::sync::{Arc};

use ash::{vk::{self, Handle}};

use crate::{
    graphics::{
        device::Device,
        render_pass::RenderPass
    },
    xr::{swapchain::Swapchain}
};

pub struct Framebuffer {
    pub framebuffer: vk::Framebuffer,
    pub color: vk::ImageView,
}

pub struct Framebuffers {
    pub handle: Vec<Framebuffer>
}

const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;
const VIEW_COUNT: u32 = 2;

impl Framebuffers {
    pub fn new(swapchain: &Swapchain,
               device: &Device,
               render_pass: &RenderPass,
    ) -> Arc<Framebuffers> {
        let images = swapchain.handle.enumerate_images().unwrap();

        let handle = images
            .into_iter()
            .map(|color_image| {
                let color_image = vk::Image::from_raw(color_image);
                unsafe {
                    let color = device
                        .handle
                        .create_image_view(
                            &vk::ImageViewCreateInfo::builder()
                                .image(color_image)
                                .view_type(vk::ImageViewType::TYPE_2D_ARRAY)
                                .format(COLOR_FORMAT)
                                .subresource_range(vk::ImageSubresourceRange {
                                    aspect_mask: vk::ImageAspectFlags::COLOR,
                                    base_mip_level: 0,
                                    level_count: 1,
                                    base_array_layer: 0,
                                    layer_count: VIEW_COUNT,
                                }),
                            None,
                        )
                        .unwrap();

                    let framebuffer = device
                        .handle
                        .create_framebuffer(
                            &vk::FramebufferCreateInfo::builder()
                                .render_pass(render_pass.handle)
                                .width(swapchain.resolution.width)
                                .height(swapchain.resolution.height)
                                .attachments(&[color])
                                .layers(1),
                            None,
                        )
                        .unwrap();
                    Framebuffer { framebuffer, color }
                }
            })
            .collect();

        Arc::new(Framebuffers{
            handle
        })
    }
}
