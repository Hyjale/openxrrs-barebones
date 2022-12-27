use ash::{vk::{self}};
use std::sync::{Arc};

const PIPELINE_DEPTH: u32 = 2;

pub struct Fence {
    fence: Vec<ash::vk::Fence>
}

impl Fence {
    pub fn new(device: &ash::Device) -> Arc<Fence> {
        unsafe {
            let fence = (0..PIPELINE_DEPTH)
                .map(|_| {
                    device
                        .create_fence(
                            &vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED),
                            None,
                        )
                        .unwrap()
                })
                .collect::<Vec<_>>();

            Arc::new(Fence {
                fence: fence
            })
        }
    }
}
