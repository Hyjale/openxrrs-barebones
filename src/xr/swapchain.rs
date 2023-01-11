use ash::{vk::{self, Handle}};
use openxr as xr;

use crate::graphics::{
    framebuffer::Framebuffer
};

const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;
const VIEW_COUNT: u32 = 2;
const VIEW_TYPE: xr::ViewConfigurationType = xr::ViewConfigurationType::PRIMARY_STEREO;

pub struct Swapchain {
    pub resolution: vk::Extent2D,
    pub handle: xr::Swapchain<xr::Vulkan>,
    pub framebuffers: Vec<Framebuffer>
}

impl Swapchain {
    pub fn new(instance: &openxr::Instance,
               device: &ash::Device,
               session: &openxr::Session<xr::Vulkan>,
               system: openxr::SystemId,
               render_pass: ash::vk::RenderPass
    ) -> Swapchain {
        let views = instance
            .enumerate_view_configuration_views(system, VIEW_TYPE)
            .unwrap();

        let resolution = vk::Extent2D {
            width: views[0].recommended_image_rect_width,
            height: views[0].recommended_image_rect_height,
        };

        let handle = session
            .create_swapchain(&xr::SwapchainCreateInfo {
                create_flags: xr::SwapchainCreateFlags::EMPTY,
                usage_flags: xr::SwapchainUsageFlags::COLOR_ATTACHMENT
                    | xr::SwapchainUsageFlags::SAMPLED,
                format: COLOR_FORMAT.as_raw() as _,
                sample_count: 1,
                width: resolution.width,
                height: resolution.height,
                face_count: 1,
                array_size: VIEW_COUNT,
                mip_count: 1,
            })
            .unwrap();

        let images = handle.enumerate_images().unwrap();

        let framebuffers = images
            .into_iter()
            .map(|color_image| {
                let color_image = vk::Image::from_raw(color_image);
                unsafe {
                    let color = device
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
                        .create_framebuffer(
                            &vk::FramebufferCreateInfo::builder()
                                .render_pass(render_pass)
                                .width(resolution.width)
                                .height(resolution.height)
                                .attachments(&[color])
                                .layers(1),
                            None,
                        )
                        .unwrap();
                    Framebuffer { framebuffer, color }
                }
            })
            .collect();

        Swapchain {
            resolution,
            handle,
            framebuffers
        }
    }
}
