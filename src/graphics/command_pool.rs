use ash::{vk::{self}};
use std::sync::{Arc};

use crate::graphics::{
    device::Device,
};

pub struct CommandPool {
    pub handle: ash::vk::CommandPool
}

impl CommandPool {
    pub fn new(device: &Arc<Device>) -> Arc<CommandPool> {
        unsafe {
            let handle = device
                .handle
                .create_command_pool(
                    &vk::CommandPoolCreateInfo::builder()
                        .queue_family_index(device.queue_family_index)
                        .flags(
                            vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER
                                | vk::CommandPoolCreateFlags::TRANSIENT,
                        ),
                    None,
                )
                .unwrap();

            Arc::new(CommandPool {
                handle
            })
        }
    }
}
