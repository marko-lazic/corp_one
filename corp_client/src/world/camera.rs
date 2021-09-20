use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::render::camera::Camera;

use corp_shared::prelude::Player;

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
        mut camera_look: Local<Vec3>,
        mut game: ResMut<Game>,
        keyboard_input: Res<Input<KeyCode>>,
        player_transform: Query<&Transform, With<Player>>,
    ) {
        const CAM_LIMIT: f32 = 3.0;
        if keyboard_input.pressed(KeyCode::Left) {
            camera_look.x += 0.1;
        } else if keyboard_input.pressed(KeyCode::Right) {
            camera_look.x -= 0.1;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            camera_look.z += 0.1;
        } else if keyboard_input.pressed(KeyCode::Down) {
            camera_look.z -= 0.1;
        }

        camera_look.x = camera_look.x.clamp(-CAM_LIMIT, CAM_LIMIT);
        camera_look.z = camera_look.z.clamp(-CAM_LIMIT, CAM_LIMIT);

        if let Some(player_entity) = game.player_entity {
            if let Ok(pt) = player_transform.get(player_entity) {
                game.camera_center = pt.translation + *camera_look;
            }
        }
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct CameraMotion;

impl Plugin for TopDownCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::target_motion.system().label(CameraMotion))
                .with_system(Self::input_camera_center.system().before(CameraMotion)),
        );
    }
}
