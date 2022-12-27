use std::sync::{Arc};

use ash::{vk::{self, Handle}};

pub struct Device {
    device: ash::Device,
    queue: ash::vk::Queue,
    queue_family_index: u32
}

impl Device {
    pub fn new(xr_instance: &openxr::Instance,
               vk_instance: &ash::Instance,
               physical_device: ash::vk::PhysicalDevice,
               system: openxr::SystemId,
    ) -> Arc<Device> {
        unsafe {
            let entry = ash::Entry::load().unwrap();

            let device = {
                let device_queue_create_info = [vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(queue_family_index)
                    .queue_priorities(&[1.0])
                    .build()];

                let mut multiview_features = vk::PhysicalDeviceMultiviewFeatures {
                    multiview: vk::TRUE,
                    ..Default::default()
                };

                let device_create_info = vk::DeviceCreateInfo::builder()
                    .queue_create_infos(&device_queue_create_info)
                    .push_next(&mut multiview_features);

                let device = xr_instance
                    .create_vulkan_device(
                        system,
                        std::mem::transmute(entry.static_fn().get_instance_proc_addr),
                        physical_device.as_raw() as _,
                        &device_create_info as *const _ as *const _,
                    )
                    .expect("OpenXR error creating Vulkan device")
                    .map_err(vk::Result::from_raw)
                    .expect("Vulkan error creating Vulkan device");

                ash::Device::load(vk_instance.fp_v1_0(), vk::Device::from_raw(device as _))
            };

            let queue = device.get_device_queue(queue_family_index, 0);

            let queue_family_index = vk_instance
                .get_physical_device_queue_family_properties(physical_device)
                .into_iter()
                .enumerate()
                .find_map(|(queue_family_index, info)| {
                    if info.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                        Some(queue_family_index as u32)
                    } else {
                        None
                    }
                })
                .unwrap();


            Arc::new(Device {
                device: device,
                queue: queue,
                queue_family_index: queue_family_index
            })
        }
    }

    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }

    pub fn begin_command_buffer(&self, cmd_buffer: ash::vk::CommandBuffer) {
        unsafe {
            self.device
                .begin_command_buffer(
                    cmd_buffer,
                    &vk::CommandBufferBeginInfo::builder()
                        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
                )
                .expect("Begin command buffer failed");
        }
    }

    pub fn end_command_buffer(&self, cmd_buffer: ash::vk::CommandBuffer) {
        unsafe {
            self.device
                .end_command_buffer(cmd_buffer)
                .expect("End command buffer failed");
        }
    }

    pub fn queue_submit(&self,
                        queue: ash::vk::Queue,
                        cmd_buffer: ash::vk::CommandBuffer,
                        fence: ash::vk::Fence

    ) {
        unsafe {
            self.device
                .queue_submit(
                    queue,
                    &[vk::SubmitInfo::builder().command_buffers(&[cmd_buffer]).build()],
                    fence
                )
                .expect("Queue submit failed")
        }
    }
}
