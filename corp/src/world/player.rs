use bevy::prelude::*;

use crate::loading::MeshAssets;
use crate::world::WorldSystem;
use crate::{Game, GameState};

pub struct Player {
    pub _movement: Movement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player.system().label(WorldSystem::PlayerSetup)),
        );
    }
}

#[derive(Debug)]
pub struct Movement {
    pub acceleration: f32,
    pub speed: f32,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            acceleration: 12.0,
            speed: 400.0,
        }
    }
}

struct PlayerPbrBundle;

impl PlayerPbrBundle {
    fn create(
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
            transform: Transform::from_xyz(10.0, 0., -10.0),
            ..Default::default()
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mesh_assets: Res<MeshAssets>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
) {
    let player = commands
        .spawn_bundle(PlayerPbrBundle::create(mesh_assets, materials))
        .insert(Player {
            _movement: Movement::default(),
        })
        .id();

    game.player = Some(player);
}
