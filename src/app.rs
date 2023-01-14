use openxr as xr;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    }
};

use crate::{
    graphics::{
        vk_base::VkBase,
        vk_renderer::VkRenderer
    },
    Renderer,
    xr::{
        xr_base::XRBase,
        xr_renderer::XRRenderer,
    }
};

const VIEW_TYPE: xr::ViewConfigurationType = xr::ViewConfigurationType::PRIMARY_STEREO;

#[allow(dead_code)]
pub struct App {
    // Maintain order for proper resource destruction
    vk_renderer: VkRenderer,
    xr_renderer: XRRenderer,
    vk_base: Arc<VkBase>,
    xr_base: Arc<XRBase>,
}

impl App {
    pub fn new() -> Self {
        let xr_base = XRBase::new();
        let vk_base = VkBase::new(&xr_base.xr_instance, xr_base.system_id);

        let xr_renderer = XRRenderer::new(xr_base.clone(), &vk_base);
        let vk_renderer = VkRenderer::new(vk_base.clone(), &xr_renderer.swapchain);

        App {
            vk_renderer,
            xr_renderer,
            vk_base,
            xr_base,
        }
    }

    pub fn run(&mut self) {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::Relaxed);
        }).expect("Error setting Ctrl-C handler");

        'main: loop {
            if !running.load(Ordering::Relaxed) {
                match self.xr_renderer.session.request_exit() {
                    Ok(()) => {}
                    Err(xr::sys::Result::ERROR_SESSION_NOT_RUNNING) => break 'main,
                    Err(e) => panic!("{}", e),
                }
            }

            while let Some(event) = &self.xr_base.xr_instance.poll_event(&mut self.xr_renderer.event_storage).unwrap() {
                use xr::Event::*;
                match event {
                    SessionStateChanged(e) => {
                        println!("OpenXR session state change: {:?}", e.state());
                        match e.state() {
                            xr::SessionState::READY => {
                                self.xr_renderer.session.begin(VIEW_TYPE).unwrap();
                            }
                            xr::SessionState::STOPPING => {
                                self.xr_renderer.session.end().unwrap();
                            }
                            xr::SessionState::EXITING | xr::SessionState::LOSS_PENDING => {
                                break 'main;
                            }
                            _ => {}
                        }
                    }
                    InstanceLossPending(_) => {
                        break 'main;
                    }
                    _ => {}
                }
            }

            self.xr_renderer.update_frame(&mut self.vk_renderer);
        }

        println!("Clean exit");
    }
}
