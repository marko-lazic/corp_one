use bevy::prelude::*;

#[derive(Component)]
pub struct Item {
    pub name: String,
}

impl Item {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Component)]
pub struct HackingTool;

#[derive(Bundle)]
pub struct HackingToolBundle {
    pub item: Item,
    pub hacking_tool: HackingTool,
}

impl Default for HackingToolBundle {
    fn default() -> Self {
        Self {
            item: Item::new("Hacking Tool".to_string()),
            hacking_tool: HackingTool,
        }
    }
}
