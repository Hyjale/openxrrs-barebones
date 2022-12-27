use ash::{vk::{self, Handle}};
use std::sync::{Arc};

pub struct PhysicalDevice {
    pub physical_device: ash::vk::PhysicalDevice
}

impl PhysicalDevice {
    pub fn new(xr_instance: &openxr::Instance,
               instance: &ash::Instance,
               system_id: openxr::SystemId
    ) -> Arc<PhysicalDevice> {
        let physical_device = vk::PhysicalDevice::from_raw(
            unsafe {
                xr_instance
                    .vulkan_graphics_device(system_id, instance.handle().as_raw() as _)
                    .unwrap() as _
            }
        );

        Arc::new(PhysicalDevice {
            physical_device: physical_device
        })
    }
}
