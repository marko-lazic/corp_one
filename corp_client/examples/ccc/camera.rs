use bevy::prelude::*;
use bevy_dolly::prelude::{Arm, Dolly, Position, Rig, Smooth, YawPitch};
use bevy_dolly::system::DollyUpdateSet;
use leafwing_input_manager::action_state::ActionState;

use crate::control::ControlAction;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CameraSet {
    Update,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct MainCameraFollow;

pub struct MainCameraPlugin;

#[derive(Bundle)]
pub struct MainCameraBundle {
    #[bundle]
    camera: Camera3dBundle,
    main_camera: MainCamera,
    rig: Rig,
}

impl Default for MainCameraBundle {
    fn default() -> Self {
        Self {
            camera: Camera3dBundle::default(),
            main_camera: MainCamera,
            rig: Rig::builder()
                .with(Position::new(Vec3::ZERO))
                .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-65.0))
                .with(Smooth::new_position(0.3))
                .with(Smooth::new_rotation(0.3))
                .with(Arm::new(Vec3::Z * 6.0))
                .build(),
        }
    }
}

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_camera.in_set(CameraSet::Update))
            .add_system(Dolly::<MainCamera>::update_active)
            .configure_set(DollyUpdateSet.after(CameraSet::Update));
    }
}

fn update_camera(
    action_state: Res<ActionState<ControlAction>>,
    mut rig_q: Query<&mut Rig>,
    q_follow_cam: Query<&Transform, With<MainCameraFollow>>,
    windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let mut rig = rig_q.single_mut();
    let camera_yp = rig.driver_mut::<YawPitch>();

    // Rotate 90 degrees
    if action_state.just_pressed(ControlAction::CameraRotateClockwise) {
        camera_yp.rotate_yaw_pitch(-45.0, 0.0);
    }
    if action_state.just_pressed(ControlAction::CameraRotateCounterClockwise) {
        camera_yp.rotate_yaw_pitch(45.0, 0.0);
    }

    if action_state.pressed(ControlAction::CameraZoomIn) {
        if let Some(arm) = rig.try_driver_mut::<Arm>() {
            let mut xz = arm.offset;
            xz.z = (xz.z - 0.01).abs();
            arm.offset = xz;
        }
    }

    if action_state.pressed(ControlAction::CameraZoomOut) {
        if let Some(arm) = rig.try_driver_mut::<Arm>() {
            let mut xz = arm.offset;
            xz.z = (xz.z + 0.01).abs();
            arm.offset = xz;
        }
    }

    let mut target_zoom_factor: f32 = 1.0;
    if action_state.pressed(ControlAction::Aim) {
        target_zoom_factor = 1.8;
    }

    let (camera, camera_transform) = q_camera.single();
    let ground = Transform::from_xyz(0.0, 0.0, 0.0);
    let Ok(follow_pos) = q_follow_cam.get_single() else {
        return;
    };

    let Some(ray) = windows.single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor)) else { return; };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(ground.translation, ground.up()) else { return; };
    let mouse_ground_pos = ray.get_point(distance);

    // Calculate the direction from the player to the mouse ground position
    let direction = mouse_ground_pos - follow_pos.translation;
    let sensitivity = 0.2; // Adjust this value to control camera movement speed

    // Calculate the new camera position by offsetting from the player position
    let new_camera_pos = follow_pos.translation + direction * sensitivity * target_zoom_factor;

    // Smoothly move the camera towards the new position
    let max_distance = 10.0;
    let distance = follow_pos.translation.distance(mouse_ground_pos);
    let scale_factor = (distance / max_distance).clamp(0.0, 1.0) * 0.1;

    // Update camera position
    if let Some(camera_pos) = rig.try_driver_mut::<Position>() {
        let camera_pos_diff = (new_camera_pos - camera_pos.position) * scale_factor;
        camera_pos.position.x += camera_pos_diff.x;
        camera_pos.position.z += camera_pos_diff.z;
    }
}
