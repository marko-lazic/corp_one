use std::time::Duration;

use bevy::app::Plugin;
use bevy::prelude::*;
use bevy_mod_picking::{HoverEvent, PickingEvent};

use crate::{App, GameState, SystemSet, Timer};

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct BarrierAccess {
    barrier_field_name: String,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct BarrierField {
    name: String,
    #[reflect(ignore)]
    close_cooldown: Timer,
    #[reflect(ignore)]
    open: bool,
}

impl Default for BarrierField {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            close_cooldown: Timer::new(Duration::from_secs(5), false),
            open: false,
        }
    }
}

pub struct BarrierPlugin;

impl BarrierPlugin {
    fn toggle_barrier() {}

    pub fn print_events(
        mut events: EventReader<PickingEvent>,
        barrier_access: Query<&BarrierAccess>,
    ) {
        for event in events.iter() {
            match event {
                PickingEvent::Hover(hover_event) => match hover_event {
                    HoverEvent::JustEntered(entity) => {
                        let access = barrier_access.get(*entity).unwrap();
                        info!("Just entered access {:?}", access);
                        // Todo BarrierAccess should have private field holding name of barrier it's responsible for.
                        // Todo Scenes should hold barrier field names and access pointers to those barriers.
                    }
                    HoverEvent::JustLeft(_) => {}
                },
                _ => {}
            }
        }
    }
}

impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BarrierField>();
        app.register_type::<BarrierAccess>();
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(Self::toggle_barrier),
        );
        app.add_system_to_stage(CoreStage::PostUpdate, Self::print_events);
    }
}
