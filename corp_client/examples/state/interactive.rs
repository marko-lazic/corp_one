use bevy::prelude::*;

use crate::backpack::BackpackInteractionEvent;
use crate::door::DoorInteractionEvent;

pub enum InteractionType {
    Door,
    Backpack,
}

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
    fn interaction_type(&self) -> InteractionType;
}

pub fn interaction_system(
    mut interactor_query: Query<(Entity, &mut Interactor)>,
    interactive_query: Query<&mut dyn Interactive>,
    mut door_interaction_event_writer: EventWriter<DoorInteractionEvent>,
    mut backpack_interaction_event_writer: EventWriter<BackpackInteractionEvent>,
) {
    for (the_interactor, mut interactor_component) in &mut interactor_query {
        if interactor_component.interacted {
            if let Some(target_entity) = interactor_component.target_entity {
                if let Ok(interactives) = interactive_query.get(target_entity) {
                    for interactive in &interactives {
                        match interactive.interaction_type() {
                            InteractionType::Door => {
                                door_interaction_event_writer.send(DoorInteractionEvent {
                                    door_entity: target_entity,
                                    interactor_entity: the_interactor,
                                });
                            }
                            InteractionType::Backpack => {
                                backpack_interaction_event_writer.send(BackpackInteractionEvent {
                                    backpack_entity: target_entity,
                                    interactor_entity: the_interactor,
                                });
                            }
                        }
                    }
                }
            }
            interactor_component.interacted = false;
        }
    }
}
