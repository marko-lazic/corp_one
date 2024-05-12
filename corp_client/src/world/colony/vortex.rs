use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, QueryFilter, RapierContext};

use corp_shared::prelude::*;

use crate::{
    asset::prelude::{Colony, ColonyConfigAssets},
    state::GameState,
    world::{ccc::PlayerEntity, colony::prelude::ColonyLoadEvent, physics, player::PlayerStore},
};

#[derive(Event)]
pub struct VortOutEvent;

#[derive(Event)]
pub struct VortInEvent {
    colony: Colony,
}

impl VortInEvent {
    pub fn vort(colony: Colony) -> Self {
        VortInEvent { colony }
    }
}

#[derive(Component, Default)]
pub struct VortexNode;

#[derive(Component, Default)]
pub struct VortexGate;

pub struct VortexPlugin;

impl Plugin for VortexPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VortInEvent>()
            .add_event::<VortOutEvent>()
            .add_systems(
                Update,
                (debug_vort_in, vort_in_event_reader)
                    .chain()
                    .run_if(in_state(GameState::StarMap)),
            )
            .add_systems(
                Update,
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
    mut r_player_store: ResMut<PlayerStore>,
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
    r_colony_config_assets: Res<ColonyConfigAssets>,
    mut r_next_state: ResMut<NextState<GameState>>,
    mut ev_vort_in: EventReader<VortInEvent>,
    mut ev_colony_load: EventWriter<ColonyLoadEvent>,
) {
    for vort_in in ev_vort_in.read() {
        info!("Vort in: {:?}", vort_in.colony);
        let colony_config_handle = match vort_in.colony {
            Colony::Cloning => r_colony_config_assets.cloning.clone(),
            Colony::Iris => r_colony_config_assets.iris.clone(),
            Colony::Liberte => r_colony_config_assets.liberte.clone(),
            Colony::Playground => Handle::default(),
        };
        ev_colony_load.send(ColonyLoadEvent(colony_config_handle));
        r_next_state.set(GameState::LoadColony);
    }
}

fn animate_nodes(mut nodes: Query<&mut Transform, With<VortexNode>>, time: Res<Time>) {
    for mut transform in nodes.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() * 0.2));
    }
}

fn vortex_gate_collider(
    q_vortex_gate_tr_co: Query<(&Transform, &Collider), With<VortexGate>>,
    rapier_context: Res<RapierContext>,
    mut ev_vort_out: EventWriter<VortOutEvent>,
) {
    let filter = QueryFilter::only_kinematic();

    for (transform, collider) in q_vortex_gate_tr_co.iter() {
        let shape_rot = transform.rotation;
        let shape_pos = transform.translation;
        rapier_context.intersections_with_shape(
            shape_pos,
            shape_rot,
            collider,
            filter,
            |_entity| {
                info!("Vortex gate collision: going to StarMap");
                ev_vort_out.send(VortOutEvent);
                false
            },
        );
    }
}
