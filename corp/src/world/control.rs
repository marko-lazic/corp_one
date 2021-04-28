use std::fs;
use std::ops::{AddAssign, SubAssign};

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use kurinji::{Kurinji, KurinjiPlugin, OnActionActive, OnActionEnd};

use crate::world::player::Player;
use crate::GameState;

pub struct ControlPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ControlSystem {
    ActionActiveEvent,
    RotateCamera,
    ActionEndEvent,
}

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(KurinjiPlugin::default())
            .init_resource::<PlayerAgency>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(
                        action_active_events_system
                            .system()
                            .label(ControlSystem::ActionActiveEvent),
                    )
                    .with_system(
                        rotate_camera_system
                            .system()
                            .label(ControlSystem::RotateCamera)
                            .after(ControlSystem::ActionActiveEvent),
                    )
                    .with_system(
                        action_end_events_system
                            .system()
                            .label(ControlSystem::ActionEndEvent)
                            .after(ControlSystem::RotateCamera),
                    ),
            );
    }
}

#[derive(Default)]
pub struct PlayerAgency {
    pub moving: bool,
}

fn setup(mut kurinji: ResMut<Kurinji>) {
    let binding_json =
        fs::read_to_string("corp/config/binding.json").expect("Error! could not open config file");
    kurinji.set_bindings_with_json(&binding_json);
}

fn action_end_events_system(
    mut reader: EventReader<OnActionEnd>,
    mut writer: EventWriter<AppExit>,
) {
    for event in reader.iter() {
        if event.action == "QUIT_APP" {
            println!("Quitting...");
            writer.send(AppExit);
        }
    }
}

fn action_active_events_system(
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

fn rotate_camera_system(
    mut reader: EventReader<OnActionActive>,
    mut cameras: Query<(&mut Transform, &Camera)>,
) {
    let mut translation: Vec3 = Vec3::default();
    for event in reader.iter() {
        rotate_camera(&mut translation, &event.action);
    }

    for (mut camera_transform, _) in cameras.iter_mut() {
        let rotation = camera_transform.rotation;
        camera_transform.translation += rotation * translation;
        camera_transform.look_at(Vec3::ZERO, Vec3::Y);
    }
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

fn rotate_camera(translation: &mut Vec3, action: &str) {
    let speed: f32 = 0.5;

    if action == "ARROW_LEFT" {
        translation.add_assign(Vec3::X * speed * 1.0);
    }
    if action == "ARROW_RIGHT" {
        translation.sub_assign(Vec3::X * speed * 1.0);
    }
}
