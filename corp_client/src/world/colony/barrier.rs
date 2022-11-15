use std::time::Duration;

use bevy::app::Plugin;
use bevy::prelude::*;
use bevy_mod_picking::{HoverEvent, PickingEvent};
use iyes_loopless::prelude::ConditionSet;

use crate::gui::CursorInfo;
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
            close_cooldown: Timer::new(Duration::from_secs(5), false),
            open: false,
        }
    }
}

pub struct BarrierPlugin;

impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BarrierField>();
        app.register_type::<BarrierControl>();
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(Self::open_close_barrier)
                .into(),
        );
        app.add_system_to_stage(CoreStage::PostUpdate, Self::pick_barrier);
    }
}

impl BarrierPlugin {
    fn open_close_barrier(
        mut barrier_query: Query<(&mut BarrierField, &mut Visibility)>,
        time: Res<Time>,
    ) {
        for (mut barrier, mut visible) in barrier_query.iter_mut() {
            if barrier.open {
                visible.is_visible = false;
                barrier.close_cooldown.tick(time.delta());
                if barrier.close_cooldown.just_finished() {
                    barrier.open = false;
                    barrier.close_cooldown.reset();
                    visible.is_visible = true;
                }
            }
        }
    }

    pub fn pick_barrier(
        mut events: EventReader<PickingEvent>,
        mut cursor_info: ResMut<CursorInfo>,
        mut game: ResMut<Game>,
    ) {
        for event in events.iter() {
            match event {
                PickingEvent::Hover(hover_event) => match hover_event {
                    HoverEvent::JustEntered(entity) => {
                        cursor_info.show_use = true;
                        game.use_entity = UseEntity::Barrier(entity.clone());
                    }
                    HoverEvent::JustLeft(_) => {
                        cursor_info.show_use = false;
                        game.use_entity = UseEntity::None;
                    }
                },
                _ => {}
            }
        }
    }
}
