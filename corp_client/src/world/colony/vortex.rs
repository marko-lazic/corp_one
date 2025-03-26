use crate::prelude::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use corp_shared::{prelude::*, world::colony::Colony};

#[derive(Event)]
pub struct VortOutEvent;

#[derive(Event, Clone)]
pub struct VortInEvent {
    colony: Colony,
}

impl VortInEvent {
    pub fn vort(colony: Colony) -> Self {
        VortInEvent { colony }
    }
}
pub struct VortexPlugin;

impl Plugin for VortexPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_scoped_event::<VortInEvent>(GameState::StarMap)
            .add_state_scoped_event::<VortOutEvent>(GameState::Playing)
            .add_systems(
                FixedUpdate,
                (debug_vort_in, vort_in_event_reader)
                    .chain()
                    .run_if(in_state(GameState::StarMap)),
            )
            .add_systems(
                FixedUpdate,
                (vort_out_event_reader, animate_nodes, vortex_gate_collider)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn debug_vort_in(mut ev_vort_in: EventWriter<VortInEvent>, mut run_once: Local<bool>) {
    if !(*run_once) {
        info!("Debug vort in");
        ev_vort_in.send(VortInEvent::vort(Colony::Iris));
        *run_once = true;
    }
}

fn vort_out_event_reader(
    mut r_player_store: ResMut<PlayerSystems>,
    r_player_entity: Res<PlayerEntity>,
    mut r_next_state: ResMut<NextState<GameState>>,
    mut ev_vort_out: EventReader<VortOutEvent>,
    q_health: Query<&Health, With<Player>>,
) {
    if ev_vort_out.read().last().is_some() {
        let Some(e_player) = r_player_entity.get() else {
            return;
        };

        let health = q_health.get(e_player).unwrap();
        r_player_store.health = health.clone();
        r_next_state.set(GameState::StarMap);
    }
}

fn vort_in_event_reader(
    mut r_next_state: ResMut<NextState<GameState>>,
    mut ev_vort_in: EventReader<VortInEvent>,
    mut ev_colony_load: EventWriter<ColonyLoadEvent>,
) {
    for vort_in in ev_vort_in.read() {
        let colony = vort_in.colony.clone();
        ev_colony_load.send(ColonyLoadEvent(colony));
        r_next_state.set(GameState::LoadColony);
    }
}

fn animate_nodes(mut nodes: Query<&mut Transform, With<VortexNode>>, time: Res<Time<Fixed>>) {
    for mut transform in nodes.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_secs() * 0.2));
    }
}

fn vortex_gate_collider(
    q_vortex_gate: Query<(&Transform, &Collider), With<VortexGate>>,
    q_spatial: SpatialQuery,
    mut ev_vort_out: EventWriter<VortOutEvent>,
) {
    for (transform, collider) in &q_vortex_gate {
        let shape_rot = transform.rotation;
        let shape_pos = transform.translation;
        q_spatial.shape_intersections_callback(
            collider,
            shape_pos,
            shape_rot,
            &SpatialQueryFilter::from_mask(GameLayer::Player),
            |entity| {
                info!("Vort {entity} to star map.");
                ev_vort_out.send(VortOutEvent);
                false
            },
        )
    }
}
