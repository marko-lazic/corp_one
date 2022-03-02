use std::time::Duration;

use bevy::app::Plugin;
use bevy::prelude::*;

use crate::{App, GameState, SystemSet, Timer};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BarrierAccess {
    #[reflect(ignore)]
    close_cooldown: Timer,
    #[reflect(ignore)]
    open: bool,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BarrierField;

pub struct BarrierPlugin;

impl Default for BarrierAccess {
    fn default() -> Self {
        Self {
            close_cooldown: Timer::new(Duration::from_secs(5), false),
            open: false,
        }
    }
}

impl BarrierPlugin {
    fn toggle_barrier() {}
}

impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BarrierField>();
        app.register_type::<BarrierAccess>();
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(Self::toggle_barrier),
        );
    }
}
