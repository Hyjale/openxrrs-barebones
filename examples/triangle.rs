#[derive(Clone, Debug, Copy)]
struct Vertex {
    pos: [f32; 4],
    color: [f32; 4],
}

use std::sync::{Arc};

use ash::{vk::{self}};

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

pub struct TriangleRenderer {

}

impl Renderer for TriangleRenderer {
    fn new(vk_base: Arc<VkBase>, swapchain: &Swapchain) -> Self {

        TriangleRenderer {

        }
    }

    fn draw(&mut self, swapchain: &mut Swapchain) {

    }
}

fn main() {
    println!("Hello world");
}
