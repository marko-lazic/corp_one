use std::fs;
use std::ops::AddAssign;

use bevy::app::AppExit;
use bevy::prelude::*;
use kurinji::{Kurinji, KurinjiPlugin, OnActionActive, OnActionEnd};

use crate::world::player::Player;
use crate::GameState;

#[derive(Default)]
pub struct PlayerAgency {
    pub moving: bool,
}

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(KurinjiPlugin::default())
            .init_resource::<PlayerAgency>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(player_movement.system())
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

fn player_movement(
    mut reader: EventReader<OnActionActive>,
    mut agency: ResMut<PlayerAgency>,
    mut player_position: Query<(&Player, &mut Transform)>,
) {
    let mut delta_move: Vec3 = Default::default();
    for event in reader.iter() {
        move_player(&mut delta_move, &event.action);
        aim_mouse(&event.action);
    }

    if let Ok((_player, mut transform)) = player_position.single_mut() {
        transform.translation.add_assign(delta_move);
        agency.moving = is_moving(&delta_move);
    }
}

fn is_moving(delta_move: &Vec3) -> bool {
    delta_move.ne(&Vec3::ZERO)
}

fn move_player(delta_move: &mut Vec3, action: &str) {
    if action == "MOVE_FORWARD" {
        delta_move.add_assign(Vec3::new(0.1, 0.0, 0.0));
    }
    if action == "MOVE_BACKWARD" {
        delta_move.add_assign(Vec3::new(-0.1, 0.0, 0.0));
    }
    if action == "MOVE_LEFT" {
        delta_move.add_assign(Vec3::new(0.0, 0.0, -0.1));
    }
    if action == "MOVE_RIGHT" {
        delta_move.add_assign(Vec3::new(0.0, 0.0, 0.1));
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
