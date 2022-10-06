use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::asset::asset_loading::TextureAssets;
use crate::constants::state::GameState;

#[derive(Component)]
struct StarmapBackground;

pub struct StarMapPlugin;

impl Plugin for StarMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::StarMap, Self::setup_starmap);
        app.add_exit_system(GameState::StarMap, Self::teardown);
    }
}

impl StarMapPlugin {
    fn setup_starmap(mut commands: Commands, texture_assets: Res<TextureAssets>) {
        // Orthographic camera is needed for showing SpriteBundle image
        commands.spawn_bundle(Camera2dBundle::default());
        commands
            .spawn_bundle(SpriteBundle {
                texture: texture_assets.nebula.clone().into(),
                ..Default::default()
            })
            .insert(StarmapBackground);
    }

    fn teardown(mut commands: Commands, entities: Query<Entity>) {
        for entity in entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
