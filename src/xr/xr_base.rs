use openxr as xr;
use std::sync::{Arc};

pub struct XRBase {
    pub xr_instance: openxr::Instance,
    pub system_id: openxr::SystemId,
}

impl XRBase {
    pub fn new() -> Arc<XRBase> {
        #[cfg(feature = "static")]
        let entry = xr::Entry::linked();
        #[cfg(not(feature = "static"))]
        let entry = unsafe {
            xr::Entry::load()
                .expect("Error finding OpenXR loader.")
        };

        #[cfg(target_os = "android")]
        entry.initialize_android_loader().unwrap();

        let mut extensions = xr::ExtensionSet::default();
        extensions.khr_vulkan_enable2 = true;

        #[cfg(target_os = "android")]
        {
            extensions.khr_android_create_instance = true;
        }

        let xr_instance = entry
            .create_instance(
                &xr::ApplicationInfo {
                    application_name: "demo",
                    application_version: 0,
                    engine_name: "demo engine",
                    engine_version: 0,
                },
                &extensions,
                &[],
            )
            .unwrap();

        let system_id = xr_instance
            .system(xr::FormFactor::HEAD_MOUNTED_DISPLAY)
            .unwrap();

        let vk_version = xr::Version::new(1, 1, 0);

        let graphics_requirements = xr_instance
            .graphics_requirements::<xr::Vulkan>(system_id)
            .unwrap();

        if vk_version < graphics_requirements.min_api_version_supported
            || vk_version.major() > graphics_requirements.max_api_version_supported.major()
        {
            panic!(
                "Vulkan version {} not supported.
                OpenXR runtime requires Vulkan version > {}, < {}.0.0",
                vk_version,
                graphics_requirements.min_api_version_supported,
                graphics_requirements.max_api_version_supported.major() + 1
            );
        }

        Arc::new(XRBase {
            xr_instance: xr_instance,
            system_id: system_id
        })
    }
}

impl Drop for XRBase {
    fn drop(&mut self) {
        println!("Dropping XRBase");
    }
}
