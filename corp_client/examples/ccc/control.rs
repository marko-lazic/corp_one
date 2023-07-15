use bevy::prelude::*;
use derive_more::Display;
use leafwing_input_manager::prelude::*;

use corp_shared::prelude::Player;

use crate::camera::MainCamera;
use crate::movement::ControlMovement;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ControlSet {
    Input,
}

pub struct ControlPlugin;

#[derive(Actionlike, Debug, PartialEq, Clone, Copy, Display)]
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
        app.add_plugin(InputManagerPlugin::<ControlAction>::default());
        app.init_resource::<ActionState<ControlAction>>();
        let control_settings = ControlSettings::default();
        app.insert_resource(control_settings.input);
        app.add_system(
            player_control_movement
                .in_set(ControlSet::Input)
                .run_if(resource_changed::<ActionState<ControlAction>>()),
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
        app.init_time();
        app.add_plugins(MinimalPlugins);
        app.add_plugin(InputPlugin);
        app.add_plugin(ControlPlugin);
        app
    }
}
