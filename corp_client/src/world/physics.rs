use crate::{util::mesh_extension::MeshExt, world::shader::ForceFieldMaterial};
use avian3d::prelude::*;
use bevy::{
    ecs::system::SystemId,
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

#[derive(PhysicsLayer, Clone, Copy, Debug)]
pub enum Layer {
    Player = 0b0001,
    Zone = 0b0010,
    VortexGate = 0b0011,
    Sensor = 0b0100,
    Fixed = 0b0101,
}

#[derive(Resource)]
pub struct PhysicsSystems {
    pub setup_colliders: SystemId,
}

pub struct WorldPhysicsPlugin;

impl Plugin for WorldPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::new(FixedUpdate),
            PhysicsDebugPlugin::new(FixedUpdate),
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
        ))
        .add_systems(Startup, setup);
    }
}

fn setup(world: &mut World) {
    let setup_colliders = world.register_system(setup_colliders);
    world.insert_resource(PhysicsSystems { setup_colliders });
}

fn setup_colliders(
    mut commands: Commands,
    q_added_name: Query<(Entity, &Name)>,
    q_children: Query<&Children>,
    r_meshes: Res<Assets<Mesh>>,
    r_mesh_handles: Query<&Handle<Mesh>>,
    mut r_force_field_materials: ResMut<Assets<ForceFieldMaterial>>,
) {
    for (entity, name) in &q_added_name {
        if ["wall", "tree", "energynode", "barriercontrol"]
            .iter()
            .any(|&s| name.to_lowercase().contains(s))
        {
            for (collider_entity, mesh) in
                Mesh::search_in_children(entity, &q_children, &r_meshes, &r_mesh_handles)
            {
                let collider = Collider::trimesh_from_mesh(&mesh)
                    .expect("Failed to initialize a collider with a Mesh.");

                commands
                    .entity(collider_entity)
                    .insert((RigidBody::Static, collider));
            }
        } else if name.to_lowercase().contains("barrierfield") {
            for (collider_entity, mesh) in
                Mesh::search_in_children(entity, &q_children, &r_meshes, &r_mesh_handles)
            {
                let rapier_collider = Collider::trimesh_from_mesh(&mesh)
                    .expect("Failed to initialize a collider with a Mesh.");

                commands
                    .entity(collider_entity)
                    .insert((RigidBody::Kinematic, rapier_collider));

                // Shaders should be refactored out of physics plugin
                commands.entity(collider_entity).insert((
                    MaterialMeshBundle {
                        mesh: r_mesh_handles.get(collider_entity).unwrap().clone(),
                        material: r_force_field_materials.add(ForceFieldMaterial {}),
                        ..default()
                    },
                    NotShadowReceiver,
                    NotShadowCaster,
                ));
                commands
                    .entity(collider_entity)
                    .remove::<Handle<StandardMaterial>>();
            }
        }
    }
}
