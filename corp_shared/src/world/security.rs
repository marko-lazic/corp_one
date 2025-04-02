use crate::prelude::Rank;
use bevy::prelude::Component;

#[derive(Component, Eq, PartialEq, Hash)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
}

impl SecurityLevel {
    pub fn has_required_rank(&self, rank: &Rank) -> bool {
        rank >= &self.required_rank()
    }
    pub fn required_rank(&self) -> Rank {
        match self {
            SecurityLevel::Low => Rank::R4,
            SecurityLevel::Medium => Rank::R5,
            SecurityLevel::High => Rank::R6,
        }
    }
}
