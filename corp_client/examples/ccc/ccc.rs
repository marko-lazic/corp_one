//! Demonstrates all common configuration options,
//! and how to modify them at runtime

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};

use corp_shared::prelude::Player;

use crate::camera::{CameraSet, MainCameraBundle, MainCameraFollow, MainCameraPlugin};
use crate::character::{CharacterPlugin, CharacterSet};
use crate::control::{ControlPlugin, ControlSet};
use crate::movement::MovementBundle;

mod camera;
mod character;
mod control;
mod movement;

fn new_window() -> Window {
    Window {
        mode: WindowMode::BorderlessFullscreen,
        present_mode: PresentMode::AutoNoVsync, // Reduces input latency
        ..default()
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(new_window()),
            ..default()
        }))
        .add_plugin(MainCameraPlugin)
        .add_plugin(ControlPlugin)
        .add_plugin(CharacterPlugin)
        .add_startup_system(setup)
        .configure_set(ControlSet::Input.before(CharacterSet::Movement))
        .configure_set(CameraSet::Update.after(CharacterSet::Movement))
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
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
        material: materials.add(Color::rgb(0.8, 0.1, 0.1).into()),
        transform: Transform::from_xyz(2.0, 0.5, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
        material: materials.add(Color::rgb(0.1, 0.1, 0.8).into()),
        transform: Transform::from_xyz(0.0, 0.5, 2.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
        material: materials.add(Color::rgb(0.1, 0.8, 0.1).into()),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..default()
    });

    // Player
    let player_pos = Transform::from_xyz(0.0, 0.5, 0.0);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.2, 1.0, 0.2))),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: player_pos,
            ..default()
        },
        Player,
        MainCameraFollow,
        MovementBundle::default(),
    ));

    // Camera
    commands.spawn(MainCameraBundle::new(player_pos.translation));

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
}

fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}
