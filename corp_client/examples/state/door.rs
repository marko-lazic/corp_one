use std::time::Duration;

use bevy::prelude::*;

mod test_utils;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_startup_system(setup)
        .add_system(check_input)
        .add_system(print_door_state)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 8.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    let position = UiRect {
        left: Val::Px(100.0),
        top: Val::Px(50.0),
        ..default()
    };

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "null",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ),
            style: Style {
                position,
                ..default()
            },
            ..Default::default()
        },
        Door::default(),
    ));
}

fn check_input(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("Pressed space");
    }
}

fn print_door_state(mut query: Query<(&mut Text, &Door)>) {
    for (mut text, door) in &mut query {
        match &door.state {
            DoorState::Open => text.sections[0].value = "Open".to_string(),
            DoorState::Closed => text.sections[0].value = "Closed".to_string(),
        }
    }
}

#[derive(Component)]
struct Door {
    state: DoorState,
    open_cooldown: Timer,
    toggle_cooldown: Timer,
}

impl Door {
    const OPEN_TIME: f32 = 10.0;
    const TOGGLE_TIME: f32 = 1.0;

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

#[derive(Component, Default, Debug, Eq, PartialEq)]
enum DoorState {
    Open,
    #[default]
    Closed,
}

#[derive(Component, Default)]
struct Player {
    interact: Option<Entity>,
}

impl Player {
    pub fn interact(&mut self, entity: Entity) {
        self.interact = Some(entity);
    }
}

fn player_door_interaction_system(
    player_query: Query<&Player>,
    mut door_query: Query<&mut Door>,
) {
    for player in &player_query {
        if let Some(door_entity) = player.interact {
            if let Ok(mut door) = door_query.get_mut(door_entity) {
                door.toggle();
            }
        }
    }
}

fn door_cooldown_system(mut door_query: Query<&mut Door>, time: Res<Time>) {
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

    use crate::*;
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
    fn player_open_door() {
        // given
        let (mut app, door_entity, player_entity) = setup();

        // when
        app.get_mut::<Player>(player_entity).interact(door_entity);
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
        app.get_mut::<Player>(player_entity).interact(door_entity);
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
    fn player_open_door_wait_3_seconds_and_close_door() {
        // given
        let (mut app, door_entity, player_entity) = setup();

        // when
        app.get_mut::<Player>(player_entity).interact(door_entity);
        app.update_after(Duration::from_secs_f32(3.0));
        app.get_mut::<Player>(player_entity).interact(door_entity);
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
        app.get_mut::<Player>(player_entity).interact(door_entity);
        app.update_after(Duration::from_secs_f32(0.5));
        app.get_mut::<Player>(player_entity).interact(door_entity);
        app.update_after(Duration::from_secs_f32(0.1));

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
        assert!(!result.toggle_cooldown.finished());
    }

    fn setup() -> (App, Entity, Entity) {
        let mut app = App::new();
        app.init_time();
        app.add_systems((door_cooldown_system, player_door_interaction_system));
        let door_entity = app.world.spawn(Door::default()).id();
        let player_entity = app.world.spawn(Player::default()).id();
        (app, door_entity, player_entity)
    }
}