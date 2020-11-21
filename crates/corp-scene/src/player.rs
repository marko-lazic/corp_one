use bevy::app::{AppBuilder, Plugin};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::ecs::{Commands, Res, ResMut};
use bevy::math::Vec3;
use bevy::pbr::prelude::StandardMaterial;
use bevy::pbr::PbrComponents;
use bevy::prelude::*;
use bevy::render::color::Color;
use bevy::render::mesh::Mesh;
use bevy::transform::components::Transform;

pub struct Player;
#[derive(Debug, Properties)]
pub struct MovementSpeed {
    pub acceleration: f32,
    pub max: f32,
}

impl Default for MovementSpeed {
    fn default() -> Self {
        Self {
            acceleration: 12.0,
            max: 400.0,
        }
    }
}
pub struct PlayerPlugin;
pub struct PlayerRes {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
        app.add_startup_system_to_stage("game_setup", spawn_player.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    let player_handle = asset_server.load("models/cube/cube.gltf#Mesh0/Primitive0");
    commands.insert_resource(PlayerRes {
        mesh: player_handle,
        material: material_handle.clone(),
    });
}

pub fn spawn_player(mut commands: Commands, player_res: Res<PlayerRes>) {
    commands
        .spawn(PbrComponents {
            mesh: player_res.mesh.clone(),
            material: player_res.material.clone(),
            transform: Transform::from_translation(Vec3::new(10.0, 1.0, -10.0)),
            ..Default::default()
        })
        .with(Player)
        .with(MovementSpeed::default());
}
