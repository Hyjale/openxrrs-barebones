use std::sync::{Arc};

use ash::{vk::Handle};
use openxr as xr;

use crate::{
    xr::{
        action::Action,
        space::Space,
        xr_base::XRBase,
        swapchain::Swapchain
    },
    Renderer,
    graphics::{
        vk_base::VkBase,
        vk_renderer::VkRenderer
    }
};

const VIEW_TYPE: xr::ViewConfigurationType = xr::ViewConfigurationType::PRIMARY_STEREO;

pub struct XRRenderer {
    pub xr_base: Arc<XRBase>,
    pub session: openxr::Session<xr::Vulkan>,
    pub frame_wait: openxr::FrameWaiter,
    pub frame_stream: openxr::FrameStream<xr::Vulkan>,
    pub event_storage: openxr::EventDataBuffer,
    pub environment_blend_mode: openxr::EnvironmentBlendMode,
    pub swapchain: Swapchain,
    pub actions: Action,
    pub spaces: Space,
}

impl XRRenderer {
    pub fn new(xr_base: Arc<XRBase>, vk_base: &VkBase) -> Self {
        unsafe {
            let (session, frame_wait, frame_stream) = xr_base.xr_instance
                .create_session::<xr::Vulkan>(
                    xr_base.system_id,
                    &xr::vulkan::SessionCreateInfo {
                        instance: vk_base.vk_instance.handle.handle().as_raw() as _,
                        physical_device: vk_base.physical_device.handle.as_raw() as _,
                        device: vk_base.device.handle.handle().as_raw() as _,
                        queue_family_index: vk_base.device.queue_family_index,
                        queue_index: 0,
                    },
                )
                .unwrap();

            let swapchain = Swapchain::new(&xr_base.xr_instance,
                                           xr_base.system_id,
                                           &session
            );

            let actions = Action::new(&xr_base.xr_instance, &session);
            let spaces = Space::new(&session);

            let event_storage = xr::EventDataBuffer::new();

            let environment_blend_mode = xr_base
                .xr_instance
                .enumerate_environment_blend_modes(xr_base.system_id, VIEW_TYPE)
                .unwrap()[0];

            XRRenderer {
                xr_base,
                session,
                frame_wait,
                frame_stream,
                event_storage,
                environment_blend_mode,
                swapchain,
                actions,
                spaces,
            }
        }
    }

    pub fn update_frame(&mut self, vk_renderer: &mut VkRenderer) {
        let xr_frame_state = self.frame_wait.wait().unwrap();
        self.frame_stream.begin().unwrap();

        if !xr_frame_state.should_render {
            self.frame_stream
                .end(
                    xr_frame_state.predicted_display_time,
                    self.environment_blend_mode,
                    &[],
                )
                .unwrap();

            return;
        }

        vk_renderer.draw(&mut self.swapchain);

        self.swapchain.handle.wait_image(xr::Duration::INFINITE).unwrap();
        self.swapchain.handle.release_image().unwrap();

        self.session.sync_actions(&[(&self.actions.action_set).into()]).unwrap();
        let (_, views) = self.session
            .locate_views(VIEW_TYPE, xr_frame_state.predicted_display_time, &self.spaces.stage_space)
            .unwrap();

        let rect = xr::Rect2Di {
            offset: xr::Offset2Di { x: 0, y: 0 },
            extent: xr::Extent2Di {
                width: self.swapchain.resolution.width as _,
                height: self.swapchain.resolution.height as _,
            },
        };
        let left_subimage = xr::SwapchainSubImage::new()
            .swapchain(&self.swapchain.handle)
            .image_array_index(0)
            .image_rect(rect);
        let right_subimage = xr::SwapchainSubImage::new()
            .swapchain(&self.swapchain.handle)
            .image_array_index(1)
            .image_rect(rect);
        let left_projection_view = xr::CompositionLayerProjectionView::new()
            .pose(views[0].pose)
            .fov(views[0].fov)
            .sub_image(left_subimage);
        let right_projection_view = xr::CompositionLayerProjectionView::new()
            .pose(views[1].pose)
            .fov(views[1].fov)
            .sub_image(right_subimage);
        let projection_views = [left_projection_view, right_projection_view];
        let projection = xr::CompositionLayerProjection::new()
            .space(&self.spaces.stage_space)
            .views(&projection_views);

        self.frame_stream
            .end(
                xr_frame_state.predicted_display_time,
                self.environment_blend_mode,
                &[&projection],
            )
            .unwrap();
    }
}

impl Drop for XRRenderer {
    fn drop(&mut self) {
        println!("Dropping XRRenderer");
    }
}
