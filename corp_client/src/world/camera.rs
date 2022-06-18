use std::ops::Neg;

use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_mod_picking::PickingCameraBundle;
use bevy_mod_raycast::RayCastSource;
use iyes_loopless::prelude::ConditionSet;

use crate::constants::state::GameState;
use crate::input::{Cursor, MyRayCastSet};
use crate::world::player::Player;
use crate::world::WorldSystem;
use crate::Game;

#[derive(Component)]
pub struct TopDownCamera {
    pub x: f32,
    pub y: f32,
    pub distance: f32,
    pub rotate_sensitivity: f32,
    pub zoom_sensitivity: f32,
}

impl TopDownCamera {
    pub fn new(distance: f32) -> TopDownCamera {
        TopDownCamera {
            distance,
            ..Default::default()
        }
    }
}

impl Default for TopDownCamera {
    fn default() -> Self {
        TopDownCamera {
            x: 0.0,
            y: 0.0,
            distance: 5.0,
            rotate_sensitivity: 1.0,
            zoom_sensitivity: 0.8,
        }
    }
}

pub struct TopDownCameraPlugin;

impl Plugin for TopDownCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::SpawnPlayer)
                .label(WorldSystem::CameraSetup)
                .after(WorldSystem::PlayerSetup)
                .with_system(Self::setup_camera)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .label(CameraMotion)
                .with_system(Self::input_camera_center)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .after(CameraMotion)
                .with_system(Self::target_motion)
                .into(),
        );
    }
}

impl TopDownCameraPlugin {
    fn setup_camera(mut commands: Commands) {
        info!("Setup Player");
        commands
            .spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_translation(Vec3::new(-3.0, 3.0, 5.0))
                    .looking_at(Vec3::default(), Vec3::Y),
                ..Default::default()
            })
            .insert(TopDownCamera::new(20.0))
            .insert_bundle(PickingCameraBundle::default())
            .insert(RayCastSource::<MyRayCastSet>::new());
    }

    fn target_motion(
        time: Res<Time>,
        game: Res<Game>,
        mut camera_query: Query<(&mut TopDownCamera, &mut Transform, &mut Camera)>,
    ) {
        for (mut camera, mut transform, _) in camera_query.iter_mut() {
            let delta = Vec2::ZERO;
            camera.x -= delta.x * camera.rotate_sensitivity * time.delta_seconds();
            camera.y -= delta.y * camera.rotate_sensitivity * time.delta_seconds();

            camera.y = camera.y.max(0.01).min(3.13);

            let rot = Quat::from_axis_angle(Vec3::Y, camera.x)
                * Quat::from_axis_angle(-Vec3::X, camera.y);

            transform.translation =
                (rot * Vec3::new(0.0, 1.0, 0.0)) * camera.distance + game.camera_center;
            transform.look_at(game.camera_center, Vec3::Y);
        }
    }

    fn input_camera_center(
        mut game: ResMut<Game>,
        cursor: Res<Cursor>,
        player_transform: Query<&Transform, With<Player>>,
    ) {
        if let Some(player_entity) = game.player_entity {
            if let Ok(transform) = player_transform.get(player_entity) {
                let player_pos = transform.translation;
                let vec_threshold = Vec3::new(3.0, 3.0, 3.0);
                let target_pos = (player_pos + cursor.world) / 2.0;
                let restrict_pos =
                    target_pos.clamp(vec_threshold.neg() + player_pos, vec_threshold + player_pos);
                game.camera_center = restrict_pos;
            }
        }
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct CameraMotion;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_negate() {
        let vec_threshold = Vec3::new(3.0, 3.0, 3.0);
        println!("{:?}", vec_threshold.neg());
    }
}
