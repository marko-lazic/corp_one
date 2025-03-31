mod client_frontend;

use crate::client_frontend::ClientFrontendPlugin;
use bevy::prelude::*;
use corp_client::prelude::*;

fn main() {
    App::new()
        .add_plugins((ClientFrontendPlugin, ClientBackendPlugin))
        .run();
}
