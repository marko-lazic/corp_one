use std::time::Duration;

use crate::prelude::ForceFieldMaterial;
use avian3d::prelude::*;
use bevy::{
    light::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    time::common_conditions::on_timer,
};
use corp_shared::prelude::*;

pub struct BarrierPlugin;

impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            change_barrier_field_visibility_and_collision
                .run_if(in_state(GameState::Playing))
                .run_if(on_timer(Duration::from_millis(300))),
        )
        .add_systems(
            FixedUpdate,
            (
                door_cooldown_system,
                process_temporary_faction_ownership_timers_system,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(PostUpdate, add_force_field_materials);
    }
}

// glTF primitives are mesh children of the node carrying Door. Keep checking
// these hierarchies because the mesh children can arrive after Door is added.
fn add_force_field_materials(
    mut commands: Commands,
    doors: Query<Entity, With<Door>>,
    children: Query<&Children>,
    meshes: Query<Entity, (With<Mesh3d>, Without<MeshMaterial3d<ForceFieldMaterial>>)>,
    mut materials: ResMut<Assets<ForceFieldMaterial>>,
    mut material: Local<Option<Handle<ForceFieldMaterial>>>,
) {
    for door in &doors {
        for entity in std::iter::once(door).chain(children.iter_descendants(door)) {
            if meshes.contains(entity) {
                let handle = material.get_or_insert_with(|| materials.add(ForceFieldMaterial {}));
                commands
                    .entity(entity)
                    .remove::<MeshMaterial3d<StandardMaterial>>()
                    .insert((
                        MeshMaterial3d(handle.clone()),
                        NotShadowReceiver,
                        NotShadowCaster,
                    ));
            }
        }
    }
}

fn change_barrier_field_visibility_and_collision(
    mut commands: Commands,
    mut q_barrier_field_visibility: Query<&mut Visibility, With<DoorId>>,
    q_door: Query<(Entity, &DoorState), With<Door>>,
) {
    for (e_door, door_state) in &q_door {
        if let Ok(mut visible) = q_barrier_field_visibility.get_mut(e_door) {
            if door_state.is_open() {
                *visible = Visibility::Hidden;
                commands.entity(e_door).insert(Sensor);
            } else if *door_state == DoorState::Closed {
                *visible = Visibility::Visible;
                commands.entity(e_door).remove::<Sensor>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_app() -> App {
        let mut app = App::new();
        app.init_resource::<Assets<ForceFieldMaterial>>()
            .add_systems(PostUpdate, add_force_field_materials);
        app
    }

    fn assert_force_field(app: &App, entity: Entity) {
        let entity = app.world().entity(entity);
        assert!(entity.contains::<MeshMaterial3d<ForceFieldMaterial>>());
        assert!(!entity.contains::<MeshMaterial3d<StandardMaterial>>());
        assert!(entity.contains::<NotShadowCaster>());
        assert!(entity.contains::<NotShadowReceiver>());
    }

    #[test]
    fn applies_to_nested_door_meshes_and_removes_gltf_material() {
        let mut app = test_app();
        let door = app.world_mut().spawn(Door).id();
        let node = app.world_mut().spawn(ChildOf(door)).id();
        let mesh = app
            .world_mut()
            .spawn((
                ChildOf(node),
                Mesh3d::default(),
                MeshMaterial3d::<StandardMaterial>::default(),
            ))
            .id();
        let unrelated = app
            .world_mut()
            .spawn((
                Mesh3d::default(),
                MeshMaterial3d::<StandardMaterial>::default(),
            ))
            .id();

        app.update();

        assert_force_field(&app, mesh);
        assert!(
            !app.world()
                .entity(door)
                .contains::<MeshMaterial3d<ForceFieldMaterial>>()
        );
        assert!(
            app.world()
                .entity(unrelated)
                .contains::<MeshMaterial3d<StandardMaterial>>()
        );
        assert!(
            !app.world()
                .entity(unrelated)
                .contains::<MeshMaterial3d<ForceFieldMaterial>>()
        );
    }

    #[test]
    fn handles_meshes_loaded_after_door_without_recreating_materials() {
        let mut app = test_app();
        let door = app.world_mut().spawn(Door).id();
        app.update();
        let mesh = app
            .world_mut()
            .spawn((
                ChildOf(door),
                Mesh3d::default(),
                MeshMaterial3d::<StandardMaterial>::default(),
            ))
            .id();
        app.update();
        assert_force_field(&app, mesh);
        let handle = app
            .world()
            .get::<MeshMaterial3d<ForceFieldMaterial>>(mesh)
            .unwrap()
            .0
            .clone();
        app.update();
        assert_eq!(
            app.world()
                .get::<MeshMaterial3d<ForceFieldMaterial>>(mesh)
                .unwrap()
                .0,
            handle
        );
        assert_eq!(
            app.world().resource::<Assets<ForceFieldMaterial>>().len(),
            1
        );
    }

    #[test]
    fn supports_mesh_on_door_itself_and_door_added_after_mesh() {
        let mut app = test_app();
        let mesh = app
            .world_mut()
            .spawn((
                Mesh3d::default(),
                MeshMaterial3d::<StandardMaterial>::default(),
            ))
            .id();
        app.update();
        app.world_mut().entity_mut(mesh).insert(Door);
        app.update();
        assert_force_field(&app, mesh);
    }
}
