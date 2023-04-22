use bevy::prelude::*;

use crate::asset::asset_loading::TextureAssets;
use crate::GameState;

#[derive(Component)]
struct StarmapBackground;

pub struct StarMapPlugin;

impl Plugin for StarMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::setup_starmap.in_schedule(OnEnter(GameState::StarMap)));
        app.add_system(Self::teardown.in_schedule(OnExit(GameState::StarMap)));
    }
}

impl StarMapPlugin {
    fn setup_starmap(mut commands: Commands, texture_assets: Res<TextureAssets>) {
        // Orthographic camera is needed for showing SpriteBundle image
        commands.spawn(Camera2dBundle::default());
        commands
            .spawn(SpriteBundle {
                texture: texture_assets.nebula.clone(),
                ..Default::default()
            })
            .insert(StarmapBackground);
    }

    fn teardown(mut commands: Commands, entities: Query<Entity, Without<Window>>) {
        for entity in entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
