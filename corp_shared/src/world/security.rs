use bevy::prelude::Component;
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::prelude::Rank;

#[derive(Component, Debug, Eq, PartialEq, Hash)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
}

lazy_static! {
    pub static ref REQUIRED_RANK_BY_SECURITY: HashMap<SecurityLevel, Rank> = {
        let mut map = HashMap::new();
        map.insert(SecurityLevel::Low, Rank::R4);
        map.insert(SecurityLevel::Medium, Rank::R5);
        map.insert(SecurityLevel::High, Rank::R6);
        map
    };
}
