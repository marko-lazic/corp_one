use bevy::prelude::*;
use corp_shared::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_player_health);
    }
}

fn update_player_health(
    player_health_query: Query<(Entity, &Health), With<Player>>,
    mut commands: Commands,
) {
    for (player_entity, health) in &player_health_query {
        if health.is_dead() {
            info!("Player {} is dead", player_entity);
            commands.entity(player_entity).insert(Dead);
        }
    }
}
