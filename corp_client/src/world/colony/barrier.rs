use bevy::{app::Plugin, prelude::*};
use bevy_rapier3d::prelude::ColliderDisabled;
use bevy_trait_query::RegisterExt;

use corp_shared::prelude::*;

use crate::{
    state::GameState,
    world::{
        ccc::UseEntity,
        colony::object_interaction::{Hover, PickingEvent},
    },
    App,
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
            .register_component_as::<dyn Interactive, Door>()
            .add_event::<DoorInteractionEvent>()
            .add_event::<DoorHackEvent>()
            .add_event::<DoorStateEvent>()
            .add_systems(
                Update,
                (
                    receive_barrier_pickings
                        .run_if(on_event::<PickingEvent<BarrierPickingEvent>>()),
                    open_close_barrier,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    door_cooldown_system,
                    process_temporary_faction_ownership_timers_system,
                    door_interaction_event_system,
                    door_hack_event_system,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn open_close_barrier(
    mut commands: Commands,
    mut barrier_query: Query<&mut Visibility, With<Door>>,
    mut door_state_reader: EventReader<DoorStateEvent>,
) {
    for door_event in door_state_reader.iter() {
        if let Ok(mut visible) = barrier_query.get_mut(door_event.entity()) {
            if door_event.state() == DoorState::Open {
                *visible = Visibility::Hidden;
                commands
                    .entity(door_event.entity())
                    .insert(ColliderDisabled);
            } else if door_event.state() == DoorState::Closed {
                *visible = Visibility::Visible;
                commands
                    .entity(door_event.entity())
                    .remove::<ColliderDisabled>();
            }
        }
    }
}

pub fn receive_barrier_pickings(
    mut pickings: EventReader<PickingEvent<BarrierPickingEvent>>,
    mut r_use_target: ResMut<UseEntity>,
    q_barrier_control: Query<&BarrierControl>,
    q_barrier_field: Query<(Entity, &BarrierField)>,
) {
    for event in pickings.iter() {
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
