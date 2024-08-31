use std::{cmp::PartialEq, time::Duration};

use bevy::prelude::*;

use crate::prelude::*;

#[derive(Component)]
pub struct Door;

#[derive(Component, Default, Eq, PartialEq, Debug, Copy, Clone)]
pub enum DoorState {
    Open,
    #[default]
    Closed,
}

impl DoorState {
    pub fn toggle(&mut self, door_cooldown: &mut DoorCooldown) {
        if door_cooldown.toggle_block.finished() {
            door_cooldown.toggle_block.reset();
            door_cooldown.autoclose.reset();
            *self = match *self {
                DoorState::Open => DoorState::Closed,
                DoorState::Closed => DoorState::Open,
            };
        }
    }
}

#[derive(Component)]
pub struct DoorCooldown {
    pub autoclose: Timer,
    pub toggle_block: Timer,
}

#[derive(Bundle)]
pub struct DoorBundle {
    pub door: Door,
    pub state: DoorState,
    pub security_level: SecurityLevel,
    pub ownership_registry: OwnershipRegistry,
    pub cooldown: DoorCooldown,
}

impl Door {
    const OPEN_TIME: f32 = 10.0;
    const TOGGLE_TIME: f32 = 1.0;
    const HACK_DURATION: f32 = 60.0 * 5.0; // 5 min
}

impl Default for DoorBundle {
    fn default() -> Self {
        let mut toggle_cooldown = Timer::from_seconds(Door::TOGGLE_TIME, TimerMode::Once);
        toggle_cooldown.tick(Duration::from_secs_f32(Door::TOGGLE_TIME));
        Self {
            door: Door,
            state: DoorState::Closed,
            security_level: SecurityLevel::Low,
            ownership_registry: OwnershipRegistry::default(),
            cooldown: DoorCooldown {
                autoclose: Timer::from_seconds(Door::OPEN_TIME, TimerMode::Once),
                toggle_block: toggle_cooldown,
            },
        }
    }
}

pub struct UseDoorEvent;

#[derive(Event)]
pub struct UseDoorHackEvent {
    pub hacker: Entity,
}

pub fn door_cooldown_system(
    mut q_door: Query<(&mut DoorCooldown, &mut DoorState), With<Door>>,
    r_time: Res<Time>,
) {
    for (mut door_cooldown, mut door_state) in &mut q_door {
        // If the door is currently open and the cooldown timer has expired, set the state to Closed
        if *door_state == DoorState::Open
            && door_cooldown.autoclose.tick(r_time.delta()).just_finished()
        {
            *door_state = DoorState::Closed;
            door_cooldown.autoclose.reset();
        }

        // If the door toggle cooldown timer has expired, allow the player to interact with the door again
        if !door_cooldown.toggle_block.finished() {
            door_cooldown.toggle_block.tick(r_time.delta());
        }
    }
}

pub fn on_use_door_event_door(
    trigger: Trigger<UseEvent>,
    mut commands: Commands,
    q_member: Query<&MemberOf>,
    mut q_door: Query<
        (
            &mut DoorCooldown,
            &mut DoorState,
            &OwnershipRegistry,
            &SecurityLevel,
        ),
        With<Door>,
    >,
) {
    if let Ok(member_of) = q_member.get(trigger.event().user) {
        if let Ok((mut door_cooldown, mut door_state, control_registry, security_level)) =
            q_door.get_mut(trigger.entity())
        {
            if let Some(control_type) = control_registry.get_control_type(&member_of.faction) {
                match control_type {
                    Ownership::Permanent(_) => {
                        if Some(&member_of.rank) >= REQUIRED_RANK_BY_SECURITY.get(security_level) {
                            door_state.toggle(&mut door_cooldown);
                        }
                    }
                    Ownership::Hacked(_, _) => {
                        door_state.toggle(&mut door_cooldown);
                    }
                }
            } else {
                commands.trigger_targets(
                    UseDoorHackEvent {
                        hacker: trigger.event().user,
                    },
                    trigger.entity(),
                );
            }
        }
    }
}

pub fn on_use_door_hack_event(
    trigger: Trigger<UseDoorHackEvent>,
    mut commands: Commands,
    mut q_door: Query<(&mut DoorCooldown, &mut DoorState, &mut OwnershipRegistry)>,
    mut q_player: Query<(&mut Inventory, &MemberOf), With<Player>>,
    q_hacking_tool: Query<&HackingTool>,
) {
    if let Ok((mut door_cooldown, mut door_state, mut ownership_registry)) =
        q_door.get_mut(trigger.entity())
    {
        if let Ok((mut inventory, member_of)) = q_player.get_mut(trigger.event().hacker) {
            if let Some(hacking_tool_entity) = inventory
                .items
                .iter()
                .find(|&&item_entity| q_hacking_tool.get(item_entity).is_ok())
                .copied()
            {
                inventory.remove_item(hacking_tool_entity);
                commands.entity(hacking_tool_entity).despawn_recursive();
                ownership_registry.add(Ownership::Hacked(
                    member_of.faction,
                    Timer::new(
                        Duration::from_secs_f32(Door::HACK_DURATION),
                        TimerMode::Once,
                    ),
                ));
                door_state.toggle(&mut door_cooldown);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::prelude::*;

    use crate::prelude::*;

    #[test]
    fn door_default_closed() {
        // given
        let mut app = setup();
        let door_entity = setup_door(&mut app, Faction::EC, SecurityLevel::Low);
        setup_player(&mut app, vec![], Faction::EC, Rank::R0);

        // when
        app.update();

        // then
        let result = app.get::<DoorState>(door_entity);
        assert_eq!(*result, DoorState::Closed);
    }

    #[test]
    fn open_door_closed_after_10s() {
        // given
        let mut app = setup();
        setup_player(&mut app, vec![], Faction::EC, Rank::R0);
        let door_entity = setup_door(&mut app, Faction::EC, SecurityLevel::Low);
        *app.get_mut::<DoorState>(door_entity) = DoorState::Open;

        // when
        app.update_after(Duration::from_secs_f32(10.0));

        // then
        assert_eq!(*app.get::<DoorState>(door_entity), DoorState::Closed);
        let result = app.get::<DoorCooldown>(door_entity);
        assert!(!result.autoclose.finished());
        assert_eq!(result.autoclose.remaining_secs(), 10.0);
    }

    #[test]
    fn player_open_door() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R5);
        let door_entity = setup_door(&mut app, Faction::EC, SecurityLevel::Low);

        // when
        app.world_mut()
            .trigger_targets(UseEvent::new(player_entity), door_entity);
        app.update();

        // then
        let result = app.get::<DoorState>(door_entity);
        assert_eq!(*result, DoorState::Open);
    }

    #[test]
    fn two_doors_one_open_other_closed() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R5);
        let door_entity_1 = setup_door(&mut app, Faction::EC, SecurityLevel::Low);
        let door_entity_2 = setup_door(&mut app, Faction::EC, SecurityLevel::Low);

        // when
        app.world_mut()
            .trigger_targets(UseEvent::new(player_entity), door_entity_1);
        app.update();

        // then
        let result_1 = app.get::<DoorState>(door_entity_1);
        assert_eq!(*result_1, DoorState::Open);
        let result_2 = app.get::<DoorState>(door_entity_2);
        assert_eq!(*result_2, DoorState::Closed);
    }

    #[test]
    fn player_close_open_door() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R4);
        let door_entity = setup_door(&mut app, Faction::EC, SecurityLevel::Low);
        *app.get_mut::<DoorState>(door_entity) = DoorState::Open;

        // when
        app.world_mut()
            .trigger_targets(UseEvent::new(player_entity), door_entity);
        app.update();

        // then
        let result = app.get::<DoorState>(door_entity);
        assert_eq!(*result, DoorState::Closed);
    }

    #[test]
    fn player_open_door_wait_3_seconds_and_close_door() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R0);
        let door_entity = setup_door(&mut app, Faction::EC, SecurityLevel::Low);

        // when
        app.world_mut()
            .trigger_targets(UseEvent::new(player_entity), door_entity);
        app.update_after(Duration::from_secs_f32(3.0));
        app.world_mut()
            .trigger_targets(UseEvent::new(player_entity), door_entity);
        app.update();

        // then
        let result = app.get::<DoorState>(door_entity);
        assert_eq!(*result, DoorState::Closed);
    }

    #[test]
    fn player_open_door_two_times_before_toggle_cooldown_finished() {
        // given
        let mut app = setup();
        let door_entity = setup_door(&mut app, Faction::EC, SecurityLevel::High);
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R6);

        // when
        app.world_mut()
            .trigger_targets(UseEvent::new(player_entity), door_entity);
        app.update_after(Duration::from_secs_f32(0.5));
        app.world_mut()
            .trigger_targets(UseEvent::new(player_entity), door_entity);
        app.update_after(Duration::from_secs_f32(0.1));

        // then
        assert_eq!(*app.get::<DoorState>(door_entity), DoorState::Open);
        assert!(!app.get::<DoorCooldown>(door_entity).toggle_block.finished());
    }

    #[test]
    fn player_open_door_two_times_open_cooldown_resets() {
        // given
        let mut app = setup();
        let door_entity = setup_door(&mut app, Faction::EC, SecurityLevel::High);
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R6);

        // when
        for _ in 0..3 {
            app.update_after(Duration::from_secs_f32(2.0));
            app.world_mut()
                .trigger_targets(UseEvent::new(player_entity), door_entity);
        }

        // then
        assert_eq!(*app.get::<DoorState>(door_entity), DoorState::Open);
        assert_eq!(
            app.get::<DoorCooldown>(door_entity)
                .autoclose
                .remaining_secs(),
            10.0
        );
    }

    #[test]
    fn cmg_player_hacks_ec_door_and_lose_hacking_tool() {
        // given
        let mut app = setup();
        let hacking_tool_entity = setup_hacking_tool(&mut app);
        let player_entity =
            setup_player(&mut app, vec![hacking_tool_entity], Faction::CMG, Rank::R0);
        let door_entity = setup_door(&mut app, Faction::EC, SecurityLevel::Low);

        // when
        app.world_mut()
            .trigger_targets(UseEvent::new(player_entity), door_entity);
        app.update();

        // then
        assert_eq!(*app.get::<DoorState>(door_entity), DoorState::Open);
        assert_eq!(app.get::<Inventory>(player_entity).items.len(), 0);
        assert!(!app.has_component::<HackingTool>(hacking_tool_entity));
    }

    #[test]
    fn cmg_player_loses_hacking_tool_and_vi_keeps_it() {
        // given
        let mut app = setup();
        let ht_1 = setup_hacking_tool(&mut app);
        let ht_2 = setup_hacking_tool(&mut app);
        let vi_player_entity = setup_player(&mut app, vec![ht_1], Faction::VI, Rank::R0);
        let cmg_player_entity = setup_player(&mut app, vec![ht_2], Faction::CMG, Rank::R0);
        let door_entity = setup_door(&mut app, Faction::EC, SecurityLevel::Low);

        // when
        app.world_mut()
            .trigger_targets(UseEvent::new(cmg_player_entity), door_entity);
        app.update();

        // then
        assert_eq!(app.get::<Inventory>(cmg_player_entity).items.len(), 0);
        assert_eq!(app.get::<Inventory>(vi_player_entity).items.len(), 1);
    }

    #[test]
    fn ec_player_hacks_cmg_door_without_hacking_tool() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R7);
        let door_entity = setup_door(&mut app, Faction::CMG, SecurityLevel::High);

        // when
        app.world_mut()
            .trigger_targets(UseEvent::new(player_entity), door_entity);
        app.update();

        // then
        let result = app.get::<DoorState>(door_entity);
        assert_eq!(*result, DoorState::Closed);
    }

    #[test]
    fn ec_player_hacks_cmg_door_with_hacking_tool() {
        // given
        let mut app = setup();
        let hacking_tool_entity = setup_hacking_tool(&mut app);
        let player_entity =
            setup_player(&mut app, vec![hacking_tool_entity], Faction::EC, Rank::R3);
        let door_entity = setup_door(&mut app, Faction::CMG, SecurityLevel::Low);

        // when
        app.world_mut()
            .trigger_targets(UseEvent::new(player_entity), door_entity);
        app.update();

        // then
        let result = app.get::<DoorState>(door_entity);
        assert_eq!(*result, DoorState::Open);
    }

    #[test]
    fn ec_player_hacks_vi_door_and_can_open_them_after_2_minutes() {
        // given
        let mut app = setup();
        let e_hacking_tool = setup_hacking_tool(&mut app);
        let e_player_ec = setup_player(&mut app, vec![e_hacking_tool], Faction::EC, Rank::R1);
        let e_door_vi = setup_door(&mut app, Faction::VI, SecurityLevel::Medium);
        app.world_mut()
            .trigger_targets(UseEvent::new(e_player_ec), e_door_vi);
        app.update();

        // when
        app.update_after(Duration::from_secs_f32(2.0 * 60.0));
        app.world_mut()
            .trigger_targets(UseEvent::new(e_player_ec), e_door_vi);
        app.update();

        // then
        assert_eq!(*app.get::<DoorState>(e_door_vi), DoorState::Open);
    }

    #[test]
    fn vi_player_hacks_ec_door_and_can_not_open_them_after_5_minutes() {
        // given
        let mut app = setup();
        let hacking_tool_entity = setup_hacking_tool(&mut app);
        let e_player_vi = setup_player(&mut app, vec![hacking_tool_entity], Faction::VI, Rank::R0);
        let e_door_ec = setup_door(&mut app, Faction::EC, SecurityLevel::Low);
        app.world_mut()
            .trigger_targets(UseEvent::new(e_player_vi), e_door_ec);
        app.update();
        app.update_after(Duration::from_secs_f32(Door::HACK_DURATION));

        // when
        app.world_mut()
            .trigger_targets(UseEvent::new(e_player_vi), e_door_ec);
        app.update();

        // then
        let result = app.get::<DoorState>(e_door_ec);
        assert_eq!(*result, DoorState::Closed);
    }

    #[test]
    fn ec_player_r3_can_not_open_ec_door_with_security_low() {
        // given
        let mut app = setup();
        let e_player_ec = setup_player(&mut app, vec![], Faction::EC, Rank::R3);
        let e_door_ec = setup_door(&mut app, Faction::EC, SecurityLevel::Low);

        // when
        app.world_mut()
            .trigger_targets(UseEvent::new(e_player_ec), e_door_ec);
        app.update();

        // then
        let result = app.get::<DoorState>(e_door_ec);
        assert_eq!(*result, DoorState::Closed);
    }

    #[test]
    fn same_faction_player_can_open_hacked_door() {
        // given
        let mut app = setup();
        let hacking_tool_entity = setup_hacking_tool(&mut app);
        let e_player_ec = setup_player(&mut app, vec![hacking_tool_entity], Faction::EC, Rank::R2);
        let e_door_cmg = setup_door(&mut app, Faction::CMG, SecurityLevel::Low);
        app.world_mut()
            .trigger_targets(UseEvent::new(e_player_ec), e_door_cmg);
        app.update();
        app.update_after(Duration::from_secs_f32(10.0));
        let e_other_player_ec = setup_player(&mut app, vec![], Faction::EC, Rank::R0);

        // when
        app.world_mut()
            .trigger_targets(UseEvent::new(e_other_player_ec), e_door_cmg);
        app.update();

        // then
        assert_eq!(*app.get::<DoorState>(e_door_cmg), DoorState::Open);
    }

    fn setup() -> App {
        let mut app = App::new();
        app.init_time();
        app.add_event::<BackpackInteractionEvent>();
        app.add_event::<UseDoorHackEvent>();
        app.add_systems(
            Update,
            (
                door_cooldown_system,
                process_temporary_faction_ownership_timers_system,
            )
                .chain(),
        );
        app
    }

    fn setup_player(app: &mut App, items: Vec<Entity>, faction: Faction, rank: Rank) -> Entity {
        let player_entity = app
            .world_mut()
            .spawn((Player, Inventory::new(items), MemberOf { faction, rank }))
            .id();
        player_entity
    }

    fn setup_hacking_tool(app: &mut App) -> Entity {
        let item_entity = app.world_mut().spawn(HackingToolBundle::default()).id();
        item_entity
    }

    fn setup_door(app: &mut App, faction: Faction, security: SecurityLevel) -> Entity {
        let mut registry = OwnershipRegistry::default();
        registry.add_permanent(faction);
        let door_entity = app
            .world_mut()
            .spawn((DoorBundle {
                security_level: security,
                ownership_registry: registry,
                ..default()
            },))
            .observe(on_use_door_event_door)
            .observe(on_use_door_hack_event)
            .id();
        door_entity
    }
}
