use std::{io::Cursor};

use ash::{
    util::read_spv,
    vk::{self}
};

pub struct ShaderModule {}

impl ShaderModule {
    pub fn new(device: &ash::Device, path: &[u8]) -> ash::vk::ShaderModule {
        unsafe {
            let code = read_spv(&mut Cursor::new(path)).unwrap();

            let shader_module = device
                .create_shader_module(&vk::ShaderModuleCreateInfo::builder().code(&code), None)
                .unwrap();

            shader_module
        }
    }
}
