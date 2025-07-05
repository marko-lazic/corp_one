use crate::prelude::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_replicon::prelude::ClientTriggerExt;
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

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum LootAction {
    List,
    TakeAll,
    TakeItem(Entity),
}

#[derive(Deserialize, Event, Serialize, Clone, Debug)]
pub struct LootCommand {
    pub user: Entity,
    pub action: LootAction,
}

pub fn on_use_backpack_event(trigger: Trigger<UseCommand>, mut commands: Commands) {
    commands.client_trigger_targets(
        LootCommand {
            user: trigger.event().user,
            action: LootAction::TakeAll,
        },
        trigger.target(),
    );
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
        assert_eq!(app.get::<Contains>(e_backpack).into_iter().count(), 1);
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
            LootCommand {
                user: e_player,
                action: LootAction::List,
            },
            e_backpack,
        );

        app.update();

        // then
        assert_eq!(app.get::<Contains>(e_player).into_iter().count(), 0);
        assert_eq!(app.get::<Contains>(e_backpack).into_iter().count(), 2);
    }

    // This should now be an integration test
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
            LootCommand {
                user: e_player,
                action: LootAction::TakeAll,
            },
            e_backpack,
        );
        app.update();

        // then
        assert_eq!(app.get::<Contains>(e_player).into_iter().count(), 2);
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
            LootCommand {
                user: e_player,
                action: LootAction::TakeItem(e_item_2),
            },
            e_backpack,
        );
        app.update();

        // then
        assert_eq!(app.get::<Contains>(e_backpack).into_iter().count(), 1);
        assert_eq!(app.get::<Contains>(e_player).into_iter().count(), 1);
        assert_eq!(
            app.get::<Contains>(e_backpack).into_iter().next(),
            Some(&e_item_1),
        );
        assert_eq!(
            app.get::<Contains>(e_player).into_iter().next(),
            Some(&e_item_2)
        );
    }

    fn setup_backpack(app: &mut App, items: Vec<Entity>) -> Entity {
        let backpack = app
            .world_mut()
            .spawn(Backpack)
            .observe(on_use_backpack_event)
            .id();

        for item in items {
            app.world_mut().entity_mut(backpack).insert(StoredIn(item));
        }

        backpack
    }

    fn setup() -> (App, Entity) {
        let mut app = App::new();
        app.init_time();
        let player_entity = app.world_mut().spawn((Player, Inventory::default())).id();
        (app, player_entity)
    }
}
