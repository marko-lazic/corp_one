use bevy::prelude::*;

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Entity>,
}

impl Inventory {
    #[allow(dead_code)]
    pub fn new(items: Vec<Entity>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[Entity] {
        &self.items
    }

    #[allow(dead_code)]
    pub fn add(&mut self, item: Entity) {
        self.items.push(item);
    }

    pub fn add_all(&mut self, items: Vec<Entity>) {
        self.items.extend(items);
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}
