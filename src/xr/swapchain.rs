use ash::{vk::self};
use openxr as xr;

const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;
const VIEW_COUNT: u32 = 2;
const VIEW_TYPE: xr::ViewConfigurationType = xr::ViewConfigurationType::PRIMARY_STEREO;

pub struct Swapchain {
    pub resolution: vk::Extent2D,
    pub handle: xr::Swapchain<xr::Vulkan>,
}

impl Swapchain {
    pub fn new(instance: &openxr::Instance,
               system: openxr::SystemId,
               session: &openxr::Session<xr::Vulkan>,
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

        Swapchain {
            resolution,
            handle,
        }
    }
}
