use crate::loading::MeshAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(spawn_player_and_camera.system()),
        );
    }
}

pub struct Player;

#[derive(Debug)]
pub struct MovementSpeed {
    pub acceleration: f32,
    pub max: f32,
}

impl Default for MovementSpeed {
    fn default() -> Self {
        Self {
            acceleration: 12.0,
            max: 400.0,
        }
    }
}

fn spawn_player_and_camera(
    mut commands: Commands,
    mesh_assets: Res<MeshAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_handle = mesh_assets.mannequiny.clone();

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
