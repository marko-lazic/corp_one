use bevy::prelude::*;

use crate::asset::loading::MeshAssets;
use crate::world::character::Movement;
use crate::world::input_command::PlayerCommand;
use crate::world::player_bundle::PlayerBundle;
use crate::world::WorldSystem;
use crate::{Game, GameState};

#[derive(Default)]
pub struct Player {
    pub is_moving: bool,
}

pub struct PlayerPlugin;

impl PlayerPlugin {
    fn spawn_player(
        mut commands: Commands,
        mesh_assets: Res<MeshAssets>,
        materials: ResMut<Assets<StandardMaterial>>,
        mut game: ResMut<Game>,
    ) {
        let player = commands
            .spawn_bundle(PlayerBundle::new(mesh_assets, materials))
            .insert(Player::default())
            .insert(Movement::default())
            .id();

        game._player_entity = Some(player);
    }

    fn move_player(
        mut command: ResMut<PlayerCommand>,
        mut query: Query<(&mut Player, &mut Movement, &mut Transform)>,
    ) {
        if let Ok((mut player, mut movement, mut transform)) = query.single_mut() {
            let direction = Self::calculate_direction(&command, &transform);
            command.reset();

            movement.velocity = direction * movement.speed;

            transform.translation += movement.velocity;
            player.is_moving = Self::is_moving(&movement.velocity);
        }
    }

    fn calculate_direction(cmd: &PlayerCommand, transform: &Mut<Transform>) -> Vec3 {
        let mut direction = Vec3::ZERO;
        if cmd.forward {
            direction += transform.local_z();
        }
        if cmd.backward {
            direction -= transform.local_z();
        }
        if cmd.left {
            direction += transform.local_x();
        }
        if cmd.right {
            direction -= transform.local_x();
        }
        direction = direction.normalize_or_zero();
        direction
    }

    fn is_moving(delta_move: &Vec3) -> bool {
        delta_move.ne(&Vec3::ZERO)
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(Self::spawn_player.system().label(WorldSystem::PlayerSetup)),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(Self::move_player.system()),
        );
    }
}
