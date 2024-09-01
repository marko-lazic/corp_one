use bevy::prelude::*;

use crate::{asset::TextureAssets, state::GameState};

pub struct StarMapPlugin;

impl Plugin for StarMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StarMap), setup_star_map);
    }
}

fn setup_star_map(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    // Orthographic camera is needed for showing SpriteBundle image
    commands.spawn((Camera2dBundle::default(), StateScoped(GameState::StarMap)));
    commands.spawn((
        Name::new("Star Map Background"),
        SpriteBundle {
            texture: texture_assets.nebula.clone(),
            ..Default::default()
        },
        StateScoped(GameState::StarMap),
    ));
}
