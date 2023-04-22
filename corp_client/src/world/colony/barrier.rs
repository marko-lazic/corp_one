use std::time::Duration;

use bevy::app::Plugin;
use bevy::prelude::*;
use bevy_mod_picking::{HoverEvent, PickingEvent};

use crate::gui::CursorVisibility;
use crate::{App, Game, GameState, Timer, UseEntity};

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
        app.add_system(Self::open_close_barrier.in_set(OnUpdate(GameState::Playing)));
        app.add_system(Self::pick_barrier.in_base_set(CoreSet::PostUpdate));
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

    pub fn pick_barrier(
        mut events: EventReader<PickingEvent>,
        mut cursor_info: ResMut<CursorVisibility>,
        mut game: ResMut<Game>,
    ) {
        for event in events.iter() {
            if let PickingEvent::Hover(hover_event) = event {
                match hover_event {
                    HoverEvent::JustEntered(entity) => {
                        cursor_info.visible = true;
                        game.use_entity = UseEntity::Barrier(*entity);
                    }
                    HoverEvent::JustLeft(_) => {
                        cursor_info.visible = false;
                        game.use_entity = UseEntity::None;
                    }
                }
            }
        }
    }
}
