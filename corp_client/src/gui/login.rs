use crate::prelude::{RequestConnect, ASSET_PATH};
use bevy::prelude::*;
use corp_shared::prelude::*;

pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Login), setup_login_screen)
            .add_systems(OnExit(GameState::Init), connect_to_star_map);
    }
}

fn connect_to_star_map(mut commands: Commands) {
    commands.trigger(RequestConnect(Colony::StarMap));
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

fn apply_login(_trigger: Trigger<Pointer<Released>>, mut commands: Commands) {
    info!("Login button pressed!");
    commands.trigger(RequestConnect(Colony::StarMap));
}
