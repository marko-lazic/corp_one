use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_mod_picking::backends::rapier::RapierPickTarget;
// use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    SpawnPlayer,
    Playing,
}

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_state::<GameState>()
        .add_plugins((
            DefaultPlugins,
            // DefaultPickingPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        ))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_systems(OnEnter(GameState::Loading), loading)
        .add_systems(OnEnter(GameState::SpawnPlayer), spawn_player)
        .run();
}

fn loading(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // spawn cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
            material: materials.add(Color::WHITE.into()),
            ..Default::default()
        },
        Collider::cuboid(2.5, 2.5, 2.5),
        // PickableBundle::default(),
        // RapierPickTarget::default(),
        // OnPointer::<Click>::run_callback(|In(event): In<ListenedEvent<Click>>| {
        //     info!("Clicked on entity {:?}", event.target);
        //     Bubble::Up
        // }),
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    next_state.set(GameState::SpawnPlayer);
}

fn spawn_player(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // RapierPickCamera::default(), // <- Sets the camera to use for picking.
    ));

    next_state.set(GameState::Playing);
}
