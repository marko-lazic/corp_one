use crate::prelude::{CorpClient, RequestConnect, ASSET_PATH};
use bevy::prelude::*;
use bevy_defer::{AsyncCommandsExtension, AsyncWorld};
use corp_shared::prelude::*;
use corp_types::prelude::LoginResponse;

pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Login), setup_login_screen)
            .add_systems(OnExit(GameState::Init), auto_login);
    }
}

fn auto_login(
    mut commands: Commands,
    credentials: Res<Credentials>,
    client_e: Single<Entity, With<CorpClient>>,
) -> Result {
    info!("Auto login initiated!");
    login(&mut commands, &credentials, *client_e);
    Ok(())
}

fn setup_login_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, StateScoped(GameState::Login)));
    commands.spawn(login_button(asset_server));
}

fn login_button(asset_server: Res<AssetServer>) -> impl Bundle {
    (
        Button,
        Observer::new(apply_login),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..default()
        },
        StateScoped(GameState::Login),
        children![(
            Text::new("Login"),
            TextFont::from_font(asset_server.load(ASSET_PATH.default_font)).with_font_size(40.0),
            TextColor::from(Color::srgb(0.9, 0.9, 0.9)),
        )],
    )
}

fn apply_login(
    _trigger: Trigger<Pointer<Released>>,
    mut commands: Commands,
    credentials: Res<Credentials>,
    client_e: Single<Entity, With<CorpClient>>,
) -> Result {
    info!("Login button pressed!");
    login(&mut commands, &credentials, *client_e);
    Ok(())
}

fn login(commands: &mut Commands, credentials: &Credentials, client_e: Entity) {
    let credentials = credentials.clone();
    commands.spawn_task(move || async move {
        match authenticate(credentials).await {
            Ok(login_response) => {
                let async_world = AsyncWorld;
                async_world.insert_resource(AuthToken(login_response.token));
                async_world
                    .entity(client_e)
                    .trigger(RequestConnect(Colony::StarMap))?;
            }
            Err(err) => {
                error!("Login failed: {:?}", err);
            }
        }
        Ok(())
    });
}

async fn authenticate(credentials: Credentials) -> surf::Result<LoginResponse> {
    surf::post("http://localhost:25550/login")
        .body_json(&credentials)?
        .recv_json()
        .await
}
