use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::seq::SliceRandom;

use corp_shared::prelude::*;

use crate::{
    asset::PlayerAssets,
    state::{Despawn, GameState},
    world::{
        animator::{AnimationComponent, PlayerAnimationAction},
        ccc::{CharacterMovement, ControlSet, MainCameraFollow, MovementBundle},
        cloning::CloningPlugin,
        colony::vortex::VortexNode,
        physics::CollideGroups,
        WorldSystemSet,
    },
    Game,
};

#[derive(Bundle)]
struct PlayerPhysicsBundle {
    rigid_body: RigidBody,
    collider: Collider,
    locked_axis: LockedAxes,
    friction: Friction,
    active_events: ActiveEvents,
    contact_force_event_threshold: ContactForceEventThreshold,
    collide_groups: CollisionGroups,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CloningPlugin)
            .add_systems(
                OnEnter(GameState::SpawnPlayer),
                setup_player.in_set(WorldSystemSet::PlayerSetup),
            )
            .add_systems(
                Update,
                handle_animation_action
                    .after(ControlSet::PlayingInput)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_player(
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
    let player_transform = Transform::from_translation(position);

    let player = commands
        .spawn((
            SceneBundle {
                scene: player_assets.mannequiny.clone(),
                transform: player_transform,
                ..default()
            },
            Player,
            MovementBundle::default(),
            MainCameraFollow,
            Interactor::default(),
            MemberOf {
                faction: Faction::EC,
                rank: Rank::R6,
            },
            game.health.clone(),
            AnimationComponent::new(PlayerAnimationAction::Idle),
            PlayerPhysicsBundle {
                rigid_body: RigidBody::Dynamic,
                collider: Collider::capsule_y(0.25, 0.25),
                locked_axis: LockedAxes::ROTATION_LOCKED,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                active_events: ActiveEvents::COLLISION_EVENTS,
                contact_force_event_threshold: ContactForceEventThreshold(30.0),
                collide_groups: CollideGroups::player(),
            },
            Despawn,
        ))
        .id();

    game.player_entity = Some(player);
}

fn handle_animation_action(
    mut query: Query<(&CharacterMovement, &mut AnimationComponent), With<Player>>,
    mut last_action: Local<PlayerAnimationAction>,
) {
    if let Ok((player_movement, mut animation_component)) = query.get_single_mut() {
        if player_movement.is_moving() && *last_action == PlayerAnimationAction::Idle {
            animation_component.next = Some(PlayerAnimationAction::Run);
            *last_action = PlayerAnimationAction::Run;
        }

        if !player_movement.is_moving() && *last_action == PlayerAnimationAction::Run {
            animation_component.next = Some(PlayerAnimationAction::Idle);
            *last_action = PlayerAnimationAction::Idle;
        }
    }
}
