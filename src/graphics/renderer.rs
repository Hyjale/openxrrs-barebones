use std::sync::{Arc};

use crate::graphics::{
    device::Device,
    instance::Instance,
    physical_device::PhysicalDevice,
    pipeline::Pipeline,
    render_pass::RenderPass
};

pub struct Renderer {
    device: Arc<Device>,
    instance: Arc<Instance>,
    physical_device: Arc<PhysicalDevice>,
    pipeline: Arc<Pipeline>,
    render_pass: Arc<RenderPass>,
}

impl Renderer {
    pub fn new(xr_instance: openxr::Instance, system_id: openxr::SystemId) -> Self {
        let instance = Instance::new(&xr_instance, system_id);

        let physical_device = PhysicalDevice::new(&xr_instance,
                                                  &instance.instance,
                                                  system_id
        );

        let device = Device::new(&xr_instance,
                                 &instance.instance,
                                 physical_device.physical_device,
                                 system_id
        );

        let render_pass = RenderPass::new(&device.device);

        let pipeline = Pipeline::new(&device.device, render_pass.render_pass);

        Renderer {
            device: device,
            instance: instance,
            physical_device: physical_device,
            pipeline: pipeline,
            render_pass: render_pass,
        }
    }
}
