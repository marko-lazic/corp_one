use bevy::prelude::*;

use crate::asset::asset_loading::MeshAssets;
use crate::world::character::{CharacterBundle, CharacterName};
use corp_shared::CLONING_SPAWN_POSITION;

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub character: CharacterBundle,

    #[bundle]
    pub pbr: PbrBundle,
}

impl PlayerBundle {
    pub fn new(
        mesh_assets: Res<MeshAssets>,
        materials: ResMut<Assets<StandardMaterial>>,
    ) -> PlayerBundle {
        PlayerBundle {
            character: CharacterBundle {
                name: CharacterName::new("The Guy"),
                ..Default::default()
            },
            pbr: Self::create_pbr(mesh_assets, materials),
        }
    }

    fn create_pbr(
        mesh_assets: Res<MeshAssets>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> PbrBundle {
        let mesh = mesh_assets.mannequiny.clone();

        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            ..Default::default()
        });

        let transform = Transform {
            translation: CLONING_SPAWN_POSITION,
            ..Default::default()
        };

        PbrBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        }
    }
}
