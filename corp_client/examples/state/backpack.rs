use bevy::prelude::*;

use crate::interactive::Interactor;
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

// On interaction we should give player a sparse set component
// Which is a control over the backpack
// A player can chose to take one item or to take all items
// either system should do the action and remove sparse set component
pub fn take_all_system(
    mut commands: Commands,
    mut interactor_query: Query<(&mut Interactor, &mut Inventory)>,
    mut query: Query<&mut Backpack>,
) {
    for (mut interactor_component, mut inventory) in &mut interactor_query {
        if interactor_component.interacted {
            if let Some(target_entity) = interactor_component.target_entity {
                if let Ok(mut backpack) = query.get_mut(target_entity) {
                    inventory.add_all(backpack.take_all());
                    commands.entity(target_entity).despawn_recursive();
                }
            }
            interactor_component.interacted = false;
        }
    }
}

#[cfg(test)]
mod tests {
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
        app.add_system(take_all_system);
        let item_entity = app.world.spawn(HackingToolBundle::default()).id();
        let backpack_entity = app.world.spawn(Backpack::new(vec![item_entity])).id();
        let player_entity = app
            .world
            .spawn((Player, Interactor::default(), Inventory::default()))
            .id();
        (app, backpack_entity, player_entity, item_entity)
    }
}
