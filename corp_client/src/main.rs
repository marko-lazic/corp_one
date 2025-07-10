mod client_frontend;
use crate::client_frontend::ClientFrontendPlugin;
use bevy::{
    ecs::error::{warn, GLOBAL_ERROR_HANDLER},
    prelude::*,
};
use clap::Parser;
use corp_client::prelude::*;
use corp_shared::prelude::Credentials;

#[derive(Parser, Debug)]
#[command(name = "corp_client")]
#[command(about = "Corp One Client")]
struct Args {
    #[arg(short = 'u', long = "username", help = "Username for authentication")]
    username: String,

    #[arg(short = 'p', long = "password", help = "Password for authentication")]
    password: String,
}

fn main() {
    let args = Args::parse();
    let credentials = Credentials::new(args.username, args.password);

    GLOBAL_ERROR_HANDLER
        .set(warn)
        .expect("The error handler can only be set once, globally.");
    App::new()
        .insert_resource(credentials)
        .add_plugins((ClientFrontendPlugin, ClientBackendPlugin))
        .run();
}
