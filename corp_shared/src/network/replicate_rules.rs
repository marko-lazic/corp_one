use crate::prelude::*;
use bevy::prelude::*;
use bevy_replicon::prelude::AppRuleExt;

pub struct ReplicateRulesPlugin;

impl Plugin for ReplicateRulesPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<Backpack>()
            .replicate::<Transform>()
            .replicate::<Inventory>()
            .replicate::<HackingTool>();
    }
}
