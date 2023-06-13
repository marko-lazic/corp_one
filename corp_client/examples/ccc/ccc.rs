//! Demonstrates all common configuration options,
//! and how to modify them at runtime

use std::f32::consts::TAU;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin, PanOrbitCameraSystemSet};

use corp_shared::prelude::Player;

use crate::camera::CameraSet;
use crate::character::{CharacterMovement, CharacterPlugin, CharacterSet};
use crate::control::ControlPlugin;

mod camera;
mod character;
mod control;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(ControlPlugin)
        .add_plugin(CharacterPlugin)
        .add_startup_system(setup)
        .add_systems(
            (
                toggle_camera_controls_system,
                camera_follow,
                camera_rotation,
            )
                .in_set(CameraSet::Movement),
        )
        .configure_set(PanOrbitCameraSystemSet.after(CameraSet::Movement))
        .configure_set(PanOrbitCameraSystemSet.after(CharacterSet::Movement))
        .configure_set(CameraSet::Movement.after(CharacterSet::Movement))
        .add_system(exit_game)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // Player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        CharacterMovement::default(),
    ));
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // Camera
    commands.spawn((
        // Note we're setting the initial position below with alpha, beta, and radius, hence
        // we don't set transform on the camera.
        Camera3dBundle::default(),
        PanOrbitCamera {
            // Set focal point (what the camera should look at)
            focus: Vec3::new(0.0, 1.0, 0.0),
            target_focus: Vec3::new(0.0, 1.0, 0.0),
            // Set the starting
            // position, relative to focus (overrides camera's transform).
            alpha: Some(0.0),
            beta: Some(TAU / 8.0),
            radius: Some(5.0),
            // Set limits on how far in will rotate
            alpha_upper_limit: None,
            alpha_lower_limit: None,
            beta_upper_limit: Some(TAU / 3.0),
            beta_lower_limit: Some(-TAU / 3.0),
            // Adjust sensitivity of controls
            orbit_sensitivity: 0.5,
            pan_sensitivity: 0.5,
            pan_smoothness: 0.6,
            zoom_sensitivity: 0.5,
            // Allow the camera to go upside down
            allow_upside_down: true,
            // Change the controls (these match Blender)
            button_orbit: MouseButton::Left,
            button_pan: MouseButton::Middle,
            modifier_pan: None,
            // Reverse the zoom direction
            reversed_zoom: true,
            ..default()
        },
    ));
}

// Press 'T' to toggle the camera controls
fn toggle_camera_controls_system(
    key_input: Res<Input<KeyCode>>,
    mut pan_orbit_query: Query<&mut PanOrbitCamera>,
) {
    if key_input.just_pressed(KeyCode::T) {
        info!("T pressed");
        for mut pan_orbit in pan_orbit_query.iter_mut() {
            pan_orbit.enabled = !pan_orbit.enabled;
        }
    }

    if key_input.pressed(KeyCode::Equals) {
        for mut pan_orbit in pan_orbit_query.iter_mut() {
            pan_orbit.radius = Some(pan_orbit.radius.unwrap() - 0.1);
            pan_orbit.force_update = true;
        }
    }

    if key_input.pressed(KeyCode::Minus) {
        for mut pan_orbit in pan_orbit_query.iter_mut() {
            pan_orbit.radius = Some(pan_orbit.radius.unwrap() + 0.1);
            pan_orbit.force_update = true;
        }
    }
}

fn camera_follow(
    player_transform_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut pan_orbit_query: Query<&mut PanOrbitCamera>,
) {
    let Ok(player_tr) = player_transform_query.get_single() else {
        return;
    };

    let Ok(mut pan_orbit) = pan_orbit_query.get_single_mut() else {
        return;
    };

    pan_orbit.target_focus = player_tr.translation;
    pan_orbit.force_update = true;
}

fn camera_rotation(
    key_input: Res<Input<KeyCode>>,
    mut pan_orbit_query: Query<&mut PanOrbitCamera>,
) {
    let Ok(mut pan_orbit) = pan_orbit_query.get_single_mut() else {
        return;
    };

    if key_input.pressed(KeyCode::Right) {
        pan_orbit.target_alpha = pan_orbit.alpha.unwrap_or(0.0) + 0.2;
    }

    if key_input.pressed(KeyCode::Left) {
        pan_orbit.target_alpha = pan_orbit.alpha.unwrap_or(0.0) - 0.2;
    }
}

fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}
