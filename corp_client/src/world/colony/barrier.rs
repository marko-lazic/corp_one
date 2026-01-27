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
        .add_observer(on_add_door);
    }
}

fn on_add_door(
    add: On<Add, Door>,
    mut commands: Commands,
    mut r_force_field_materials: ResMut<Assets<ForceFieldMaterial>>,
) {
    commands.entity(add.entity).insert((
        MeshMaterial3d(r_force_field_materials.add(ForceFieldMaterial {})),
        NotShadowReceiver,
        NotShadowCaster,
    ));
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
