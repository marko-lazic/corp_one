use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_rand::prelude::{GlobalRng, WyRand};
use bevy_replicon::prelude::*;
use corp_shared::prelude::*;
use rand::Rng;
use std::time::Duration;

pub struct LootPlugin;

impl Plugin for LootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                despawn_empty_backpack,
                loot_spawner.run_if(on_timer(Duration::from_secs_f32(2.0))),
            )
                .chain(),
        )
        .add_observer(on_add_dead_drop_loot)
        .add_observer(on_loot_command);
    }
}

pub fn despawn_empty_backpack(
    mut commands: Commands,
    q_entity_backpack: Query<Entity, (Without<Contains>, With<Backpack>)>,
) {
    for e_bag in &q_entity_backpack {
        info!("Despawning empty backpack {:?}", e_bag);
        commands.entity(e_bag).try_despawn();
    }
}

fn loot_spawner(
    mut commands: Commands,
    q_backpacks: Query<&Backpack>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    const MAX_BACKPACKS: usize = 10;
    if q_backpacks.iter().count() > MAX_BACKPACKS {
        return;
    }

    let x: f32 = rng.random_range(-10.0..=10.0);
    let z: f32 = rng.random_range(-10.0..=10.0);

    commands.spawn((
        Backpack,
        Transform::from_xyz(x, 0.1, z),
        Replicated,
        related!(Contains[(HackingTool, Replicated)]),
    ));
}

fn on_loot_command(
    loot_command: On<FromClient<LootCommand>>,
    loot_bag_query: Query<&Contains, With<Backpack>>,
    mut commands: Commands,
) -> Result {
    let player_e = loot_command
        .client_id
        .entity()
        .expect("Client ID is not an entity ID");
    let backpack_e = loot_command.loot_target;
    let backpack_action = loot_command.action;
    debug!("Backpack action: {:?}", backpack_action);
    let backpack_content = loot_bag_query.get(backpack_e)?;
    debug!("Backpack {:?} content: {:?}", backpack_e, backpack_content);

    match backpack_action {
        LootAction::TakeAll => {
            for item_entity in backpack_content.into_iter() {
                commands.entity(*item_entity).insert(StoredIn(player_e));
                info!(
                    "Transferring item {} to inventory {}",
                    item_entity, player_e
                );
            }
        }
        action => {
            warn!("not implemented: {:?}", action);
        }
    }
    Ok(())
}

fn on_add_dead_drop_loot(
    add: On<Add, Dead>,
    mut commands: Commands,
    q_player: Query<&Transform, With<Player>>,
    contains_query: Query<&Contains, With<Inventory>>,
) -> Result {
    let dead_player_entity = add.entity;
    let dead_player_transform = q_player.get(dead_player_entity)?;

    if let Ok(dead_player_loot) = contains_query.get(dead_player_entity) {
        let drop_loot_bag = commands
            .spawn((Backpack, Replicated, *dead_player_transform))
            .id();
        for item_entity in dead_player_loot.into_iter() {
            commands
                .entity(*item_entity)
                .insert(StoredIn(drop_loot_bag));
        }
    }

    Ok(())
}
