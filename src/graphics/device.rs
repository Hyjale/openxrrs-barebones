use std::sync::{Arc};

use ash::{vk::{self, Handle}};

pub struct Device {
    pub device: ash::Device,
    pub queue: ash::vk::Queue,
    pub queue_family_index: u32
}

impl Device {
    pub fn new(xr_instance: &openxr::Instance,
               vk_instance: &ash::Instance,
               physical_device: ash::vk::PhysicalDevice,
               system_id: openxr::SystemId,
    ) -> Arc<Device> {
        unsafe {
            let entry = ash::Entry::load().unwrap();

            let queue_family_index = vk_instance
                .get_physical_device_queue_family_properties(physical_device)
                .into_iter()
                .enumerate()
                .find_map(|(queue_family_index, info)| {
                    if info.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                        Some(queue_family_index as u32)
                    } else {
                        None
                    }
                })
                .unwrap();

            let device = {
                let device_queue_create_info = [vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(queue_family_index)
                    .queue_priorities(&[1.0])
                    .build()];

                let mut multiview_features = vk::PhysicalDeviceMultiviewFeatures {
                    multiview: vk::TRUE,
                    ..Default::default()
                };

                let device_create_info = vk::DeviceCreateInfo::builder()
                    .queue_create_infos(&device_queue_create_info)
                    .push_next(&mut multiview_features);

                let device = xr_instance
                    .create_vulkan_device(
                        system_id,
                        std::mem::transmute(entry.static_fn().get_instance_proc_addr),
                        physical_device.as_raw() as _,
                        &device_create_info as *const _ as *const _,
                    )
                    .expect("OpenXR error creating Vulkan device")
                    .map_err(vk::Result::from_raw)
                    .expect("Vulkan error creating Vulkan device");

                ash::Device::load(vk_instance.fp_v1_0(), vk::Device::from_raw(device as _))
            };

            let queue = device.get_device_queue(queue_family_index, 0);

            Arc::new(Device {
                device: device,
                queue: queue,
                queue_family_index: queue_family_index
            })
        }
    }

    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }

    pub fn begin_command_buffer(&self, cmd_buffer: ash::vk::CommandBuffer) {
        unsafe {
            self.device
                .begin_command_buffer(
                    cmd_buffer,
                    &vk::CommandBufferBeginInfo::builder()
                        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
                )
                .unwrap()
        }
    }

    pub fn end_command_buffer(&self, cmd_buffer: ash::vk::CommandBuffer) {
        unsafe {
            self.device
                .end_command_buffer(cmd_buffer)
                .unwrap()
        }
    }

    pub fn cmd_begin_render_pass(&self,
                             cmd_buffer: ash::vk::CommandBuffer,
                             render_pass: ash::vk::RenderPass,
                             framebuffer: ash::vk::Framebuffer,
                             extent: ash::vk::Extent2D
    ) {
        unsafe {
            self.device.cmd_begin_render_pass(
                cmd_buffer,
                &vk::RenderPassBeginInfo::builder()
                    .render_pass(render_pass)
                    .framebuffer(framebuffer)
                    .render_area(vk::Rect2D {
                        offset: vk::Offset2D::default(),
                        extent: extent,
                    })
                    .clear_values(&[vk::ClearValue {
                        color: vk::ClearColorValue {
                            float32: [0.0, 0.0, 0.0, 1.0],
                        },
                    }]),
                vk::SubpassContents::INLINE,
            );
        }
    }

    pub fn cmd_end_render_pass(&self, cmd_buffer: ash::vk::CommandBuffer) {
        unsafe { self.device.cmd_end_render_pass(cmd_buffer); }
    }

    pub fn wait_for_fences(&self, fences: &Vec<ash::vk::Fence>, timeout: u64) {
        unsafe {
            self.device
                .wait_for_fences(fences, true, timeout)
                .unwrap();
        }
    }

    pub fn reset_fences(&self, fences: ash::vk::Fence) {
        unsafe {
            self.device.reset_fences(&[fences]).unwrap();
        }
    }

    pub fn cmd_set_viewport_and_scissor(&self,
                                        cmd_buffer: ash::vk::CommandBuffer,
                                        viewport: ash::vk::Viewport,
                                        scissor: ash::vk::Rect2D
    ) {
        unsafe {
            self.device.cmd_set_viewport(cmd_buffer, 0, &[viewport]);
            self.device.cmd_set_scissor(cmd_buffer, 0, &[scissor]);
        }
    }

    pub fn cmd_bind_pipeline(&self,
                             cmd_buffer: ash::vk::CommandBuffer,
                             pipeline: ash::vk::Pipeline
    ) {
        unsafe {
            self.device.cmd_bind_pipeline(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        }
    }

    pub fn cmd_draw(&self,
                    cmd_buffer: ash::vk::CommandBuffer,
                    vertex_count: u32,
                    instance_count: u32,
                    first_vertex: u32,
                    first_instance: u32,
    ) {
        unsafe {
            self.device.cmd_draw(cmd_buffer,
                                 vertex_count,
                                 instance_count,
                                 first_vertex,
                                 first_instance
            );
        }
    }

    pub fn queue_submit(&self,
                        cmd_buffer: ash::vk::CommandBuffer,
                        fence: ash::vk::Fence
    ) {
        unsafe {
            self.device
                .queue_submit(
                    self.queue,
                    &[vk::SubmitInfo::builder().command_buffers(&[cmd_buffer]).build()],
                    fence
                )
                .unwrap()
        }
    }

    pub fn destroy_fence(&self, fence: ash::vk::Fence) {
        unsafe {
            self.device.destroy_fence(fence, None);
        }
    }

    pub fn destroy_framebuffer(&self, framebuffer: ash::vk::Framebuffer) {
        unsafe {
            self.device.destroy_framebuffer(framebuffer, None);
        }
    }

    pub fn destroy_image_view(&self, image_view: ash::vk::ImageView) {
        unsafe {
            self.device.destroy_image_view(image_view, None);
        }
    }

    pub fn destroy_pipeline(&self, pipeline: ash::vk::Pipeline) {
        unsafe {
            self.device.destroy_pipeline(pipeline, None);
        }
    }

    pub fn destroy_pipeline_layout(&self, pipeline_layout: ash::vk::PipelineLayout) {
        unsafe {
            self.device.destroy_pipeline_layout(pipeline_layout, None);
        }
    }

    pub fn destroy_command_pool(&self, cmd_pool: ash::vk::CommandPool) {
        unsafe {
            self.device.destroy_command_pool(cmd_pool, None);
        }
    }

    pub fn destroy_render_pass(&self, render_pass: ash::vk::RenderPass) {
        unsafe {
            self.device.destroy_render_pass(render_pass, None);
        }
    }

    pub fn destroy_device(&self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
