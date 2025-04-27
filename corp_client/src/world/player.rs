use crate::prelude::*;
use avian3d::prelude::*;
use bevy::{ecs::system::SystemId, prelude::*, scene::SceneInstanceReady};
use bevy_tnua::prelude::TnuaController;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use corp_shared::prelude::*;
use leafwing_input_manager::InputManagerBundle;
use rand::seq::SliceRandom;

#[derive(Resource)]
pub struct PlayerSystems {
    pub health: Health,
    pub setup_player: SystemId<In<Entity>>,
    pub setup_camera: SystemId,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_player_one_shoot_systems)
            .add_systems(OnEnter(LoadingSubState::SpawnPlayer), spawn_player);
    }
}

fn register_player_one_shoot_systems(mut commands: Commands) {
    let player_data = PlayerSystems {
        health: Default::default(),
        setup_player: commands.register_system(setup_player),
        setup_camera: commands.register_system(setup_camera),
    };
    commands.insert_resource(player_data)
}

fn spawn_player(
    mut commands: Commands,
    r_player_systems: Res<PlayerSystems>,
    r_player_entity: Res<PlayerEntity>,
) {
    if let Some(player_entity) = r_player_entity.0 {
        commands.run_system_with_input(r_player_systems.setup_player, player_entity);
    } else {
        error!("Failed to spawn player. Player entity not set.");
    }
}

pub fn setup_player(
    In(e_player): In<Entity>,
    r_player_data: Res<PlayerSystems>,
    r_player_assets: Res<PlayerAssets>,
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

    commands
        .entity(e_player)
        .insert((
            Name::new("Player"),
            InputManagerBundle::with_map(CharacterAction::player_input_map()),
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
                    [GameLayer::Area, GameLayer::Sensor, GameLayer::Structure],
                ),
            ),
        ))
        .with_children(|child_builder| {
            child_builder
                .spawn((
                    SceneRoot(r_player_assets.mannequiny.clone()),
                    // Offset the mesh y position by capsule total height
                    Transform::from_xyz(0.0, -1.5, 0.0),
                ))
                .observe(
                    |_trigger: Trigger<SceneInstanceReady>,
                     mut commands: Commands,
                     r_player_systems: Res<PlayerSystems>,
                     mut next_state: ResMut<NextState<GameState>>| {
                        info!("Player Scene Instance Ready");
                        commands.run_system(r_player_systems.setup_camera);
                        next_state.set(GameState::Playing);
                    },
                );
        });

    info!("Spawned player entity: {:?}", e_player);
}
