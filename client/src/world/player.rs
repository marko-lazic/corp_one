use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_mod_raycast::RayCastSource;

use crate::asset::asset_loading::MeshAssets;
use crate::constants::state::GameState;
use crate::constants::tick;
use crate::world::camera::{CameraCenter, TopDownCamera};
use crate::world::character::Movement;
use crate::world::cursor::MyRaycastSet;
use crate::world::input_command::PlayerCommand;
use crate::world::player_bundle::PlayerBundle;
use crate::world::world_utils::Label;
use crate::world::WorldSystem;
use crate::Game;

#[derive(Default)]
pub struct Player {
    pub is_moving: bool,
}

pub struct PlayerPlugin;

impl PlayerPlugin {
    fn setup_player(
        mut commands: Commands,
        mesh_assets: Res<MeshAssets>,
        materials: ResMut<Assets<StandardMaterial>>,
        mut game: ResMut<Game>,
    ) {
        let player = commands
            .spawn_bundle(PlayerBundle::new(mesh_assets, materials))
            .insert(Player::default())
            .insert(Movement::default())
            .insert(CameraCenter)
            .id();

        game._player_entity = Some(player);
    }

    fn setup_camera(mut commands: Commands) {
        commands
            .spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_translation(Vec3::new(-3.0, 3.0, 5.0))
                    .looking_at(Vec3::default(), Vec3::Y),
                ..Default::default()
            })
            .insert(TopDownCamera::new(20.0))
            .insert(RayCastSource::<MyRaycastSet>::new());
    }

    fn move_player(
        mut game: ResMut<Game>,
        mut command: ResMut<PlayerCommand>,
        mut query: Query<(&mut Player, &mut Movement, &mut Transform)>,
    ) {
        if let Ok((mut player, mut movement, mut position)) = query.single_mut() {
            let direction = command.new_direction(&position);
            position.translation += movement.update_velocity(direction);

            player.is_moving = Self::is_moving(&movement.velocity);
            command.reset();
            game.camera_center = position.translation;
        }
    }

    fn is_moving(delta_move: &Vec3) -> bool {
        delta_move.ne(&Vec3::ZERO)
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(Self::setup_player.system().label(WorldSystem::SetupPlayer))
                .with_system(
                    Self::setup_camera
                        .system()
                        .label(WorldSystem::SetupCamera)
                        .after(WorldSystem::SetupPlayer),
                ),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::move_player.system().label(Label::Movement)),
        );
    }
}
