use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use corp_shared::prelude::*;

pub struct WorldPhysicsPlugin;

impl Plugin for WorldPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::new(FixedUpdate),
            PhysicsDebugPlugin::new(FixedUpdate),
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
        ))
        .add_systems(Update, add_trimesh_collider);
    }
}

fn add_trimesh_collider(
    mut commands: Commands,
    tri_mesh_entities: Query<Entity, Added<CreateTriMesh>>,
    children: Query<&Children>,
    mesh_3d: Query<&Mesh3d>,
    meshes: Res<Assets<Mesh>>,
) {
    for entity in &tri_mesh_entities {
        for child in children.iter_descendants(entity) {
            if let Ok(Mesh3d(handle)) = mesh_3d.get(child) {
                let mesh = meshes.get(handle).unwrap();
                if let Some(collider) = Collider::trimesh_from_mesh(mesh) {
                    commands
                        .entity(entity)
                        .insert((collider, CollisionMargin(0.05)));
                }
            }
        }
    }
}
