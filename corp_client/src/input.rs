use std::fs;

use bevy::app::AppExit;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use kurinji::{Kurinji, KurinjiPlugin, OnActionActive, OnActionEnd};

use input_command::PlayerAction;

use crate::constants::state::GameState;
use crate::constants::tick;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystemLabel {
    Input,
    Movement,
}

pub mod input_command;

pub struct InputControlPlugin;

impl InputControlPlugin {
    fn setup(mut kurinji: ResMut<Kurinji>) {
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

    fn player_action(
        mut player_action: ResMut<PlayerAction>,
        mut reader: EventReader<OnActionActive>,
    ) {
        for event in reader.iter() {
            player_action.key_action(&event.action);
            player_action.mouse_action(&event.action);
        }
    }
}

impl Plugin for InputControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(KurinjiPlugin::default()).add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(Self::setup.system()),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::player_action.system().label(InputSystemLabel::Input))
                .with_system(Self::quit_app.system()),
        );
    }
}
