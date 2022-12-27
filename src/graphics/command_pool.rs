use ash::{vk::{self}};
use std::sync::{Arc};

pub struct CommandPool {
    pub command_pool: ash::vk::CommandPool
}

impl CommandPool {
    pub fn new(device: &ash::Device, queue_family_index: u32) -> Arc<CommandPool> {
        unsafe {
            let command_pool = device
                .create_command_pool(
                    &vk::CommandPoolCreateInfo::builder()
                        .queue_family_index(queue_family_index)
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
}
