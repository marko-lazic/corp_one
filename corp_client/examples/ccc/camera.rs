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
                .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-60.0))
                .with(Smooth::new_position(0.3))
                .with(Smooth::new_rotation(0.3))
                .with(Arm::new(Vec3::Z * 4.0))
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
    follow_trans_q: Query<&Transform, With<MainCameraFollow>>,
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

    // Update camera position
    if let Some(camera_pos) = rig.try_driver_mut::<Position>() {
        for pos in follow_trans_q.iter() {
            camera_pos.position = pos.translation + Vec3::new(0., 1., 0.);
        }
    }
}
