use bevy::prelude::*;
use corp_client::prelude::{GuiPlugin, ShaderPlugin};

pub struct FrontendPlugin;

impl Plugin for FrontendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ShaderPlugin, GuiPlugin));
    }
}
