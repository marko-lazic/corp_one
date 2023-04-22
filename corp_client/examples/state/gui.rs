use bevy::prelude::*;

#[derive(Component, Default)]
pub struct UiBackpack {
    pub items: Vec<String>,
}

impl UiBackpack {
    pub fn items(&self) -> &[String] {
        &self.items
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
    }
}

#[derive(Bundle, Default)]
pub struct UiBundle {
    pub ui_backpack: UiBackpack,
}
