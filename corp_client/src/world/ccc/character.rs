use crate::prelude::*;
use bevy::prelude::*;
use bevy_tnua::{builtins::TnuaBuiltinWalk, prelude::TnuaController};
use corp_shared::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CharacterSet {
    Movement,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                is_movement_enabled,
                calculate_character_movement,
                apply_controller_controls,
                rotate_character,
            )
                .chain()
                .run_if(in_state(GameState::Playing))
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

fn apply_controller_controls(
    q_movement: Query<&CharacterMovement, With<Player>>,
    mut q_tnua: Query<&mut TnuaController>,
) {
    let Ok(mut controller) = q_tnua.get_single_mut() else {
        warn!("Failed to get tnua controller");
        return;
    };

    let Ok(movement) = q_movement.get_single() else {
        return;
    };

    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: movement.velocity,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 1.5,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });
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
                let rotation_angle = std::f32::consts::PI + direction_2d.angle_to(Vec2::Y);

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
