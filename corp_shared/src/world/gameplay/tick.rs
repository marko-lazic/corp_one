use bevy::prelude::*;

#[derive(Component, Clone, Deref)]
pub struct TickRate(Timer);

impl TickRate {
    pub fn per_seconds(seconds: f32) -> Self {
        TickRate(Timer::from_seconds(seconds, TimerMode::Repeating))
    }
}
