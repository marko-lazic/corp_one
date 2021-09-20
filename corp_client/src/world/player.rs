use bevy::core::FixedTimestep;
use bevy::prelude::*;

use corp_shared::prelude::*;

use crate::asset::asset_loading::MeshAssets;
use crate::constants::state::GameState;
use crate::constants::tick;
use crate::input::input_command::PlayerAction;
use crate::input::InputSystem;
use crate::world::character::{CharacterBundle, CharacterName, Movement};
use crate::world::cloning::CloningPlugin;

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub character: CharacterBundle,

    #[bundle]
    pub pbr: PbrBundle,
}

impl PlayerBundle {
    pub fn new(
        mesh_assets: Res<MeshAssets>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        spawn_position: Vec3,
    ) -> PlayerBundle {
        PlayerBundle {
            character: CharacterBundle {
                name: CharacterName::new("The Guy"),
                ..Default::default()
            },
            pbr: PbrBundle {
                mesh: mesh_assets.mannequiny.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.8, 0.7, 0.6),
                    ..Default::default()
                }),
                transform: Transform::from_translation(spawn_position),
                ..Default::default()
            },
        }
    }
}

pub struct PlayerPlugin;

impl PlayerPlugin {
    fn move_player(
        mut command: ResMut<PlayerAction>,
        mut query: Query<(&mut Player, &mut Movement, &mut Transform, &Health)>,
    ) {
        if let Ok((mut player, mut movement, mut position, health)) = query.single_mut() {
            let direction = command.new_direction(&position);
            if !health.is_dead() {
                position.translation += movement.update_velocity(direction);
            }

            player.is_moving = Self::is_moving(&movement.velocity);
            command.reset();
        }
    }

    fn is_moving(delta_move: &Vec3) -> bool {
        delta_move.ne(&Vec3::ZERO)
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(CloningPlugin);
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::move_player.system().after(InputSystem::Playing)),
        );
    }
}
