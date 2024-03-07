use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::InputManagerBundle;
use rand::seq::SliceRandom;

use corp_shared::prelude::*;

use crate::{
    asset::PlayerAssets,
    state::{Despawn, GameState},
    world::{
        animator::{AnimationComponent, PlayerAnimationAction},
        ccc::{
            CharacterMovement, ControlSet, MainCameraFollow, MovementBundle, PlayerAction,
            PlayerEntity,
        },
        cloning::CloningPlugin,
        colony::vortex::VortexNode,
        physics::CollideGroups,
        WorldSystemSet,
    },
};

#[derive(Resource, Default)]
pub struct PlayerStore {
    pub health: Health,
}

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
            .init_resource::<PlayerStore>()
            .add_systems(
                OnEnter(GameState::SpawnPlayer),
                setup_player.in_set(WorldSystemSet::PlayerSetup),
            )
            .add_systems(
                Update,
                (handle_animation_action)
                    .after(ControlSet::PlayingInput)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_player(
    r_player_store: Res<PlayerStore>,
    r_player_assets: Res<PlayerAssets>,
    mut r_player_entity: ResMut<PlayerEntity>,
    mut q_vortex_node_pos: Query<&mut Transform, With<VortexNode>>,
    mut commands: Commands,
) {
    let rnd_node_position = q_vortex_node_pos
        .iter_mut()
        .map(|t| t.translation)
        .collect::<Vec<Vec3>>()
        .choose(&mut rand::thread_rng())
        .map(|p| p.to_owned())
        .unwrap_or_else(|| Vec3::new(1.0, 1.0, 1.0));

    let player_transform = Transform::from_translation(rnd_node_position);

    let player = commands
        .spawn((
            Player,
            InputManagerBundle {
                input_map: PlayerAction::player_input_map(),
                ..default()
            },
            SpatialBundle::from_transform(player_transform),
            MovementBundle::default(),
            MainCameraFollow,
            MemberOf {
                faction: Faction::EC,
                rank: Rank::R6,
            },
            r_player_store.health.clone(),
            AnimationComponent::new(PlayerAnimationAction::Idle),
            PlayerPhysicsBundle {
                rigid_body: RigidBody::Dynamic,
                collider: Collider::capsule_y(0.65, 0.25),
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
        .with_children(|parent| {
            parent.spawn((SceneBundle {
                scene: r_player_assets.mannequiny.clone(),
                // Offset the mesh y position by capsule total height
                transform: Transform::from_xyz(0.0, -0.9, 0.0),
                global_transform: GlobalTransform::default(),
                ..default()
            },));
        })
        .id();

    *r_player_entity = player.into();
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
