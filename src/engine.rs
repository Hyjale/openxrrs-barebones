use ash::{vk::{self, Handle}};
use openxr as xr;
use std::sync::{Arc};

use crate::graphics::{renderer::Renderer};
use crate::xr::{
    action::Action,
    space::Space,
    xr_instance::XRInstance,
    swapchain::Swapchain
};

pub struct Engine {
    xr_instance: Arc<XRInstance>,
    actions: Action,
    spaces: Space,
    renderer: Renderer,
    swapchain: Option<Swapchain>,
    event_storage: xr::EventDataBuffer,
    is_running: bool
}

impl Engine {
    pub fn new() -> Self {
        let swapchain = None;
        let event_storage = xr::EventDataBuffer::new();

        let xr_instance = XRInstance::new();
        let renderer = Renderer::new(&xr_instance.xr_instance, xr_instance.system_id);

        unsafe {
            let (session, mut frame_wait, mut frame_stream) = xr_instance.xr_instance
                .create_session::<xr::Vulkan>(
                    xr_instance.system_id,
                    &xr::vulkan::SessionCreateInfo {
                        instance: renderer.get_instance().handle().as_raw() as _,
                        physical_device: renderer.get_physical_device().as_raw() as _,
                        device: renderer.get_device().handle().as_raw() as _,
                        queue_family_index: renderer.device.queue_family_index,
                        queue_index: 0,
                    },
                )
                .unwrap();


            let actions = Action::new(&xr_instance.xr_instance, &session);
            let spaces = Space::new(&session);

            Engine {
                xr_instance: xr_instance,
                renderer: renderer,
                actions: actions,
                spaces: spaces,
                swapchain: swapchain,
                event_storage: event_storage,
                is_running: false
            }
        }
    }

    pub fn run() {

    }
}
