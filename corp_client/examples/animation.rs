use bevy::prelude::*;
use bevy::utils::HashMap;
use impl_tools::impl_default;

#[derive(Default)]
struct Game {
    current_action: Action,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_startup_system(setup)
        .add_system(setup_scene_once_loaded)
        .add_system(keyboard_animation_control)
        .add_system(play_animations)
        .run();
}

#[derive(PartialEq, Eq, Hash)]
#[impl_default(Action::IDLE)]
enum Action {
    IDLE = 0,
    RUN = 1,
}

#[derive(Deref, DerefMut)]
struct Animations(HashMap<Action, Handle<AnimationClip>>);

fn setup(
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(5.0, 5.0, 8.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    // Plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 500000.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // Light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        )),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // Mannequiny
    // Insert a resource with the current scene information
    let mut hm: HashMap<Action, Handle<AnimationClip>> = HashMap::new();
    hm.insert(
        Action::RUN,
        asset_server.load("mesh/mannequiny/mannequiny.gltf#Animation9"),
    );
    hm.insert(
        Action::IDLE,
        asset_server.load("mesh/mannequiny/mannequiny.gltf#Animation7"),
    );
    commands.insert_resource(Animations(hm));

    let mannequiny: Handle<Scene> = asset_server.load("mesh/mannequiny/mannequiny.gltf#Scene0");
    scene_spawner.spawn(mannequiny);
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player
                .play(animations.get(&Action::IDLE).unwrap().clone_weak())
                .repeat();
            *done = true;
        }
    }
}

fn play_animations(
    game: Res<Game>,
    animations: Res<Animations>,
    mut animation_player: Query<&mut AnimationPlayer>,
    mut last_action: Local<Action>,
) {
    if let Ok(mut player) = animation_player.get_single_mut() {
        if game.current_action == Action::RUN && *last_action != Action::RUN {
            player
                .play(animations.get(&Action::RUN).unwrap().clone_weak())
                .repeat();
            *last_action = Action::RUN;
        } else if game.current_action == Action::IDLE && *last_action != Action::IDLE {
            player
                .play(animations.get(&Action::IDLE).unwrap().clone_weak())
                .repeat();
            *last_action = Action::IDLE;
        }
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_player: Query<&mut AnimationPlayer>,
    mut game: ResMut<Game>,
) {
    if let Ok(mut player) = animation_player.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::Up) {
            let speed = player.speed();
            player.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Right) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed + 0.1);
        }

        if keyboard_input.pressed(KeyCode::W) {
            game.current_action = Action::RUN
        } else {
            game.current_action = Action::IDLE
        }
    }
}
