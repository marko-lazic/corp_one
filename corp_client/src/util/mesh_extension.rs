use bevy::{prelude::*, render::mesh::PrimitiveTopology};

pub trait MeshExt {
    fn search_in_children<'a>(
        parent: Entity,
        children: &'a Query<&Children>,
        meshes: &'a Assets<Mesh>,
        mesh_handles: &'a Query<&Mesh3d>,
    ) -> Vec<(Entity, &'a Mesh)>;
}

impl MeshExt for Mesh {
    fn search_in_children<'a>(
        parent: Entity,
        children_query: &'a Query<&Children>,
        r_meshes: &'a Assets<Mesh>,
        q_mesh_handles: &'a Query<&Mesh3d>,
    ) -> Vec<(Entity, &'a Mesh)> {
        if let Ok(children) = children_query.get(parent) {
            let mut result: Vec<_> = children
                .iter()
                .filter_map(|entity| {
                    q_mesh_handles
                        .get(*entity)
                        .ok()
                        .map(|mesh_handle| (*entity, mesh_handle))
                })
                .map(|(entity, mesh_handle)| {
                    (
                        entity,
                        r_meshes
                            .get(mesh_handle)
                            .expect("Failed to get mesh from handle"),
                    )
                })
                .map(|(entity, mesh)| {
                    assert_eq!(mesh.primitive_topology(), PrimitiveTopology::TriangleList);
                    (entity, mesh)
                })
                .collect();
            let mut inner_result = children
                .iter()
                .flat_map(|entity| {
                    Self::search_in_children(*entity, children_query, r_meshes, q_mesh_handles)
                })
                .collect();
            result.append(&mut inner_result);
            result
        } else {
            Vec::new()
        }
    }
}
