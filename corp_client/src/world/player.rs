use crate::prelude::*;
use avian3d::prelude::*;
use bevy::{prelude::*, scene::SceneInstanceReady};
use bevy_tnua::prelude::TnuaController;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use corp_shared::prelude::*;
use rand::seq::SliceRandom;

/// Marks [`Player`] as locally controlled.
#[derive(Component)]
pub struct LocalPlayer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_setup_local_player);
    }
}

fn on_setup_local_player(
    trigger: Trigger<SetupPlayerServerCommand>,
    r_player_assets: Res<PlayerAssets>,
    mut q_vortex_node_pos: Query<&mut Transform, With<VortexNode>>,
    mut commands: Commands,
    q_local_player: Query<&LocalPlayer>,
) {
    info!("MakeLocal received {:?}", trigger.target());
    let player_e = trigger.target();

    // Validate no LocalPlayer exists
    if q_local_player.iter().count() > 0 {
        error!("Tried to spawn a second local player");
    }

    let rnd_node_position = q_vortex_node_pos
        .iter_mut()
        .map(|t| t.translation)
        .collect::<Vec<Vec3>>()
        .choose(&mut rand::thread_rng())
        .map(|p| p.to_owned())
        .unwrap_or_else(|| Vec3::new(1.0, 10.0, 1.0));

    commands
        .entity(player_e)
        .insert((
            Name::new("Player"),
            LocalPlayer,
            Transform::from_translation(rnd_node_position + Vec3::Y),
            Visibility::default(),
            MovementBundle::default(),
            MainCameraFollow,
            Inventory,
            PlayerFactionInfo {
                faction: Faction::EC,
                rank: Rank::R6,
            },
            StateScoped(GameState::Playing),
            // Physics
            (
                RigidBody::Dynamic,
                CollisionEventsEnabled,
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
                    |_trigger: Trigger<SceneInstanceReady>, mut commands: Commands| {
                        info!("Player Scene Instance Ready");
                        commands.trigger(SetupLocalPlayerCamera);
                        commands.set_state(GameState::Playing);
                    },
                );
        });

    info!("Spawned player entity: {:?}", player_e);
}
