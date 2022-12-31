use ash::{vk::{self}};
use std::sync::{Arc};

use crate::graphics::{
    device::Device,
    command_pool::CommandPool
};

const PIPELINE_DEPTH: u32 = 2;

pub struct CommandBuffer {
    pub command_buffers: Vec<ash::vk::CommandBuffer>
}

impl CommandBuffer {
    pub fn new(device: &Arc<Device>, command_pool: &Arc<CommandPool>) -> Arc<CommandBuffer> {
        unsafe {
            let command_buffers = device
                .get()
                .allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo::builder()
                        .command_pool(command_pool.get())
                        .command_buffer_count(PIPELINE_DEPTH),
                )
                .unwrap();

            Arc::new(CommandBuffer {
                command_buffers: command_buffers
            })
        }
    }

    pub fn get(&self) -> &Vec<ash::vk::CommandBuffer> {
        &self.command_buffers
    }
}
