use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};

use crate::{
    asset::{Colony, ColonyConfig, MaterialAssets, SceneAssets},
    state::{Despawn, GameState},
    world::{
        colony::{
            colony_interaction::ColonyInteractionPlugin,
            vortex::{VortexNode, VortexPlugin},
            zone::Zone,
        },
        physics, WorldSystemSet,
    },
};

pub mod barrier;
mod colony_interaction;
mod scene_hook;
pub mod territory;
pub mod vortex;
pub mod zone;

#[derive(Resource, Default)]
pub struct ColonyStore {
    pub current_colony_config: Handle<ColonyConfig>,
}

pub struct ColonyPlugin;

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ColonyStore>()
            .add_plugins((
                VortexPlugin,
                ColonyInteractionPlugin,
                HookPlugin,
                RonAssetPlugin::<ColonyConfig>::new(&["colony"]),
                DefaultPickingPlugins,
            ))
            .add_systems(
                OnEnter(GameState::LoadColony),
                (setup_colony, setup_debug_plane, setup_zones).chain(),
            )
            .add_systems(
                Update,
                next_state_spawn_player
                    .run_if(is_colony_loaded)
                    .run_if(in_state(GameState::LoadColony)),
            )
            .add_systems(
                Update,
                next_state_playing
                    .after(WorldSystemSet::CameraSetup)
                    .run_if(in_state(GameState::SpawnPlayer)),
            )
            .add_systems(OnEnter(GameState::Playing), update_lights);
    }
}

fn setup_colony(
    r_colony_config: Res<Assets<ColonyConfig>>,
    r_scene_assets: Res<SceneAssets>,
    r_colony_store: Res<ColonyStore>,
    mut commands: Commands,
) {
    info!("Setup colony");
    let current_colony = r_colony_config
        .get(&r_colony_store.current_colony_config)
        .unwrap();

    let colony_scene = match current_colony.name {
        Colony::Cloning => r_scene_assets.cloning.clone(),
        Colony::Iris => r_scene_assets.iris.clone(),
        Colony::Liberte => r_scene_assets.liberte.clone(),
        _ => r_scene_assets.liberte.clone(),
    };

    commands.spawn((
        HookedSceneBundle {
            scene: SceneBundle {
                scene: colony_scene,
                ..default()
            },
            hook: SceneHook::new(|entity_ref, commands| {
                if let Some(name) = entity_ref.get::<Name>().map(|t| t.as_str()) {
                    scene_hook::scene_hook_insert_components(name, commands)
                }
            }),
        },
        Name::new("Colony"),
        Despawn,
    ));
}

fn setup_debug_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(10., 0., 0.),
            ..Default::default()
        },
        Collider::cuboid(2.6, 2.6, 2.6),
        PickableBundle::default(),
        On::<Pointer<Click>>::run(|event: Listener<Pointer<Click>>| {
            info!("Clicked on entity {:?}", event.target);
        }),
        Despawn,
    ));

    info!("Setup debug plane");
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 100.0,
                ..default()
            })),
            transform: Transform::from_translation(Vec3::new(4., -0.01, 4.)),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 0.0,
                reflectance: 0.0,
                metallic: 0.0,
                ..Default::default()
            }),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(100.0, 0.01, 100.0),
        Despawn,
    ));
}

fn setup_zones(
    r_colony_store: Res<ColonyStore>,
    r_colony_config_assets: Res<Assets<ColonyConfig>>,
    r_material_assets: Res<MaterialAssets>,
    mut r_mesh_assets: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    info!("Setup zones");
    if let Some(colony_asset) = r_colony_config_assets.get(&r_colony_store.current_colony_config) {
        for zone_asset in &colony_asset.zones {
            commands.spawn((
                PbrBundle {
                    mesh: r_mesh_assets.add(Mesh::from(shape::Plane {
                        size: zone_asset.size,
                        ..default()
                    })),
                    transform: Transform::from_translation(zone_asset.position),
                    material: r_material_assets.get_material(&zone_asset.material),
                    ..Default::default()
                },
                Sensor,
                Collider::cuboid(0.5, 1.0, 0.5),
                Zone::from(*zone_asset),
                physics::CollideGroups::zone(),
                Despawn,
            ));
        }
    }
}

fn is_colony_loaded(vortex_nodes: Query<Entity, With<VortexNode>>) -> bool {
    vortex_nodes.iter().count() > 0
}

fn next_state_spawn_player(mut next_state: ResMut<NextState<GameState>>) {
    info!("State: Spawn Player");
    next_state.set(GameState::SpawnPlayer);
}

fn next_state_playing(mut next_state: ResMut<NextState<GameState>>) {
    info!("State: Playing");
    next_state.set(GameState::Playing);
}

// Temporary fixes the problem with shadows not working
fn update_lights(mut query: Query<&mut PointLight>) {
    info!("Update lights");
    for mut point_light in query.iter_mut() {
        point_light.shadows_enabled = true;
    }
}
