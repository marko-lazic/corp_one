use bevy::{app::AppExit, prelude::*, reflect::TypePath};
use leafwing_input_manager::prelude::*;

use corp_shared::prelude::{Health, Interactor, Player};

use crate::{
    asset::Colony,
    state::GameState,
    world::{
        ccc::{ControlMovement, DoubleTap, MainCamera, MainCameraFollow, OrientationMode},
        colony::vortex::VortInEvent,
    },
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ControlSet {
    PlayingInput,
    StarmapInput,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct PlayerEntity(pub Option<Entity>);

impl PlayerEntity {
    pub fn get(&self) -> Option<Entity> {
        self.0
    }
}

impl From<Entity> for PlayerEntity {
    fn from(entity: Entity) -> Self {
        Self(Some(entity))
    }
}

#[derive(Resource, Default)]
pub struct UseEntity(Option<Entity>);

impl UseEntity {
    pub fn set(&mut self, target: Option<Entity>) {
        self.0 = target;
    }

    pub fn get(&self) -> Option<Entity> {
        self.0
    }
}

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

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ControlAction>::default())
            .init_resource::<ActionState<ControlAction>>()
            .init_resource::<PlayerEntity>()
            .init_resource::<CursorWorld>()
            .init_resource::<UseEntity>()
            .insert_resource(ControlSettings::default().input)
            .add_systems(Update, double_tap_to_exit)
            .add_systems(
                Update,
                (
                    update_cursor_world,
                    player_control_movement,
                    player_control_orientation,
                    use_event,
                    kill,
                    toggle_window_cursor_visible,
                )
                    .chain()
                    .in_set(ControlSet::PlayingInput)
                    .run_if(resource_changed::<ActionState<ControlAction>>())
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), enable_cursor_visible)
            .add_systems(
                Update,
                starmap_keyboard
                    .in_set(ControlSet::StarmapInput)
                    .run_if(in_state(GameState::StarMap)),
            );
    }
}

fn double_tap_to_exit(
    action_state: Res<ActionState<ControlAction>>,
    time: Res<Time>,
    mut app_exit_events: EventWriter<AppExit>,
    mut double_tap: Local<DoubleTap>,
) {
    if action_state.just_pressed(ControlAction::Escape) {
        double_tap.increment();
    }
    double_tap
        .tick(time.delta())
        .on_complete(|| app_exit_events.send(AppExit));
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

fn use_event(
    r_use_entity: Res<UseEntity>,
    r_player_entity: Res<PlayerEntity>,
    r_action_state: Res<ActionState<ControlAction>>,
    mut q_interactor: Query<&mut Interactor>,
) {
    if r_action_state.just_pressed(ControlAction::Use) {
        let Some(use_target) = r_use_entity.get() else {
            return;
        };
        let Some(player) = r_player_entity.get() else {
            return;
        };
        let Ok(mut interactor) = q_interactor.get_mut(player) else {
            return;
        };
        interactor.interact(use_target);
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

fn toggle_window_cursor_visible(
    action_state: Res<ActionState<ControlAction>>,
    mut windows: Query<&mut Window>,
) {
    if action_state.just_pressed(ControlAction::Escape) {
        let mut window = windows.single_mut();
        window.cursor.visible = !window.cursor.visible;
    }
}

fn enable_cursor_visible(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.visible = true;
}

fn starmap_keyboard(
    r_action_state: Res<ActionState<ControlAction>>,
    mut ev_vort_in: EventWriter<VortInEvent>,
) {
    if r_action_state.just_pressed(ControlAction::ColonyIris) {
        ev_vort_in.send(VortInEvent::vort(Colony::Iris));
    } else if r_action_state.just_pressed(ControlAction::ColonyLiberte) {
        ev_vort_in.send(VortInEvent::vort(Colony::Liberte));
    } else if r_action_state.just_pressed(ControlAction::ColonyPlayground) {
        ev_vort_in.send(VortInEvent::vort(Colony::Playground));
    }
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
            .add_state::<GameState>()
            .add_plugins(MinimalPlugins)
            .add_plugins((InputPlugin, ControlPlugin));
        app
    }
}
