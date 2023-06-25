use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use corp_shared::prelude::{Health, Player};

use crate::camera::MainCamera;
use crate::control::ControlAction;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CharacterSet {
    Movement,
}

pub struct CharacterPlugin;

#[derive(Component)]
pub struct CharacterMovement {
    pub enabled: bool,
    pub direction: Vec3,
    pub velocity: Vec3,
    pub speed: f32,
}

impl CharacterMovement {
    pub fn is_moving(&self) -> bool {
        self.direction != Vec3::ZERO
    }
}

impl Default for CharacterMovement {
    fn default() -> Self {
        Self {
            enabled: true,
            direction: Vec3::ZERO,
            velocity: Vec3::ZERO,
            speed: 1.42,
        }
    }
}

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                is_movement_enabled,
                calculate_character_movement,
                move_player,
            )
                .chain()
                .in_set(CharacterSet::Movement),
        );
    }
}

fn is_movement_enabled(mut query: Query<(&mut CharacterMovement, &Health), Changed<Health>>) {
    for (mut character_movement, health) in &mut query {
        character_movement.enabled = health.is_alive();
    }
}

fn calculate_character_movement(
    action_state: Res<ActionState<ControlAction>>,
    mut q_camera: Query<&Transform, With<MainCamera>>,
    mut query: Query<&mut CharacterMovement, With<Player>>,
) {
    let Ok(cam) = q_camera.get_single_mut() else {
        return;
    };

    let cam_forward = Vec3::new(
        cam.rotation.mul_vec3(Vec3::Z).x,
        0.0,
        cam.rotation.mul_vec3(Vec3::Z).z,
    )
    .normalize_or_zero();
    let cam_right = Vec3::new(
        cam.rotation.mul_vec3(Vec3::X).x,
        0.0,
        cam.rotation.mul_vec3(Vec3::X).z,
    )
    .normalize_or_zero();

    let mut direction = Vec3::ZERO;
    if action_state.pressed(ControlAction::Forward) {
        direction -= cam_forward;
    }
    if action_state.pressed(ControlAction::Backward) {
        direction += cam_forward;
    }
    if action_state.pressed(ControlAction::Left) {
        direction -= cam_right;
    }
    if action_state.pressed(ControlAction::Right) {
        direction += cam_right;
    }

    for mut character_movement in &mut query {
        if !character_movement.enabled {
            continue;
        }
        character_movement.direction = direction.normalize_or_zero().clamp_length_max(1.0);
        character_movement.velocity = character_movement.direction * character_movement.speed;
    }
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &CharacterMovement), With<Player>>,
) {
    let delta_seconds = time.delta_seconds();
    for (mut transform, character_movement) in &mut query {
        let new_position = transform.translation + character_movement.velocity * delta_seconds;
        transform.translation = new_position;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::input::InputPlugin;
    use bevy_dolly::prelude::{Rig, YawPitch};
    use float_eq::assert_float_eq;

    use corp_shared::prelude::{Health, TestUtils};

    use crate::camera::{CameraSet, MainCameraBundle, MainCameraPlugin};
    use crate::control::ControlPlugin;

    use super::*;

    #[test]
    fn direction_up() {
        // given
        let mut app = setup();
        let player = setup_player(&mut app);
        let camera = setup_camera(&mut app);
        app.get_mut::<Rig>(camera)
            .driver_mut::<YawPitch>()
            .rotate_yaw_pitch(-45.0, 0.0);

        // when
        app.send_input(KeyCode::W);
        app.update();

        // then
        let character_movement = app.get::<CharacterMovement>(player);
        assert_eq!(character_movement.direction, -Vec3::Z);
        assert!(character_movement.is_moving());
    }

    #[test]
    fn direction_up_left() {
        // given
        let mut app = setup();
        setup_camera(&mut app);
        let player = setup_player(&mut app);

        // when
        app.send_input(KeyCode::W);
        app.send_input(KeyCode::A);
        app.update();

        // then
        let expected_direction = Vec3::new(-0.70710677, 0.0, -0.70710677);
        let direction_result = app.get::<CharacterMovement>(player).direction;
        assert_eq!(direction_result, expected_direction);
    }

    #[test]
    fn move_north() {
        // given
        let mut app = setup();
        let player = setup_player(&mut app);

        // when
        app.send_input(KeyCode::W);
        app.update();

        // then
        let expected_translation = Vec3::new(0.0, 0.0, -0.0017810349);
        let translation_result = app.get::<Transform>(player).translation;
        assert_float_eq!(translation_result.z, expected_translation.z, abs <= 0.01);
    }

    #[test]
    fn move_north_1_second() {
        // given
        let mut app = setup();
        setup_camera(&mut app);
        let player = setup_player(&mut app);

        // when
        app.send_input(KeyCode::W);
        app.update_after(Duration::from_secs_f32(1.0));

        // then
        let expected_translation = Vec3::new(0.0, 0.0, -1.42);
        let translation_result = app.get::<Transform>(player).translation;
        assert_float_eq!(translation_result.z, expected_translation.z, abs <= 0.01);
    }

    #[test]
    fn dead_player_cannot_move() {
        // given
        let mut app = setup();
        setup_camera(&mut app);
        let player = setup_player(&mut app);

        // when
        app.get_mut::<Health>(player).kill_mut();
        app.send_input(KeyCode::W);
        app.update();

        // then
        let character_movement = app.get::<CharacterMovement>(player);
        assert!(!character_movement.enabled);
        assert_eq!(character_movement.direction, Vec3::ZERO);
        assert_eq!(character_movement.velocity, Vec3::ZERO);
    }

    fn setup() -> App {
        let mut app = App::new();
        app.init_time()
            .add_plugin(InputPlugin)
            .add_plugin(ControlPlugin)
            .add_plugin(CharacterPlugin)
            .add_plugin(MainCameraPlugin)
            .configure_set(CameraSet::Update.after(CharacterSet::Movement));
        app
    }

    fn setup_player(app: &mut App) -> Entity {
        app.world
            .spawn((
                TransformBundle::default(),
                Player,
                Health::default(),
                CharacterMovement::default(),
            ))
            .id()
    }

    fn setup_camera(app: &mut App) -> Entity {
        app.world.spawn(MainCameraBundle::default()).id()
    }
}
