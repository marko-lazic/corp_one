use bevy::prelude::*;

#[derive(Component)]
#[derive(Default)]
pub struct Inventory {
    pub items: Vec<Entity>,
}

impl Inventory {
    pub fn new(items: Vec<Entity>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[Entity] {
        &self.items
    }

    pub fn remove_item(&mut self, item: Entity) -> Option<Entity> {
        if let Some(index) = self.items.iter().position(|&i| i == item) {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, item: Entity) {
        self.items.push(item);
    }

    pub fn add_all(&mut self, items: Vec<Entity>) {
        self.items.extend(items);
    }
}


