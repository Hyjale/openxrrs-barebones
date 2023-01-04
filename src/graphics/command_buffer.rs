use ash::{vk::{self}};
use std::sync::{Arc};

use crate::graphics::{
    device::Device,
    command_pool::CommandPool
};

const PIPELINE_DEPTH: u32 = 2;

pub struct CommandBuffer {
    pub handle: Vec<ash::vk::CommandBuffer>
}

impl CommandBuffer {
    pub fn new(device: &Arc<Device>, command_pool: &Arc<CommandPool>) -> Arc<CommandBuffer> {
        unsafe {
            let handle = device
                .handle
                .allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo::builder()
                        .command_pool(command_pool.handle)
                        .command_buffer_count(PIPELINE_DEPTH),
                )
                .unwrap();

            Arc::new(CommandBuffer {
                handle
            })
        }
    }
}
