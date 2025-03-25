use crate::prelude::*;
use bevy::prelude::*;

use crate::prelude::{Faction, OwnershipRegistry, SecurityLevel, UseEvent};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(
    Name(|| Name::new("Energy Node")),
    Structure,
    Use,
    SecurityLevel(|| SecurityLevel::Low),
    OwnershipRegistry(lookup_ownership)
)]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState>(|| StateScoped(GameState::Playing)))
)]
pub struct EnergyNode;

fn lookup_ownership() -> OwnershipRegistry {
    OwnershipRegistry::new_permanent(Faction::EC)
}

pub fn on_use_territory_node_event(trigger: Trigger<UseEvent>) {
    info!("Interaction with territory node: {:?}", trigger.entity());
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::prelude::{
        Faction, Inventory, MemberOf, OwnershipRegistry, Player, Rank, TestUtils, UseEvent,
    };

    use super::*;

    #[test]
    fn player_interacts_with_energy_node() {
        // given
        let mut app = setup();
        let e_energy_node = setup_territory_node(&mut app, Faction::EC, SecurityLevel::Low);
        let e_player = setup_player(&mut app, Vec::new(), Faction::EC, Rank::R7);

        // when
        app.world_mut()
            .trigger_targets(UseEvent::new(e_player), e_energy_node);
        app.update();

        // then
        // It should show colony faction ownership, military and economy presence per faction
    }

    fn setup() -> App {
        let mut app = App::new();
        app.init_time();
        app
    }

    fn setup_territory_node(app: &mut App, faction: Faction, level: SecurityLevel) -> Entity {
        let ownership = OwnershipRegistry::new_permanent(faction);
        let door_entity = app.world_mut().spawn(EnergyNode).id();
        door_entity
    }

    fn setup_player(app: &mut App, items: Vec<Entity>, faction: Faction, rank: Rank) -> Entity {
        let player_entity = app
            .world_mut()
            .spawn((Player, Inventory::new(items), MemberOf { faction, rank }))
            .id();
        player_entity
    }
}
