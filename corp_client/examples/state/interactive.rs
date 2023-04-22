use bevy::prelude::*;

use crate::backpack::BackpackInteractionEvent;
use crate::door::DoorInteractionEvent;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BackpackAction {
    List,
    TakeAll,
    TakeItem(Entity),
}

#[derive(Default, PartialEq, Copy, Clone, Debug)]
pub enum InteractionEvent {
    #[default]
    Use,
    Backpack {
        action: BackpackAction,
    },
}

#[derive(Debug)]
pub enum InteractionType {
    Door,
    Backpack,
}

#[derive(Component, Default)]
pub struct Interactor {
    interacted: bool,
    pub target_entity: Option<Entity>,
    pub action: InteractionEvent,
}

impl Interactor {
    pub fn interact(&mut self, target_entity: Entity) {
        self.interacted = true;
        self.target_entity = Some(target_entity);
        self.action = InteractionEvent::Use;
    }

    pub fn interact_with(&mut self, target_entity: Entity, action: InteractionEvent) {
        self.interacted = true;
        self.target_entity = Some(target_entity);
        self.action = action;
    }

    pub fn just_interacted(&mut self) -> bool {
        if self.interacted {
            self.interacted = false;
            true
        } else {
            false
        }
    }

    pub fn event(&self) -> InteractionEvent {
        self.action
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
    for (interactor_entity, mut interactor) in &mut interactor_query {
        if interactor.just_interacted() {
            if let Some(target_entity) = interactor.target_entity {
                if let Ok(interactives) = interactive_query.get(target_entity) {
                    for interactive in &interactives {
                        match (interactive.interaction_type(), interactor.event()) {
                            (InteractionType::Door, InteractionEvent::Use) => {
                                door_interaction_event_writer.send(DoorInteractionEvent {
                                    door_entity: target_entity,
                                    interactor_entity,
                                });
                            }
                            (InteractionType::Backpack, InteractionEvent::Backpack { action }) => {
                                backpack_interaction_event_writer.send(BackpackInteractionEvent {
                                    action,
                                    backpack_entity: target_entity,
                                    interactor_entity,
                                });
                            }
                            (interaction_type, interaction_event) => {
                                error!(
                                    "Unhandled interaction: {:?} {:?}",
                                    interaction_type, interaction_event
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
