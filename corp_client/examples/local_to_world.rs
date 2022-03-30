use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(InspectorPlugin::<InspectorData>::new())
        .add_startup_system(setup_system)
        .add_system(move_parent)
        .add_system(move_child)
        .run();
}

fn move_parent(
    mut parent_query: Query<&mut Transform, With<ParentPoint>>,
    data: Res<InspectorData>,
) {
    let mut parent_tr = Option::unwrap(parent_query.iter_mut().last());
    parent_tr.translation.x = data.parent.x;
    parent_tr.translation.y = data.parent.y;
}

fn move_child(mut child_query: Query<&mut Transform, With<ChildPoint>>, data: Res<InspectorData>) {
    let mut child_tr = Option::unwrap(child_query.iter_mut().last());
    let world_space_point = local_to_world(data.child, data.parent);
    child_tr.translation = Vec3::new(world_space_point.x, world_space_point.y, 0.0);
}

fn local_to_world(local_pt: Vec2, global_pt: Vec2) -> Vec2 {
    let right = Vec2::X;
    let up = Vec2::Y;
    let world_offset = right * local_pt.x + up * local_pt.y;
    global_pt + world_offset
}

fn setup_system(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(bevy_transform_gizmo::GizmoPickSource::default());

    let point = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(10.0),
        ..shapes::RegularPolygon::default()
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &point,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 2.0),
            },
            Transform::default(),
        ))
        .insert(ParentPoint);

    let sphere = shapes::RegularPolygon {
        sides: 12,
        feature: shapes::RegularPolygonFeature::Radius(10.0),
        ..shapes::RegularPolygon::default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &sphere,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::YELLOW),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform::from_xyz(100.0, 50.0, 0.0),
        ))
        .insert(ChildPoint);
}

#[derive(Inspectable)]
struct InspectorData {
    parent: Vec2,
    child: Vec2,
}

#[derive(Component)]
struct ParentPoint;

#[derive(Component)]
struct ChildPoint;

impl Default for InspectorData {
    fn default() -> InspectorData {
        InspectorData {
            parent: Default::default(),
            child: Vec2::new(100.0, 50.0),
        }
    }
}
