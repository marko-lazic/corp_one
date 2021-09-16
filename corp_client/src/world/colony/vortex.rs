use bevy::prelude::*;

use crate::constants::state::GameState;
use crate::Game;

pub struct VortexPlugin;

pub struct VortexGateEvent;

pub struct VortexNode;

impl VortexPlugin {
    fn vortex_gate_event(
        mut vortex_gate_event: EventReader<VortexGateEvent>,
        mut state: ResMut<State<crate::GameState>>,
        mut game: ResMut<Game>,
    ) {
        for _ in vortex_gate_event.iter() {
            if !game.is_vorting {
                game.is_vorting = true;
                state.set(crate::GameState::StarMap).unwrap();
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
