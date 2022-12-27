use ash::{vk::{self}};
use std::sync::{Arc};

const PIPELINE_DEPTH: u32 = 2;

pub struct CommandBuffer {
    command_buffer: Vec<ash::vk::CommandBuffer>
}

impl CommandBuffer {
    pub fn new(device: &ash::Device, command_pool: ash::vk::CommandPool) -> Arc<CommandBuffer> {
        unsafe {
            let command_buffers = device
                .allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo::builder()
                        .command_pool(command_pool)
                        .command_buffer_count(PIPELINE_DEPTH),
                )
                .unwrap();

            Arc::new(CommandBuffer {
                command_buffer: command_buffers
            })
        }
    }
}
