use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_input_actionmap::*;
use bevy_mod_picking::RayCastSource;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastMethod};
use iyes_loopless::prelude::ConditionSet;

use corp_shared::prelude::Health;
use input_command::PlayerDirection;

use crate::constants::state::GameState;
use crate::input::double_tap::DoubleTap;
use crate::world::colony::barrier::{BarrierAccess, BarrierField};
use crate::world::colony::vortex::VortInEvent;
use crate::world::colony::Colony;
use crate::world::player::Player;
use crate::{Game, UseEntity};

mod double_tap;
pub mod input_command;

#[derive(Default)]
pub struct Cursor {
    pub screen: Vec2,
    pub world: Vec3,
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystem {
    CheckInteraction,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Action {
    Forward,
    Backward,
    Left,
    Right,
    Use,
    Shoot,
    Escape,
    Kill,
    ColonyIris,
    ColonyPlayground,
    ColonyLiberte,
}

pub struct MyRayCastSet;

pub struct InputControlPlugin;

impl InputControlPlugin {
    fn keyboard_escape_action(
        input: Res<InputMap<Action>>,
        time: Res<Time>,
        mut game: ResMut<Game>,
        mut windows: ResMut<Windows>,
        mut app_exit_events: EventWriter<AppExit>,
        mut double_tap: Local<DoubleTap>,
    ) {
        if input.just_active(Action::Escape) {
            double_tap.increment();
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
        }

        double_tap
            .tick(time.delta())
            .on_complete(|| app_exit_events.send(AppExit));
    }

    fn player_keyboard_action(
        input: Res<InputMap<Action>>,
        mut player_action: ResMut<PlayerDirection>,
    ) {
        player_action.key_action(&input);
    }

    fn player_mouse_action(
        buttons: Res<Input<MouseButton>>,
        mut player_action: ResMut<PlayerDirection>,
    ) {
        player_action.mouse_action(&buttons);
    }

    fn setup(mut input: ResMut<InputMap<Action>>) {
        input
            .bind(Action::Forward, KeyCode::W)
            .bind(Action::Backward, KeyCode::S)
            .bind(Action::Left, KeyCode::A)
            .bind(Action::Right, KeyCode::D)
            .bind(Action::Use, KeyCode::E)
            // .bind(Action::Shoot, MouseButton::Left)
            .bind(Action::Escape, KeyCode::Escape)
            .bind(Action::Kill, KeyCode::K)
            .bind(Action::ColonyIris, KeyCode::I)
            .bind(Action::ColonyPlayground, KeyCode::P)
            .bind(Action::ColonyLiberte, KeyCode::L);
    }

    fn starmap_keyboard(input: Res<InputMap<Action>>, mut vortex_events: EventWriter<VortInEvent>) {
        if input.just_active(Action::ColonyIris) {
            vortex_events.send(VortInEvent::vort(Colony::Iris));
        } else if input.just_active(Action::ColonyLiberte) {
            vortex_events.send(VortInEvent::vort(Colony::Liberte));
        } else if input.just_active(Action::ColonyPlayground) {
            vortex_events.send(VortInEvent::vort(Colony::Playground));
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

    fn kill(input: Res<InputMap<Action>>, mut healths: Query<&mut Health, With<Player>>) {
        if input.just_active(Action::Kill) {
            if let Some(mut health) = healths.iter_mut().next() {
                health.kill_mut();
            }
        }
    }

    fn use_barrier(
        input: Res<InputMap<Action>>,
        mut barriers_query: Query<&mut BarrierField>,
        barrier_access_query: Query<&BarrierAccess>,
        game: Res<Game>,
    ) {
        if input.just_active(Action::Use) {
            if let UseEntity::Barrier(entity) = game.use_entity {
                let access = barrier_access_query.get(entity).unwrap();
                for mut barrier in barriers_query.iter_mut() {
                    if barrier.name == access.barrier_field_name {
                        barrier.open = true;
                    }
                }
            }
        }
    }
}

impl Plugin for InputControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ActionPlugin::<Action>::default());
        app.add_startup_system(Self::setup);
        app.init_resource::<Cursor>();
        app.init_resource::<PlayerDirection>();
        app.add_plugin(DefaultRaycastingPlugin::<MyRayCastSet>::default());
        app.add_system(Self::keyboard_escape_action);

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::StarMap)
                .with_system(Self::starmap_keyboard)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .label(InputSystem::CheckInteraction)
                .with_system(Self::update_cursor_position)
                .with_system(Self::player_keyboard_action)
                .with_system(Self::player_mouse_action)
                .with_system(Self::use_barrier)
                .with_system(Self::kill)
                .into(),
        );
    }
}
