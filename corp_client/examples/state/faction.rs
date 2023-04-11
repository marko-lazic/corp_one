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

pub enum FactionControl {
    Permanent(Faction),
    Temporary(Faction, Timer),
}

impl FactionControl {
    pub fn faction(&self) -> Faction {
        match self {
            Self::Permanent(faction) => *faction,
            Self::Temporary(faction, _) => *faction,
        }
    }
}

#[derive(Component, Default)]
pub struct FactionOwnershipRegistry {
    pub factions: Vec<FactionControl>,
}

impl FactionOwnershipRegistry {
    pub fn new_permanent(faction: Faction) -> Self {
        Self {
            factions: vec![FactionControl::Permanent(faction)],
        }
    }

    pub fn add_temporary(&mut self, faction: Faction, timer: Timer) {
        self.factions
            .push(FactionControl::Temporary(faction, timer));
    }

    pub fn is_member(&self, faction: Faction) -> bool {
        self.factions.iter().any(|f| f.faction() == faction)
    }

    pub fn process_temporary_factions(&mut self, time: &Time) {
        // Update temporary factions
        for faction in self.factions.iter_mut() {
            if let FactionControl::Temporary(_, timer) = faction {
                timer.tick(time.delta());
            }
        }

        // Remove finished temporary factions
        self.factions.retain(|faction| {
            if let FactionControl::Temporary(_, timer) = faction {
                !timer.finished()
            } else {
                true
            }
        });
    }
}

pub fn process_temporary_faction_ownership_timers_system(
    mut query: Query<&mut FactionOwnershipRegistry>,
    time: Res<Time>,
) {
    for mut faction_ownership_registry in &mut query {
        faction_ownership_registry.process_temporary_factions(&time);
    }
}
