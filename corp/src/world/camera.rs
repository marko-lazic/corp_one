use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::constants::state::GameState;
use crate::constants::tick;
use crate::Game;

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

pub struct CameraCenter;

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

impl TopDownCameraPlugin {
    fn target_motion(
        time: Res<Time>,
        game: Res<Game>,
        mut query: Query<(&mut TopDownCamera, &mut Transform, &mut Camera)>,
    ) {
        for (mut camera, mut transform, _) in query.iter_mut() {
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
}

impl Plugin for TopDownCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::target_motion.system()),
        );
    }
}
