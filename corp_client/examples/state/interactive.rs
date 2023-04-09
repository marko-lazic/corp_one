use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Interactor {
    pub interacted: bool,
    pub target_entity: Option<Entity>,
}

impl Interactor {
    pub fn interact(&mut self, target_entity: Entity) {
        self.interacted = true;
        self.target_entity = Some(target_entity);
    }
}

#[bevy_trait_query::queryable]
pub trait Interactive {
    fn interact(&mut self, entity: Entity);
}

// Define a system that iterates over entities with the Interactive component and calls their
// InteractiveHandler implementation when the entity is interacted with.
pub fn interaction_system(
    mut interactor_query: Query<(Entity, &mut Interactor)>,
    mut interactive_query: Query<&mut dyn Interactive>,
) {
    for (the_interactor, mut interactor_component) in &mut interactor_query {
        if interactor_component.interacted {
            if let Some(target_entity) = interactor_component.target_entity {
                if let Ok(mut interactives) = interactive_query.get_mut(target_entity) {
                    for mut interactive in &mut interactives {
                        interactive.interact(the_interactor);
                    }
                }
            }
            interactor_component.interacted = false;
        }
    }
}