use std::fs;

use bevy::app::AppExit;
use bevy::prelude::*;
use kurinji::{Kurinji, KurinjiPlugin, OnActionActive, OnActionEnd};

use crate::world::command::PlayerCommand;
use crate::GameState;

pub struct ControlPlugin;

impl ControlPlugin {
    fn setup(mut kurinji: ResMut<Kurinji>) {
        let binding_json = fs::read_to_string("corp/config/binding.json")
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

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(KurinjiPlugin::default())
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(Self::setup.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(Self::player_command.system())
                    .with_system(Self::quit_app.system()),
            );
    }
}
