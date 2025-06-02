mod client_frontend;
use crate::client_frontend::ClientFrontendPlugin;
use bevy::{
    ecs::error::{warn, GLOBAL_ERROR_HANDLER},
    prelude::*,
};
use corp_client::prelude::*;

fn main() {
    GLOBAL_ERROR_HANDLER
        .set(warn)
        .expect("The error handler can only be set once, globally.");
    App::new()
        .add_plugins((ClientFrontendPlugin, ClientBackendPlugin))
        .run();
}
