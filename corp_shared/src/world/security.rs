use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::prelude::Rank;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Security {
    Low,
    Medium,
    High,
}

lazy_static! {
    pub static ref REQUIRED_RANK_BY_SECURITY: HashMap<Security, Rank> = {
        let mut map = HashMap::new();
        map.insert(Security::Low, Rank::R4);
        map.insert(Security::Medium, Rank::R5);
        map.insert(Security::High, Rank::R6);
        map
    };
}
