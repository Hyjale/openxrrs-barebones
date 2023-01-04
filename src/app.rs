use ash::{vk::Handle};
use openxr as xr;
use std::{sync::Arc};

use crate::{
    graphics::{renderer::Renderer},
    xr::{
        action::Action,
        space::Space,
        xr_instance::XRInstance,
        swapchain::Swapchain
    }
};

const PIPELINE_DEPTH: u32 = 2;
const VIEW_TYPE: xr::ViewConfigurationType = xr::ViewConfigurationType::PRIMARY_STEREO;

pub struct App {
    xr_instance: Arc<XRInstance>,
    renderer: Renderer,
    swapchain: Option<Swapchain>,
    frame: usize,
}

impl App {
    pub fn new() -> Self {
        let xr_instance = XRInstance::new();
        let renderer = Renderer::new(&xr_instance.xr_instance, xr_instance.system_id);

        App {
            xr_instance: xr_instance,
            renderer: renderer,
            swapchain: None,
            frame: 0
        }
    }

    pub fn run(&mut self) -> Result<bool, bool>{
        unsafe {
            let (session, mut frame_wait, mut frame_stream) = self.xr_instance.xr_instance
                .create_session::<xr::Vulkan>(
                    self.xr_instance.system_id,
                    &xr::vulkan::SessionCreateInfo {
                        instance: self.renderer.vk_instance.handle.handle().as_raw() as _,
                        physical_device: self.renderer.physical_device.handle.as_raw() as _,
                        device: self.renderer.device.handle.handle().as_raw() as _,
                        queue_family_index: self.renderer.device.queue_family_index,
                        queue_index: 0,
                    },
                )
                .unwrap();

            let actions = Action::new(&self.xr_instance.xr_instance, &session);
            let spaces = Space::new(&session);

            'main: loop {
                if !self.update_frame(&session, &spaces, &actions, &mut frame_wait, &mut frame_stream)? {
                    break 'main;
                }
            }

            drop((
                session,
                frame_wait,
                frame_stream,
                spaces,
                actions
            ));

            if let Some(swapchain) = &self.swapchain {
                for framebuffer in &swapchain.framebuffers {
                    self.renderer.device.destroy_framebuffer(framebuffer.framebuffer);
                    self.renderer.device.destroy_image_view(framebuffer.color);
                }
            }
            self.renderer.destroy();

            Ok(true)
        }
    }

    fn update_frame(&mut self,
                    session: &openxr::Session<xr::Vulkan>,
                    spaces: &Space,
                    actions: &Action,
                    frame_wait: &mut openxr::FrameWaiter,
                    frame_stream: &mut openxr::FrameStream<xr::Vulkan>
    ) -> Result<bool, bool> {
        if !self.handle_session_events(&session)? {
            return Err(false)
        }

        let xr_frame_state = frame_wait.wait().unwrap();
        frame_stream.begin().unwrap();

        if !xr_frame_state.should_render {
            frame_stream
                .end(
                    xr_frame_state.predicted_display_time,
                    self.xr_instance.environment_blend_mode,
                    &[],
                )
                .unwrap();

            return Ok(true);
        }

        let swapchain = self.swapchain.get_or_insert_with(|| {
            Swapchain::new(&self.xr_instance.xr_instance,
                            &self.renderer.device.handle,
                            &session,
                            self.xr_instance.system_id,
                            self.renderer.render_pass.handle
            )
        });

        // Render
        let image_index = swapchain.handle.acquire_image().unwrap();
        self.renderer.draw(self.frame,
                            swapchain.framebuffers[image_index as usize].framebuffer,
                            swapchain.resolution
        );

        swapchain.handle.wait_image(xr::Duration::INFINITE).unwrap();
        swapchain.handle.release_image().unwrap();

        session.sync_actions(&[(&actions.action_set).into()]).unwrap();
        let (_, views) = session
            .locate_views(VIEW_TYPE, xr_frame_state.predicted_display_time, &spaces.stage_space)
            .unwrap();

        let rect = xr::Rect2Di {
            offset: xr::Offset2Di { x: 0, y: 0 },
            extent: xr::Extent2Di {
                width: swapchain.resolution.width as _,
                height: swapchain.resolution.height as _,
            },
        };
        let left_subimage = xr::SwapchainSubImage::new()
            .swapchain(&swapchain.handle)
            .image_array_index(0)
            .image_rect(rect);
        let right_subimage = xr::SwapchainSubImage::new()
            .swapchain(&swapchain.handle)
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
            .space(&spaces.stage_space)
            .views(&projection_views);

        frame_stream
            .end(
                xr_frame_state.predicted_display_time,
                self.xr_instance.environment_blend_mode,
                &[&projection],
            )
            .unwrap();

        self.frame = (self.frame + 1) % PIPELINE_DEPTH as usize;

        Ok(true)
    }

    fn handle_session_events(&self, session: &openxr::Session<xr::Vulkan>) -> Result<bool, bool> {
        let mut buffer = xr::EventDataBuffer::new();
        while let Some(event) = self.xr_instance.xr_instance.poll_event(&mut buffer).unwrap() {
            use xr::Event::*;

            match event {
                SessionStateChanged(e) => {
                    match e.state() {
                        xr::SessionState::READY => {
                            session.begin(VIEW_TYPE).unwrap();
                            return Ok(true);
                        }
                        xr::SessionState::STOPPING => {
                            session.end().unwrap();
                        }
                        xr::SessionState::EXITING | xr::SessionState::LOSS_PENDING => {
                            return Err(false);
                        }
                        _ => println!(
                            "OpenXR session state change: {:?}", e.state()
                        ),
                    }
                }
                InstanceLossPending(_) => {
                    return Err(false);
                }
                _ => {},
            }
        }

        Ok(true)
    }
}
