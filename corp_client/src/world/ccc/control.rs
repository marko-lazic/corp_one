use bevy::{app::AppExit, prelude::*, reflect::TypePath};
use leafwing_input_manager::prelude::*;

use corp_shared::prelude::{Health, Interactor, Player};

use crate::{
    state::GameState,
    world::{
        ccc::{
            camera::{MainCamera, MainCameraFollow},
            double_tap::DoubleTap,
            movement::{ControlMovement, OrientationMode},
        },
        colony::{
            barrier::{BarrierControl, BarrierField},
            vortex::VortInEvent,
            Colony,
        },
    },
    Game,
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ControlSet {
    PlayingInput,
    StarmapInput,
}

pub struct ControlPlugin;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CursorWorld(Vec3);

#[derive(Actionlike, Debug, PartialEq, Clone, Copy, TypePath)]
pub enum ControlAction {
    Forward,
    Backward,
    Left,
    Right,
    Aim,
    OrientationMode,
    Use,
    Shoot,
    Escape,
    CameraZoomIn,
    CameraZoomOut,
    CameraRotateClockwise,
    CameraRotateCounterClockwise,
    Kill,
    ColonyIris,
    ColonyPlayground,
    ColonyLiberte,
}

#[derive(Resource)]
pub struct ControlSettings {
    input: InputMap<ControlAction>,
}

impl Default for ControlSettings {
    fn default() -> Self {
        let mut input = InputMap::default();
        input
            // Movement
            .insert(KeyCode::W, ControlAction::Forward)
            .insert(KeyCode::S, ControlAction::Backward)
            .insert(KeyCode::A, ControlAction::Left)
            .insert(KeyCode::D, ControlAction::Right)
            // Weapon
            .insert(MouseButton::Right, ControlAction::Aim)
            // Abilities
            .insert(KeyCode::E, ControlAction::Use)
            .insert(KeyCode::K, ControlAction::Kill)
            .insert(MouseButton::Left, ControlAction::Shoot)
            // Options
            .insert(KeyCode::Escape, ControlAction::Escape)
            .insert(KeyCode::Space, ControlAction::OrientationMode)
            .insert(KeyCode::Equals, ControlAction::CameraZoomIn)
            .insert(KeyCode::Minus, ControlAction::CameraZoomOut)
            .insert(KeyCode::Z, ControlAction::CameraRotateClockwise)
            .insert(KeyCode::C, ControlAction::CameraRotateCounterClockwise)
            .insert(KeyCode::I, ControlAction::ColonyIris)
            .insert(KeyCode::P, ControlAction::ColonyPlayground)
            .insert(KeyCode::L, ControlAction::ColonyLiberte);

        Self { input }
    }
}

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ControlAction>::default())
            .init_resource::<ActionState<ControlAction>>()
            .init_resource::<CursorWorld>()
            .insert_resource(ControlSettings::default().input)
            .add_systems(Update, keyboard_escape_action)
            .add_systems(
                Update,
                (
                    update_cursor_world,
                    player_control_movement,
                    player_control_orientation,
                    use_barrier,
                    kill,
                )
                    .chain()
                    .in_set(ControlSet::PlayingInput)
                    .run_if(resource_changed::<ActionState<ControlAction>>())
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                starmap_keyboard
                    .in_set(ControlSet::StarmapInput)
                    .run_if(in_state(GameState::StarMap)),
            );
    }
}

fn player_control_movement(
    action_state: Res<ActionState<ControlAction>>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mut q_movement: Query<&mut ControlMovement, With<Player>>,
) {
    let Ok(cam) = q_camera.get_single() else {
        return;
    };

    let cam_forward = Vec3::new(
        cam.rotation.mul_vec3(Vec3::Z).x,
        0.0,
        cam.rotation.mul_vec3(Vec3::Z).z,
    )
    .normalize_or_zero();
    let cam_right = Vec3::new(
        cam.rotation.mul_vec3(Vec3::X).x,
        0.0,
        cam.rotation.mul_vec3(Vec3::X).z,
    )
    .normalize_or_zero();

    let Ok(mut movement) = q_movement.get_single_mut() else {
        return;
    };
    let mut direction = Vec3::ZERO;
    if action_state.pressed(ControlAction::Forward) {
        direction -= cam_forward;
    }
    if action_state.pressed(ControlAction::Backward) {
        direction += cam_forward;
    }
    if action_state.pressed(ControlAction::Left) {
        direction -= cam_right;
    }
    if action_state.pressed(ControlAction::Right) {
        direction += cam_right;
    }

    movement.direction = direction;
}

fn player_control_orientation(
    cursor_world: Res<CursorWorld>,
    action_state: Res<ActionState<ControlAction>>,
    mut q_orientation: Query<&mut OrientationMode, With<Player>>,
) {
    if action_state.just_pressed(ControlAction::OrientationMode) {
        for mut orientation_mode in &mut q_orientation {
            *orientation_mode = match *orientation_mode {
                OrientationMode::Direction => {
                    OrientationMode::Location(Vec2::new(cursor_world.x, cursor_world.z))
                }
                OrientationMode::Location(_) => OrientationMode::Direction,
            }
        }
    }
}

fn update_cursor_world(
    windows: Query<&Window>,
    mut cursor_world: ResMut<CursorWorld>,
    q_follow_cam: Query<&Transform, With<MainCameraFollow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_orientation: Query<&mut OrientationMode, With<Player>>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        return;
    };
    let ground = Transform::from_xyz(0.0, 0.0, 0.0);
    let Ok(follow_pos) = q_follow_cam.get_single() else {
        return;
    };

    let ray = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .unwrap_or_else(|| Ray {
            origin: follow_pos.translation,
            direction: follow_pos.down(),
        });

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(ground.translation, ground.up()) else {
        return;
    };
    let mouse_ground_pos = ray.get_point(distance);
    cursor_world.0 = mouse_ground_pos;

    for mut orientation_mode in &mut q_orientation {
        if let OrientationMode::Location(_) = *orientation_mode {
            *orientation_mode =
                OrientationMode::Location(Vec2::new(mouse_ground_pos.x, mouse_ground_pos.z));
        }
    }
}

fn use_barrier(
    action_state: Res<ActionState<ControlAction>>,
    barrier_control_query: Query<&BarrierControl>,
    barrier_field_query: Query<(Entity, &BarrierField)>,
    mut interactor_query: Query<&mut Interactor>,
    game: Res<Game>,
) {
    if action_state.just_pressed(ControlAction::Use) {
        let Some(target_entity) = game.use_entity else {
            return;
        };

        let Ok(barrier_control) = barrier_control_query.get(target_entity) else {
            return;
        };

        let Some((target_barrier, _)) = barrier_field_query
            .iter()
            .find(|(_e, b)| b.name == barrier_control.barrier_field_name)
        else {
            return;
        };

        let Ok(mut interactor) = interactor_query.get_single_mut() else {
            return;
        };
        interactor.interact(target_barrier);
    }
}

fn starmap_keyboard(
    action_state: Res<ActionState<ControlAction>>,
    mut vortex_events: EventWriter<VortInEvent>,
) {
    if action_state.just_pressed(ControlAction::ColonyIris) {
        vortex_events.send(VortInEvent::vort(Colony::Iris));
    } else if action_state.just_pressed(ControlAction::ColonyLiberte) {
        vortex_events.send(VortInEvent::vort(Colony::Liberte));
    } else if action_state.just_pressed(ControlAction::ColonyPlayground) {
        vortex_events.send(VortInEvent::vort(Colony::Playground));
    }
}

fn kill(
    action_state: Res<ActionState<ControlAction>>,
    mut healths: Query<&mut Health, With<Player>>,
) {
    if action_state.just_pressed(ControlAction::Kill) {
        if let Some(mut health) = healths.iter_mut().next() {
            health.kill_mut();
        }
    }
}

fn keyboard_escape_action(
    action_state: Res<ActionState<ControlAction>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut windows: Query<&mut Window>,
    mut app_exit_events: EventWriter<AppExit>,
    mut double_tap: Local<DoubleTap>,
) {
    if action_state.just_pressed(ControlAction::Escape) {
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

#[cfg(test)]
mod tests {
    use bevy::input::InputPlugin;

    use corp_shared::prelude::*;

    use super::*;

    #[test]
    fn send_input() {
        // given
        let mut app = setup();

        // when
        app.send_input(KeyCode::W);
        app.update();

        // then
        assert!(app
            .world
            .resource::<ActionState<ControlAction>>()
            .pressed(ControlAction::Forward));
    }

    fn setup() -> App {
        let mut app = App::new();
        app.init_time()
            .add_event::<VortInEvent>()
            .init_resource::<Game>()
            .add_state::<GameState>()
            .add_plugins(MinimalPlugins)
            .add_plugins((InputPlugin, ControlPlugin));
        app
    }
}
