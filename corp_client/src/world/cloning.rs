use crate::prelude::*;
use bevy::prelude::*;
use corp_shared::{prelude::*, world::colony::Colony};

pub struct CloningPlugin;

impl Plugin for CloningPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_you_died);
    }
}

fn on_you_died(_trigger: Trigger<YouDied>, mut commands: Commands) {
    commands.trigger(RequestConnect(Colony::Cloning));
}
