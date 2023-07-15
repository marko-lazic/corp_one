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

impl MainCameraBundle {
    pub fn new(position: Vec3) -> Self {
        Self {
            camera: Camera3dBundle::default(),
            main_camera: MainCamera,
            rig: Rig::builder()
                .with(Position::new(position))
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
    time: Res<Time>,
    mut rig_q: Query<&mut Rig>,
    q_follow_cam: Query<&Transform, With<MainCameraFollow>>,
    windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Ok(mut rig) = rig_q.get_single_mut() else {
        eprintln!("No camera rig found");
        return;
    };
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
            xz.z = (xz.z - 4.0 * time.delta_seconds()).abs();
            arm.offset = xz.clamp_length_min(2.0);
        }
    }

    if action_state.pressed(ControlAction::CameraZoomOut) {
        if let Some(arm) = rig.try_driver_mut::<Arm>() {
            let mut xz = arm.offset;
            xz.z = (xz.z + 4.0 * time.delta_seconds()).abs();
            arm.offset = xz.clamp_length_max(6.0);
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

    let ray = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .unwrap_or_else(|| Ray {
            origin: follow_pos.translation,
            direction: follow_pos.down(),
        });

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(ground.translation, ground.up()) else {
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
    use leafwing_input_manager::input_mocking::MockInput;

    use corp_shared::prelude::{Health, Player, TestUtils};

    use crate::character::{CharacterPlugin, CharacterSet};
    use crate::control::{ControlPlugin, ControlSet};
    use crate::movement::MovementBundle;

    use super::*;

    #[test]
    fn camera_follows_the_character() {
        // given
        let mut app = setup();
        let player_pos = Transform::from_xyz(0.0, 0.5, 0.0);
        setup_player(&mut app, player_pos);
        let camera = setup_camera(&mut app, player_pos.translation);

        // when
        app.send_input(KeyCode::D);
        app.update();
        app.update();

        // then
        let camp_pos_result = app.get::<Transform>(camera).translation;
        assert_relative_eq!(camp_pos_result.x, 1.42);
    }

    #[test]
    fn camera_zoom_out_limit() {
        // given
        let mut app = setup();
        let player_pos = Transform::from_xyz(0.0, 0.5, 0.0);
        setup_player(&mut app, player_pos);
        let camera = setup_camera(&mut app, player_pos.translation);

        // when
        app.send_input(KeyCode::Minus);
        app.update_after(Duration::from_secs_f32(10.0));

        // then
        let arm = app.get::<Rig>(camera).try_driver::<Arm>().unwrap();
        assert_relative_eq!(arm.offset.z, 6.0);
    }

    #[test]
    fn camera_zoom_in_limit() {
        // given
        let mut app = setup();
        let player_pos = Transform::from_xyz(0.0, 0.5, 0.0);
        setup_player(&mut app, player_pos);
        let camera = setup_camera(&mut app, player_pos.translation);

        // when
        app.send_input(KeyCode::Equals);
        app.update_after(Duration::from_secs_f32(1.6));

        // then
        let arm = app.get::<Rig>(camera).try_driver::<Arm>().unwrap();
        assert_relative_eq!(arm.offset.z, 2.0);
    }

    fn setup() -> App {
        let mut app = App::new();
        app.init_time()
            .add_plugin(InputPlugin)
            .add_plugin(ControlPlugin)
            .add_plugin(CharacterPlugin)
            .add_plugin(MainCameraPlugin)
            .configure_set(ControlSet::Input.before(CharacterSet::Movement))
            .configure_set(CameraSet::Update.after(CharacterSet::Movement));
        app.world.spawn(Window::default());
        app
    }

    fn setup_player(app: &mut App, transform: Transform) -> Entity {
        app.world
            .spawn((
                TransformBundle::from_transform(transform),
                Player,
                MainCameraFollow,
                Health::default(),
                MovementBundle::default(),
            ))
            .id()
    }

    fn setup_camera(app: &mut App, position: Vec3) -> Entity {
        let camera = app.world.spawn(MainCameraBundle::new(position)).id();
        app.get_mut::<Rig>(camera)
            .driver_mut::<YawPitch>()
            .rotate_yaw_pitch(-45.0, 0.0);
        app.update_after(Duration::from_secs_f32(1.0));
        camera
    }
}
