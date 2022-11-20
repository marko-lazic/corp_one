use std::time::Duration;

use bevy::prelude::{Timer, TimerMode};

pub struct DoubleTap {
    taps: u32,
    counter: u32,
    timer: Timer,
}

impl DoubleTap {
    pub fn increment(&mut self) {
        self.counter += 1;
    }

    pub fn tick(&mut self, delta: Duration) -> &mut Self {
        self.timer.tick(delta);
        self
    }

    pub fn on_complete<F>(&mut self, f: F)
    where
        F: FnOnce(),
    {
        if self.timer.finished() {
            if self.fulfilled() {
                f();
            }
            self.reset();
        }
    }

    fn fulfilled(&self) -> bool {
        self.counter >= self.taps
    }

    fn reset(&mut self) {
        self.counter = 0;
    }
}

impl Default for DoubleTap {
    fn default() -> Self {
        Self {
            taps: 2,
            counter: 0,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}
