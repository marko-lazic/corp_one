use std::fs;

use bevy::app::{AppExit, Events};
use bevy::ecs::ResMut;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use kurinji::{Kurinji, KurinjiPlugin, OnActionActive, OnActionEnd};
use std::ops::AddAssign;

use crate::world::player::{MovementSpeed, Player};

use super::control;

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
            .add_system(rotate_camera_system.system())
            .add_system(action_end_events_system.system());
    }
}

fn setup(mut kurinji: ResMut<Kurinji>) {
    let binding_json =
        fs::read_to_string("corp/config/binding.json").expect("Error! could not open config file");
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
    mut player_position: Query<(&Player, &mut Transform, &mut MovementSpeed)>,
) {
    let mut delta_move: Vec3 = Default::default();
    let event_iter = state.active_reader.iter(&action_active_event);
    for event in event_iter {
        control::move_player(&mut delta_move, &event.action);
        control::aim_mouse(&event.action);
    }
    for (_player, mut transform, mut movement) in player_position.iter_mut() {
        transform.translation.add_assign(delta_move);
        movement.is_moving = is_moving(&delta_move);
    }
}

fn is_moving(delta_move: &Vec3) -> bool {
    let zero_vec: Vec3 = Default::default();
    delta_move.ne(&zero_vec)
}

fn rotate_camera_system(
    mut state: Local<ActionState>,
    action_active_event: Res<Events<OnActionActive>>,
    mut cameras: Query<(&mut Transform, &Camera)>,
) {
    let mut translation: Vec3 = Vec3::default();
    let event_iter = state.active_reader.iter(&action_active_event);
    for event in event_iter {
        control::rotate_camera(&mut translation, &event.action);
    }

    for (mut camera_transform, _) in cameras.iter_mut() {
        let rotation = camera_transform.rotation;
        camera_transform.translation += rotation * translation;
        camera_transform.look_at(Vec3::zero(), Vec3::unit_y());
    }
}
