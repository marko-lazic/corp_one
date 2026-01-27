use crate::prelude::*;
use bevy::prelude::*;

use crate::prelude::{Faction, OwnershipRegistry, SecurityLevel, UseCommand};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(
    Name::new("Energy Node"),
    Structure,
    Use,
    SecurityLevel::Low,
    OwnershipRegistry = lookup_ownership()
)]
#[cfg_attr(feature = "client", require(
    DespawnOnExit<GameState> = DespawnOnExit(GameState::Playing))
)]
pub struct EnergyNode;

fn lookup_ownership() -> OwnershipRegistry {
    OwnershipRegistry::new_permanent(Faction::EC)
}

pub fn on_use_territory_node_event(use_command: On<UseCommand>) {
    info!("Interaction with territory node: {:?}", use_command.usable);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::prelude::{
        Faction, OwnershipRegistry, Player, PlayerFactionInfo, Rank, TestUtils, UseCommand,
    };

    use super::*;

    #[test]
    fn player_interacts_with_energy_node() {
        // given
        let mut app = setup();
        let e_energy_node = setup_territory_node(&mut app, Faction::EC, SecurityLevel::Low);
        let e_player = setup_player(&mut app, Faction::EC, Rank::R7);

        // when
        app.world_mut()
            .trigger(UseCommand::new(e_energy_node, e_player));
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
        app.world_mut().spawn((EnergyNode, ownership, level)).id()
    }

    fn setup_player(app: &mut App, faction: Faction, rank: Rank) -> Entity {
        app.world_mut()
            .spawn((Player, PlayerFactionInfo { faction, rank }))
            .id()
    }
}
