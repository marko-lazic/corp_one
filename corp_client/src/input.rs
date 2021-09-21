use std::fs;

use bevy::app::AppExit;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_mod_picking::RayCastSource;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastMethod};
use kurinji::{Kurinji, KurinjiPlugin, OnActionActive, OnActionEnd};

use corp_shared::prelude::{Health, Player};
use input_command::PlayerAction;

use crate::constants::state::GameState;
use crate::constants::tick;
use crate::world::colony::vortex::VortexEvent;
use crate::world::colony::Colony;
use crate::Game;

#[derive(Default)]
pub struct Cursor {
    pub screen: Vec2,
    pub world: Vec3,
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystem {
    Playing,
    Starmap,
}

pub mod input_command;

pub struct MyRayCastSet;

pub struct InputControlPlugin;

impl InputControlPlugin {
    fn setup_kurinji_binding(mut kurinji: ResMut<Kurinji>) {
        let binding_json = fs::read_to_string("corp_client/config/binding.json")
            .expect("Error! could not open config file");
        kurinji.set_bindings_with_json(&binding_json);
    }

    fn keyboard_escape_action(
        mut game: ResMut<Game>,
        mut windows: ResMut<Windows>,
        mut reader: EventReader<OnActionEnd>,
        mut app_exit_events: EventWriter<AppExit>,
        mut exit_tap: Local<ExitTap>,
        time: Res<Time>,
    ) {
        for event in reader.iter() {
            if event.action == "ESCAPE" {
                let window = windows.get_primary_mut().unwrap();
                if game.cursor_locked {
                    window.set_cursor_lock_mode(false);
                    window.set_cursor_visibility(true);
                    game.cursor_locked = false;
                } else {
                    window.set_cursor_lock_mode(true);
                    window.set_cursor_visibility(false);
                    game.cursor_locked = true;
                }
                exit_tap.counter += 1;
            }
            exit_tap.cooldown.tick(time.delta());
            if exit_tap.cooldown.finished() {
                if exit_tap.counter >= 2 {
                    println!("Quitting...");
                    app_exit_events.send(AppExit);
                } else {
                    exit_tap.counter = 0;
                    exit_tap.cooldown.reset();
                }
            }
        }
    }

    fn player_keyboard_and_mouse_action(
        mut player_action: ResMut<PlayerAction>,
        mut reader: EventReader<OnActionActive>,
    ) {
        for event in reader.iter() {
            player_action.key_action(&event.action);
            player_action.mouse_action(&event.action);
        }
    }

    fn starmap_keyboard(
        keyboard_input: Res<Input<KeyCode>>,
        mut vortex_events: EventWriter<VortexEvent>,
    ) {
        if keyboard_input.just_pressed(KeyCode::I) {
            vortex_events.send(VortexEvent::vort(Colony::Iris));
        } else if keyboard_input.just_pressed(KeyCode::L) {
            vortex_events.send(VortexEvent::vort(Colony::Liberte));
        }
    }

    pub fn update_cursor_position(
        mut crsor_moved_events: EventReader<CursorMoved>,
        mut ray_cast_source: Query<&mut RayCastSource<MyRayCastSet>>,
        mut cursor: ResMut<Cursor>,
    ) {
        for mut pick_source in &mut ray_cast_source.iter_mut() {
            // Grab the most recent cursor event if it exists:
            if let Some(cursor_latest) = crsor_moved_events.iter().last() {
                cursor.screen = cursor_latest.position;
            }

            pick_source.cast_method = RayCastMethod::Screenspace(cursor.screen);
            if let Some((_entity, intersect)) = pick_source.intersect_top() {
                cursor.world = intersect.position();
            }
        }
    }

    fn kill(keyboard_input: Res<Input<KeyCode>>, mut healths: Query<&mut Health, With<Player>>) {
        if keyboard_input.just_pressed(KeyCode::K) {
            if let Some(mut health) = healths.iter_mut().next() {
                health.kill();
            }
        }
    }
}

impl Plugin for InputControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Cursor>();
        app.init_resource::<PlayerAction>();
        app.add_plugin(DefaultRaycastingPlugin::<MyRayCastSet>::default());
        app.add_plugin(KurinjiPlugin::default());
        app.add_startup_system(Self::setup_kurinji_binding.system());
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::keyboard_escape_action.system()),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::StarMap)
                .label(InputSystem::Starmap)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::starmap_keyboard.system()),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label(InputSystem::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::update_cursor_position.system())
                .with_system(Self::player_keyboard_and_mouse_action.system())
                .with_system(Self::kill.system()),
        );
    }
}

struct ExitTap {
    counter: u32,
    cooldown: Timer,
}

impl Default for ExitTap {
    fn default() -> Self {
        Self {
            counter: 0,
            cooldown: Timer::from_seconds(0.4, false),
        }
    }
}
