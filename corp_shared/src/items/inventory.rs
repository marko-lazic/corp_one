use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Inventory {
    pub items: Vec<Entity>,
}

impl Inventory {
    pub fn new(items: Vec<Entity>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> impl Iterator<Item = &Entity> {
        self.items.iter()
    }
    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.items.iter_mut()
    }

    pub fn add(&mut self, item: Entity) -> &mut Self {
        self.items.push(item);
        self
    }

    pub fn add_all(&mut self, items: Vec<Entity>) -> &mut Self {
        self.items.extend(items);
        self
    }

    pub fn remove(&mut self, item: Entity) -> Option<Entity> {
        self.items
            .iter()
            .position(|&i| i == item)
            .map(|index| self.items.remove(index))
    }

    pub fn remove_all(&mut self) -> Vec<Entity> {
        std::mem::take(&mut self.items)
    }
}
