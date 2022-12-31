use ash::{vk::{self}};
use std::sync::{Arc};

use crate::graphics::{
    device::Device
};

const PIPELINE_DEPTH: u32 = 2;

pub struct Fence {
    fences: Vec<ash::vk::Fence>
}

impl Fence {
    pub fn new(device: &Arc<Device>) -> Arc<Fence> {
        unsafe {
            let fences = (0..PIPELINE_DEPTH)
                .map(|_| {
                    device
                        .get()
                        .create_fence(
                            &vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED),
                            None,
                        )
                        .unwrap()
                })
                .collect::<Vec<_>>();

            Arc::new(Fence {
                fences: fences
            })
        }
    }

    pub fn get(&self) -> &Vec<ash::vk::Fence> {
        &self.fences
    }
}
