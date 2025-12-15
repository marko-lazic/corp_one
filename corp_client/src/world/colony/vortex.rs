use crate::prelude::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use corp_shared::prelude::*;

pub struct VortexPlugin;

impl Plugin for VortexPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (animate_nodes, handle_vortex_collision)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn handle_vortex_collision(
    mut collision_events: MessageReader<CollisionStart>,
    q_vortex_gate: Query<(), With<VortexGate>>,
    q_player: Query<(), With<Player>>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        let (_vortex_entity, player_entity) = if q_vortex_gate.contains(event.collider1)
            && q_player.contains(event.collider2)
        {
            (event.collider1, event.collider2)
        } else if q_vortex_gate.contains(event.collider2) && q_player.contains(event.collider1) {
            (event.collider2, event.collider1)
        } else {
            continue;
        };

        info!("Vort {player_entity} to Star Map.");
        commands.trigger(RequestConnect(Colony::StarMap));
    }
}

fn animate_nodes(mut nodes: Query<&mut Transform, With<VortexNode>>, time: Res<Time<Fixed>>) {
    for mut transform in nodes.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_secs() * 0.2));
    }
}
