use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .init_resource::<GameData>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ResourceInspectorPlugin::<InspectorData>::new())
        .add_plugin(DebugLinesPlugin::default())
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        // .add_plugin(TransformGizmoPlugin::default())
        .add_startup_system(setup)
        .add_system(change_color)
        .run();
}

fn change_color(
    radials: Query<&Transform, With<Radial>>,
    players: Query<&Transform, With<Player>>,
    inspector: Res<InspectorData>,
    game: Res<GameData>,
) {
    let radial_entity = game.radial.unwrap();
    let radial = radials.get(radial_entity).unwrap();
    let player_entity = game.player.unwrap();
    let player = players.get(player_entity).unwrap();
    // Calculate distance
    // let dist = radial.translation.distance(player.translation);
    // Or use optimized squared distance but radius has to be squared
    // This only works if you checking threshold
    let dist_sq = radial.translation.distance_squared(player.translation);

    if dist_sq < inspector.radius.sqrt() {
        println!("Intersect");
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<GameData>,
    inspector: Res<InspectorData>,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            intensity: 200.0,
            range: 20.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0)),
            ..Default::default()
        })
        .insert((
            bevy_mod_picking::PickingCameraBundle::default(),
            // bevy_transform_gizmo::GizmoPickSource::default(),
        ));

    // radial
    let material = materials.add(Color::rgb(0.1, 0.4, 0.8).into());
    let mesh = meshes.add(
        Mesh::try_from(shape::Icosphere {
            subdivisions: 4,
            radius: inspector.radius,
        })
        .unwrap(),
    );

    let radial_id = commands
        .spawn(PbrBundle {
            mesh,
            material,
            ..Default::default()
        })
        .insert((
            bevy_mod_picking::PickableBundle::default(),
            // bevy_transform_gizmo::GizmoTransformable,
            Radial,
        ))
        .id();

    game.radial = Some(radial_id);

    // player
    let material = materials.add(Color::AZURE.into());
    let mesh = meshes.add(
        Mesh::try_from(shape::Icosphere {
            subdivisions: 4,
            radius: inspector.radius,
        })
        .unwrap(),
    );

    let player_id = commands
        .spawn(PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(Vec3::new(4.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert((
            bevy_mod_picking::PickableBundle::default(),
            // bevy_transform_gizmo::GizmoTransformable,
            Player,
        ))
        .id();

    game.player = Some(player_id);
}

#[derive(Component)]
struct Radial;

#[derive(Component)]
struct Player;

#[derive(Resource, Default)]
struct GameData {
    radial: Option<Entity>,
    player: Option<Entity>,
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct InspectorData {
    #[inspector(min = 0.0, max = 4.0)]
    radius: f32,
}

impl Default for InspectorData {
    fn default() -> Self {
        Self { radius: 2.0 }
    }
}
