use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::state::GameState;
use crate::util::mesh_extension::MeshExt;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        info!("Physics Plugin");
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugin(RapierDebugRenderPlugin::default());
        app.add_system(Self::setup_colliders.in_schedule(OnEnter(GameState::SpawnPlayer)));
    }
}

impl PhysicsPlugin {
    fn setup_colliders(
        mut commands: Commands,
        added_name: Query<(Entity, &Name)>,
        children: Query<&Children>,
        meshes: Res<Assets<Mesh>>,
        mesh_handles: Query<&Handle<Mesh>>,
    ) {
        for (entity, name) in &added_name {
            if name.to_lowercase().contains("wall") || name.to_lowercase().contains("barrier") {
                for (collider_entity, collider_mesh) in
                    Mesh::search_in_children(entity, &children, &meshes, &mesh_handles)
                {
                    let rapier_collider =
                        Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::TriMesh)
                            .expect("Failed to initialize a collider with a Mesh.");

                    commands.entity(collider_entity).insert(rapier_collider);
                }
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
