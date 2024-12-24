use crate::prelude::*;
use avian3d::prelude::*;
use bevy::{ecs::system::SystemId, prelude::*, scene::SceneInstanceReady};
use bevy_tnua::prelude::TnuaController;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use corp_shared::prelude::*;
use leafwing_input_manager::InputManagerBundle;
use rand::seq::SliceRandom;

#[derive(Resource)]
pub struct PlayerData {
    pub health: Health,
    pub setup_player: SystemId,
    pub setup_camera: SystemId,
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

fn setup(mut commands: Commands) {
    let player_data = PlayerData {
        health: Default::default(),
        setup_player: commands.register_system(setup_player),
        setup_camera: commands.register_system(setup_camera),
    };
    commands.insert_resource(player_data)
}

fn player_spawn_event_reader(
    mut r_player_spawn_event: EventReader<PlayerSpawnEvent>,
    mut commands: Commands,
    player_store: Res<PlayerData>,
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
    r_player_data: Res<PlayerData>,
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
        .unwrap_or_else(|| Vec3::new(1.0, 10.0, 1.0));

    let player =
        commands
            .spawn((
                Name::new("Player"),
                InputManagerBundle::with_map(PlayerAction::player_input_map()),
                Transform::from_translation(rnd_node_position + Vec3::Y),
                Visibility::default(),
                Player,
                MovementBundle::default(),
                MainCameraFollow,
                Inventory::default(),
                MemberOf {
                    faction: Faction::EC,
                    rank: Rank::R6,
                },
                r_player_data.health.clone(),
                StateScoped(GameState::Playing),
                // Physics
                (
                    RigidBody::Dynamic,
                    Collider::capsule(0.3, 0.75),
                    TnuaController::default(),
                    TnuaAvian3dSensorShape(Collider::cylinder(0.29, 0.0)),
                    LockedAxes::ROTATION_LOCKED,
                    CollisionLayers::new(
                        [GameLayer::Player],
                        [GameLayer::Zone, GameLayer::Sensor, GameLayer::Fixed],
                    ),
                ),
            ))
            .with_children(|child_builder| {
                child_builder.spawn((
                SceneRoot(r_player_assets.mannequiny.clone()),
                // Offset the mesh y position by capsule total height
                Transform::from_xyz(0.0, -1.5, 0.0),
            )).observe(
                |trigger: Trigger<SceneInstanceReady>,
                 mut commands: Commands,
                 r_player_data: Res<PlayerData>,
                 mut ev_player_spawn: EventWriter<PlayerSpawnEvent>| {
                    info!("observed: {:?}", trigger);
                    commands.run_system(r_player_data.setup_camera);
                    ev_player_spawn.send(PlayerSpawnEvent::PlayerSpawned);
                },
            );
            })
            .id();

    *r_player_entity = player.into();
    info!("Player entity: {:?}", r_player_entity);
}
