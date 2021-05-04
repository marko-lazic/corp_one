use std::fs;

use bevy::app::AppExit;
use bevy::prelude::*;
use kurinji::{Kurinji, KurinjiPlugin, OnActionActive, OnActionEnd};

use crate::world::player::Player;
use crate::GameState;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(KurinjiPlugin::default())
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(player_input.system())
                    .with_system(quit_app.system()),
            );
    }
}

fn setup(mut kurinji: ResMut<Kurinji>) {
    let binding_json =
        fs::read_to_string("corp/config/binding.json").expect("Error! could not open config file");
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

fn player_input(mut reader: EventReader<OnActionActive>, mut query: Query<&mut Player>) {
    if let Ok(mut player) = query.single_mut() {
        for event in reader.iter() {
            move_player(&mut player, &event.action);
            aim_mouse(&event.action);
        }
    }
}

fn move_player(player: &mut Player, action: &str) {
    if action == "MOVE_FORWARD" {
        player.input.forward = true;
    }
    if action == "MOVE_BACKWARD" {
        player.input.backward = true;
    }
    if action == "MOVE_LEFT" {
        player.input.left = true;
    }
    if action == "MOVE_RIGHT" {
        player.input.right = true;
    }
}

fn aim_mouse(action: &str) {
    if action == "MOUSE_SHOOT" {
        info!("Bang");
    }
    if action == "AIM_UP" {}
    if action == "AIM_DOWN" {}
    if action == "AIM_LEFT" {}
    if action == "AIM_RIGHT" {}
}
