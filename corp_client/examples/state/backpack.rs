use bevy::prelude::*;

use crate::gui::UiBackpack;
use crate::interactive::{BackpackAction, InteractionType, Interactive, Interactor};
use crate::inventory::Inventory;
use crate::item::Item;

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
    pub action: BackpackAction,
    pub backpack_entity: Entity,
    pub interactor_entity: Entity,
}

pub fn backpack_interaction_event_system(
    mut event_reader: EventReader<BackpackInteractionEvent>,
    mut inventory_query: Query<(&mut Inventory, &mut UiBackpack), With<Interactor>>,
    mut backpack_query: Query<&mut Backpack>,
    item_query: Query<&Item>,
) {
    for event in &mut event_reader {
        if let Ok((mut inventory, mut ui_backpack)) =
            inventory_query.get_mut(event.interactor_entity)
        {
            if let Ok(mut backpack) = backpack_query.get_mut(event.backpack_entity) {
                match event.action {
                    BackpackAction::List => {
                        let mut items: Vec<String> = Vec::new();
                        for backpack_item in backpack.items() {
                            items.push(item_query.get(*backpack_item).unwrap().name.clone());
                        }
                        ui_backpack.set_items(items);
                    }
                    BackpackAction::TakeAll => {
                        inventory.add_all(backpack.take_all());
                    }
                    BackpackAction::TakeItem(item_entity) => {
                        if let Some(item) = backpack.items.iter().position(|&i| i == item_entity) {
                            inventory.add(backpack.items.remove(item));
                        }
                    }
                }
            }
        }
    }
}

pub fn despawn_backpack_system(
    mut commands: Commands,
    mut backpack_entities: Query<(Entity, &Backpack), Changed<Backpack>>,
) {
    for (entity, backpack) in &mut backpack_entities {
        if backpack.items().is_empty() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy_trait_query::RegisterExt;

    use crate::door::DoorInteractionEvent;
    use crate::gui::UiBundle;
    use crate::interactive::{interaction_system, InteractionEvent};
    use crate::inventory::Inventory;
    use crate::item::HackingToolBundle;
    use crate::player::Player;
    use crate::test_utils::TestUtils;

    use super::*;

    #[test]
    fn one_item_is_in_backpack() {
        // given
        let (mut app, _) = setup();
        let item_entity = app.world.spawn(HackingToolBundle::default()).id();
        let backpack_entity = app.world.spawn(Backpack::new(vec![item_entity])).id();

        // when
        app.update();

        // then
        assert_eq!(app.get::<Backpack>(backpack_entity).items().len(), 1);
    }

    #[test]
    fn player_list_items_in_backpack() {
        // given
        let (mut app, player_entity) = setup();
        let item_entity_1 = app.world.spawn(HackingToolBundle::default()).id();
        let item_entity_2 = app.world.spawn(HackingToolBundle::default()).id();
        let backpack_entity = app
            .world
            .spawn(Backpack::new(vec![item_entity_1, item_entity_2]))
            .id();
        let mut interactor = app.get_mut::<Interactor>(player_entity);

        // when
        interactor.interact_with(
            backpack_entity,
            InteractionEvent::Backpack {
                action: BackpackAction::List,
            },
        );
        app.update();

        // then
        assert_eq!(
            app.get::<UiBackpack>(player_entity).items(),
            vec!["Hacking Tool", "Hacking Tool"]
        );
        assert_eq!(app.get::<Inventory>(player_entity).items().len(), 0);
        assert_eq!(app.get::<Backpack>(backpack_entity).items().len(), 2);
    }

    #[test]
    fn player_take_all_items_from_backpack() {
        // given
        let (mut app, player_entity) = setup();
        let item_entity_1 = app.world.spawn(HackingToolBundle::default()).id();
        let item_entity_2 = app.world.spawn(HackingToolBundle::default()).id();
        let backpack_entity = app
            .world
            .spawn(Backpack::new(vec![item_entity_1, item_entity_2]))
            .id();
        let mut interactor = app.get_mut::<Interactor>(player_entity);

        // when
        interactor.interact_with(
            backpack_entity,
            InteractionEvent::Backpack {
                action: BackpackAction::TakeAll,
            },
        );
        app.update();

        // then
        assert!(!app.has_component::<Backpack>(backpack_entity));
        assert_eq!(app.get::<Inventory>(player_entity).items().len(), 2);
    }

    #[test]
    fn player_take_one_item_from_backpack() {
        // given
        let (mut app, player_entity) = setup();
        let item_entity_1 = app.world.spawn(HackingToolBundle::default()).id();
        let item_entity_2 = app.world.spawn(HackingToolBundle::default()).id();
        let backpack_entity = app
            .world
            .spawn(Backpack::new(vec![item_entity_1, item_entity_2]))
            .id();
        let mut interactor = app.get_mut::<Interactor>(player_entity);

        // when
        interactor.interact_with(
            backpack_entity,
            InteractionEvent::Backpack {
                action: BackpackAction::TakeItem(item_entity_2),
            },
        );
        app.update();

        // then
        assert_eq!(app.get::<Backpack>(backpack_entity).items().len(), 1);
        assert_eq!(app.get::<Inventory>(player_entity).items().len(), 1);
        assert_eq!(
            app.get::<Backpack>(backpack_entity).items()[0],
            item_entity_1,
        );
        assert_eq!(
            app.get::<Inventory>(player_entity).items()[0],
            item_entity_2
        );
    }

    fn setup() -> (App, Entity) {
        let mut app = App::new();
        app.init_time();
        app.add_event::<BackpackInteractionEvent>();
        app.add_event::<DoorInteractionEvent>();
        app.register_component_as::<dyn Interactive, Backpack>();
        app.add_systems(
            (
                interaction_system,
                backpack_interaction_event_system,
                despawn_backpack_system,
            )
                .chain(),
        );
        let player_entity = app
            .world
            .spawn((
                Player,
                Interactor::default(),
                Inventory::default(),
                UiBundle::default(),
            ))
            .id();
        (app, player_entity)
    }
}
