use std::sync::{Arc};

use ash::{vk::{self, Handle}};

pub struct Instance {
    pub instance: ash::Instance
}

impl Instance {
    pub fn new(instance: &openxr::Instance,
               system_id: openxr::SystemId,
    ) -> Arc<Instance> {
        let entry = unsafe { ash::Entry::load().unwrap() };

        let api_version = vk::make_api_version(0, 1, 1, 0);
        let application_info = vk::ApplicationInfo::builder()
            .application_version(0)
            .engine_version(0)
            .api_version(api_version);

        let instance = {
            unsafe {
                let instance = instance
                    .create_vulkan_instance(
                        system_id,
                        std::mem::transmute(entry.static_fn().get_instance_proc_addr),
                        &vk::InstanceCreateInfo::builder().application_info(&application_info) as *const _
                            as *const _,
                    )
                    .expect("OpenXR error creating Vulkan instance")
                    .map_err(vk::Result::from_raw)
                    .expect("Vulkan error creating Vulkan instance");

                ash::Instance::load(
                    entry.static_fn(),
                    vk::Instance::from_raw(instance as _),
                )
            }
        };

        Arc::new(Instance {
            instance: instance
        })
    }
}
