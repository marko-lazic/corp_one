use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            DefaultPlugins,
            ShapePlugin,
            ResourceInspectorPlugin::<InspectorData>::default(),
        ))
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (move_parent, move_child, move_world, update_world_to_local),
        )
        .run();
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct InspectorData {
    parent: Vec2,
    child: Vec2,
    child_dir: Vec3,
    world: Vec2,
    world_to_local: Vec3,
}

impl Default for InspectorData {
    fn default() -> InspectorData {
        InspectorData {
            parent: Default::default(),
            child: Vec2::new(100.0, 50.0),
            child_dir: Vec3::default(),
            world: Vec2::new(100.0, -150.0),
            world_to_local: Vec3::new(100.0, -150.0, 0.0),
        }
    }
}

#[derive(Component)]
struct ParentPoint;

#[derive(Component)]
struct ChildPoint;

#[derive(Component)]
struct WorldPoint;

fn move_parent(
    mut parent_query: Query<&mut Transform, With<ParentPoint>>,
    data: Res<InspectorData>,
) {
    let mut parent_tr = Option::unwrap(parent_query.iter_mut().last());
    parent_tr.translation.x = data.parent.x;
    parent_tr.translation.y = data.parent.y;
}

fn move_child(
    mut child_query: Query<&mut Transform, With<ChildPoint>>,
    parent_query: Query<&Transform, (With<ParentPoint>, Without<ChildPoint>)>,
    mut data: ResMut<InspectorData>,
) {
    let mut child_tr = Option::unwrap(child_query.iter_mut().last());
    let parent_tr = Option::unwrap(parent_query.iter().last());

    let world_space_point = local_to_world(data.child, parent_tr);
    child_tr.translation = Vec3::new(world_space_point.x, world_space_point.y, 0.0);
    data.child_dir = local_to_world_dir(data.child, parent_tr);
}

fn move_world(mut world_query: Query<&mut Transform, With<WorldPoint>>, data: Res<InspectorData>) {
    let mut world_tr = Option::unwrap(world_query.iter_mut().last());
    world_tr.translation.x = data.world.x;
    world_tr.translation.y = data.world.y;
}

fn update_world_to_local(
    world_query: Query<&Transform, With<WorldPoint>>,
    child_query: Query<&Transform, (With<ChildPoint>, Without<WorldPoint>)>,
    mut data: ResMut<InspectorData>,
) {
    let child_tr = Option::unwrap(child_query.iter().last());
    let world_tr = Option::unwrap(world_query.iter().last());
    let to_local = world_to_local(child_tr, world_tr);
    data.world_to_local = to_local;
}

fn local_to_world(local_pt: Vec2, parent_tr: &Transform) -> Vec3 {
    // let right = parent_tr.right();
    // let up = parent_tr.up();
    // let world_offset = right * local_pt.x + up * local_pt.y;
    // parent_tr.translation + world_offset
    parent_tr
        .compute_matrix()
        .transform_point3(Vec3::new(local_pt.x, local_pt.y, 0.0))
}

fn local_to_world_dir(local_pt: Vec2, parent_tr: &Transform) -> Vec3 {
    parent_tr
        .compute_matrix()
        .transform_vector3(Vec3::new(local_pt.x, local_pt.y, 0.0))
        .normalize()
}

fn world_to_local(child_pt: &Transform, world_tr: &Transform) -> Vec3 {
    // let right = child_pt.right();
    // let up = child_pt.up();
    // let world_pt = world_tr.translation;
    // let relative_pt = world_pt - child_pt.translation;
    // let x = relative_pt.dot(right);
    // let y = relative_pt.dot(up);
    // Vec2::new(x, y)
    world_tr
        .compute_matrix()
        .inverse()
        .transform_point3(child_pt.translation)
}

fn setup_system(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        // bevy_mod_picking::PickingCameraBundle::default(),
        // bevy_transform_gizmo::GizmoPickSource::default(),
    ));

    let point = RegularPolygon {
        sides: 6,
        feature: RegularPolygonFeature::Radius(10.0),
        ..RegularPolygon::default()
    };
    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&point),
                ..default()
            },
            Fill::color(Color::CYAN),
            Stroke::new(Color::BLACK, 2.0),
            Transform::default(),
        ))
        .insert(ParentPoint);

    let sphere = RegularPolygon {
        sides: 12,
        feature: RegularPolygonFeature::Radius(10.0),
        ..RegularPolygon::default()
    };

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&sphere),
                ..default()
            },
            Stroke::new(Color::BLACK, 1.0),
            Fill::color(Color::YELLOW),
            Transform::from_xyz(100.0, 50.0, 0.0),
        ))
        .insert(ChildPoint);

    let square = RegularPolygon {
        sides: 4,
        feature: RegularPolygonFeature::Radius(10.0),
        ..RegularPolygon::default()
    };

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&square),
                ..default()
            },
            Fill::color(Color::RED),
            Stroke::new(Color::BLACK, 1.0),
            Transform::from_xyz(100.0, -150.0, 0.0),
        ))
        .insert(WorldPoint);
}
