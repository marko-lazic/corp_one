use bevy::prelude::*;
use corp_shared::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_player_health);
    }
}

fn update_player_health(
    changed_player_health: Query<(Entity, &Health), (With<Player>, Changed<Health>)>,
    mut commands: Commands,
) {
    for (player_e, health) in &changed_player_health {
        if health.is_dead() {
            info!("Player {} is dead", player_e);
            commands.entity(player_e).insert(Dead);
        }
    }
}
