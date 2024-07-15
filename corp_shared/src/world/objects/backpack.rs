use bevy::prelude::*;

use crate::prelude::*;

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

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BackpackAction {
    List,
    TakeAll,
    TakeItem(Entity),
}

#[derive(Event)]
pub struct BackpackInteractionEvent {
    pub action: BackpackAction,
    pub interactor_entity: Entity,
    pub backpack_entity: Entity,
}

pub fn backpack_interaction_event_system(
    mut ev_backpack_interaction_event: EventReader<BackpackInteractionEvent>,
    mut q_inventory: Query<&mut Inventory, With<Player>>,
    mut q_backpack: Query<&mut Backpack>,
    q_item: Query<&Item>,
) {
    for event in &mut ev_backpack_interaction_event.read() {
        if let Ok(mut inventory) = q_inventory.get_mut(event.interactor_entity) {
            if let Ok(mut backpack) = q_backpack.get_mut(event.backpack_entity) {
                match event.action {
                    BackpackAction::List => {
                        let mut items: Vec<String> = Vec::new();
                        for backpack_item in backpack.items() {
                            items.push(q_item.get(*backpack_item).unwrap().name.clone());
                        }
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
    mut q_entity_backpack: Query<(Entity, &Backpack), Changed<Backpack>>,
) {
    for (entity, backpack) in &mut q_entity_backpack {
        if backpack.items().is_empty() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_item_is_in_backpack() {
        // given
        let (mut app, _) = setup();
        let item_entity = app.world().spawn(HackingToolBundle::default()).id();
        let backpack_entity = app.world().spawn(Backpack::new(vec![item_entity])).id();

        // when
        app.update();

        // then
        assert_eq!(app.get::<Backpack>(backpack_entity).items().len(), 1);
    }

    #[test]
    fn player_list_items_in_backpack() {
        // given
        let (mut app, player_entity) = setup();
        let item_entity_1 = app.world().spawn(HackingToolBundle::default()).id();
        let item_entity_2 = app.world().spawn(HackingToolBundle::default()).id();
        let backpack_entity = app
            .world
            .spawn(Backpack::new(vec![item_entity_1, item_entity_2]))
            .id();

        // when
        app.world().send_event(BackpackInteractionEvent {
            action: BackpackAction::List,
            interactor_entity: player_entity,
            backpack_entity,
        });

        app.update();

        // then
        assert_eq!(app.get::<Inventory>(player_entity).items().len(), 0);
        assert_eq!(app.get::<Backpack>(backpack_entity).items().len(), 2);
    }

    #[test]
    fn player_take_all_items_from_backpack() {
        // given
        let (mut app, player_entity) = setup();
        let item_entity_1 = app.world().spawn(HackingToolBundle::default()).id();
        let item_entity_2 = app.world().spawn(HackingToolBundle::default()).id();
        let backpack_entity = app
            .world
            .spawn(Backpack::new(vec![item_entity_1, item_entity_2]))
            .id();

        // when
        app.world().send_event(BackpackInteractionEvent {
            action: BackpackAction::TakeAll,
            interactor_entity: player_entity,
            backpack_entity,
        });
        app.update();

        // then
        assert!(!app.has_component::<Backpack>(backpack_entity));
        assert_eq!(app.get::<Inventory>(player_entity).items().len(), 2);
    }

    #[test]
    fn player_take_one_item_from_backpack() {
        // given
        let (mut app, player_entity) = setup();
        let item_entity_1 = app.world().spawn(HackingToolBundle::default()).id();
        let item_entity_2 = app.world().spawn(HackingToolBundle::default()).id();
        let backpack_entity = app
            .world
            .spawn(Backpack::new(vec![item_entity_1, item_entity_2]))
            .id();

        // when
        app.world().send_event(BackpackInteractionEvent {
            action: BackpackAction::TakeItem(item_entity_2),
            interactor_entity: player_entity,
            backpack_entity,
        });
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
        app.init_time()
            .add_event::<BackpackInteractionEvent>()
            .add_systems(
                Update,
                (backpack_interaction_event_system, despawn_backpack_system).chain(),
            );
        let player_entity = app.world().spawn((Player, Inventory::default())).id();
        (app, player_entity)
    }
}
