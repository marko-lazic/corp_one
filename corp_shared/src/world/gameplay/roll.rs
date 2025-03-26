use bevy::prelude::*;
use std::ops::RangeInclusive;

#[derive(Component, Clone, Deref)]
pub struct Range(pub RangeInclusive<u32>);

impl Range {
    pub fn new(min: u32, max: u32) -> Self {
        Self(min..=max)
    }
}
