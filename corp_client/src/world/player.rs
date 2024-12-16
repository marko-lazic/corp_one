use crate::{asset::PlayerAssets, state::GameState, world::prelude::*};
use avian3d::prelude::*;
use bevy::{ecs::system::SystemId, prelude::*};
use bevy_tnua::prelude::TnuaControllerBundle;
use corp_shared::prelude::*;
use leafwing_input_manager::InputManagerBundle;
use rand::seq::SliceRandom;

#[derive(Resource)]
pub struct PlayerStore {
    pub health: Health,
    pub setup_player: SystemId,
}

#[derive(Event)]
pub enum PlayerSpawnEvent {
    SpawnRandom,
    PlayerSpawned,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerSpawnEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
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
            InputManagerBundle::with_map(PlayerAction::player_input_map()),
            SpatialBundle::from_transform(player_spawn_position),
            Player,
            MovementBundle::default(),
            MainCameraFollow,
            Inventory::default(),
            MemberOf {
                faction: Faction::EC,
                rank: Rank::R6,
            },
            r_player_store.health.clone(),
            StateScoped(GameState::Playing),
            // Physics
            (
                RigidBody::Dynamic,
                TnuaControllerBundle::default(),
                Collider::capsule(0.3, 0.75),
                LockedAxes::ROTATION_LOCKED,
                CollisionLayers::new(
                    [Layer::Player],
                    [Layer::VortexGate, Layer::Zone, Layer::Sensor],
                ),
            ),
        ))
        .with_children(|child_builder| {
            child_builder.spawn((SceneBundle {
                scene: r_player_assets.mannequiny.clone(),
                // Offset the mesh y position by capsule total height
                transform: Transform::from_xyz(0.0, -1.5, 0.0),
                ..default()
            },));
        })
        .id();

    info!("Setup Camera");
    commands.spawn(MainCameraBundle::new(rnd_node_position));
    *r_player_entity = player.into();
    r_player_spawn_event.send(PlayerSpawnEvent::PlayerSpawned);
}
