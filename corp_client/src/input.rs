use std::fs;

use bevy::app::AppExit;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use kurinji::{Kurinji, KurinjiPlugin, OnActionActive, OnActionEnd};

use corp_shared::prelude::{Health, Player};
use input_command::PlayerAction;

use crate::constants::state::GameState;
use crate::constants::tick;
use crate::world::colony::vortex::VortexEvent;
use crate::world::colony::Colony;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystem {
    Playing,
    Starmap,
}

pub mod input_command;

pub struct InputControlPlugin;

impl InputControlPlugin {
    fn setup_kurinji_binding(mut kurinji: ResMut<Kurinji>) {
        let binding_json = fs::read_to_string("corp_client/config/binding.json")
            .expect("Error! could not open config file");
        kurinji.set_bindings_with_json(&binding_json);
    }

    fn quit_app(mut reader: EventReader<OnActionEnd>, mut writer: EventWriter<AppExit>) {
        for event in reader.iter() {
            if event.action == "QUIT_APP" {
                println!("Quitting...");
                writer.send(AppExit);
            }
        }
    }

    fn player_keyboard_action(
        mut player_action: ResMut<PlayerAction>,
        mut reader: EventReader<OnActionActive>,
    ) {
        for event in reader.iter() {
            player_action.key_action(&event.action);
            player_action.mouse_action(&event.action);
        }
    }

    fn starmap_keyboard(
        keyboard_input: Res<Input<KeyCode>>,
        mut vortex_events: EventWriter<VortexEvent>,
    ) {
        if keyboard_input.just_pressed(KeyCode::I) {
            vortex_events.send(VortexEvent::vort(Colony::Iris));
        } else if keyboard_input.just_pressed(KeyCode::L) {
            vortex_events.send(VortexEvent::vort(Colony::Liberte));
        }
    }

    fn kill(keyboard_input: Res<Input<KeyCode>>, mut healths: Query<&mut Health, With<Player>>) {
        if keyboard_input.just_pressed(KeyCode::K) {
            if let Some(mut health) = healths.iter_mut().next() {
                health.kill();
            }
        }
    }
}

impl Plugin for InputControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(KurinjiPlugin::default());
        app.add_system(Self::setup_kurinji_binding.system());
        app.add_system(Self::quit_app.system());

        app.add_system_set(
            SystemSet::on_update(GameState::StarMap)
                .label(InputSystem::Starmap)
                .with_system(Self::starmap_keyboard.system()),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label(InputSystem::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::player_keyboard_action.system())
                .with_system(Self::kill.system()),
        );
    }
}
