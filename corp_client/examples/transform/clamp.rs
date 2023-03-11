use bevy::app::App;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_prototype_lyon::prelude::*;

use crate::shape::Icosphere;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .init_resource::<Entities>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(ResourceInspectorPlugin::<InspectorData>::new())
        .add_startup_system(setup)
        .add_system(update_p)
        .run();
}

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct InspectorData {
    t: f32,
}

#[derive(Component)]
struct PointA;

#[derive(Component)]
struct PointB;

#[derive(Component)]
struct PointP;

#[derive(Resource, Default, Debug)]
struct Entities {
    a: Option<Entity>,
    b: Option<Entity>,
    p: Option<Entity>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entities: ResMut<Entities>,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            intensity: 2000.0,
            range: 20.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, -10.0, 65.0)),
        ..Default::default()
    });

    // PBRs
    let mesh_abc = meshes.add(
        Mesh::try_from(Icosphere {
            radius: 0.45,
            subdivisions: 32,
        })
        .unwrap(),
    );

    let a = commands
        .spawn(PbrBundle {
            mesh: mesh_abc.clone(),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(5.0, -15.0, 0.0),
            ..Default::default()
        })
        .insert(PointA)
        .id();
    entities.a = Some(a);

    let b = commands
        .spawn(PbrBundle {
            mesh: mesh_abc.clone(),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_xyz(1.0, 5.0, 0.0),
            ..Default::default()
        })
        .insert(PointB)
        .id();
    entities.b = Some(b);

    let p = commands
        .spawn(PbrBundle {
            mesh: mesh_abc.clone(),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(5.0, -15.0, 0.0),
            ..Default::default()
        })
        .insert(PointP)
        .id();
    entities.p = Some(p);

    info!("Setup entities {:?}", *entities);
}

fn update_p(
    data: Res<InspectorData>,
    entities: Res<Entities>,
    mut transforms: Query<&mut Transform>,
) {
    let t = data.t;
    let a = transforms.get(entities.a.unwrap()).unwrap().translation;
    let b = transforms.get(entities.b.unwrap()).unwrap().translation;

    // t = 0.0 start, t = 1.0 end
    let p = a + t * (b - a);

    // This is just an interesting rising spiral formula
    // p = Vec3::from((t, t * t.cos(), t * t.sin()));

    transforms.get_mut(entities.p.unwrap()).unwrap().translation = p;
}
