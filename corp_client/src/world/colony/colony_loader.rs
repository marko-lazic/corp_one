use crate::prelude::*;
use avian3d::prelude::*;
use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    scene::SceneInstanceReady,
};
use bevy_skein::SkeinPlugin;
use corp_shared::prelude::*;

pub fn colony_loader_plugin(app: &mut App) {
    app.add_plugins(SkeinPlugin::default())
        .add_systems(OnEnter(LoadingSubState::ColonyLoading), load_colony)
        .add_systems(OnExit(LoadingSubState::ColonyLoading), update_lights);
}

fn load_colony(
    r_state: Res<State<GameState>>,
    r_scene_assets: Res<SceneAssets>,
    mut r_meshes: ResMut<Assets<Mesh>>,
    mut r_materials: ResMut<Assets<StandardMaterial>>,
    mut r_force_field_materials: ResMut<Assets<ForceFieldMaterial>>,
    mut commands: Commands,
) {
    let colony = match r_state.get() {
        GameState::Load(colony) => colony,
        _ => return,
    };

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
}

fn on_colony_loaded(
    _trigger: Trigger<SceneInstanceReady>,
    mut r_next_loading_sub_state: ResMut<NextState<LoadingSubState>>,
) {
    info!("Colony Scene Instance Ready");
    r_next_loading_sub_state.set(LoadingSubState::Connect);
}

// Temporarily fixes the problem with shadows not working
fn update_lights(mut query: Query<&mut PointLight>) {
    for mut point_light in query.iter_mut() {
        point_light.shadows_enabled = true;
        point_light.intensity = 100_000.0;
    }
}
