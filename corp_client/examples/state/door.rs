use std::time::Duration;

use bevy::prelude::*;

use crate::faction::{MemberOf, OwnedBy};
use crate::interactive::{InteractionType, Interactive};
use crate::inventory::Inventory;
use crate::item::Item;

pub struct DoorInteractionEvent {
    pub door_entity: Entity,
    pub interactor_entity: Entity,
}

pub struct DoorHackEvent {
    pub door_entity: Entity,
    pub interactor_entity: Entity,
}

#[derive(Component, Default, Debug, Eq, PartialEq)]
pub enum DoorState {
    Open,
    #[default]
    Closed,
}

#[derive(Component)]
pub struct Door {
    state: DoorState,
    open_cooldown: Timer,
    toggle_cooldown: Timer,
}

impl Door {
    const OPEN_TIME: f32 = 10.0;
    const TOGGLE_TIME: f32 = 1.0;

    pub fn state(&self) -> &DoorState {
        &self.state
    }

    pub fn toggle(&mut self) {
        if self.toggle_cooldown.finished() {
            self.toggle_cooldown.reset();
            self.toggle_state();
        }
    }

    fn toggle_state(&mut self) {
        if self.state == DoorState::Open {
            self.state = DoorState::Closed;
        } else {
            self.state = DoorState::Open;
        }
    }
}

impl Default for Door {
    fn default() -> Self {
        let mut toggle_cooldown = Timer::from_seconds(Self::TOGGLE_TIME, TimerMode::Once);
        toggle_cooldown.tick(Duration::from_secs_f32(Self::TOGGLE_TIME));
        Self {
            state: DoorState::Closed,
            open_cooldown: Timer::from_seconds(Self::OPEN_TIME, TimerMode::Once),
            toggle_cooldown,
        }
    }
}

impl Interactive for Door {
    fn interaction_type(&self) -> InteractionType {
        InteractionType::Door
    }
}

pub fn door_cooldown_system(mut door_query: Query<&mut Door>, time: Res<Time>) {
    for mut door in &mut door_query {
        // If the door is currently open and the cooldown timer has expired, set the state to Closed
        if door.state == DoorState::Open && door.open_cooldown.tick(time.delta()).just_finished() {
            door.state = DoorState::Closed;
        }

        // If the door toggle cooldown timer has expired, allow the player to interact with the door again
        if !door.toggle_cooldown.finished() {
            door.toggle_cooldown.tick(time.delta());
        }
    }
}

pub fn door_interaction_event_system(
    mut door_interaction_event_reader: EventReader<DoorInteractionEvent>,
    mut door_hack_event_writer: EventWriter<DoorHackEvent>,
    interactor_query: Query<&MemberOf>,
    mut door_query: Query<(&mut Door, &OwnedBy)>,
) {
    for event in door_interaction_event_reader.iter() {
        if let Ok(member_of) = interactor_query.get(event.interactor_entity) {
            if let Ok((mut door, owned_by)) = door_query.get_mut(event.door_entity) {
                if member_of.faction == owned_by.faction {
                    door.toggle();
                } else {
                    door_hack_event_writer.send(DoorHackEvent {
                        door_entity: event.door_entity,
                        interactor_entity: event.interactor_entity,
                    });
                }
            }
        }
    }
}

pub fn door_hack_event_system(
    mut door_hack_event_reader: EventReader<DoorHackEvent>,
    mut door_query: Query<&mut Door>,
    mut inventory_query: Query<&mut Inventory>,
    item_query: Query<(Entity, &Item)>,
    mut commands: Commands,
) {
    for event in door_hack_event_reader.iter() {
        if let Ok(mut door) = door_query.get_mut(event.door_entity) {
            if let Ok(mut inventory) = inventory_query.get_mut(event.interactor_entity) {
                if let Some((hacking_tool_entity, _hacking_tool_item)) = item_query
                    .iter_many(&inventory.items)
                    .find(|item| item.1.name == "Hacking tool")
                {
                    inventory.remove_item(hacking_tool_entity);
                    commands.entity(hacking_tool_entity).despawn_recursive();
                    door.toggle();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::prelude::*;
    use bevy_trait_query::RegisterExt;

    use crate::door::{
        door_cooldown_system, door_hack_event_system, door_interaction_event_system, Door,
        DoorHackEvent, DoorInteractionEvent, DoorState,
    };
    use crate::faction::{Faction, MemberOf, OwnedBy};
    use crate::interactive::{interaction_system, Interactive, Interactor};
    use crate::inventory::Inventory;
    use crate::item::Item;
    use crate::player::Player;
    use crate::test_utils::TestUtils;

    #[test]
    fn door_default_closed() {
        // given
        let (mut app, door_entity, _) = setup();

        // when
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Closed);
    }

    #[test]
    fn open_door_closed_after_10s() {
        // given
        let (mut app, door_entity, _) = setup();
        app.get_mut::<Door>(door_entity).state = DoorState::Open;

        // when
        app.update_after(Duration::from_secs_f32(10.0));

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Closed);
    }

    #[test]
    fn player_open_door() {
        // given
        let (mut app, door_entity, player_entity) = setup();

        // when
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
    }

    #[test]
    fn player_close_open_door() {
        // given
        let (mut app, door_entity, player_entity) = setup();
        app.get_mut::<Door>(door_entity).state = DoorState::Open;

        // when
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Closed);
    }

    #[test]
    fn player_open_door_wait_3_seconds_and_close_door() {
        // given
        let (mut app, door_entity, player_entity) = setup();

        // when
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update_after(Duration::from_secs_f32(3.0));
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Closed);
    }

    #[test]
    fn player_open_door_two_times_before_toggle_cooldown_finished() {
        // given
        let (mut app, door_entity, player_entity) = setup();

        // when
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update_after(Duration::from_secs_f32(0.5));
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update_after(Duration::from_secs_f32(0.1));

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
        assert!(!result.toggle_cooldown.finished());
    }

    fn setup() -> (App, Entity, Entity) {
        let mut app = App::new();
        app.init_time();
        app.add_event::<DoorInteractionEvent>();
        app.add_event::<DoorHackEvent>();
        app.register_component_as::<dyn Interactive, Door>();
        app.add_systems(
            (
                door_cooldown_system,
                interaction_system,
                door_interaction_event_system,
                door_hack_event_system,
            )
                .chain(),
        );
        let door_entity = app
            .world
            .spawn((Door::default(), OwnedBy::new(Faction::EC)))
            .id();
        let item_entity = app.world.spawn(Item::new("Hacking tool".to_string())).id();
        let player_entity = app
            .world
            .spawn((
                Player,
                Interactor::default(),
                Inventory::new(vec![item_entity]),
                MemberOf::new(Faction::EC),
            ))
            .id();
        (app, door_entity, player_entity)
    }
}
