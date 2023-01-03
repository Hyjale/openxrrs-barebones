use std::sync::{Arc};

use ash::{vk::{self, Handle}};

use crate::graphics::{
    command_buffer::CommandBuffer,
    command_pool::CommandPool,
    device::Device,
    fence::Fence,
    framebuffer::Framebuffer,
    vk_instance::VkInstance,
    physical_device::PhysicalDevice,
    pipeline::Pipeline,
    render_pass::RenderPass
};

pub struct Renderer {
    pub command_buffers: Arc<CommandBuffer>,
    pub command_pool: Arc<CommandPool>,
    pub device: Arc<Device>,
    pub fences: Arc<Fence>,
    pub vk_instance: Arc<VkInstance>,
    pub physical_device: Arc<PhysicalDevice>,
    pub pipeline: Arc<Pipeline>,
    pub render_pass: Arc<RenderPass>,
}

impl Renderer {
    pub fn new(xr_instance: &openxr::Instance, system_id: openxr::SystemId) -> Self {
        let vk_instance = VkInstance::new(&xr_instance, system_id);

        let physical_device = PhysicalDevice::new(&xr_instance,
                                                  &vk_instance,
                                                  system_id
        );

        let device = Device::new(&xr_instance,
                                 &vk_instance,
                                 &physical_device,
                                 system_id
        );

        let render_pass = RenderPass::new(&device);

        let pipeline = Pipeline::new(&device, &render_pass);

        let command_pool = CommandPool::new(&device);

        let command_buffers = CommandBuffer::new(&device, &command_pool);

        let fences = Fence::new(&device);

        Renderer {
            command_buffers: command_buffers,
            command_pool: command_pool,
            device: device,
            fences: fences,
            vk_instance: vk_instance,
            physical_device: physical_device,
            pipeline: pipeline,
            render_pass: render_pass,
        }
    }

    pub fn draw(&self,
                frame: usize,
                framebuffer: ash::vk::Framebuffer,
                resolution: ash::vk::Extent2D
    ) {
        let cmd_buffer = self.command_buffers.get()[frame];
        self.device.begin_command_buffer(cmd_buffer);

        self.device.cmd_begin_render_pass(cmd_buffer,
                                          self.render_pass.get(),
                                          framebuffer,
                                          resolution
        );
        let viewports = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: resolution.width as f32,
            height: resolution.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        let scissors = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: resolution,
        };
        self.device.cmd_set_viewport_and_scissor(cmd_buffer, viewports, scissors);
        self.device.cmd_bind_pipeline(cmd_buffer, self.pipeline.get());
        self.device.cmd_draw(cmd_buffer, 3, 1, 0, 0);
        self.device.cmd_end_render_pass(cmd_buffer);

        self.device.end_command_buffer(cmd_buffer);

        self.device.queue_submit(cmd_buffer, self.fences.get()[frame]);

        self.device.wait_for_fences(&[self.fences.get()[frame]].to_vec(), u64::MAX);
        self.device.reset_fences(self.fences.get()[frame]);
    }

    pub fn destroy(&self) {
        self.device.wait_for_fences(self.fences.get(), !0);

        self.device.destroy_fences(self.fences.get());
        self.device.destroy_pipeline(self.pipeline.get());
        self.device.destroy_pipeline_layout(self.pipeline.pipeline_layout());
        self.device.destroy_command_pool(self.command_pool.get());
        self.device.destroy_render_pass(self.render_pass.get());
        self.device.destroy_device();
        self.vk_instance.destroy_instance();
    }
}
