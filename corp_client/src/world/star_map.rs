use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::asset::asset_loading::TextureAssets;
use crate::constants::state::GameState;

pub struct StarMapPlugin;

#[derive(Component)]
struct StarmapBackground;

impl StarMapPlugin {
    fn setup_starmap(mut commands: Commands, texture_assets: Res<TextureAssets>) {
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
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

impl Plugin for StarMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::StarMap, Self::setup_starmap);
        app.add_exit_system(GameState::StarMap, Self::teardown);
    }
}
