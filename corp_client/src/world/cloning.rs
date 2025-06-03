use crate::prelude::*;
use bevy::prelude::*;
use corp_shared::{prelude::*, world::colony::Colony};

pub struct CloningPlugin;

impl Plugin for CloningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::StarMap),
            star_map_vort_dead_player_to_cloning,
        )
        .add_systems(
            FixedUpdate,
            dead_player_system.run_if(in_state(GameState::Playing)),
        )
        .add_observer(player_loot_drop);
    }
}

#[derive(Event)]
struct PlayerDeadEvent {
    dead_player: Entity,
}

fn player_loot_drop(
    trigger: Trigger<PlayerDeadEvent>,
    mut q_player: Query<(&Transform, &mut Inventory), With<Player>>,
    mut commands: Commands,
    r_mesh_assets: Res<MeshAssets>,
) {
    if let Ok((transform, mut inventory)) = q_player.get_mut(trigger.event().dead_player) {
        if inventory.items.is_empty() {
            return;
        }

        commands
            .spawn((
                Backpack,
                Inventory::new(inventory.remove_all()),
                *transform,
                SceneRoot(r_mesh_assets.low_poly_backpack.clone()),
                StateScoped(GameState::Playing),
            ))
            .observe(on_use_backpack_event)
            .observe(on_use_backpack_action_event);
    }
}

fn dead_player_system(
    r_time: Res<Time<Fixed>>,
    mut q_health: Query<(Entity, &mut Health), With<Player>>,
    mut commands: Commands,
) {
    if let Some((e_player, mut health)) = q_health.iter_mut().next() {
        if health.is_dead() {
            commands.trigger(PlayerDeadEvent {
                dead_player: e_player,
            });
            health.cloning_cooldown.tick(r_time.delta());
            if health.cloning_cooldown.finished() {
                commands.trigger(RequestConnect(Colony::Cloning));
            }
        }
    }
}

fn star_map_vort_dead_player_to_cloning(
    mut r_player_store: ResMut<PlayerSystems>,
    mut commands: Commands,
) {
    if r_player_store.health.is_dead() {
        r_player_store.health.set_hit_points(CLONE_HEALTH_80);
        r_player_store.health.cloning_cooldown.reset();
        commands.trigger(RequestConnect(Colony::Cloning));
    }
}

#[cfg(test)]
mod tests {
    use crate::world::player::setup_player;

    use super::*;

    #[test]
    fn test_vort_in_dead_player() {
        // Setup stage
        let mut schedule = Schedule::default();
        schedule.add_systems(star_map_vort_dead_player_to_cloning);

        // Setup world
        let mut world = World::default();
        // Setup test entities
        let _player_entity = world.spawn((Player, Health::default())).id();

        let setup_player = world.register_system(setup_player);
        let setup_camera = world.register_system(setup_camera);
        let mut player_store = PlayerSystems {
            health: Health::default(),
            setup_player,
            setup_camera,
        };
        player_store.health.kill_mut();
        world.insert_resource(player_store);

        // Run systems
        schedule.run(&mut world);

        assert_eq!(
            world.resource::<PlayerSystems>().health.get_health(),
            &CLONE_HEALTH_80,
            "PlayerStore health is set to clone health"
        );
    }
}
