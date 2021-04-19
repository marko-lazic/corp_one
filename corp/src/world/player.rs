use crate::SystemLoading;
use bevy::prelude::*;

pub struct Player;

#[derive(Debug)]
pub struct MovementSpeed {
    pub acceleration: f32,
    pub max: f32,
    pub is_moving: bool,
    pub moving_happen: bool,
}

impl Default for MovementSpeed {
    fn default() -> Self {
        Self {
            acceleration: 12.0,
            max: 400.0,
            is_moving: false,
            moving_happen: false,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(
            setup
                .system()
                .after(SystemLoading::Scene)
                .label(SystemLoading::PlayerSetup),
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_handle =
        asset_server.load("models/mannequiny/mannequiny-0.3.0.glb#Mesh0/Primitive0");

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    // player
    let player = commands
        .spawn_bundle(PbrBundle {
            mesh: player_handle,
            material: material_handle.clone(),
            transform: Transform::from_xyz(10.0, 0., -10.0),
            ..Default::default()
        })
        .insert(Player {})
        .insert(MovementSpeed::default())
        .id();

    let camera = commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        })
        .id();

    commands.entity(player).push_children(&[camera]);
}
