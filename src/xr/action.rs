use openxr as xr;

pub struct Action {
    action_set: openxr::ActionSet,
    left_action: openxr::Action<xr::Posef>,
    right_action: openxr::Action<xr::Posef>,
}

impl Action {
    pub fn new(xr_instance: &openxr::Instance,
               session: &openxr::Session<xr::Vulkan>
    ) -> Self {
        let action_set = xr_instance
            .create_action_set("input", "input pose information", 0)
            .unwrap();

        let right_action = action_set
            .create_action::<xr::Posef>("right_hand", "Right Hand Controller", &[])
            .unwrap();

        let left_action = action_set
            .create_action::<xr::Posef>("left_hand", "Left Hand Controller", &[])
            .unwrap();

        xr_instance
            .suggest_interaction_profile_bindings(
                xr_instance
                    .string_to_path("/interaction_profiles/khr/simple_controller")
                    .unwrap(),
                &[
                    xr::Binding::new(
                        &right_action,
                        xr_instance
                            .string_to_path("/user/hand/right/input/grip/pose")
                            .unwrap(),
                    ),
                    xr::Binding::new(
                        &left_action,
                        xr_instance
                            .string_to_path("/user/hand/left/input/grip/pose")
                            .unwrap(),
                    ),
                ],
            )
            .unwrap();

        session.attach_action_sets(&[&action_set]).unwrap();

        Self {
            action_set: action_set,
            right_action: right_action,
            left_action: left_action,
        }
    }
}
