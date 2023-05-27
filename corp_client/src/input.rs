use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RaycastMethod, RaycastSource, RaycastSystem};
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::Actionlike;

use corp_shared::prelude::Health;
use corp_shared::prelude::*;
use input_command::PlayerDirection;

use crate::input::double_tap::DoubleTap;
use crate::state::GameState;
use crate::world::colony::barrier::{BarrierControl, BarrierField};
use crate::world::colony::vortex::VortInEvent;
use crate::world::colony::Colony;
use crate::{Game, UseEntity};

mod double_tap;
pub mod input_command;

#[derive(Resource, Hash, PartialEq, Eq, Clone, Debug)]
pub enum OrientationMode {
    Direction,
    Aim,
}

#[derive(Resource, Default)]
pub struct Cursor {
    pub screen_ui: Vec2,
    pub world: Vec3,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct InputSystemSet;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum CorpAction {
    Forward,
    Backward,
    Left,
    Right,
    OrientationMode,
    Use,
    Shoot,
    Escape,
    Kill,
    ColonyIris,
    ColonyPlayground,
    ColonyLiberte,
}

#[derive(Component, Reflect, Clone)]
pub struct Ground;

pub struct InputControlPlugin;

impl Plugin for InputControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OrientationMode::Direction);
        app.add_plugin(InputManagerPlugin::<CorpAction>::default());
        app.insert_resource(Self::default_input_map());
        app.insert_resource(ActionState::<CorpAction>::default());
        app.init_resource::<Cursor>();
        app.init_resource::<PlayerDirection>();
        app.add_plugin(DefaultRaycastingPlugin::<Ground>::default());
        app.add_system(Self::keyboard_escape_action);

        app.add_system(Self::starmap_keyboard.in_set(OnUpdate(GameState::StarMap)));

        app.add_systems(
            (
                Self::player_keyboard_action,
                Self::player_mouse_action,
                Self::use_barrier,
                Self::switch_orientation_mode,
                Self::kill,
            )
                .chain()
                .in_set(InputSystemSet)
                .in_set(OnUpdate(GameState::Playing)),
        );
        app.add_systems(
            (
                Self::update_raycast_with_cursor,
                Self::update_screen_cursor_position,
            )
                .before(RaycastSystem::BuildRays::<Ground>)
                .in_base_set(CoreSet::First),
        );
    }
}

impl InputControlPlugin {
    fn default_input_map() -> InputMap<CorpAction> {
        use CorpAction::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(KeyCode::W, Forward);
        input_map.insert(KeyCode::S, Backward);
        input_map.insert(KeyCode::A, Left);
        input_map.insert(KeyCode::D, Right);

        // Abilities
        input_map.insert(KeyCode::E, Use);
        input_map.insert(KeyCode::K, Kill);
        input_map.insert(MouseButton::Left, Shoot);

        // Options
        input_map.insert(KeyCode::Escape, Escape);
        input_map.insert(KeyCode::Space, OrientationMode);
        input_map.insert(KeyCode::I, ColonyIris);
        input_map.insert(KeyCode::P, ColonyPlayground);
        input_map.insert(KeyCode::L, ColonyLiberte);

        input_map
    }

    fn keyboard_escape_action(
        action_state: Res<ActionState<CorpAction>>,
        time: Res<Time>,
        mut game: ResMut<Game>,
        mut windows: Query<&mut Window>,
        mut app_exit_events: EventWriter<AppExit>,
        mut double_tap: Local<DoubleTap>,
    ) {
        if action_state.just_pressed(CorpAction::Escape) {
            double_tap.increment();
            let mut window = windows.single_mut();

            if game.cursor_locked {
                window.cursor.visible = true;
                game.cursor_locked = false;
            } else {
                window.cursor.visible = false;
                game.cursor_locked = true;
            }
        }

        double_tap
            .tick(time.delta())
            .on_complete(|| app_exit_events.send(AppExit));
    }

    fn player_keyboard_action(
        action_state: Res<ActionState<CorpAction>>,
        mut player_action: ResMut<PlayerDirection>,
    ) {
        player_action.move_action(&action_state);
    }

    fn player_mouse_action(
        action_state: Res<ActionState<CorpAction>>,
        mut player_action: ResMut<PlayerDirection>,
    ) {
        player_action.shoot_action(&action_state);
    }

    fn starmap_keyboard(
        action_state: Res<ActionState<CorpAction>>,
        mut vortex_events: EventWriter<VortInEvent>,
    ) {
        if action_state.just_pressed(CorpAction::ColonyIris) {
            vortex_events.send(VortInEvent::vort(Colony::Iris));
        } else if action_state.just_pressed(CorpAction::ColonyLiberte) {
            vortex_events.send(VortInEvent::vort(Colony::Liberte));
        } else if action_state.just_pressed(CorpAction::ColonyPlayground) {
            vortex_events.send(VortInEvent::vort(Colony::Playground));
        }
    }

    fn update_raycast_with_cursor(
        mut cursor: EventReader<CursorMoved>,
        mut query: Query<&mut RaycastSource<Ground>>,
        mut corp_cursor: ResMut<Cursor>,
    ) {
        // Grab the most recent cursor event if it exists:
        let cursor_position = match cursor.iter().last() {
            Some(cursor_moved) => cursor_moved.position,
            None => return,
        };

        for mut pick_source in &mut query {
            pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
            if let Some((_entity, intersect)) = pick_source.intersections().first() {
                corp_cursor.world = intersect.position();
            }
        }
    }

    fn update_screen_cursor_position(
        primary_query: Query<&Window, With<PrimaryWindow>>,
        mut cursor: ResMut<Cursor>,
    ) {
        let Ok(primary) = primary_query.get_single() else {
            return;
        };
        if let Some(position) = primary.cursor_position() {
            cursor.screen_ui.x = position.x;
            cursor.screen_ui.y = position.y;
        }
    }

    fn kill(
        action_state: Res<ActionState<CorpAction>>,
        mut healths: Query<&mut Health, With<Player>>,
    ) {
        if action_state.just_pressed(CorpAction::Kill) {
            if let Some(mut health) = healths.iter_mut().next() {
                health.kill_mut();
            }
        }
    }

    fn use_barrier(
        action_state: Res<ActionState<CorpAction>>,
        mut barriers_query: Query<&mut BarrierField>,
        barrier_access_query: Query<&BarrierControl>,
        game: Res<Game>,
    ) {
        if action_state.just_pressed(CorpAction::Use) {
            if let UseEntity::Barrier(entity) = game.use_entity {
                if let Ok(access) = barrier_access_query.get(entity) {
                    for mut barrier in barriers_query.iter_mut() {
                        if barrier.name == access.barrier_field_name {
                            barrier.open = true;
                        }
                    }
                } else {
                    info!("Unimplemented");
                }
            }
        }
    }

    fn switch_orientation_mode(
        action_state: Res<ActionState<CorpAction>>,
        mut orientation_mode: ResMut<OrientationMode>,
        mut change_mode: Local<bool>,
    ) {
        if action_state.just_pressed(CorpAction::OrientationMode) && !*change_mode {
            if *orientation_mode == OrientationMode::Aim {
                *orientation_mode = OrientationMode::Direction;
            } else {
                *orientation_mode = OrientationMode::Aim;
            }
            *change_mode = true;
        } else {
            *change_mode = false;
        }
    }
}
