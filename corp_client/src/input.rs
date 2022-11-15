use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_mod_picking::RayCastSource;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastMethod, RaycastSystem};
use iyes_loopless::prelude::ConditionSet;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::Actionlike;

use corp_shared::prelude::Health;
use input_command::PlayerDirection;

use crate::constants::state::GameState;
use crate::input::double_tap::DoubleTap;
use crate::world::colony::barrier::{BarrierControl, BarrierField};
use crate::world::colony::vortex::VortInEvent;
use crate::world::colony::Colony;
use crate::world::player::Player;
use crate::{Game, UseEntity};

mod double_tap;
pub mod input_command;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum OrientationMode {
    Direction,
    Aim,
}

#[derive(Default)]
pub struct Cursor {
    pub screen: Vec2,
    pub world: Vec3,
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystem {
    CheckInteraction,
}

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

#[derive(Component)]
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
                .with_system(Self::player_keyboard_action)
                .with_system(Self::player_mouse_action)
                .with_system(Self::use_barrier)
                .with_system(Self::switch_orientation_mode)
                .with_system(Self::kill)
                .into(),
        );
        app.add_system_to_stage(
            CoreStage::First,
            Self::update_cursor_and_raycast.before(RaycastSystem::BuildRays::<Ground>),
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
        mut windows: ResMut<Windows>,
        mut app_exit_events: EventWriter<AppExit>,
        mut double_tap: Local<DoubleTap>,
    ) {
        if action_state.just_pressed(CorpAction::Escape) {
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

    pub fn update_cursor_and_raycast(
        mut cursor_event: EventReader<CursorMoved>,
        mut query: Query<&mut RayCastSource<Ground>>,
        mut cursor: ResMut<Cursor>,
    ) {
        for mut pick_source in &mut query.iter_mut() {
            if let Some(cursor_latest) = cursor_event.iter().last() {
                cursor.screen = cursor_latest.position.clone();
                pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
            }
            if let Some((_entity, intersect)) = pick_source.intersect_top() {
                cursor.world = intersect.position();
            }
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
                let access = barrier_access_query.get(entity).unwrap();
                for mut barrier in barriers_query.iter_mut() {
                    if barrier.name == access.barrier_field_name {
                        barrier.open = true;
                    }
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
