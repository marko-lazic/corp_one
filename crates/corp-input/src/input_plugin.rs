pub(crate) mod input {
    use std::fs;

    use bevy::app::{AppExit, Events};
    use bevy::ecs::ResMut;
    use bevy::prelude::*;
    use corp_scene::player::Player;
    use kurinji::{Kurinji, KurinjiPlugin, OnActionActive, OnActionEnd};
    use std::ops::AddAssign;

    #[derive(Default)]
    pub struct ActionState {
        active_reader: EventReader<OnActionActive>,
        end_reader: EventReader<OnActionEnd>,
    }

    pub struct InputPlugin;

    impl Plugin for InputPlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_plugin(KurinjiPlugin::default())
                .add_startup_system(setup.system())
                .add_system(action_active_events_system.system())
                .add_system(action_end_events_system.system());
        }
    }

    fn setup(mut kurinji: ResMut<Kurinji>) {
        let binding_json =
            fs::read_to_string("config/binding.json").expect("Error! could not open config file");
        kurinji.set_bindings_with_json(&binding_json);
    }

    fn action_end_events_system(
        mut state: Local<ActionState>,
        mut app_exit_events: ResMut<Events<AppExit>>,
        action_end_event: Res<Events<OnActionEnd>>,
    ) {
        if let Some(value) = state.end_reader.latest(&action_end_event) {
            if value.action == "QUIT_APP" {
                println!("Quiting...");
                app_exit_events.send(AppExit);
            }
        }
    }
    fn action_active_events_system(
        mut state: Local<ActionState>,
        action_active_event: Res<Events<OnActionActive>>,
        mut player_position: Query<(&Player, &mut Transform)>,
    ) {
        let mut delta_vec: Vec3 = Default::default();
        let event_iter = state.active_reader.iter(&action_active_event);
        for event in event_iter {
            println!("{}", event.action);
            // move_actions
            if event.action == "MOVE_FORWARD" {
                delta_vec.add_assign(Vec3::new(0.0, 0.0, -0.1));
            }

            if event.action == "MOVE_BACKWARD" {
                delta_vec.add_assign(Vec3::new(0.0, 0.0, 0.1));
                delta_vec.z().add_assign(0.1);
            }

            if event.action == "MOVE_LEFT" {
                delta_vec.add_assign(Vec3::new(-0.1, 0.0, 0.0));
            }

            if event.action == "MOVE_RIGHT" {
                delta_vec.add_assign(Vec3::new(0.1, 0.0, 0.0));
            }

            // aim_actions
            if event.action == "MOUSE_SHOOT" {
                println!("Bang");
            }

            if event.action == "AIM_UP" {
                println!("AIM_UP... [ strength: {}] ", event.strength);
            }

            if event.action == "AIM_DOWN" {
                println!("AIM_DOWN... [ strength: {}] ", event.strength);
            }

            if event.action == "AIM_LEFT" {
                println!("AIM_LEFT... [ strength: {}] ", event.strength);
            }

            if event.action == "AIM_RIGHT" {
                println!("AIM_RIGHT... [ strength: {}] ", event.strength);
            }
        }
        for (_player, mut transform) in player_position.iter_mut() {
            transform.translation.add_assign(delta_vec);
        }
    }
}
