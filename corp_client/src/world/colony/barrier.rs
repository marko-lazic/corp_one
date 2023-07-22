use bevy::app::Plugin;
use bevy::prelude::*;
// use bevy_mod_picking::events::{Out, Over};
// use bevy_mod_picking::prelude::ListenedEvent;
use bevy_rapier3d::prelude::ColliderDisabled;

use corp_shared::prelude::*;

use crate::gui::CursorVisibility;
use crate::state::GameState;
use crate::{App, Game};

#[derive(Debug, Eq, PartialEq)]
pub enum Hover {
    Over,
    Out,
}

#[derive(Event)]
pub struct BarrierPickingEvent(Entity, Hover);

// impl From<ListenedEvent<Over>> for BarrierPickingEvent {
//     fn from(event: ListenedEvent<Over>) -> Self {
//         BarrierPickingEvent(event.target, Hover::Over)
//     }
// }
//
// impl From<ListenedEvent<Out>> for BarrierPickingEvent {
//     fn from(event: ListenedEvent<Out>) -> Self {
//         BarrierPickingEvent(event.target, Hover::Out)
//     }
// }

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
        app.add_event::<BarrierPickingEvent>();
        app.add_event::<DoorInteractionEvent>();
        app.add_event::<DoorHackEvent>();
        app.add_event::<DoorStateEvent>();
        app.add_systems(
            Update,
            (
                receive_barrier_pickings.run_if(on_event::<BarrierPickingEvent>()),
                open_close_barrier,
            )
                .run_if(in_state(GameState::Playing)),
        );
        app.add_systems(
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
    mut pickings: EventReader<BarrierPickingEvent>,
    mut cursor_info: ResMut<CursorVisibility>,
    mut game: ResMut<Game>,
) {
    for event in pickings.iter() {
        if event.1 == Hover::Over {
            cursor_info.visible = true;
            game.use_entity = Some(event.0);
        } else if event.1 == Hover::Out {
            cursor_info.visible = false;
            game.use_entity = None;
        }
    }
}
