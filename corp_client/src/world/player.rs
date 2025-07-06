use crate::prelude::*;
use aeronet::io::Session;
use avian3d::prelude::*;
use bevy::{ecs::system::SystemId, prelude::*, scene::SceneInstanceReady};
use bevy_tnua::prelude::TnuaController;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use corp_shared::prelude::*;
use rand::seq::SliceRandom;

#[derive(Resource)]
pub struct PlayerSystems {
    pub health: Health,
    pub spawn_player_body: SystemId<In<Entity>>,
    pub setup_camera: SystemId,
}

/// Marks [`Player`] as locally controlled.
#[derive(Component)]
pub struct LocalPlayer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_player_one_shoot_systems)
            .add_observer(on_make_local);
    }
}

fn register_player_one_shoot_systems(mut commands: Commands) {
    let player_data = PlayerSystems {
        health: Default::default(),
        spawn_player_body: commands.register_system(spawn_player_body),
        setup_camera: commands.register_system(setup_camera),
    };
    commands.insert_resource(player_data)
}

fn on_make_local(
    trigger: Trigger<MakeLocal>,
    mut commands: Commands,
    r_player_systems: Res<PlayerSystems>,
) -> Result {
    info!("MakeLocal received {:?}", trigger.target());
    let player_e = trigger.target();
    commands.entity(trigger.target()).insert(LocalPlayer);
    commands.run_system_with(r_player_systems.spawn_player_body, player_e);
    Ok(())
}

pub fn spawn_player_body(
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
            Transform::from_translation(rnd_node_position + Vec3::Y),
            Visibility::default(),
            MovementBundle::default(),
            MainCameraFollow,
            Inventory,
            PlayerFactionInfo {
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
                     r_player_systems: Res<PlayerSystems>| {
                        info!("Player Scene Instance Ready");
                        commands.run_system(r_player_systems.setup_camera);
                        commands.set_state(GameState::Playing);
                    },
                );
        });

    info!("Spawned player entity: {:?}", e_player);
}
