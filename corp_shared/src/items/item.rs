use bevy::prelude::*;

#[derive(Component)]
pub struct HackingTool;

#[derive(Bundle)]
pub struct HackingToolBundle {
    pub name: Name,
    pub hacking_tool: HackingTool,
}

impl Default for HackingToolBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Hacking Tool"),
            hacking_tool: HackingTool,
        }
    }
}
