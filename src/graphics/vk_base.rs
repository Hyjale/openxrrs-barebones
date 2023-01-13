use std::sync::{Arc};

use crate::graphics::{
    command_buffer::CommandBuffer,
    command_pool::CommandPool,
    device::Device,
    fence::Fence,
    vk_instance::VkInstance,
    physical_device::PhysicalDevice,
};

pub struct VkBase {
    pub command_buffers: Arc<CommandBuffer>,
    pub command_pool: Arc<CommandPool>,
    pub device: Arc<Device>,
    pub fences: Arc<Fence>,
    pub vk_instance: Arc<VkInstance>,
    pub physical_device: Arc<PhysicalDevice>,
}

impl VkBase {
    pub fn new(xr_instance: &openxr::Instance, system_id: openxr::SystemId) -> Arc<VkBase> {
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

        let command_pool = CommandPool::new(&device);

        let command_buffers = CommandBuffer::new(&device, &command_pool);

        let fences = Fence::new(&device);

        Arc::new(VkBase {
            command_buffers: command_buffers,
            command_pool: command_pool,
            device: device,
            fences: fences,
            vk_instance: vk_instance,
            physical_device: physical_device,
        })
    }
}

impl Drop for VkBase {
    fn drop(&mut self) {
        println!("Dropping VkBase");

        self.device.device_wait_idle();
        self.device.destroy_fences(&self.fences.handle);
        self.device.destroy_command_pool(self.command_pool.handle);
        self.device.destroy_device();
        self.vk_instance.destroy_instance();
    }
}
