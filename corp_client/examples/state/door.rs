use std::time::Duration;

use bevy::prelude::*;

use crate::interactive::*;

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
    fn interact(&mut self, entity: Entity) {
        info!("Interacting with door {:#?}", entity);
        self.toggle();
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::prelude::*;
    use bevy_trait_query::RegisterExt;

    use crate::door::{Door, door_cooldown_system, DoorState};
    use crate::interactive::{interaction_system, Interactive};
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
        app.get_mut::<Door>(door_entity).interact(player_entity);
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
        app.get_mut::<Door>(door_entity).interact(player_entity);
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
        app.get_mut::<Door>(door_entity).interact(player_entity);
        app.update_after(Duration::from_secs_f32(3.0));
        app.get_mut::<Door>(door_entity).interact(player_entity);
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
        app.get_mut::<Door>(door_entity).interact(player_entity);
        app.update_after(Duration::from_secs_f32(0.5));
        app.get_mut::<Door>(door_entity).interact(player_entity);
        app.update_after(Duration::from_secs_f32(0.1));

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
        assert!(!result.toggle_cooldown.finished());
    }

    fn setup() -> (App, Entity, Entity) {
        let mut app = App::new();
        app.init_time();
        app.add_systems((door_cooldown_system, interaction_system));
        app.register_component_as::<dyn Interactive, Door>();
        let door_entity = app.world.spawn(Door::default()).id();
        let player_entity = app.world.spawn(Player).id();
        (app, door_entity, player_entity)
    }
}