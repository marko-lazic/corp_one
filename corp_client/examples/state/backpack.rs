use bevy::prelude::*;

use crate::interactive::{InteractionType, Interactive, Interactor};
use crate::inventory::Inventory;

#[derive(Component)]
pub struct Backpack {
    pub items: Vec<Entity>,
}

impl Backpack {
    pub fn new(items: Vec<Entity>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[Entity] {
        &self.items
    }

    pub fn take_all(&mut self) -> Vec<Entity> {
        std::mem::take(&mut self.items)
    }
}

impl Interactive for Backpack {
    fn interaction_type(&self) -> InteractionType {
        InteractionType::Backpack
    }
}

pub struct BackpackInteractionEvent {
    pub backpack_entity: Entity,
    pub interactor_entity: Entity,
}

pub fn backpack_interaction_event_system(
    mut commands: Commands,
    mut event_reader: EventReader<BackpackInteractionEvent>,
    mut inventory_query: Query<&mut Inventory, With<Interactor>>,
    mut backpack_query: Query<&mut Backpack>,
) {
    for event in &mut event_reader {
        if let Ok(mut inventory) = inventory_query.get_mut(event.interactor_entity) {
            if let Ok(mut backpack) = backpack_query.get_mut(event.backpack_entity) {
                inventory.add_all(backpack.take_all());
                commands.entity(event.backpack_entity).despawn_recursive();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy_trait_query::RegisterExt;

    use crate::door::DoorInteractionEvent;
    use crate::interactive::interaction_system;
    use crate::inventory::Inventory;
    use crate::item::{HackingTool, HackingToolBundle};
    use crate::player::Player;
    use crate::test_utils::TestUtils;

    use super::*;

    #[test]
    fn one_item_is_in_backpack() {
        // given
        let (mut app, backpack_entity, _, _) = setup();

        // when
        app.update();

        // then
        assert_eq!(app.get::<Backpack>(backpack_entity).items().len(), 1);
    }

    #[test]
    fn player_take_all_items_from_backpack() {
        // given
        let (mut app, backpack_entity, player_entity, item_entity) = setup();
        let mut interactor = app.get_mut::<Interactor>(player_entity);

        // when
        interactor.interact(backpack_entity);
        app.update();

        // then
        assert!(!app.has_component::<Backpack>(backpack_entity));
        assert!(app.has_component::<HackingTool>(item_entity));
        assert_eq!(app.get::<Inventory>(player_entity).items().len(), 1);
    }

    fn setup() -> (App, Entity, Entity, Entity) {
        let mut app = App::new();
        app.init_time();
        app.add_event::<BackpackInteractionEvent>();
        app.add_event::<DoorInteractionEvent>();
        app.register_component_as::<dyn Interactive, Backpack>();
        app.add_systems((interaction_system, backpack_interaction_event_system).chain());
        let item_entity = app.world.spawn(HackingToolBundle::default()).id();
        let backpack_entity = app.world.spawn(Backpack::new(vec![item_entity])).id();
        let player_entity = app
            .world
            .spawn((Player, Interactor::default(), Inventory::default()))
            .id();
        (app, backpack_entity, player_entity, item_entity)
    }
}
