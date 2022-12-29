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
                                                  &vk_instance.vk_instance,
                                                  system_id
        );

        let device = Device::new(&xr_instance,
                                 &vk_instance.vk_instance,
                                 physical_device.physical_device,
                                 system_id
        );

        let render_pass = RenderPass::new(&device.device);

        let pipeline = Pipeline::new(&device.device, render_pass.render_pass);

        let command_pool = CommandPool::new(&device.device, device.queue_family_index);

        let command_buffers = CommandBuffer::new(&device.device, command_pool.command_pool);

        let fences = Fence::new(&device.device);

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

    pub fn get_instance(&self) -> &ash::Instance {
        &self.vk_instance.vk_instance
    }

    pub fn get_physical_device(&self) -> ash::vk::PhysicalDevice {
        self.physical_device.physical_device
    }

    pub fn get_device(&self) -> &ash::Device {
        &self.device.device
    }

    pub fn get_render_pass(&self) -> ash::vk::RenderPass {
        self.render_pass.render_pass
    }

    pub fn get_fences(&self) -> &Vec<ash::vk::Fence> {
        &self.fences.fences
    }

    pub fn get_command_buffers(&self) -> &Vec<ash::vk::CommandBuffer> {
        &self.command_buffers.command_buffers
    }

    pub fn get_pipeline(&self) -> ash::vk::Pipeline {
        self.pipeline.pipeline
    }
}
