use bevy::prelude::*;
use bevy_rapier3d::prelude::ColliderDisabled;

use corp_shared::prelude::*;

use crate::state::GameState;

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

#[derive(Component, Debug)]
pub struct BarrierField {
    pub entity: Entity,
    pub name: String,
}

impl BarrierField {
    pub fn new(entity: Entity, name: &str) -> Self {
        Self {
            entity,
            name: name.to_string(),
        }
    }
}

pub struct BarrierPlugin;

impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractionEvent<UseDoorEvent>>()
            .add_event::<DoorHackEvent>()
            .add_systems(
                Update,
                (change_barrier_field_visibility_and_collision,)
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
    q_door: Query<(Entity, &DoorState), (Changed<DoorState>, With<Door>)>,
) {
    for (e_door, door_state) in &q_door {
        if let Ok(mut visible) = q_barrier_field_visibility.get_mut(e_door) {
            if *door_state == DoorState::Open {
                *visible = Visibility::Hidden;
                commands.entity(e_door).insert(ColliderDisabled);
            } else if *door_state == DoorState::Closed {
                *visible = Visibility::Visible;
                commands.entity(e_door).remove::<ColliderDisabled>();
            }
        }
    }
}
