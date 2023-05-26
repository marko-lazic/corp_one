use std::time::Duration;

use bevy::app::Plugin;
use bevy::prelude::*;
use bevy_mod_picking::events::{Out, Over};
use bevy_mod_picking::prelude::ListenedEvent;

use crate::gui::CursorVisibility;
use crate::{App, Game, GameState, Timer, UseEntity};

#[derive(Debug, Eq, PartialEq)]
pub enum Hover {
    Over,
    Out,
}

pub struct BarrierPickingEvent(Entity, Hover);

impl From<ListenedEvent<Over>> for BarrierPickingEvent {
    fn from(event: ListenedEvent<Over>) -> Self {
        BarrierPickingEvent(event.target, Hover::Over)
    }
}

impl From<ListenedEvent<Out>> for BarrierPickingEvent {
    fn from(event: ListenedEvent<Out>) -> Self {
        BarrierPickingEvent(event.target, Hover::Out)
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct BarrierControl {
    pub barrier_field_name: String,
}

impl BarrierControl {
    pub fn new(name: &str) -> Self {
        Self {
            barrier_field_name: name.to_string(),
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct BarrierField {
    pub name: String,
    #[reflect(ignore)]
    close_cooldown: Timer,
    #[reflect(ignore)]
    pub open: bool,
}

impl BarrierField {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..default()
        }
    }
}

impl Default for BarrierField {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            close_cooldown: Timer::new(Duration::from_secs(5), TimerMode::Once),
            open: false,
        }
    }
}

pub struct BarrierPlugin;

impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BarrierField>();
        app.register_type::<BarrierControl>();
        app.add_event::<BarrierPickingEvent>();
        app.add_system(Self::receive_barrier_pickings.run_if(on_event::<BarrierPickingEvent>()));
        app.add_system(Self::open_close_barrier.in_set(OnUpdate(GameState::Playing)));
    }
}

impl BarrierPlugin {
    fn open_close_barrier(
        mut barrier_query: Query<(&mut BarrierField, &mut Visibility)>,
        time: Res<Time>,
    ) {
        for (mut barrier, mut visible) in barrier_query.iter_mut() {
            if barrier.open {
                *visible = Visibility::Hidden;
                barrier.close_cooldown.tick(time.delta());
                if barrier.close_cooldown.just_finished() {
                    barrier.open = false;
                    barrier.close_cooldown.reset();
                    *visible = Visibility::Visible;
                }
            }
        }
    }

    pub fn receive_barrier_pickings(
        mut pickings: EventReader<BarrierPickingEvent>,
        mut cursor_info: ResMut<CursorVisibility>,
        mut game: ResMut<Game>,
    ) {
        for event in pickings.iter() {
            if event.1 == Hover::Over {
                cursor_info.visible = true;
                game.use_entity = UseEntity::Barrier(event.0);
            } else if event.1 == Hover::Out {
                cursor_info.visible = false;
                game.use_entity = UseEntity::None;
            }
        }
    }
}
