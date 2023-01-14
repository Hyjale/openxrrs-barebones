use std::sync::{Arc};

use ash::{vk::self};

use crate::{
    graphics::{
        framebuffers::Framebuffers,
        pipeline::Pipeline,
        render_pass::RenderPass,
        vk_base::VkBase
    },
    Renderer,
    xr::{swapchain::Swapchain}
};

pub struct VkRenderer {
    pub pipeline: Arc<Pipeline>,
    pub render_pass: Arc<RenderPass>,
    pub framebuffers: Arc<Framebuffers>,
    pub vk_base: Arc<VkBase>,
}

impl Renderer for VkRenderer {
    fn new(vk_base: Arc<VkBase>, swapchain: &Swapchain) -> Self {
        let render_pass = RenderPass::new(&vk_base.device);

        let pipeline = Pipeline::new(&vk_base.device, &render_pass);

        let framebuffers = Framebuffers::new(&swapchain, &vk_base.device, &render_pass);

        VkRenderer {
            pipeline,
            render_pass,
            framebuffers,
            vk_base,
        }
    }

    fn draw(&self,
                frame: usize,
                swapchain: &mut Swapchain
    ) {
        let cmd_buffer = self.vk_base.command_buffers.handle[frame];
        self.vk_base.device.begin_command_buffer(cmd_buffer);

        let image_index = swapchain.handle.acquire_image().unwrap();
        let framebuffer = self.framebuffers.handle[image_index as usize].framebuffer;
        self.vk_base.device.cmd_begin_render_pass(cmd_buffer,
                                          self.render_pass.handle,
                                          framebuffer,
                                          swapchain.resolution
        );
        let viewports = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: swapchain.resolution.width as f32,
            height: swapchain.resolution.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        let scissors = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swapchain.resolution,
        };
        self.vk_base.device.cmd_set_viewport_and_scissor(cmd_buffer, viewports, scissors);
        self.vk_base.device.cmd_bind_pipeline(cmd_buffer, self.pipeline.handle);
        self.vk_base.device.cmd_draw(cmd_buffer, 3, 1, 0, 0);
        self.vk_base.device.cmd_end_render_pass(cmd_buffer);

        self.vk_base.device.end_command_buffer(cmd_buffer);

        self.vk_base.device.queue_submit(cmd_buffer, self.vk_base.fences.handle[frame]);

        self.vk_base.device.wait_for_fences(&[self.vk_base.fences.handle[frame]].to_vec(), u64::MAX);
        self.vk_base.device.reset_fences(self.vk_base.fences.handle[frame]);
    }
}

impl Drop for VkRenderer {
    fn drop(&mut self) {
        println!("Dropping VkRenderer");

        self.vk_base.device.device_wait_idle();
        for framebuffer in &self.framebuffers.handle {
            self.vk_base.device.destroy_framebuffer(framebuffer.framebuffer);
            self.vk_base.device.destroy_image_view(framebuffer.color);
        }
        self.vk_base.device.destroy_render_pass(self.render_pass.handle);
        self.vk_base.device.destroy_pipeline(self.pipeline.handle);
        self.vk_base.device.destroy_pipeline_layout(self.pipeline.pipeline_layout);
    }
}
