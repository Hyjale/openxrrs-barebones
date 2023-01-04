use ash::{vk::{self}};
use std::sync::{Arc};

use crate::graphics::{
    device::Device
};

const PIPELINE_DEPTH: u32 = 2;

pub struct Fence {
    pub handle: Vec<ash::vk::Fence>
}

impl Fence {
    pub fn new(device: &Arc<Device>) -> Arc<Fence> {
        unsafe {
            let handle = (0..PIPELINE_DEPTH)
                .map(|_| {
                    device
                        .handle
                        .create_fence(
                            &vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED),
                            None,
                        )
                        .unwrap()
                })
                .collect::<Vec<_>>();

            Arc::new(Fence {
                handle
            })
        }
    }
}
