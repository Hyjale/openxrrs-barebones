use std::sync::{Arc};

use crate::graphics::{
    command_buffer::CommandBuffer,
    command_pool::CommandPool,
    device::Device,
    fence::Fence,
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
}
