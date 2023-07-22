use bevy::prelude::*;

use crate::asset::asset_loading::TextureAssets;
use crate::state::Despawn;
use crate::state::GameState;

pub struct StarMapPlugin;

impl Plugin for StarMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StarMap), setup_starmap);
    }
}

fn setup_starmap(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    // Orthographic camera is needed for showing SpriteBundle image
    commands.spawn((Camera2dBundle::default(), Despawn));
    commands.spawn((
        SpriteBundle {
            texture: texture_assets.nebula.clone(),
            ..Default::default()
        },
        Despawn,
    ));
}
