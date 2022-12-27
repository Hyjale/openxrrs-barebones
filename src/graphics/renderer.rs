use std::sync::{Arc};

use crate::graphics::{
    instance::Instance,
    physical_device::PhysicalDevice
};

pub struct Renderer {
    instance: Arc<Instance>,
    physical_device: Arc<PhysicalDevice>,
}

impl Renderer {
    pub fn new(xr_instance: openxr::Instance, system_id: openxr::SystemId) -> Self {
        let instance = Instance::new(&xr_instance, system_id);

        let physical_device = PhysicalDevice::new(&xr_instance,
                                                  &instance.instance,
                                                  system_id
        );

        Renderer {
            instance: instance,
            physical_device: physical_device
        }
    }
}
