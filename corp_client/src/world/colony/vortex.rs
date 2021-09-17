use bevy::prelude::*;

use corp_shared::prelude::{Health, Player};

use crate::constants::state::GameState;
use crate::Game;

pub struct VortexPlugin;

pub struct VortexGateEvent;

pub struct VortexNode;

impl VortexPlugin {
    fn vortex_gate_event(
        mut game: ResMut<Game>,
        mut state: ResMut<State<crate::GameState>>,
        mut vortex_gate_event: EventReader<VortexGateEvent>,
        healths: Query<&Health, With<Player>>,
    ) {
        for _ in vortex_gate_event.iter() {
            if !game.is_vorting {
                game.is_vorting = true;
                let health = healths.get(game.player_entity.unwrap()).unwrap();
                game.health = health.clone();
                let _result = state.set(crate::GameState::StarMap);
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
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<VortexGateEvent>();
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(Self::vortex_gate_event.system())
                .with_system(Self::animate_nodes.system()),
        );
    }
}
