use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use bevy_replicon::prelude::*;
use corp_shared::prelude::*;
use rand::Rng;
use std::time::Duration;
pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            backpack_spawner.run_if(on_timer(Duration::from_secs_f32(2.0))),
        );
    }
}

fn backpack_spawner(
    mut commands: Commands,
    q_backpacks: Query<&Backpack>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    const MAX_BACKPACKS: usize = 10;
    if q_backpacks.iter().count() > MAX_BACKPACKS {
        return;
    }

    let x: f32 = rng.gen_range(-10.0..=10.0);
    let z: f32 = rng.gen_range(-10.0..=10.0);

    let e_item = commands.spawn((HackingTool, Replicated)).id();
    commands.spawn((
        Backpack,
        Inventory::new(vec![e_item]),
        Transform::from_xyz(x, 0.1, z),
        Replicated,
    ));
}
