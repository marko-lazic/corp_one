use bevy::prelude::*;

use crate::door::{Door, door_cooldown_system, DoorState};
use crate::interactive::interaction_system;

mod test_utils;
mod interactive;
mod door;
mod backpack;
mod player;
mod inventory;
mod item;

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
        .add_system(door_cooldown_system)
        .add_system(interaction_system)
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
        match &door.state() {
            DoorState::Open => text.sections[0].value = "Open".to_string(),
            DoorState::Closed => text.sections[0].value = "Closed".to_string(),
        }
    }
}

