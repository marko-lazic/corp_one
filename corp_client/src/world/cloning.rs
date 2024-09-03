use crate::{
    asset::{Colony, ColonyConfigAssets, MeshAssets},
    state::GameState,
    world::prelude::*,
};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use corp_shared::prelude::*;

pub struct CloningPlugin;

impl Plugin for CloningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StarMap), vort_in_dead_player_to_cloning)
            .add_systems(
                FixedUpdate,
                dead_player_system.run_if(in_state(GameState::Playing)),
            )
            .observe(player_loot_drop);
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
                Name::new("Loot Bag"),
                SceneBundle {
                    scene: r_mesh_assets.low_poly_backpack.clone(),
                    transform: Transform::from_translation(transform.translation)
                        .with_scale(Vec3::splat(0.2)),
                    ..default()
                },
                BackpackBundle::with_items(inventory.remove_all()),
                PickableBundle::default(),
                InteractionObjectType::Backpack,
                RigidBody::Dynamic,
                Collider::cuboid(1.5, 1.5, 1.5),
                StateScoped(GameState::Playing),
            ))
            .observe(on_use_backpack_event)
            .observe(on_use_backpack_action_event);
    }
}

fn dead_player_system(
    r_colony_config_assets: Res<ColonyConfigAssets>,
    r_time: Res<Time>,
    mut r_player_entity: ResMut<PlayerEntity>,
    mut ev_colony_load: EventWriter<ColonyLoadEvent>,
    mut q_health: Query<(Entity, &mut Health), With<Player>>,
    mut r_next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    if let Some((e_player, mut health)) = q_health.iter_mut().next() {
        if health.is_dead() {
            r_player_entity.0 = None;
            commands.trigger(PlayerDeadEvent {
                dead_player: e_player,
            });
            health.cloning_cooldown.tick(r_time.delta());
            if health.cloning_cooldown.finished() {
                ev_colony_load.send(ColonyLoadEvent(r_colony_config_assets.cloning.clone()));
                r_next_state.set(GameState::LoadColony);
            }
        }
    }
}

fn vort_in_dead_player_to_cloning(
    mut r_player_store: ResMut<PlayerStore>,
    mut ev_vort_in: EventWriter<VortInEvent>,
) {
    if r_player_store.health.is_dead() {
        r_player_store.health.set_hit_points(CLONE_HEALTH_80);
        r_player_store.health.cloning_cooldown.reset();
        ev_vort_in.send(VortInEvent::vort(Colony::Cloning));
    }
}

#[cfg(test)]
mod tests {
    use crate::world::player::setup_player;

    use super::*;

    fn kill_player(mut healths: Query<&mut Health, With<Player>>) {
        for mut health in healths.iter_mut() {
            health.kill_mut();
        }
    }

    #[test]
    fn test_vort_out_dead_player() {
        // given
        let mut app = App::new();
        init_time(&mut app);
        app.init_state::<GameState>();
        app.add_systems(Update, (kill_player, dead_player_system).chain());
        app.insert_resource(create_colony_assets());
        let setup_player = app.world_mut().register_system(setup_player);
        app.insert_resource(PlayerStore {
            health: Health::default(),
            setup_player,
        });
        app.add_event::<ColonyLoadEvent>();
        let player_entity = app.world_mut().spawn((Player, Health::default())).id();

        // when
        app.update();

        // Check resulting changes
        assert!(app.world().get::<Player>(player_entity).is_some());

        assert_eq!(
            *app.world()
                .get::<Health>(player_entity)
                .unwrap()
                .get_health(),
            MIN_HEALTH.clone(),
            "Player is dead"
        );
    }

    #[test]
    fn test_vort_in_dead_player() {
        // Setup stage
        let mut schedule = Schedule::default();
        schedule.add_systems(vort_in_dead_player_to_cloning);

        // Setup world
        let mut world = World::default();
        // Setup test entities
        let _player_entity = world.spawn((Player, Health::default())).id();

        let setup_player = world.register_system(setup_player);
        let mut player_store = PlayerStore {
            health: Health::default(),
            setup_player,
        };
        player_store.health.kill_mut();
        world.insert_resource(player_store);
        world.init_resource::<Events<VortInEvent>>();
        world.insert_resource(create_colony_assets());

        // Run systems
        schedule.run(&mut world);

        assert_eq!(
            world.resource::<PlayerStore>().health.get_health(),
            &CLONE_HEALTH_80,
            "PlayerStore health is set to clone health"
        );
    }

    fn init_time(app: &mut App) {
        app.init_resource::<Time>();
        let mut time = Time::default();
        time.update();
        app.world_mut().insert_resource(time);
    }

    fn create_colony_assets() -> ColonyConfigAssets {
        ColonyConfigAssets {
            iris: Default::default(),
            liberte: Default::default(),
            cloning: Default::default(),
        }
    }
}
