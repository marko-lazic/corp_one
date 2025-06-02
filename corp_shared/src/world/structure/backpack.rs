use crate::prelude::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[require(
    Name::new("Backpack"),
    Use,
    Inventory,
    Transform,
    RigidBody::Dynamic,
    Collider = init_backpack_collider(),
    Sensor,
    CollisionLayers = init_backpack_collision_layers(),
)]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState> = StateScoped(GameState::Playing))
)]
pub struct Backpack;

pub fn init_backpack_collider() -> Collider {
    Collider::cuboid(0.6, 2.4, 0.6)
}

pub fn init_backpack_collision_layers() -> CollisionLayers {
    CollisionLayers::new([GameLayer::Sensor], [GameLayer::Player])
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BackpackAction {
    List,
    TakeAll,
    TakeItem(Entity),
}

#[derive(Debug, Event)]
pub struct UseBackpackEvent {
    pub user: Entity,
    pub action: BackpackAction,
}

pub fn on_use_backpack_event(trigger: Trigger<UseEvent>, mut commands: Commands) {
    commands.trigger_targets(
        UseBackpackEvent {
            user: trigger.event().user,
            action: BackpackAction::TakeAll,
        },
        trigger.target(),
    );
}

pub fn on_use_backpack_action_event(
    trigger: Trigger<UseBackpackEvent>,
    mut q_inventory: Query<&mut Inventory>,
) {
    let e_user = trigger.event().user;
    let e_backpack = trigger.target();

    let Ok([mut user_inventory, mut backpack]) = q_inventory.get_many_mut([e_user, e_backpack])
    else {
        warn!("Could not find user_inventory and backpack entities");
        return;
    };

    match trigger.event().action {
        BackpackAction::List => {
            for backpack_item in backpack.items() {
                info!("{:?}", backpack_item);
            }
        }
        BackpackAction::TakeAll => {
            user_inventory.add_all(backpack.remove_all());
        }
        BackpackAction::TakeItem(item_entity) => {
            if let Some(backpack_item) = backpack.remove(item_entity) {
                user_inventory.add(backpack_item);
            }
        }
    }
}

pub fn despawn_empty_backpack_system(
    mut commands: Commands,
    mut q_entity_backpack: Query<(Entity, &Inventory), (Changed<Inventory>, With<Backpack>)>,
) {
    for (entity, inventory) in &mut q_entity_backpack {
        if inventory.items().count() == 0 {
            commands.entity(entity).try_despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_item_is_in_backpack() {
        // given
        let (mut app, _) = setup();
        let e_item = app.world_mut().spawn(HackingTool).id();
        let e_backpack = setup_backpack(&mut app, vec![e_item]);

        // when
        app.update();

        // then
        assert_eq!(app.get::<Inventory>(e_backpack).items().count(), 1);
    }

    #[test]
    fn player_list_items_in_backpack() {
        // given
        let (mut app, e_player) = setup();
        let e_item_1 = app.world_mut().spawn(HackingTool).id();
        let e_item_2 = app.world_mut().spawn(HackingTool).id();
        let e_backpack = setup_backpack(&mut app, vec![e_item_1, e_item_2]);

        // when
        app.world_mut().trigger_targets(
            UseBackpackEvent {
                user: e_player,
                action: BackpackAction::List,
            },
            e_backpack,
        );

        app.update();

        // then
        assert_eq!(app.get::<Inventory>(e_player).items().count(), 0);
        assert_eq!(app.get::<Inventory>(e_backpack).items().count(), 2);
    }

    #[test]
    fn player_take_all_items_from_backpack() {
        // given
        let (mut app, e_player) = setup();
        let e_item_1 = app.world_mut().spawn(HackingTool).id();
        let e_item_2 = app.world_mut().spawn(HackingTool).id();
        let e_backpack = setup_backpack(&mut app, vec![e_item_1, e_item_2]);

        // when
        app.update();
        app.world_mut().trigger_targets(
            UseBackpackEvent {
                user: e_player,
                action: BackpackAction::TakeAll,
            },
            e_backpack,
        );
        app.update();

        // then
        assert_eq!(app.get::<Inventory>(e_player).items().count(), 2);
        assert!(!app.has_component::<Backpack>(e_backpack));
    }

    #[test]
    fn player_take_one_item_from_backpack() {
        // given
        let (mut app, e_player) = setup();
        let e_item_1 = app.world_mut().spawn(HackingTool).id();
        let e_item_2 = app.world_mut().spawn(HackingTool).id();
        let e_backpack = setup_backpack(&mut app, vec![e_item_1, e_item_2]);

        // when
        app.update();
        app.world_mut().trigger_targets(
            UseBackpackEvent {
                user: e_player,
                action: BackpackAction::TakeItem(e_item_2),
            },
            e_backpack,
        );
        app.update();

        // then
        assert_eq!(app.get::<Inventory>(e_backpack).items().count(), 1);
        assert_eq!(app.get::<Inventory>(e_player).items().count(), 1);
        assert_eq!(
            app.get::<Inventory>(e_backpack).items().next(),
            Some(&e_item_1),
        );
        assert_eq!(
            app.get::<Inventory>(e_player).items().next(),
            Some(&e_item_2)
        );
    }

    fn setup_backpack(app: &mut App, items: Vec<Entity>) -> Entity {
        app.world_mut()
            .spawn((Backpack, Inventory::new(items)))
            .observe(on_use_backpack_event)
            .observe(on_use_backpack_action_event)
            .id()
    }

    fn setup() -> (App, Entity) {
        let mut app = App::new();
        app.init_time()
            .add_systems(Update, despawn_empty_backpack_system.chain());
        let player_entity = app.world_mut().spawn((Player, Inventory::default())).id();
        (app, player_entity)
    }
}
