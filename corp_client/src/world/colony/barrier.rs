use bevy::prelude::*;
use bevy_rapier3d::prelude::ColliderDisabled;

use corp_shared::prelude::*;

use crate::{
    state::GameState,
    world::{
        ccc::UseEntity,
        colony::object_interaction::{Hover, PickingEvent},
    },
};

pub struct BarrierPickingEvent;

#[derive(Component, Default, Debug)]
pub struct BarrierControl {
    pub barrier_field_name: String,
}

impl BarrierControl {
    pub fn new(name: &str) -> Self {
        Self {
            barrier_field_name: name.to_string(),
        }
    }
}

#[derive(Component, Default, Debug)]
pub struct BarrierField {
    pub name: String,
}

impl BarrierField {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

pub struct BarrierPlugin;

impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickingEvent<BarrierPickingEvent>>()
            .add_event::<InteractionEvent<UseDoorEvent>>()
            .add_event::<DoorHackEvent>()
            .add_event::<DoorStateEvent>()
            .add_systems(
                Update,
                (
                    receive_barrier_pickings
                        .run_if(on_event::<PickingEvent<BarrierPickingEvent>>()),
                    change_barrier_field_visibility_and_collision,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    door_cooldown_system,
                    process_temporary_faction_ownership_timers_system,
                    door_interaction_event_system
                        .run_if(on_event::<InteractionEvent<UseDoorEvent>>()),
                    door_hack_event_system,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn change_barrier_field_visibility_and_collision(
    mut commands: Commands,
    mut q_barrier_field_visibility: Query<&mut Visibility, With<BarrierField>>,
    mut ev_door_state_event: EventReader<DoorStateEvent>,
) {
    for door_state_event in ev_door_state_event.read() {
        if let Ok(mut visible) = q_barrier_field_visibility.get_mut(door_state_event.entity()) {
            if door_state_event.state() == DoorState::Open {
                *visible = Visibility::Hidden;
                commands
                    .entity(door_state_event.entity())
                    .insert(ColliderDisabled);
            } else if door_state_event.state() == DoorState::Closed {
                *visible = Visibility::Visible;
                commands
                    .entity(door_state_event.entity())
                    .remove::<ColliderDisabled>();
            }
        }
    }
}

pub fn receive_barrier_pickings(
    mut ev_barrier_picking_event: EventReader<PickingEvent<BarrierPickingEvent>>,
    mut r_use_target: ResMut<UseEntity>,
    q_barrier_control: Query<&BarrierControl>,
    q_barrier_field: Query<(Entity, &BarrierField)>,
) {
    for event in ev_barrier_picking_event.read() {
        if event.mode == Hover::Over {
            let Ok(barrier_control) = q_barrier_control.get(event.target) else {
                return;
            };

            // Try spawning barrier field and control as one entity with parent/child
            let Some((target_barrier, _)) = q_barrier_field
                .iter()
                .find(|(_e, b)| b.name == barrier_control.barrier_field_name)
            else {
                return;
            };
            r_use_target.set(Some(target_barrier));
        } else if event.mode == Hover::Out {
            r_use_target.set(None);
        }
    }
}
