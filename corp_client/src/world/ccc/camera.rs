use crate::prelude::*;
use bevy::{
    core_pipeline::{
        bloom::Bloom,
        prepass::{DepthPrepass, MotionVectorPrepass, NormalPrepass},
    },
    prelude::*,
    render::camera::{Exposure, PhysicalCameraParameters},
};
use bevy_dolly::prelude::{Arm, Dolly, Position, Rig, Smooth, YawPitch};
use corp_shared::prelude::Player;
use leafwing_input_manager::action_state::ActionState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CameraSet {
    Update,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct MainCameraFollow;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (update_camera, Dolly::<MainCamera>::update_active)
                .chain()
                .in_set(CameraSet::Update)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn setup_camera(mut commands: Commands, q_player_transform: Single<&Transform, With<Player>>) {
    info!("Setup Camera");
    let position = q_player_transform.translation;
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Rig::builder()
            .with(Position::new(position))
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-65.0))
            .with(Smooth::new_position(0.3))
            .with(Smooth::new_rotation(0.3))
            .with(Arm::new(Vec3::Z * 18.0))
            .build(),
        Camera {
            hdr: true,
            ..default()
        },
        Exposure::from_physical_camera(PhysicalCameraParameters {
            aperture_f_stops: 1.0,
            shutter_speed_s: 1.0 / 125.0,
            sensitivity_iso: 100.0,
            sensor_height: 0.01866,
        }),
        DepthPrepass,
        // This will generate a texture containing world normals (with normal maps applied)
        NormalPrepass,
        // This will generate a texture containing screen space pixel motion vectors
        MotionVectorPrepass,
        Bloom::NATURAL,
        StateScoped(GameState::Playing),
    ));
}

fn update_camera(
    q_action_state: Query<&ActionState<PlayerAction>, With<Player>>,
    time: Res<Time<Fixed>>,
    mut rig_q: Query<&mut Rig>,
    q_follow_cam: Query<&Transform, With<MainCameraFollow>>,
    windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Ok(mut rig) = rig_q.get_single_mut() else {
        error!("No camera rig found");
        return;
    };
    let camera_yp = rig.driver_mut::<YawPitch>();

    let Ok(action_state) = q_action_state.get_single() else {
        warn!("PlayerAction state is missing.");
        return;
    };

    if action_state.just_pressed(&PlayerAction::CameraRotateClockwise) {
        camera_yp.rotate_yaw_pitch(-45.0, 0.0);
    }
    if action_state.just_pressed(&PlayerAction::CameraRotateCounterClockwise) {
        camera_yp.rotate_yaw_pitch(45.0, 0.0);
    }

    if action_state.pressed(&PlayerAction::CameraZoomIn) {
        if let Some(arm) = rig.try_driver_mut::<Arm>() {
            let mut xz = arm.offset;
            xz.z = (xz.z - 4.0 * time.delta_secs()).abs();
            arm.offset = xz.clamp_length_min(6.0);
        }
    }

    if action_state.pressed(&PlayerAction::CameraZoomOut) {
        if let Some(arm) = rig.try_driver_mut::<Arm>() {
            let mut xz = arm.offset;
            xz.z = (xz.z + 4.0 * time.delta_secs()).abs();
            arm.offset = xz.clamp_length_max(18.0);
        }
    }

    let mut target_zoom_factor: f32 = 1.0;
    if action_state.pressed(&PlayerAction::Aim) {
        target_zoom_factor = 1.8;
    }

    let (camera, camera_transform) = q_camera.single();
    let ground_origin = Vec3::ZERO;

    let Ok(follow_pos) = q_follow_cam.get_single() else {
        return;
    };

    let Ok(window) = windows.get_single() else {
        return;
    };

    let ray = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .unwrap_or_else(|| Ray3d {
            origin: follow_pos.translation,
            direction: follow_pos.down(),
        });

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(ground_origin, InfinitePlane3d::new(Vec3::Y)) else {
        return;
    };
    let mouse_ground_pos = ray.get_point(distance);

    // Calculate the direction from the player to the mouse ground position
    let direction = mouse_ground_pos - follow_pos.translation;
    let sensitivity = 0.2; // Adjust this value to control camera movement speed

    // Calculate the new camera position by offsetting from the player position
    let new_camera_pos = follow_pos.translation + direction * sensitivity * target_zoom_factor;

    // Update camera position
    if let Some(camera_pos) = rig.try_driver_mut::<Position>() {
        // Calculate the distance between the player and the new camera position
        let max_distance = (new_camera_pos - follow_pos.translation).length();

        // Limit the distance to ensure the player is always visible
        let distance = distance.min(max_distance);

        // Calculate the new camera position
        let camera_pos_diff = (new_camera_pos - follow_pos.translation).normalize() * distance;

        let player_and_camera_pos_diff = follow_pos.translation + camera_pos_diff;
        camera_pos.position.x = player_and_camera_pos_diff.x;
        camera_pos.position.z = player_and_camera_pos_diff.z;
    }
}
