use crate::camera;
use bevy::prelude::*;

pub struct Player;
#[derive(Debug)]
pub struct MovementSpeed {
    pub acceleration: f32,
    pub max: f32,
    pub is_moving: bool,
    pub moving_happen: bool,
}

impl Default for MovementSpeed {
    fn default() -> Self {
        Self {
            acceleration: 12.0,
            max: 400.0,
            is_moving: false,
            moving_happen: false,
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
        app.add_startup_stage("game_setup", SystemStage::single(spawn_player.system()));
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    let player_handle =
        asset_server.load("models/mannequiny/mannequiny-0.3.0.glb#Mesh0/Primitive0");
    commands.insert_resource(PlayerRes {
        mesh: player_handle,
        material: material_handle.clone(),
    });
}

pub fn spawn_player(commands: &mut Commands, player_res: Res<PlayerRes>) {
    // player
    let player = commands
        .spawn(PbrBundle {
            mesh: player_res.mesh.clone(),
            material: player_res.material.clone(),
            transform: Transform::from_translation(Vec3::new(10.0, 0., -10.0)),
            ..Default::default()
        })
        .with(Player)
        .with(MovementSpeed::default())
        .current_entity();

    let camera = camera::spawn_camera(commands);

    // Append camera to player as child.
    commands.push_children(player.unwrap(), &[camera.unwrap()]);
}
