use crate::prelude::ForceFieldMaterial;
use avian3d::prelude::*;
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
            (change_barrier_field_visibility_and_collision,).run_if(in_state(GameState::Playing)),
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
        .add_observer(on_add_door);
    }
}

fn on_add_door(
    trigger: Trigger<OnAdd, Door>,
    mut commands: Commands,
    mut r_force_field_materials: ResMut<Assets<ForceFieldMaterial>>,
) {
    commands.entity(trigger.target()).insert((
        MeshMaterial3d(r_force_field_materials.add(ForceFieldMaterial {})),
        NotShadowReceiver,
        NotShadowCaster,
    ));
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
