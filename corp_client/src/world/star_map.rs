use bevy::prelude::*;

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
        app.add_system_set(
            SystemSet::on_enter(GameState::StarMap).with_system(Self::setup_starmap),
        );

        app.add_system_set(SystemSet::on_exit(GameState::StarMap).with_system(Self::teardown));
    }
}
