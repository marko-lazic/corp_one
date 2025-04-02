use crate::{prelude::ForceFieldMaterial, util::mesh_extension::MeshExt};
use avian3d::{collision::CollisionLayers, prelude::LayerMask};
use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use corp_shared::prelude::*;

pub struct BarrierPlugin;

impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                add_force_field_shader,
                change_barrier_field_visibility_and_collision,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            FixedUpdate,
            (
                door_cooldown_system,
                process_temporary_faction_ownership_timers_system,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn add_force_field_shader(
    mut commands: Commands,
    query: Query<Entity, Added<Door>>,
    q_children: Query<&Children>,
    r_meshes: Res<Assets<Mesh>>,
    q_meshes: Query<&Mesh3d>,
    mut r_force_field_materials: ResMut<Assets<ForceFieldMaterial>>,
) {
    for entity in &query {
        for (entity, _) in Mesh::search_in_children(entity, &q_children, &r_meshes, &q_meshes) {
            commands.entity(entity).insert((
                MeshMaterial3d(r_force_field_materials.add(ForceFieldMaterial {})),
                NotShadowReceiver,
                NotShadowCaster,
            ));
        }
    }
}

fn change_barrier_field_visibility_and_collision(
    mut commands: Commands,
    mut q_barrier_field_visibility: Query<&mut Visibility, With<DoorId>>,
    q_door: Query<(Entity, &DoorState), (Changed<DoorState>, With<Door>)>,
) {
    for (e_door, door_state) in &q_door {
        if let Ok(mut visible) = q_barrier_field_visibility.get_mut(e_door) {
            if door_state.is_open() {
                *visible = Visibility::Hidden;
                commands
                    .entity(e_door)
                    .insert(CollisionLayers::new([GameLayer::Sensor], [LayerMask::NONE]));
            } else if *door_state == DoorState::Closed {
                *visible = Visibility::Visible;
                commands
                    .entity(e_door)
                    .insert(CollisionLayers::new([LayerMask::ALL], [LayerMask::ALL]));
            }
        }
    }
}
