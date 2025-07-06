use crate::server::ColonyAppConfig;
use bevy::prelude::*;
use corp_shared::prelude::*;

pub struct CloningPlugin;

impl Plugin for CloningPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(heal_player);
    }
}

fn is_cloning(config: Res<ColonyAppConfig>) -> bool {
    config.colony == Colony::Cloning
}

fn heal_player(
    trigger: Trigger<OnAdd, Player>,
    config: Res<ColonyAppConfig>,
    mut healths: Query<&mut Health>,
) {
    if !is_cloning(config) {
        return;
    }
    let player_e = trigger.target();
    if let Ok(mut health) = healths.get_mut(player_e) {
        if health.is_dead() {
            health.heal(80.0);
        }
    }
}
