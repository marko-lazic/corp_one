use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, QueryFilter, RapierContext};
use iyes_loopless::condition::ConditionSet;
use iyes_loopless::prelude::NextState;

use corp_shared::prelude::Health;

use crate::asset::asset_loading::ColonyAssets;
use crate::constants::state::GameState;
use crate::Game;
use crate::world::colony::Colony;
use crate::world::physics;
use crate::world::player::Player;

pub struct VortOutEvent;

pub struct VortInEvent {
    colony: Colony,
}

impl VortInEvent {
    pub fn vort(colony: Colony) -> Self {
        VortInEvent { colony }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct VortexNode;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct VortexGate;

pub struct VortexPlugin;

impl Plugin for VortexPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VortInEvent>();
        app.add_event::<VortOutEvent>();
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::StarMap)
                .with_system(Self::vort_in_event_reader)
                .into(),
        );
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(Self::vort_out_event_reader)
                .with_system(Self::animate_nodes)
                .with_system(Self::vortex_gate_collider)
                .into(),
        );
    }
}

impl VortexPlugin {
    fn vort_out_event_reader(
        healths: Query<&Health, With<Player>>,
        mut game: ResMut<Game>,
        mut commands: Commands,
        mut vort_out_events: EventReader<VortOutEvent>,
    ) {
        if let Some(_) = vort_out_events.iter().last() {
            let health = healths.get(game.player_entity.unwrap()).unwrap();
            game.health = health.clone();
            commands.insert_resource(NextState(GameState::StarMap));
        }
    }

    fn vort_in_event_reader(
        colony_assets: Res<ColonyAssets>,
        mut vort_in_events: EventReader<VortInEvent>,
        mut game: ResMut<Game>,
        mut commands: Commands,
    ) {
        for vort_in in vort_in_events.iter() {
            match vort_in.colony {
                Colony::Cloning => {
                    info!("Cloning Facility");
                    game.current_colony_asset = colony_assets.cloning.clone();
                    commands.insert_resource(NextState(GameState::LoadColony));
                }
                Colony::Iris => {
                    info!("Moonbase: Station Iris");
                    game.current_colony_asset = colony_assets.iris.clone();
                    commands.insert_resource(NextState(GameState::LoadColony));
                }
                Colony::Liberte => {
                    info!("Mars: Colony Liberte");
                    game.current_colony_asset = colony_assets.liberte.clone();
                    commands.insert_resource(NextState(GameState::LoadColony));
                }
                Colony::Playground => {
                    info!("Alien Planet");
                    game.current_colony_asset = Handle::default();
                    commands.insert_resource(NextState(GameState::LoadColony));
                }
            }
        }
    }

    fn animate_nodes(mut nodes: Query<&mut Transform, With<VortexNode>>, time: Res<Time>) {
        for mut transform in nodes.iter_mut() {
            transform.rotate(Quat::from_rotation_y(time.delta_seconds() * 0.2));
        }
    }

    fn vortex_gate_collider(
        zones: Query<(&Transform, &Collider), With<VortexGate>>,
        rapier_context: Res<RapierContext>,
        mut vort_out_events: EventWriter<VortOutEvent>,
    ) {
        let filter =
            QueryFilter::only_dynamic().groups(physics::CollideGroups::vortex_gate().into());

        for (transform, collider) in zones.iter() {
            let shape_rot = transform.rotation;
            let shape_pos = transform.translation;
            rapier_context.intersections_with_shape(
                shape_pos,
                shape_rot,
                collider,
                filter,
                |_entity| {
                    vort_out_events.send(VortOutEvent);
                    false
                },
            );
        }
    }
}
