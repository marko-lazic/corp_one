use bevy::prelude::*;
use bevy_rapier3d::prelude::KinematicCharacterController;

use corp_shared::prelude::Health;

use crate::world::ccc::{CharacterMovement, ControlMovement, OrientationMode};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CharacterSet {
    Movement,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                is_movement_enabled,
                calculate_character_movement,
                move_character,
                rotate_character,
            )
                .chain()
                .in_set(CharacterSet::Movement),
        );
    }
}

fn is_movement_enabled(mut query: Query<(&mut CharacterMovement, &Health), Changed<Health>>) {
    for (mut character_movement, health) in &mut query {
        character_movement.can_move = health.is_alive();
    }
}

fn calculate_character_movement(
    mut q_movement: Query<(&mut CharacterMovement, &ControlMovement), Changed<ControlMovement>>,
) {
    for (mut character_movement, control_movement) in &mut q_movement {
        let direction = control_movement.direction;
        if !character_movement.can_move || direction == Vec3::ZERO {
            character_movement.velocity = Vec3::ZERO;
            continue;
        }

        character_movement.direction = direction.normalize_or_zero().clamp_length_max(1.0);
        character_movement.velocity = character_movement.direction * character_movement.speed;
    }
}

fn move_character(
    time: Res<Time>,
    mut controllers: Query<
        (&mut KinematicCharacterController, &CharacterMovement),
        Changed<CharacterMovement>,
    >,
) {
    let delta_seconds = time.delta_seconds();
    for (mut controller, character_movement) in &mut controllers {
        let new_position = character_movement.velocity * delta_seconds;
        controller.translation = Some(new_position);
    }
}

fn rotate_character(
    mut query: Query<
        (&mut Transform, &CharacterMovement, &OrientationMode),
        Or<(Changed<OrientationMode>, Changed<CharacterMovement>)>,
    >,
) {
    for (mut transform, character_movement, orientation) in &mut query {
        match orientation {
            OrientationMode::Direction => {
                if character_movement.direction == Vec3::ZERO {
                    continue;
                }
                let direction_2d = Vec2::new(
                    character_movement.direction.x,
                    character_movement.direction.z,
                );
                let rotation_angle = std::f32::consts::PI + direction_2d.angle_between(Vec2::Y);

                let current_rotation = transform.rotation;
                let target_rotation = Quat::from_rotation_y(rotation_angle);
                let interpolated_rotation = current_rotation.lerp(target_rotation, 0.1);

                transform.rotation = interpolated_rotation;
            }
            OrientationMode::Location(location_2d) => {
                let target_position =
                    Vec3::new(location_2d.x, transform.translation.y, location_2d.y);
                let look_direction = transform.translation - target_position; // Reverse direction vector

                if look_direction.length_squared() > 0.0 {
                    let rotation_angle = look_direction.x.atan2(look_direction.z);

                    let current_rotation = transform.rotation;
                    let target_rotation = Quat::from_rotation_y(rotation_angle);
                    let interpolated_rotation = current_rotation.lerp(target_rotation, 0.1);

                    transform.rotation = interpolated_rotation;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use approx::assert_relative_eq;
    use bevy::input::InputPlugin;
    use bevy_dolly::prelude::{Rig, YawPitch};
    use leafwing_input_manager::{prelude::MockInput, InputManagerBundle};

    use corp_shared::prelude::{Player, TestUtils};

    use crate::{
        sound::InteractionSoundEvent,
        state::GameState,
        world::ccc::{
            CameraSet, ControlPlugin, ControlSet, MainCameraBundle, MainCameraPlugin,
            MovementBundle, PlayerAction,
        },
    };

    use super::*;

    #[test]
    fn direction_up() {
        // given
        let mut app = setup();
        let player = setup_player(&mut app);
        setup_camera(&mut app);

        // when
        app.send_input(KeyCode::KeyW);
        app.update();

        // then
        let character_movement = app.get::<CharacterMovement>(player);
        assert_eq!(character_movement.direction, -Vec3::Z);
    }

    #[test]
    fn direction_up_left() {
        // given
        let mut app = setup();
        let player = setup_player(&mut app);
        setup_camera(&mut app);

        // when
        app.send_input(KeyCode::KeyW);
        app.send_input(KeyCode::KeyA);
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
        setup_camera(&mut app);

        // when
        app.send_input(KeyCode::KeyW);
        app.update();

        // then
        let expected_translation = Vec3::new(0.0, 0.0, -1.42 * 3.0);
        let translation_result = app.get::<Transform>(player).translation;
        assert_relative_eq!(
            translation_result.z,
            expected_translation.z,
            epsilon = 0.00001
        );
        assert_eq!(translation_result.z, expected_translation.z);
    }

    #[test]
    fn move_north_1_second() {
        // given
        let mut app = setup();
        setup_camera(&mut app);
        let player = setup_player(&mut app);

        // when
        app.send_input(KeyCode::KeyW);
        app.update();

        // then
        let expected_translation = Vec3::new(0.0, 0.0, -1.42 * 3.0);
        let translation_result = app.get::<Transform>(player).translation;
        assert_relative_eq!(translation_result.z, expected_translation.z, epsilon = 0.01);
    }

    #[test]
    fn dead_player_cannot_move() {
        // given
        let mut app = setup();
        setup_camera(&mut app);
        let player = setup_player(&mut app);

        // when
        app.get_mut::<Health>(player).kill_mut();
        app.send_input(KeyCode::KeyW);
        app.update();

        // then
        let character_movement = app.get::<CharacterMovement>(player);
        assert!(!character_movement.can_move);
        assert_eq!(character_movement.direction, Vec3::ZERO);
        assert_eq!(character_movement.velocity, Vec3::ZERO);
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
        app.set_state(GameState::Playing);
        app
    }

    fn setup_player(app: &mut App) -> Entity {
        app.world_mut()
            .spawn((
                TransformBundle::default(),
                Player,
                InputManagerBundle {
                    input_map: PlayerAction::player_input_map(),
                    ..default()
                },
                Health::default(),
                MovementBundle::default(),
            ))
            .id()
    }

    fn setup_camera(app: &mut App) -> Entity {
        let camera = app
            .world_mut()
            .spawn(MainCameraBundle::new(Vec3::ZERO))
            .id();
        app.get_mut::<Rig>(camera)
            .driver_mut::<YawPitch>()
            .rotate_yaw_pitch(-45.0, 0.0);
        app.update_after(Duration::from_secs_f32(1.0));
        camera
    }
}
