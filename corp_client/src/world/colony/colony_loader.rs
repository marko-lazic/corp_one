use crate::{prelude::*, world::colony::scene_hook};
use avian3d::prelude::*;
use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    scene::SceneInstanceReady,
};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};
use corp_shared::prelude::*;

#[derive(Event)]
pub struct ColonyLoadEvent(pub Handle<ColonyConfig>);

pub fn colony_loader_plugin(app: &mut App) {
    app.add_event::<ColonyLoadEvent>()
        .add_systems(OnEnter(GameState::LoadColony), load_colony_event)
        .add_systems(OnExit(GameState::LoadColony), update_lights);
}

fn load_colony_event(
    mut ev_colony_load: EventReader<ColonyLoadEvent>,
    r_colony_config: Res<Assets<ColonyConfig>>,
    r_scene_assets: Res<SceneAssets>,
    mut r_meshes: ResMut<Assets<Mesh>>,
    mut r_materials: ResMut<Assets<StandardMaterial>>,
    mut r_force_field_materials: ResMut<Assets<ForceFieldMaterial>>,
    mut commands: Commands,
) {
    info!("Setup colony");
    let Some(colony_load_event) = ev_colony_load.read().last() else {
        return;
    };
    let current_colony = r_colony_config.get(&colony_load_event.0).unwrap();
    info!("Setup colony {:?}", current_colony);

    let colony_scene = match current_colony.name {
        Colony::Cloning => r_scene_assets.cloning.clone(),
        Colony::Iris => r_scene_assets.iris.clone(),
        Colony::Liberte => r_scene_assets.liberte.clone(),
        _ => r_scene_assets.liberte.clone(),
    };

    // spawn scene
    commands
        .spawn((
            Name::new("Colony"),
            HookedSceneBundle {
                scene: SceneRoot(colony_scene),
                hook: SceneHook::new(move |entity_ref, commands| {
                    if let Some(name) = entity_ref.get::<Name>().map(|t| t.as_str()) {
                        scene_hook::components(entity_ref.id(), name, commands)
                    }
                }),
            },
            StateScoped(GameState::Playing),
        ))
        .observe(
            |_trigger: Trigger<SceneInstanceReady>,
             mut commands: Commands,
             r_physics_systems: Res<PhysicsSystems>| {
                info!("Colony loaded");
                commands.run_system(r_physics_systems.setup_colliders);
            },
        );

    commands
        .spawn((
            Name::new("Debug Cube"),
            Mesh3d(r_meshes.add(Cuboid::new(5.0, 5.0, 5.0))),
            MeshMaterial3d(r_force_field_materials.add(ForceFieldMaterial {})),
            Transform::from_xyz(10., 0., 0.),
            NotShadowReceiver,
            NotShadowCaster,
            RigidBody::Static,
            Collider::cuboid(5.0, 5.0, 5.0),
            StateScoped(GameState::Playing),
        ))
        .observe(|click: Trigger<Pointer<Click>>| {
            info!("Clicked on debug cube {:?}", click.target);
        });

    commands.spawn((
        Name::new("Debug Ground"),
        Mesh3d(r_meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
        Transform::from_translation(Vec3::new(4., -0.01, 4.)),
        MeshMaterial3d(r_materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.0,
            reflectance: 0.0,
            metallic: 0.0,
            ..default()
        })),
        RigidBody::Static,
        CollisionLayers::new([GameLayer::Fixed], [GameLayer::Player]),
        Collider::cuboid(100.0, 0.01, 100.0),
        StateScoped(GameState::Playing),
    ));

    // spawn zones
    for zone_asset in &current_colony.zones {
        commands.spawn((
            Name::new(format!("Zone {:?}", zone_asset.zone_type)),
            Zone::from(*zone_asset),
            Sensor,
            Collider::cuboid(zone_asset.size, 2.0, zone_asset.size),
            Transform::from_translation(zone_asset.position + Vec3::Y),
            Visibility::default(),
            CollisionLayers::new([GameLayer::Zone], [GameLayer::Player]),
            StateScoped(GameState::Playing),
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
