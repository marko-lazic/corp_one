use bevy::prelude::*;

use corp_shared::prelude::{Health, Player};

use crate::asset::asset_loading::ColonyAssets;
use crate::constants::state::GameState;
use crate::world::colony::Colony;
use crate::Game;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct VortexSystemLabel;

pub struct VortexPlugin;

pub struct VortexEvent {
    colony: Colony,
}

impl VortexEvent {
    pub fn vort(colony: Colony) -> Self {
        VortexEvent { colony }
    }
}

#[derive(Component)]
pub struct VortexNode;

impl VortexPlugin {
    fn vortex_event(
        mut game: ResMut<Game>,
        mut game_state: ResMut<State<GameState>>,
        colony_assets: Res<ColonyAssets>,
        mut vortex_events: EventReader<VortexEvent>,
        healths: Query<&Health, With<Player>>,
    ) {
        for event in vortex_events.iter() {
            match event.colony {
                Colony::StarMap => {
                    if game_state.current() != &GameState::StarMap {
                        let health = healths.get(game.player_entity.unwrap()).unwrap();
                        game.health = health.clone();
                        let _result = game_state.set(GameState::StarMap);
                    }
                }
                Colony::Cloning => {
                    info!("Cloning Facility");
                    game.current_colony_asset = colony_assets.cloning.clone();
                    let _result = game_state.set(GameState::Playing);
                }
                Colony::Iris => {
                    info!("Moonbase: Station Iris");
                    game.current_colony_asset = colony_assets.iris.clone();
                    let _result = game_state.set(GameState::Playing);
                }
                Colony::Liberte => {
                    info!("Mars: Colony Liberte");
                    game.current_colony_asset = colony_assets.liberte.clone();
                    let _result = game_state.set(GameState::Playing);
                }
            }
        }
    }

    fn animate_nodes(mut nodes: Query<&mut Transform, With<VortexNode>>, time: Res<Time>) {
        for mut transform in nodes.iter_mut() {
            transform.rotate(Quat::from_rotation_y(time.delta_seconds() * 0.2));
        }
    }
}

impl Plugin for VortexPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VortexEvent>();
        app.add_system_set(
            SystemSet::on_update(GameState::StarMap).with_system(Self::vortex_event.system()),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(Self::vortex_event.system().label(VortexSystemLabel))
                .with_system(Self::animate_nodes.system()),
        );
    }
}
