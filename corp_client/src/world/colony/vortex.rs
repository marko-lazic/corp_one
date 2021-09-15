use bevy::prelude::*;

use crate::constants::state::GameState;
use crate::Game;

pub struct VortexPlugin;

pub struct VortexGateEvent;

impl VortexPlugin {
    fn vortex_gate_event(
        mut ev_vortex_gate: EventReader<VortexGateEvent>,
        mut state: ResMut<State<crate::GameState>>,
        mut game: ResMut<Game>,
    ) {
        for _ in ev_vortex_gate.iter() {
            if !game.is_vorting {
                game.is_vorting = true;
                state.set(crate::GameState::StarMap).unwrap();
            }
        }
    }
}

impl Plugin for VortexPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<VortexGateEvent>();
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(VortexPlugin::vortex_gate_event.system()),
        );
    }
}
