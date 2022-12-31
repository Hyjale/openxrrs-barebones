use ash::{vk::{self}};
use std::sync::{Arc};

use crate::graphics::{
    device::Device,
};

pub struct CommandPool {
    command_pool: ash::vk::CommandPool
}

impl CommandPool {
    pub fn new(device: &Arc<Device>) -> Arc<CommandPool> {
        unsafe {
            let command_pool = device
                .get()
                .create_command_pool(
                    &vk::CommandPoolCreateInfo::builder()
                        .queue_family_index(device.queue_family_index())
                        .flags(
                            vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER
                                | vk::CommandPoolCreateFlags::TRANSIENT,
                        ),
                    None,
                )
                .unwrap();

            Arc::new(CommandPool {
                command_pool: command_pool
            })
        }
    }

    pub fn get(&self) -> ash::vk::CommandPool {
        self.command_pool
    }
}
