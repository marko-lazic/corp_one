use bevy::prelude::*;

use crate::loading::MeshAssets;
use crate::world::character::{CharacterBundle, CharacterName, Movement};
use crate::world::WorldSystem;
use crate::{Game, GameState};

#[derive(Default)]
pub struct Input {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Default)]
pub struct Player {
    pub input: Input,
    pub is_moving: bool,
}

impl Player {
    fn move_player(
        game: Res<Game>,
        mut query: Query<(&mut Player, &mut Movement, &mut Transform)>,
    ) {
        if let Ok((mut player, mut movement, mut transform)) = query.single_mut() {
            let direction = Player::calculate_direction(&player, &transform);
            player.input = Input::default();

            movement.velocity = direction * movement.speed;

            if let Some(_cam_transform) = game.camera_transform {}

            transform.translation += movement.velocity;
            player.is_moving = is_moving(&movement.velocity);
        }
    }

    fn calculate_direction(player: &Player, transform: &Mut<Transform>) -> Vec3 {
        let mut direction = Vec3::ZERO;
        if player.input.forward {
            direction += transform.local_z();
        }
        if player.input.backward {
            direction -= transform.local_z();
        }
        if player.input.left {
            direction += transform.local_x();
        }
        if player.input.right {
            direction -= transform.local_x();
        }
        direction = direction.normalize_or_zero();
        direction
    }
}

fn is_moving(delta_move: &Vec3) -> bool {
    delta_move.ne(&Vec3::ZERO)
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player.system().label(WorldSystem::PlayerSetup)),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(Player::move_player.system()),
        );
    }
}

fn create_pbr(
    mesh_assets: Res<MeshAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> PbrBundle {
    let mesh = mesh_assets.mannequiny.clone();

    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    PbrBundle {
        mesh,
        material,
        transform: Transform::from_xyz(0.0, 0., 0.0),
        ..Default::default()
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    pub character: CharacterBundle,

    #[bundle]
    pub pbr: PbrBundle,
}

fn spawn_player(
    mut commands: Commands,
    mesh_assets: Res<MeshAssets>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
) {
    let player = commands
        .spawn_bundle(PlayerBundle {
            character: CharacterBundle {
                name: CharacterName::new("The Guy"),
                ..Default::default()
            },
            pbr: create_pbr(mesh_assets, materials),
        })
        .insert(Player::default())
        .insert(Movement::default())
        .id();

    game._player = Some(player);
}
