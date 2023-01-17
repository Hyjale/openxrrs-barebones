use std::mem::align_of;
use std::sync::{Arc};

use ash::{vk::{self, Handle}};
use ash::util::*;

use xrrs::{
    graphics::{
        framebuffers::Framebuffers,
        pipeline::Pipeline,
        render_pass::RenderPass,
        vk_base::VkBase
    },
    Renderer,
    xr::{swapchain::Swapchain}
};

const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;
const DEPTH_FORMAT: vk::Format = vk::Format::D16_UNORM;
const VIEW_COUNT: u32 = 2;

pub fn find_memorytype_index(
    memory_req: &vk::MemoryRequirements,
    memory_prop: &vk::PhysicalDeviceMemoryProperties,
    flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    memory_prop.memory_types[..memory_prop.memory_type_count as _]
        .iter()
        .enumerate()
        .find(|(index, memory_type)| {
            (1 << index) & memory_req.memory_type_bits != 0
                && memory_type.property_flags & flags == flags
        })
        .map(|(index, _memory_type)| index as _)
}

#[derive(Clone, Debug, Copy)]
struct Vertex {
    pos: [f32; 4],
    color: [f32; 4],
}

struct Framebuffer {
    framebuffer: vk::Framebuffer,
    color_image_view: vk::ImageView,
    depth_image_view: vk::ImageView,
}

pub struct TriangleRenderer {
    renderpass: vk::RenderPass,
    framebuffers: Vec<Framebuffer>,
    index_buffer: vk::Buffer,
    index_buffer_memory: vk::DeviceMemory,
    vertex_input_buffer: vk::Buffer,
    vertex_input_buffer_memory: vk::DeviceMemory
}

impl Renderer for TriangleRenderer {
    fn new(vk_base: Arc<VkBase>, swapchain: &Swapchain) -> Self {
        unsafe {
            let renderpass_attachments = [
                vk::AttachmentDescription {
                    format: vk::Format::R8G8B8A8_SRGB,
                    samples: vk::SampleCountFlags::TYPE_1,
                    load_op: vk::AttachmentLoadOp::CLEAR,
                    store_op: vk::AttachmentStoreOp::STORE,
                    final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                    ..Default::default()
                },
                vk::AttachmentDescription {
                    format: DEPTH_FORMAT,
                    samples: vk::SampleCountFlags::TYPE_1,
                    load_op: vk::AttachmentLoadOp::CLEAR,
                    initial_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                    final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                    ..Default::default()
                },
            ];

            let color_attachment_references = [vk::AttachmentReference {
                attachment: 0,
                layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            }];
            let depth_attachment_reference = vk::AttachmentReference {
                attachment: 1,
                layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            };

            let subpass = [vk::SubpassDescription::builder()
                .color_attachments(&color_attachment_references)
                .depth_stencil_attachment(&depth_attachment_reference)
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .build()];

            let subpass_dependencies = [vk::SubpassDependency {
                src_subpass: vk::SUBPASS_EXTERNAL,
                src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
                    | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                ..Default::default()
            }];

            let view_mask = !(!0 << VIEW_COUNT);
            let renderpass_create_info = vk::RenderPassCreateInfo::builder()
                .attachments(&renderpass_attachments)
                .subpasses(&subpass)
                .dependencies(&subpass_dependencies)
                .push_next(
                    &mut vk::RenderPassMultiviewCreateInfo::builder()
                        .view_masks(&[view_mask])
                        .correlation_masks(&[view_mask]),
                )
                .build();

            let renderpass = vk_base
                .device
                .handle
                .create_render_pass(&renderpass_create_info, None)
                .unwrap();

            let images = swapchain.handle.enumerate_images().unwrap();

            let device_memory_properties = vk_base
                .vk_instance
                .handle
                .get_physical_device_memory_properties(vk_base.physical_device.handle);
            let depth_image_create_info = vk::ImageCreateInfo::builder()
                .image_type(vk::ImageType::TYPE_2D)
                .format(DEPTH_FORMAT)
                .extent(swapchain.resolution.into())
                .mip_levels(1)
                .array_layers(VIEW_COUNT)
                .samples(vk::SampleCountFlags::TYPE_1)
                .tiling(vk::ImageTiling::OPTIMAL)
                .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
                .sharing_mode(vk::SharingMode::EXCLUSIVE);
            let depth_image = vk_base.device.handle.create_image(&depth_image_create_info, None).unwrap();
            let depth_image_memory_requirements = vk_base.device.handle.get_image_memory_requirements(depth_image);
            let depth_image_memory_index = find_memorytype_index(
                &depth_image_memory_requirements,
                &device_memory_properties,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            ).expect("Error finding memory index for depth image");
            let depth_image_allocate_info = vk::MemoryAllocateInfo::builder()
                .allocation_size(depth_image_memory_requirements.size)
                .memory_type_index(depth_image_memory_index);
            let depth_image_memory = vk_base.device
                .handle
                .allocate_memory(&depth_image_allocate_info, None)
                .unwrap();
            vk_base.device
                   .handle
                   .bind_image_memory(depth_image, depth_image_memory, 0)
                   .expect("Error binding depth image memory");

            let depth_image_view_info = vk::ImageViewCreateInfo::builder()
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::DEPTH,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: VIEW_COUNT,
                })
                .image(depth_image)
                .format(depth_image_create_info.format)
                .view_type(vk::ImageViewType::TYPE_2D_ARRAY);
            let depth_image_view = vk_base.device
                                          .handle
                                          .create_image_view(&depth_image_view_info, None)
                                          .unwrap();

            let framebuffers: Vec<Framebuffer> = images
                .into_iter()
                .map(|color_image| {
                    let color_image = vk::Image::from_raw(color_image);
                    unsafe {
                        let color_image_view = vk_base.device
                            .handle
                            .create_image_view(
                                &vk::ImageViewCreateInfo::builder()
                                    .image(color_image)
                                    .view_type(vk::ImageViewType::TYPE_2D_ARRAY)
                                    .format(COLOR_FORMAT)
                                    .subresource_range(vk::ImageSubresourceRange {
                                        aspect_mask: vk::ImageAspectFlags::COLOR,
                                        base_mip_level: 0,
                                        level_count: 1,
                                        base_array_layer: 0,
                                        layer_count: VIEW_COUNT,
                                    }),
                                None,
                            )
                            .unwrap();

                        let framebuffer = vk_base.device
                            .handle
                            .create_framebuffer(
                                &vk::FramebufferCreateInfo::builder()
                                    .render_pass(renderpass)
                                    .width(swapchain.resolution.width)
                                    .height(swapchain.resolution.height)
                                    .attachments(&[color_image_view, depth_image_view])
                                    .layers(1),
                                None,
                            )
                            .unwrap();

                        Framebuffer { framebuffer, color_image_view, depth_image_view }
                    }
                })
                .collect();

            let index_buffer_data = [0u32, 1, 2];
            let index_buffer_info = vk::BufferCreateInfo::builder()
                .size(std::mem::size_of_val(&index_buffer_data) as u64)
                .usage(vk::BufferUsageFlags::INDEX_BUFFER)
                .sharing_mode(vk::SharingMode::EXCLUSIVE);
            let index_buffer = vk_base.device
                .handle
                .create_buffer(&index_buffer_info, None)
                .unwrap();
            let index_buffer_memory_requirements = vk_base.device
                .handle
                .get_buffer_memory_requirements(index_buffer);
            let index_buffer_memory_index = find_memorytype_index(
                &index_buffer_memory_requirements,
                &device_memory_properties,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
            ).expect("Error finding memory type for index buffer");
            let index_allocate_info = vk::MemoryAllocateInfo {
                allocation_size: index_buffer_memory_requirements.size,
                memory_type_index: index_buffer_memory_index,
                ..Default::default()
            };
            let index_buffer_memory = vk_base.device
                .handle
                .allocate_memory(&index_allocate_info, None)
                .unwrap();
            let index_ptr = vk_base.device
                .handle
                .map_memory(
                    index_buffer_memory,
                    0,
                    index_buffer_memory_requirements.size,
                    vk::MemoryMapFlags::empty()
                )
                .unwrap();
            let mut index_slice = Align::new(
                index_ptr,
                align_of::<u32> as u64,
                index_buffer_memory_requirements.size
            );
            index_slice.copy_from_slice(&index_buffer_data);
            vk_base.device.handle.unmap_memory(index_buffer_memory);
            vk_base.device
                .handle
                .bind_buffer_memory(index_buffer, index_buffer_memory, 0)
                .unwrap();

            let vertex_input_buffer_info = vk::BufferCreateInfo {
                size: 3 * std::mem::size_of::<Vertex>() as u64,
                usage: vk::BufferUsageFlags::VERTEX_BUFFER,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                ..Default::default()
            };

            let vertex_input_buffer = vk_base
                .device
                .handle
                .create_buffer(&vertex_input_buffer_info, None)
                .unwrap();

            let vertex_input_buffer_memory_requirements = vk_base
                .device
                .handle
                .get_buffer_memory_requirements(vertex_input_buffer);

            let vertex_input_buffer_memory_index = find_memorytype_index(
                &vertex_input_buffer_memory_requirements,
                &device_memory_properties,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            )
            .expect("Error finding memory type for vertex buffer.");

            let vertex_buffer_allocate_info = vk::MemoryAllocateInfo {
                allocation_size: vertex_input_buffer_memory_requirements.size,
                memory_type_index: vertex_input_buffer_memory_index,
                ..Default::default()
            };

            let vertex_input_buffer_memory = vk_base
                .device
                .handle
                .allocate_memory(&vertex_buffer_allocate_info, None)
                .unwrap();

            let vertices = [
                Vertex {
                    pos: [-1.0, 1.0, 0.0, 1.0],
                    color: [0.0, 1.0, 0.0, 1.0],
                },
                Vertex {
                    pos: [1.0, 1.0, 0.0, 1.0],
                    color: [0.0, 0.0, 1.0, 1.0],
                },
                Vertex {
                    pos: [0.0, -1.0, 0.0, 1.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                },
            ];

            let vert_ptr = vk_base
                .device
                .handle
                .map_memory(
                    vertex_input_buffer_memory,
                    0,
                    vertex_input_buffer_memory_requirements.size,
                    vk::MemoryMapFlags::empty(),
                )
                .unwrap();

            let mut vert_align = Align::new(
                vert_ptr,
                align_of::<Vertex>() as u64,
                vertex_input_buffer_memory_requirements.size,
            );
            vert_align.copy_from_slice(&vertices);
            vk_base.device.handle.unmap_memory(vertex_input_buffer_memory);
            vk_base.device
                .handle
                .bind_buffer_memory(vertex_input_buffer, vertex_input_buffer_memory, 0)
                .unwrap();


            TriangleRenderer {
                renderpass,
                framebuffers,
                index_buffer,
                index_buffer_memory,
                vertex_input_buffer,
                vertex_input_buffer_memory
            }
        }
    }

    fn draw(&mut self, swapchain: &mut Swapchain) {

    }
}

fn main() {
    println!("Hello world");
}
