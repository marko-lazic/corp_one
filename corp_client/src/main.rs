mod frontend;

use crate::frontend::FrontendPlugin;
use bevy::prelude::*;
use corp_client::prelude::*;

fn main() {
    App::new()
        .add_plugins((FrontendPlugin, BackendPlugin))
        .run();
}
