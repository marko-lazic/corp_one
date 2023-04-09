use bevy::prelude::*;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Faction {
    EC,
    CMG,
    VI,
}

#[derive(Component, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct MemberOf {
    pub faction: Faction,
}

impl MemberOf {
    pub fn new(faction_type: Faction) -> Self {
        Self {
            faction: faction_type,
        }
    }
}

#[derive(Component, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct OwnedBy {
    pub faction: Faction,
}

impl OwnedBy {
    pub fn new(faction: Faction) -> Self {
        Self { faction }
    }
}
