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
    query: Query<(Entity, &MeshCollider), Added<MeshCollider>>,
    children_query: Query<&Children>,
    mesh_3d: Query<&Mesh3d>,
    meshes: Res<Assets<Mesh>>,
) {
    for (entity, mesh_collider) in &query {
        for child in children_query.iter_descendants(entity) {
            if let Ok(Mesh3d(handle)) = mesh_3d.get(child) {
                let mesh = meshes.get(handle).unwrap();
                if let Some(collider) = Collider::trimesh_from_mesh(mesh) {
                    commands.entity(entity).insert((
                        collider,
                        CollisionMargin(0.05),
                        RigidBody::from(*mesh_collider),
                        CollisionLayers::new([GameLayer::Structure], [GameLayer::Player]),
                    ));
                    info!("Collider added to map entity  {}", child);
                } else {
                    warn!("Info this entity didnt have a mesh {}", child);
                }
            }
        }
    }
}
