use crate::server::GameServerConfig;
use bevy::prelude::*;
use corp_shared::prelude::*;

pub struct CloningRemotePlugin;

impl Plugin for CloningRemotePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(heal_player);
    }
}

fn is_cloning_server(config: Res<GameServerConfig>) -> bool {
    config.colony == Colony::Cloning
}

fn heal_player(
    add: On<Add, Player>,
    config: Res<GameServerConfig>,
    mut healths: Query<&mut Health>,
) {
    if is_cloning_server(config) {
        if let Ok(mut health) = healths.get_mut(add.entity)
            && health.is_dead()
        {
            health.heal(80.0);
        }
    } else {
        warn!("This is not cloning server.");
    }
}
