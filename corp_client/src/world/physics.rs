use bevy::{
    ecs::system::SystemId,
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::{util::mesh_extension::MeshExt, world::shader::ForceFieldMaterial};

#[derive(Resource)]
pub struct PhysicsSystems {
    pub setup_colliders: SystemId,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
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
            for (collider_entity, collider_mesh) in
                Mesh::search_in_children(entity, &q_children, &r_meshes, &r_mesh_handles)
            {
                let rapier_collider =
                    Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::TriMesh)
                        .expect("Failed to initialize a collider with a Mesh.");

                commands
                    .entity(collider_entity)
                    .insert((RigidBody::Fixed, rapier_collider));
            }
        } else if name.to_lowercase().contains("barrierfield") {
            for (collider_entity, collider_mesh) in
                Mesh::search_in_children(entity, &q_children, &r_meshes, &r_mesh_handles)
            {
                let rapier_collider =
                    Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::TriMesh)
                        .expect("Failed to initialize a collider with a Mesh.");

                commands
                    .entity(collider_entity)
                    .insert((RigidBody::KinematicPositionBased, rapier_collider));

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

pub struct CollideGroups;

impl CollideGroups {
    const PLAYER: Group = Group::GROUP_1;
    const ZONE: Group = Group::GROUP_2;
    const VORTEX_GATE: Group = Group::GROUP_3;

    pub fn player() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::PLAYER,
            filters: Self::VORTEX_GATE | Self::ZONE,
        }
    }

    pub fn zone() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::ZONE,
            filters: Self::PLAYER,
        }
    }

    pub fn vortex_gate() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::VORTEX_GATE,
            filters: Self::PLAYER,
        }
    }
}
