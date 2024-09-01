use crate::{
    asset::{Colony, ColonyConfig, MaterialAssets, MeshAssets, SceneAssets},
    state::{Despawn, GameState},
    world::{
        colony::{prelude::Zone, scene_hook},
        prelude::{CollideGroups, PhysicsSystems, PlayerSpawnEvent},
        shader::ForceFieldMaterial,
    },
};
use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    scene::SceneInstanceReady,
};
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::{Listener, On},
    PickableBundle,
};
use bevy_rapier3d::{
    dynamics::RigidBody,
    geometry::{Collider, Sensor},
};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};
use corp_shared::prelude::{
    on_use_backpack_action_event, on_use_backpack_event, BackpackBundle, HackingToolBundle,
    InteractionObjectType,
};

#[derive(Event)]
pub struct ColonyLoadEvent(pub Handle<ColonyConfig>);

#[derive(Resource)]
struct ColonyScene(Entity);

pub fn colony_loader_plugin(app: &mut App) {
    app.add_event::<ColonyLoadEvent>()
        .add_systems(OnEnter(GameState::LoadColony), load_colony_event)
        .add_systems(
            Update,
            check_colony_loaded
                .run_if(in_state(GameState::LoadColony))
                .run_if(resource_exists::<ColonyScene>),
        )
        .add_systems(OnEnter(GameState::Playing), update_lights);
}

fn check_colony_loaded(
    mut ev_scene_instance_ready: EventReader<SceneInstanceReady>,
    mut ev_player_spawn: EventWriter<PlayerSpawnEvent>,
    r_colony_scene: Res<ColonyScene>,
    r_physics_systems: Res<PhysicsSystems>,
    mut commands: Commands,
) {
    for event in ev_scene_instance_ready.read() {
        if event.parent == r_colony_scene.0 {
            commands.run_system(r_physics_systems.setup_colliders);
            ev_player_spawn.send(PlayerSpawnEvent::SpawnRandom);
        }
    }
}

fn load_colony_event(
    mut ev_colony_load: EventReader<ColonyLoadEvent>,
    r_colony_config: Res<Assets<ColonyConfig>>,
    r_scene_assets: Res<SceneAssets>,
    mut r_meshes: ResMut<Assets<Mesh>>,
    mut r_materials: ResMut<Assets<StandardMaterial>>,
    r_mesh_assets: Res<MeshAssets>,
    r_material_assets: Res<MaterialAssets>,
    mut r_force_field_materials: ResMut<Assets<ForceFieldMaterial>>,
    mut commands: Commands,
) {
    info!("Setup colony");
    let Some(colony_load_event) = ev_colony_load.read().last() else {
        return;
    };

    let current_colony = r_colony_config.get(&colony_load_event.0).unwrap();

    let colony_scene = match current_colony.name {
        Colony::Cloning => r_scene_assets.cloning.clone(),
        Colony::Iris => r_scene_assets.iris.clone(),
        Colony::Liberte => r_scene_assets.liberte.clone(),
        _ => r_scene_assets.liberte.clone(),
    };

    // spawn scene
    let colony_scene = commands
        .spawn((
            Name::new("Colony"),
            HookedSceneBundle {
                scene: SceneBundle {
                    scene: colony_scene,
                    ..default()
                },
                hook: SceneHook::new(move |entity_ref, commands| {
                    if let Some(name) = entity_ref.get::<Name>().map(|t| t.as_str()) {
                        scene_hook::scene_hook_insert_components(entity_ref.id(), name, commands)
                    }
                }),
            },
            Despawn,
        ))
        .id();

    commands.insert_resource(ColonyScene(colony_scene));

    let e_hacking_tool = commands.spawn(HackingToolBundle::default()).id();

    commands
        .spawn((
            Name::new("Backpack"),
            SceneBundle {
                scene: r_mesh_assets.low_poly_backpack.clone(),
                transform: Transform::from_xyz(6.0, 0.5, -3.0).with_scale(Vec3::splat(0.2)),
                ..default()
            },
            BackpackBundle::with_items(vec![e_hacking_tool]),
            PickableBundle::default(),
            InteractionObjectType::Backpack,
            Collider::cuboid(1.5, 3.5, 1.5),
            Despawn,
        ))
        .observe(on_use_backpack_event)
        .observe(on_use_backpack_action_event);

    commands.spawn((
        Name::new("Debug Cube"),
        MaterialMeshBundle {
            mesh: r_meshes.add(Cuboid::new(5.0, 5.0, 5.0)),
            material: r_force_field_materials.add(ForceFieldMaterial {}),
            transform: Transform::from_xyz(10., 0., 0.),
            ..default()
        },
        NotShadowReceiver,
        NotShadowCaster,
        Collider::cuboid(2.6, 2.6, 2.6),
        PickableBundle::default(),
        On::<Pointer<Click>>::run(|event: Listener<Pointer<Click>>| {
            info!("Clicked on entity {:?}", event.target);
        }),
        Despawn,
    ));

    commands.spawn((
        Name::new("Debug Ground"),
        PbrBundle {
            mesh: r_meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
            transform: Transform::from_translation(Vec3::new(4., -0.01, 4.)),
            material: r_materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 0.0,
                reflectance: 0.0,
                metallic: 0.0,
                ..default()
            }),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(100.0, 0.01, 100.0),
        Despawn,
    ));

    // spawn zones
    for zone_asset in &current_colony.zones {
        commands.spawn((
            Name::new(format!("Zone {:?}", zone_asset.zone_type)),
            PbrBundle {
                mesh: r_meshes.add(
                    Plane3d::default()
                        .mesh()
                        .size(zone_asset.size, zone_asset.size),
                ),
                transform: Transform::from_translation(zone_asset.position),
                material: r_material_assets.get_material(&zone_asset.material),
                ..default()
            },
            Sensor,
            Collider::cuboid(0.5, 1.0, 0.5),
            Zone::from(*zone_asset),
            CollideGroups::zone(),
            Despawn,
        ));
    }
}

// Temporary fixes the problem with shadows not working
fn update_lights(mut query: Query<&mut PointLight>) {
    info!("Update lights");
    for mut point_light in query.iter_mut() {
        point_light.shadows_enabled = true;
        point_light.intensity = 100_000.0;
    }
}
