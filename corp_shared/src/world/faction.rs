use bevy::prelude::*;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Faction {
    EC,
    CMG,
    VI,
}

#[derive(Default, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    #[default]
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

#[derive(Component, Eq, PartialEq)]
pub struct MemberOf {
    pub faction: Faction,
    pub rank: Rank,
}

pub enum ControlType {
    Permanent(Faction),
    Hacked(Faction, Timer),
}

impl ControlType {
    pub fn faction(&self) -> &Faction {
        match self {
            Self::Permanent(faction) => faction,
            Self::Hacked(faction, _) => faction,
        }
    }
}

#[derive(Component, Default)]
pub struct ControlRegistry {
    pub factions: Vec<ControlType>,
}

impl ControlRegistry {
    pub fn new_permanent(faction: Faction) -> Self {
        Self {
            factions: vec![ControlType::Permanent(faction)],
        }
    }
    pub fn add_permanent(&mut self, faction: Faction) {
        self.factions.push(ControlType::Permanent(faction));
    }

    pub fn add_temporary(&mut self, faction: Faction, timer: Timer) {
        self.factions.push(ControlType::Hacked(faction, timer));
    }

    pub fn get_control_type(&self, faction: &Faction) -> Option<&ControlType> {
        self.factions.iter().find(|f| f.faction() == faction)
    }

    pub fn process_temporary_factions(&mut self, time: &Time) {
        // Update temporary factions
        for faction in self.factions.iter_mut() {
            if let ControlType::Hacked(_, timer) = faction {
                timer.tick(time.delta());
            }
        }

        // Remove finished temporary factions
        self.factions.retain(|faction| {
            if let ControlType::Hacked(_, timer) = faction {
                !timer.finished()
            } else {
                true
            }
        });
    }
}

pub fn process_temporary_faction_ownership_timers_system(
    mut query: Query<&mut ControlRegistry>,
    time: Res<Time>,
) {
    for mut faction_ownership_registry in &mut query {
        faction_ownership_registry.process_temporary_factions(&time);
    }
}
