use crate::prelude::*;
use aeronet_replicon::client::AeronetRepliconClient;
use avian3d::prelude::*;
use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    scene::SceneInstanceReady,
};
use bevy_replicon::prelude::ClientTriggerExt;
use bevy_skein::SkeinPlugin;
use corp_shared::prelude::*;
use log::error;

#[derive(Event)]
pub struct LoadColonyCommand;

#[derive(Event)]
pub struct LoadStarMapCommand;

#[derive(Event)]
struct UpdateLightsCommand;

pub fn colony_loader_plugin(app: &mut App) {
    app.add_plugins(SkeinPlugin::default())
        .add_observer(on_load_star_map_command)
        .add_observer(on_load_colony_command)
        .add_observer(on_update_lights_command);
}

fn on_load_star_map_command(_trigger: Trigger<LoadStarMapCommand>, mut commands: Commands) {
    info!("Star Map trigger");
    commands.set_state(GameState::StarMap);
}

fn on_load_colony_command(
    _trigger: Trigger<LoadColonyCommand>,
    mut commands: Commands,
    mut r_meshes: ResMut<Assets<Mesh>>,
    mut r_materials: ResMut<Assets<StandardMaterial>>,
    mut r_force_field_materials: ResMut<Assets<ForceFieldMaterial>>,
    r_scene_assets: Res<SceneAssets>,
    client_colony: Single<&Colony, With<AeronetRepliconClient>>,
) -> Result {
    let colony = *client_colony;

    if *colony == Colony::StarMap {
        error!("Inconsistent state, StarMap is not a colony");
        return Ok(());
    }

    info!("Setup Colony {:?}", colony);

    let colony_scene = match colony {
        Colony::Cloning => r_scene_assets.cloning.clone(),
        Colony::Iris => r_scene_assets.iris.clone(),
        Colony::Liberte => r_scene_assets.liberte.clone(),
        _ => r_scene_assets.liberte.clone(),
    };

    // spawn scene
    commands
        .spawn((
            Name::new("Colony"),
            SceneRoot(colony_scene),
            StateScoped(GameState::Playing),
        ))
        .observe(on_colony_loaded);

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
        Mesh3d(r_meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        Transform::from_translation(Vec3::new(4., -0.01, 4.)),
        MeshMaterial3d(r_materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.0,
            reflectance: 0.0,
            metallic: 0.0,
            ..default()
        })),
        RigidBody::Static,
        CollisionLayers::new([GameLayer::Structure], [GameLayer::Player]),
        Collider::cuboid(50.0, 0.01, 50.0),
        StateScoped(GameState::Playing),
    ));
    Ok(())
}

fn on_colony_loaded(_trigger: Trigger<SceneInstanceReady>, mut commands: Commands) {
    info!("Colony Scene Instance Ready");
    commands.client_trigger(PlayerSpawnClientCommand);
    commands.trigger(UpdateLightsCommand);
}

// Temporarily fixes the problem with shadows not working
fn on_update_lights_command(
    _trigger: Trigger<UpdateLightsCommand>,
    mut query: Query<&mut PointLight>,
) {
    for mut point_light in query.iter_mut() {
        point_light.shadows_enabled = true;
        point_light.intensity = 100_000.0;
    }
}
