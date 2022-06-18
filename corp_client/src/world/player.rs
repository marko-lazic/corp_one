use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody};
use iyes_loopless::condition::ConditionSet;
use rand::seq::SliceRandom;

use corp_shared::prelude::*;

use crate::asset::asset_loading::PlayerAssets;
use crate::constants::state::GameState;
use crate::input::input_command::PlayerDirection;
use crate::input::{Cursor, InputSystem};
use crate::world::animator::{AnimationComponent, PlayerAnimationAction};
use crate::world::character::Movement;
use crate::world::cloning::CloningPlugin;
use crate::world::colony::vortex::VortexNode;
use crate::world::colony::Layer;
use crate::world::WorldSystem;
use crate::Game;

#[derive(Default, bevy::ecs::component::Component)]
pub struct Player {
    pub is_moving: bool,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CloningPlugin);

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::SpawnPlayer)
                .label(WorldSystem::PlayerSetup)
                .with_system(PlayerPlugin::setup_player)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .after(InputSystem::CheckInteraction)
                .with_system(Self::move_player)
                .with_system(Self::handle_animation_action)
                .with_system(Self::orient_player)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(Self::handle_dead)
                .into(),
        );
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
            .spawn_bundle(TransformBundle::from(player_tr))
            .with_children(|parent| {
                parent.spawn_scene(player_assets.mannequiny.clone());
            })
            .insert(Player::default())
            .insert(Movement::default())
            .insert(game.health.clone())
            .insert(RigidBody::Dynamic)
            .insert(AnimationComponent::new(PlayerAnimationAction::IDLE))
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(0.5, 1.0, 0.5),
                border_radius: None,
            })
            .insert(
                CollisionLayers::none()
                    .with_group(Layer::Player)
                    .with_masks(vec![Layer::Zone, Layer::VortexGate]),
            )
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
        mut query: Query<(&mut Player, &mut Movement, &mut Transform)>,
    ) {
        if let Ok((mut player, mut movement, mut position)) = query.get_single_mut() {
            let direction = player_direction.new_direction();
            if movement.can_move {
                position.translation += movement.update_velocity(direction) * time.delta_seconds();
            }

            player.is_moving = Self::is_moving(&movement.velocity);
            player_direction.reset();
        }
    }

    fn orient_player(mut query: Query<(&Player, &mut Transform)>, cursor: Res<Cursor>) {
        if let Ok((_, mut transform)) = query.get_single_mut() {
            let direction = Vec3::new(cursor.world.x, 0.0, cursor.world.z);
            transform.look_at(direction, Vec3::Y);
        }
    }

    fn handle_animation_action(
        mut query: Query<(&Player, &mut AnimationComponent)>,
        mut last_action: Local<PlayerAnimationAction>,
    ) {
        if let Ok((player, mut animation_component)) = query.get_single_mut() {
            if player.is_moving && *last_action == PlayerAnimationAction::IDLE {
                animation_component.next = Some(PlayerAnimationAction::RUN);
                *last_action = PlayerAnimationAction::RUN;
            }

            if !player.is_moving && *last_action == PlayerAnimationAction::RUN {
                animation_component.next = Some(PlayerAnimationAction::IDLE);
                *last_action = PlayerAnimationAction::IDLE;
            }
        }
    }

    fn is_moving(delta_move: &Vec3) -> bool {
        delta_move.ne(&Vec3::ZERO)
    }
}
