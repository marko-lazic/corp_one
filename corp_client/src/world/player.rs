use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::seq::SliceRandom;

use corp_shared::prelude::*;

use crate::asset::asset_loading::PlayerAssets;
use crate::input::input_command::PlayerDirection;
use crate::input::{Cursor, InputSystemSet, OrientationMode};
use crate::world::animator::{AnimationComponent, PlayerAnimationAction};
use crate::world::character::Movement;
use crate::world::cloning::CloningPlugin;
use crate::world::colony::vortex::VortexNode;
use crate::world::{physics, WorldSystemSet};
use crate::Game;
use crate::GameState;

#[derive(Default, bevy::ecs::component::Component)]
pub struct Player {
    pub is_moving: bool,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CloningPlugin);
        app.add_system(
            PlayerPlugin::setup_player
                .in_set(WorldSystemSet::PlayerSetup)
                .in_schedule(OnEnter(GameState::SpawnPlayer)),
        );

        app.add_systems(
            (Self::move_player, Self::handle_animation_action)
                .after(InputSystemSet)
                .in_set(OnUpdate(GameState::Playing)),
        );

        app.add_system(
            Self::orientation_aim
                .in_set(OnUpdate(GameState::Playing))
                .after(InputSystemSet)
                .run_if(move |res: Option<Res<OrientationMode>>| {
                    if let Some(res) = res {
                        *res == OrientationMode::Aim
                    } else {
                        false
                    }
                }),
        );
        app.add_system(
            Self::orientation_direction
                .in_set(OnUpdate(GameState::Playing))
                .after(InputSystemSet)
                .run_if(move |res: Option<Res<OrientationMode>>| {
                    if let Some(res) = res {
                        *res == OrientationMode::Direction
                    } else {
                        false
                    }
                }),
        );

        app.add_system(Self::handle_dead.in_set(OnUpdate(GameState::Playing)));
    }
}

impl PlayerPlugin {
    pub fn setup_player(
        mut game: ResMut<Game>,
        mut commands: Commands,
        player_assets: Res<PlayerAssets>,
        mut vortex_nodes: Query<&mut Transform, With<VortexNode>>,
    ) {
        let random_position = vortex_nodes
            .iter_mut()
            .map(|t| t.translation)
            .collect::<Vec<Vec3>>()
            .choose(&mut rand::thread_rng())
            .map(|p| p.to_owned());

        let position = random_position.unwrap_or_else(|| Vec3::new(1.0, 1.0, 1.0));
        let player_tr = Transform::from_translation(position);

        let player = commands
            .spawn(SceneBundle {
                scene: player_assets.mannequiny.clone(),
                transform: player_tr,
                ..default()
            })
            .insert((
                Player::default(),
                Movement::default(),
                game.health.clone(),
                AnimationComponent::new(PlayerAnimationAction::Idle),
                RigidBody::Dynamic,
                Collider::capsule_y(0.25, 0.25),
                LockedAxes::ROTATION_LOCKED,
                Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                ActiveEvents::COLLISION_EVENTS,
                ContactForceEventThreshold(30.0),
                physics::CollideGroups::player(),
            ))
            .id();

        game.player_entity = Some(player);
    }

    fn handle_dead(mut query: Query<(&mut Movement, &Health)>) {
        for (mut movement, health) in query.iter_mut() {
            if !health.is_dead() {
                movement.can_move = true;
            } else {
                movement.can_move = false;
            }
        }
    }

    fn move_player(
        mut player_direction: ResMut<PlayerDirection>,
        time: Res<Time>,
        mut query: Query<(&mut Player, &mut Movement, &mut Transform, &Health)>,
    ) {
        if let Ok((mut player, mut movement, mut position, health)) = query.get_single_mut() {
            let direction = player_direction.new_direction();
            movement.update_direction(direction);
            if movement.can_move {
                movement.update_velocity();
                position.translation += movement.velocity * time.delta_seconds();
            }

            player.is_moving = Self::is_moving(&movement.velocity) && health.is_alive();
            player_direction.reset();
        }
    }

    fn orientation_aim(mut query: Query<(&Player, &mut Transform)>, cursor: Res<Cursor>) {
        if let Ok((_, mut transform)) = query.get_single_mut() {
            let direction = Vec3::new(cursor.world.x, 0.0, cursor.world.z);
            transform.look_at(direction, Vec3::Y);
        }
    }

    fn orientation_direction(
        mut query: Query<(&Player, &mut Movement, &mut Transform)>,
        time: Res<Time>,
        mut prev_dir: Local<Vec3>,
    ) {
        if let Ok((_, mut movement, mut transform)) = query.get_single_mut() {
            if !movement.is_direction_zero() && *prev_dir != movement.direction {
                movement.target_rotation =
                    Self::look_at(&transform.translation, movement.velocity * 20.0);
                movement.rotating = true;
                movement.rotation_time = 0.0;
                *prev_dir = movement.direction;
            }

            if movement.rotating {
                movement.rotation_time += time.delta_seconds();
                transform.rotation = transform
                    .rotation
                    .lerp(movement.target_rotation, movement.rotation_time);
            }

            if movement.rotation_time > 1.0 {
                movement.rotating = false;
            }
        }
    }

    fn look_at(translation: &Vec3, target: Vec3) -> Quat {
        let up = Vec3::Y;
        let forward = Vec3::normalize(*translation - target);
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);
        Quat::from_mat3(&Mat3::from_cols(right, up, forward))
    }

    fn handle_animation_action(
        mut query: Query<(&Player, &mut AnimationComponent)>,
        mut last_action: Local<PlayerAnimationAction>,
    ) {
        if let Ok((player, mut animation_component)) = query.get_single_mut() {
            if player.is_moving && *last_action == PlayerAnimationAction::Idle {
                animation_component.next = Some(PlayerAnimationAction::Run);
                *last_action = PlayerAnimationAction::Run;
            }

            if !player.is_moving && *last_action == PlayerAnimationAction::Run {
                animation_component.next = Some(PlayerAnimationAction::Idle);
                *last_action = PlayerAnimationAction::Idle;
            }
        }
    }

    fn is_moving(delta_move: &Vec3) -> bool {
        delta_move.ne(&Vec3::ZERO)
    }
}
