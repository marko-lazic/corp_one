use bevy::prelude::*;

use crate::prelude::{OwnershipRegistry, SecurityLevel, UseEvent};

#[derive(Component)]
pub struct TerritoryNode;

#[derive(Component)]
pub enum TerritoryNodeType {
    EnergyNode,
    PowerPlant,
    ControlCenter,
}

#[derive(Bundle)]
pub struct TerritoryNodeBundle {
    pub territory_node: TerritoryNode,
    pub node_type: TerritoryNodeType,
    pub security_level: SecurityLevel,
    pub ownership: OwnershipRegistry,
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
        let e_energy_node = setup_territory_node(
            &mut app,
            TerritoryNodeType::EnergyNode,
            Faction::EC,
            SecurityLevel::Low,
        );
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

    fn setup_territory_node(
        app: &mut App,
        node_type: TerritoryNodeType,
        faction: Faction,
        level: SecurityLevel,
    ) -> Entity {
        let ownership = OwnershipRegistry::new_permanent(faction);
        let door_entity = app
            .world_mut()
            .spawn((TerritoryNodeBundle {
                territory_node: TerritoryNode,
                node_type,
                security_level: level,
                ownership,
            },))
            .id();
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
