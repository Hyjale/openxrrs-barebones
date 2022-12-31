use std::sync::{Arc};

use ash::{vk::{self, Handle}};

pub struct VkInstance {
    vk_instance: ash::Instance
}

impl VkInstance {
    pub fn new(xr_instance: &openxr::Instance,
               system_id: openxr::SystemId,
    ) -> Arc<VkInstance> {
        let entry = unsafe { ash::Entry::load().unwrap() };

        let api_version = vk::make_api_version(0, 1, 1, 0);
        let application_info = vk::ApplicationInfo::builder()
            .application_version(0)
            .engine_version(0)
            .api_version(api_version);

        let vk_instance = {
            unsafe {
                let vk_instance = xr_instance
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
                    vk::Instance::from_raw(vk_instance as _),
                )
            }
        };

        Arc::new(VkInstance {
            vk_instance: vk_instance
        })
    }

    pub fn get(&self) -> &ash::Instance {
        &self.vk_instance
    }

    pub fn destroy_instance(&self) {
        unsafe { self.vk_instance.destroy_instance(None); }
    }
}
