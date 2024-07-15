use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        prepass::{DepthPrepass, MotionVectorPrepass, NormalPrepass},
    },
    prelude::*,
    render::camera::{Exposure, PhysicalCameraParameters},
};
use bevy_dolly::prelude::{Arm, Dolly, Position, Rig, Smooth, YawPitch};
use leafwing_input_manager::action_state::ActionState;

use corp_shared::prelude::Player;

use crate::{
    state::{Despawn, GameState},
    world::ccc::PlayerAction,
};

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
    camera: Camera3dBundle,
    depth_prepass: DepthPrepass,
    normal_prepass: NormalPrepass,
    motion_vector_prepass: MotionVectorPrepass,
    bloom: BloomSettings,
    main_camera: MainCamera,
    rig: Rig,
    despawn: Despawn,
}

impl MainCameraBundle {
    pub fn new(position: Vec3) -> Self {
        Self {
            camera: Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                exposure: Exposure::from_physical_camera(PhysicalCameraParameters {
                    aperture_f_stops: 1.0,
                    shutter_speed_s: 1.0 / 125.0,
                    sensitivity_iso: 100.0,
                    sensor_height: 0.01866,
                }),
                ..default()
            },
            depth_prepass: DepthPrepass,
            // This will generate a texture containing world normals (with normal maps applied)
            normal_prepass: NormalPrepass,
            // This will generate a texture containing screen space pixel motion vectors
            motion_vector_prepass: MotionVectorPrepass,
            bloom: BloomSettings::NATURAL,
            main_camera: MainCamera,
            rig: Rig::builder()
                .with(Position::new(position))
                .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-65.0))
                .with(Smooth::new_position(0.3))
                .with(Smooth::new_rotation(0.3))
                .with(Arm::new(Vec3::Z * 18.0))
                .build(),
            despawn: Despawn,
        }
    }
}

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Sample4).add_systems(
            Update,
            (update_camera, Dolly::<MainCamera>::update_active)
                .chain()
                .in_set(CameraSet::Update)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn update_camera(
    q_action_state: Query<&ActionState<PlayerAction>, With<Player>>,
    time: Res<Time>,
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
            xz.z = (xz.z - 4.0 * time.delta_seconds()).abs();
            arm.offset = xz.clamp_length_min(6.0);
        }
    }

    if action_state.pressed(&PlayerAction::CameraZoomOut) {
        if let Some(arm) = rig.try_driver_mut::<Arm>() {
            let mut xz = arm.offset;
            xz.z = (xz.z + 4.0 * time.delta_seconds()).abs();
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
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use approx::assert_relative_eq;
    use bevy::input::InputPlugin;
    use leafwing_input_manager::{input_mocking::MockInput, InputManagerBundle};

    use corp_shared::prelude::{Health, TestUtils};

    use crate::{
        sound::InteractionSoundEvent,
        world::ccc::{CharacterPlugin, CharacterSet, ControlPlugin, ControlSet, MovementBundle},
    };

    use super::*;

    #[test]
    fn camera_follows_the_character() {
        // given
        let mut app = setup();
        let player_pos = Transform::from_xyz(0.0, 0.5, 0.0);
        setup_player(&mut app, player_pos);
        app.set_state(GameState::Playing);
        let camera = get_camera_entity(&mut app);
        rotate_camera_yaw_minus_45(&mut app, camera);
        app.update_after(Duration::from_secs_f32(1.0));

        // when
        app.send_input(KeyCode::KeyD);
        app.update();

        // then
        let camp_pos_result = app.get::<Transform>(camera).translation;
        assert_relative_eq!(camp_pos_result.x, 1.42 * 3.0);
    }

    #[test]
    fn camera_zoom_out_limit() {
        // given
        let mut app = setup();
        let player_pos = Transform::from_xyz(0.0, 0.5, 0.0);
        setup_player(&mut app, player_pos);
        app.set_state(GameState::Playing);
        let camera = get_camera_entity(&mut app);
        rotate_camera_yaw_minus_45(&mut app, camera);

        // when
        app.send_input(KeyCode::Minus);
        app.update_after(Duration::from_secs_f32(10.0));

        // then
        let arm = app.get::<Rig>(camera).try_driver::<Arm>().unwrap();
        assert_relative_eq!(arm.offset.z, 18.0);
    }

    #[test]
    fn camera_zoom_in_limit() {
        // given
        let mut app = setup();
        let player_pos = Transform::from_xyz(0.0, 0.5, 0.0);
        setup_player(&mut app, player_pos);
        app.set_state(GameState::Playing);
        let camera = get_camera_entity(&mut app);
        rotate_camera_yaw_minus_45(&mut app, camera);
        app.update();

        // when
        app.send_input(KeyCode::Equal);
        app.update_after(Duration::from_secs_f32(4.0));

        // then
        let arm = app.get::<Rig>(camera).try_driver::<Arm>().unwrap();
        assert_relative_eq!(arm.offset.z, 6.0);
    }

    fn setup() -> App {
        let mut app = App::new();
        app.init_time()
            .add_event::<InteractionSoundEvent>()
            .init_state::<GameState>()
            .add_plugins((
                InputPlugin,
                ControlPlugin,
                CharacterPlugin,
                MainCameraPlugin,
            ))
            .configure_sets(
                Update,
                (
                    ControlSet::PlayingInput.before(CharacterSet::Movement),
                    CameraSet::Update.after(CharacterSet::Movement),
                ),
            );
        app.world().spawn(Window::default());
        app
    }

    fn setup_player(app: &mut App, transform: Transform) -> Entity {
        let player_entity = app
            .world()
            .spawn((
                TransformBundle::from_transform(transform),
                Player,
                InputManagerBundle {
                    input_map: PlayerAction::player_input_map(),
                    ..default()
                },
                MainCameraFollow,
                Health::default(),
                MovementBundle::default(),
            ))
            .id();
        app.world()
            .spawn(MainCameraBundle::new(transform.translation));
        player_entity
    }

    fn rotate_camera_yaw_minus_45(app: &mut App, camera: Entity) {
        app.get_mut::<Rig>(camera)
            .driver_mut::<YawPitch>()
            .rotate_yaw_pitch(-45.0, 0.0);
    }

    fn get_camera_entity(app: &mut App) -> Entity {
        app.world()
            .query_filtered::<Entity, With<MainCamera>>()
            .iter(&mut app.world())
            .next()
            .unwrap()
    }
}
