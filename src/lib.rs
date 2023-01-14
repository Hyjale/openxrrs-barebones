use std::sync::{Arc};

use crate::graphics::{vk_base::VkBase};
use crate::xr::{swapchain::Swapchain};

pub mod app;
pub mod graphics;
pub mod xr;

pub trait Renderer {
    fn new(vk_base: Arc<VkBase>, swapchain: &Swapchain) -> Self;

    fn draw(&self, frame: usize, swapchain: &mut Swapchain);
}
