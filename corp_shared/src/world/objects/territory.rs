use bevy::prelude::*;

use crate::prelude::Security;

pub struct UseTerritoryNodeEvent;

pub enum TerritoryNodeType {
    EnergyNode,
    PowerPlant,
    ControlCenter,
}

#[derive(Component)]
pub struct TerritoryNode {
    pub r#type: TerritoryNodeType,
    pub security: Security,
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::prelude::{
        Faction, InteractionEvent, Inventory, MemberOf, OwnershipRegistry, Player, Rank, TestUtils,
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
            Security::Low,
        );
        let e_player = setup_player(&mut app, Vec::new(), Faction::EC, Rank::R7);

        // when
        app.world_mut().send_event(InteractionEvent::new(
            e_player,
            e_energy_node,
            UseTerritoryNodeEvent,
        ));
        app.update();

        // then
        // It should show colony faction ownership, military and economy presence per faction
    }

    fn setup() -> App {
        let mut app = App::new();
        app.init_time()
            .add_event::<InteractionEvent<UseTerritoryNodeEvent>>();
        app
    }

    fn setup_territory_node(
        app: &mut App,
        r#type: TerritoryNodeType,
        faction: Faction,
        security: Security,
    ) -> Entity {
        let mut registry = OwnershipRegistry::default();
        registry.add_permanent(faction);
        let door_entity = app
            .world_mut()
            .spawn((TerritoryNode { r#type, security }, registry))
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
