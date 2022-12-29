use openxr as xr;

pub struct Space {
    pub stage_space: openxr::Space
}

impl Space {
    pub fn new(session: &openxr::Session<xr::Vulkan>,
    ) -> Self {
        let stage_space = session
            .create_reference_space(xr::ReferenceSpaceType::STAGE, xr::Posef::IDENTITY)
            .unwrap();

        Self {
            stage_space: stage_space
        }
    }
}
