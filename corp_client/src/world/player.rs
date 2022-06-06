use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody};
use iyes_loopless::condition::ConditionSet;
use rand::seq::SliceRandom;

use corp_shared::prelude::*;

use crate::asset::asset_loading::SceneAssets;
use crate::constants::state::GameState;
use crate::input::input_command::PlayerAction;
use crate::input::InputSystem;
use crate::world::camera::CameraCenter;
use crate::world::character::Movement;
use crate::world::cloning::CloningPlugin;
use crate::world::colony::vortex::VortexNode;
use crate::world::colony::Layer;
use crate::world::WorldSystem;
use crate::Game;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CloningPlugin);

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::SpawnPlayer)
                .label(WorldSystem::PlayerSetup)
                .with_system(PlayerPlugin::setup_player)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .after(InputSystem::CheckInteraction)
                .with_system(Self::move_player)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(Self::handle_dead)
                .into(),
        );
    }
}

impl PlayerPlugin {
    pub fn setup_player(
        mut game: ResMut<Game>,
        mut commands: Commands,
        scene_assets: Res<SceneAssets>,
        mut vortex_nodes: Query<&mut Transform, With<VortexNode>>,
    ) {
        let random_position = vortex_nodes
            .iter_mut()
            .map(|t| t.translation)
            .collect::<Vec<Vec3>>()
            .choose(&mut rand::thread_rng())
            .map(|p| p.to_owned());

        let position = random_position.unwrap_or_else(|| Vec3::new(1.0, 1.0, 1.0));

        let player = commands
            .spawn_bundle(TransformBundle::from(Transform::from_xyz(
                position.x, position.y, position.z,
            )))
            .with_children(|parent| {
                parent.spawn_scene(scene_assets.mannequiny.clone());
            })
            .insert(Player::default())
            .insert(Movement::default())
            .insert(game.health.clone())
            .insert(CameraCenter)
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(0.5, 1.0, 0.5),
                border_radius: None,
            })
            .insert(
                CollisionLayers::none()
                    .with_group(Layer::Player)
                    .with_masks(vec![Layer::Zone, Layer::VortexGate]),
            )
            .id();

        game.player_entity = Some(player);
    }

    fn handle_dead(mut query: Query<(&mut Movement, &Health)>) {
        for (mut movement, health) in query.iter_mut() {
            if !health.is_dead() {
                movement.can_move = true;
            } else {
                movement.can_move = false;
            }
        }
    }

    fn move_player(
        mut command: ResMut<PlayerAction>,
        time: Res<Time>,
        mut query: Query<(&mut Player, &mut Movement, &mut Transform)>,
    ) {
        if let Ok((mut player, mut movement, mut position)) = query.get_single_mut() {
            let direction = command.new_direction(&position);
            if movement.can_move {
                position.translation += movement.update_velocity(direction) * time.delta_seconds();
            }

            player.is_moving = Self::is_moving(&movement.velocity);
            command.reset();
        }
    }

    fn is_moving(delta_move: &Vec3) -> bool {
        delta_move.ne(&Vec3::ZERO)
    }
}
