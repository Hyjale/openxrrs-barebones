use ash::{vk::{self, Handle}};
use std::sync::{Arc};

use crate::graphics::{
    vk_instance::VkInstance
};

pub struct PhysicalDevice {
    pub handle: ash::vk::PhysicalDevice
}

impl PhysicalDevice {
    pub fn new(xr_instance: &openxr::Instance,
               vk_instance: &Arc<VkInstance>,
               system_id: openxr::SystemId
    ) -> Arc<PhysicalDevice> {
        let handle = vk::PhysicalDevice::from_raw(
            unsafe {
                xr_instance
                    .vulkan_graphics_device(system_id, vk_instance.handle.handle().as_raw() as _)
                    .unwrap() as _
            }
        );

        Arc::new(PhysicalDevice {
            handle
        })
    }
}
