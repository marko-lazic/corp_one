use crate::prelude::*;
use bevy::prelude::*;
use corp_shared::prelude::*;

pub struct StarMapPlugin;

impl Plugin for StarMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StarMap), setup_star_map);
    }
}

fn setup_star_map(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    // Orthographic camera is needed for showing SpriteBundle image
    commands.spawn((Camera2d, StateScoped(GameState::StarMap)));
    commands.spawn((
        Name::new("Star Map Background"),
        Sprite::from_image(texture_assets.nebula.clone()),
        StateScoped(GameState::StarMap),
    ));
}
