use std::fs;

use bevy::app::AppExit;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use input_command::PlayerCommand;
use kurinji::{Kurinji, KurinjiPlugin, OnActionActive, OnActionEnd};

use crate::constants::state::GameState;
use crate::constants::tick;
use crate::input::mouse::MousePlugin;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerLabel {
    Input,
    Movement,
}

pub mod input_command;
pub mod mouse;

pub struct InputControlPlugin;

impl InputControlPlugin {
    fn setup(mut kurinji: ResMut<Kurinji>) {
        let binding_json = fs::read_to_string("client/config/binding.json")
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

    fn player_command(mut command: ResMut<PlayerCommand>, mut reader: EventReader<OnActionActive>) {
        for event in reader.iter() {
            command.key_command(&event.action);
            command.mouse_command(&event.action);
        }
    }
}

impl Plugin for InputControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(KurinjiPlugin::default())
            .add_plugin(MousePlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(Self::setup.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                    .with_system(
                        Self::player_command
                            .system()
                            .label(PlayerLabel::Input)
                            .before(PlayerLabel::Movement),
                    )
                    .with_system(Self::quit_app.system()),
            );
    }
}
