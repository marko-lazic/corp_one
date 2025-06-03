use crate::prelude::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use corp_shared::{prelude::*, world::colony::Colony};

pub struct VortexPlugin;

impl Plugin for VortexPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (animate_nodes, leave_colony)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn leave_colony(
    q_vortex_gate: Query<(&Transform, &Collider), With<VortexGate>>,
    q_spatial: SpatialQuery,
    mut r_player_store: ResMut<PlayerSystems>,
    player_health: Single<&Health, With<Player>>,
    mut commands: Commands,
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
                info!("Vort {entity} to Star Map.");
                r_player_store.health = player_health.clone();
                commands.trigger(RequestConnect(Colony::StarMap));
                false
            },
        )
    }
}

fn animate_nodes(mut nodes: Query<&mut Transform, With<VortexNode>>, time: Res<Time<Fixed>>) {
    for mut transform in nodes.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_secs() * 0.2));
    }
}
