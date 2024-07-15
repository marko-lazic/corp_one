use bevy::{ecs::system::SystemId, prelude::*};
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::InputManagerBundle;
use rand::seq::SliceRandom;

use corp_shared::prelude::*;

use crate::{
    asset::PlayerAssets,
    state::{Despawn, GameState},
    world::{
        ccc::{MainCameraBundle, MainCameraFollow, MovementBundle, PlayerAction, PlayerEntity},
        cloning::CloningPlugin,
        colony::prelude::VortexNode,
        physics::CollideGroups,
    },
};

#[derive(Resource)]
pub struct PlayerStore {
    pub health: Health,
    pub setup_player: SystemId,
}

#[derive(Bundle)]
struct PlayerPhysicsBundle {
    rigid_body: RigidBody,
    kcc: KinematicCharacterController,
    collider: Collider,
    locked_axis: LockedAxes,
    collide_groups: CollisionGroups,
    active_events: ActiveEvents,
}

#[derive(Event)]
pub enum PlayerSpawnEvent {
    SpawnRandom,
    PlayerSpawned,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CloningPlugin)
            .add_event::<PlayerSpawnEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                player_spawn_event_reader.run_if(in_state(GameState::LoadColony)),
            );
    }
}

fn setup(world: &mut World) {
    let setup_player = world.register_system(setup_player);
    world.insert_resource(PlayerStore {
        health: Default::default(),
        setup_player,
    })
}

fn player_spawn_event_reader(
    mut r_player_spawn_event: EventReader<PlayerSpawnEvent>,
    mut commands: Commands,
    player_store: Res<PlayerStore>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in r_player_spawn_event.read() {
        match event {
            PlayerSpawnEvent::SpawnRandom => commands.run_system(player_store.setup_player),
            PlayerSpawnEvent::PlayerSpawned => next_state.set(GameState::Playing),
        }
    }
}

pub fn setup_player(
    r_player_store: Res<PlayerStore>,
    r_player_assets: Res<PlayerAssets>,
    mut r_player_entity: ResMut<PlayerEntity>,
    mut q_vortex_node_pos: Query<&mut Transform, With<VortexNode>>,
    mut commands: Commands,
    mut r_player_spawn_event: EventWriter<PlayerSpawnEvent>,
) {
    let rnd_node_position = q_vortex_node_pos
        .iter_mut()
        .map(|t| t.translation)
        .collect::<Vec<Vec3>>()
        .choose(&mut rand::thread_rng())
        .map(|p| p.to_owned())
        .unwrap_or_else(|| Vec3::new(1.0, 1.0, 1.0));

    let player_spawn_position = Transform::from_translation(rnd_node_position + Vec3::Y);

    let player = commands
        .spawn((
            Name::new("Player"),
            InputManagerBundle {
                input_map: PlayerAction::player_input_map(),
                ..default()
            },
            SpatialBundle::from_transform(player_spawn_position),
            Player,
            MovementBundle::default(),
            MainCameraFollow,
            MemberOf {
                faction: Faction::EC,
                rank: Rank::R6,
            },
            r_player_store.health.clone(),
            PlayerPhysicsBundle {
                rigid_body: RigidBody::KinematicPositionBased,
                kcc: KinematicCharacterController {
                    offset: CharacterLength::Absolute(0.01),
                    ..default()
                },
                collider: Collider::capsule_y(0.65, 0.25),
                locked_axis: LockedAxes::ROTATION_LOCKED,
                collide_groups: CollideGroups::player(),
                active_events: ActiveEvents::COLLISION_EVENTS,
            },
            Despawn,
        ))
        .with_children(|child_builder| {
            child_builder.spawn(SceneBundle {
                scene: r_player_assets.mannequiny.clone(),
                // Offset the mesh y position by capsule total height
                transform: Transform::from_xyz(0.0, -1.0, 0.0),
                ..default()
            });
        })
        .id();

    info!("Setup Camera");
    commands.spawn(MainCameraBundle::new(rnd_node_position));
    *r_player_entity = player.into();
    r_player_spawn_event.send(PlayerSpawnEvent::PlayerSpawned);
}
