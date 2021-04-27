use bevy::prelude::*;
use rand::Rng;

use crate::loading::{MaterialAssets, MeshAssets};
use crate::world::camera::MyRaycastSet;
use crate::GameState;
use bevy_mod_raycast::RayCastMesh;

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(SpawnerTimer::default());
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(cube_movement.system())
                .with_system(cube_spawner.system()),
        );
    }
}

struct Cube;

struct SpawnerState {
    cube_count: u32,
}

struct SpawnerTimer {
    timer: Timer,
}

impl Default for SpawnerState {
    fn default() -> Self {
        SpawnerState { cube_count: 0 }
    }
}

impl Default for SpawnerTimer {
    fn default() -> Self {
        SpawnerTimer {
            timer: Timer::from_seconds(5.0, true),
        }
    }
}

fn cube_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnerTimer>,
    mut spawner: Local<SpawnerState>,
    mesh_assets: Res<MeshAssets>,
    material_assets: Res<MaterialAssets>,
) {
    if timer.timer.tick(time.delta()).just_finished() {
        if spawner.cube_count < 10 {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(-10.0..10.0);
            let y = rng.gen_range(0.0..3.0);
            let z = rng.gen_range(-10.0..10.0);

            commands
                .spawn_bundle(PbrBundle {
                    mesh: mesh_assets.cube.clone(),
                    material: material_assets.cube.clone(),
                    transform: Transform::from_translation(Vec3::new(x, y, z)),
                    ..Default::default()
                })
                .insert(Cube)
                .insert(RayCastMesh::<MyRaycastSet>::default());
            spawner.cube_count += 1;
        }
    }
}

fn cube_movement(mut cube_positions: Query<(&Cube, &mut Transform)>) {
    for (_cube, mut transform) in cube_positions.iter_mut() {
        transform.translation.y += 0.01;
    }
}
