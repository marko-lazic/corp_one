use crate::prelude::*;
use bevy::prelude::*;
use bevy_replicon::prelude::ClientTriggerExt;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(
    Name::new("Door"),
    DynamicStructure,
    DoorState,
    SecurityLevel::Low,
    OwnershipRegistry = lookup_door_ownership()
)]
pub struct Door;

fn lookup_door_ownership() -> OwnershipRegistry {
    OwnershipRegistry::new_permanent(Faction::EC)
}

#[derive(Component, Reflect, Debug, PartialEq, Eq)]
#[reflect(Component)]
pub struct DoorId(pub i32);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Name::new("Door Terminal"), Structure, Use)]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState> = StateScoped(GameState::Playing))
)]
pub struct DoorTerminal;

#[derive(Component, Default, Eq, PartialEq, Debug, Clone)]
pub enum DoorState {
    Open {
        autoclose: Timer,
        toggle_block: Timer,
    },
    #[default]
    Closed,
}

impl DoorState {
    const AUTOCLOSE_SECS: f32 = 10.0;
    const TOGGLE_BLOCK_SECS: f32 = 1.0;
    pub const HACK_DURATION_SECS: f32 = 60.0 * 5.0;
    pub fn open() -> Self {
        DoorState::Open {
            autoclose: Timer::from_seconds(Self::AUTOCLOSE_SECS, TimerMode::Once),
            toggle_block: Timer::from_seconds(Self::TOGGLE_BLOCK_SECS, TimerMode::Once),
        }
    }

    pub fn is_open(&self) -> bool {
        matches!(self, DoorState::Open { .. })
    }
    pub fn toggle(&mut self) {
        match self {
            DoorState::Open {
                toggle_block,
                autoclose: _autoclose,
            } => {
                if toggle_block.finished() {
                    *self = DoorState::Closed;
                }
            }
            DoorState::Closed => {
                *self = Self::open();
            }
        }
    }
}

pub struct UseDoorEvent;

#[derive(Deserialize, Event, Serialize, Clone, Debug)]
pub struct DoorHackCommand;

#[derive(Deserialize, Event, Serialize, Clone, Debug)]
pub enum DoorHackedEvent {
    Successful,
    Failure,
}

pub fn door_cooldown_system(mut q_door: Query<&mut DoorState, With<Door>>, r_time: Res<Time>) {
    for mut door_state in &mut q_door {
        match *door_state {
            DoorState::Open {
                ref mut autoclose,
                ref mut toggle_block,
            } => {
                autoclose.tick(r_time.delta());
                toggle_block.tick(r_time.delta());

                if autoclose.finished() {
                    *door_state = DoorState::Closed;
                }
            }
            _ => {}
        }
    }
}

pub fn on_use_command(
    trigger: Trigger<UseCommand>,
    mut commands: Commands,
    q_member: Query<&PlayerFactionInfo>,
    mut q_door: Query<(&mut DoorState, &OwnershipRegistry, &SecurityLevel), With<Door>>,
) {
    if let Ok(member_of) = q_member.get(trigger.event().user) {
        if let Ok((mut door_state, control_registry, security_level)) =
            q_door.get_mut(trigger.target())
        {
            if let Some(control_type) = control_registry.get_control_type(&member_of.faction) {
                match control_type {
                    Ownership::Permanent(_) => {
                        if security_level.has_required_rank(&member_of.rank) {
                            door_state.toggle();
                        }
                    }
                    Ownership::Hacked(_, _) => {
                        door_state.toggle();
                    }
                }
            } else {
                commands.client_trigger_targets(DoorHackCommand, trigger.target());
            }
        }
    }
}

pub fn on_use_door_terminal(
    trigger: Trigger<UseCommand>,
    mut commands: Commands,
    q_door_id: Query<&DoorId>,
    q_door: Query<(Entity, &DoorId), With<Door>>,
) {
    if let Ok(terminal_door_id) = q_door_id.get(trigger.target()) {
        for (door_entity, door_id) in &q_door {
            if terminal_door_id == door_id {
                commands.trigger_targets(UseCommand::new(trigger.user), door_entity);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use bevy::prelude::*;
    use std::time::Duration;

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
        *app.get_mut::<DoorState>(door_entity) = DoorState::open();

        // when
        app.update_after(Duration::from_secs_f32(10.0));

        // then
        assert!(matches!(
            *app.get::<DoorState>(door_entity),
            DoorState::Closed
        ));
    }

    #[test]
    fn player_open_door() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R5);
        let door_entity = setup_door(&mut app, Faction::EC, SecurityLevel::Low);

        // when
        app.world_mut()
            .trigger_targets(UseCommand::new(player_entity), door_entity);

        // then
        let result = app.get::<DoorState>(door_entity);
        assert!(matches!(*result, DoorState::Open { .. }));
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
            .trigger_targets(UseCommand::new(player_entity), door_entity_1);
        app.update();

        // then
        let result_1 = app.get::<DoorState>(door_entity_1);
        assert!(matches!(*result_1, DoorState::Open { .. }));
        let result_2 = app.get::<DoorState>(door_entity_2);
        assert_eq!(*result_2, DoorState::Closed);
    }

    #[test]
    fn player_close_open_door() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R4);
        let door_entity = setup_door(&mut app, Faction::EC, SecurityLevel::Low);
        *app.get_mut::<DoorState>(door_entity) = DoorState::open();
        app.update_after(Duration::from_secs_f32(1.0));

        // when
        app.world_mut()
            .trigger_targets(UseCommand::new(player_entity), door_entity);
        app.update();

        // then
        assert!(matches!(
            *app.get::<DoorState>(door_entity),
            DoorState::Closed
        ));
    }

    #[test]
    fn player_open_door_wait_3_seconds_and_close_door() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R0);
        let door_entity = setup_door(&mut app, Faction::EC, SecurityLevel::Low);

        // when
        app.world_mut()
            .trigger_targets(UseCommand::new(player_entity), door_entity);
        app.update_after(Duration::from_secs_f32(3.0));
        app.world_mut()
            .trigger_targets(UseCommand::new(player_entity), door_entity);

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
            .trigger_targets(UseCommand::new(player_entity), door_entity);
        app.update_after(Duration::from_secs_f32(0.5));
        app.world_mut()
            .trigger_targets(UseCommand::new(player_entity), door_entity);
        app.update_after(Duration::from_secs_f32(0.1));

        // then
        if let DoorState::Open {
            autoclose,
            toggle_block,
        } = app.get::<DoorState>(door_entity)
        {
            assert_eq!(autoclose.finished(), false);
            assert_eq!(toggle_block.finished(), false);
        } else {
            panic!("Expected DoorState::Open");
        }
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
                .trigger_targets(UseCommand::new(player_entity), door_entity);
        }

        // then
        if let DoorState::Open {
            autoclose,
            toggle_block,
        } = app.get::<DoorState>(door_entity)
        {
            assert_eq!(autoclose.remaining_secs(), 10.0);
            assert_eq!(toggle_block.remaining_secs(), 1.0);
        } else {
            panic!("Expected DoorState::Open");
        }
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
            .trigger_targets(UseCommand::new(player_entity), door_entity);
        app.update();

        // then
        assert!(matches!(
            *app.get::<DoorState>(door_entity),
            DoorState::Open { .. }
        ));
        assert_eq!(app.get::<Contains>(player_entity).len(), 0);
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
            .trigger_targets(UseCommand::new(cmg_player_entity), door_entity);
        app.update();

        // then
        assert_eq!(app.get::<Contains>(cmg_player_entity).len(), 0);
        assert_eq!(app.get::<Contains>(vi_player_entity).len(), 1);
    }

    #[test]
    fn ec_player_hacks_cmg_door_without_hacking_tool() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R7);
        let door_entity = setup_door(&mut app, Faction::CMG, SecurityLevel::High);

        // when
        app.world_mut()
            .trigger_targets(UseCommand::new(player_entity), door_entity);
        app.update();

        // then
        let result = app.get::<DoorState>(door_entity);
        assert_eq!(*result, DoorState::Closed);
    }

    #[test]
    fn ec_player_hacks_cmg_door_with_hacking_tool() {
        // given
        let mut app = setup();
        let e_hacking_tool = setup_hacking_tool(&mut app);
        let e_player_ec = setup_player(&mut app, vec![e_hacking_tool], Faction::EC, Rank::R3);
        let e_door_cmg_low = setup_door(&mut app, Faction::CMG, SecurityLevel::Low);

        // when
        app.world_mut()
            .trigger_targets(UseCommand::new(e_player_ec), e_door_cmg_low);
        app.update();

        // then
        let result = app.get::<DoorState>(e_door_cmg_low);
        assert!(matches!(*result, DoorState::Open { .. }));
    }

    #[test]
    fn ec_player_hacks_vi_door_and_can_open_them_after_2_minutes() {
        // given
        let mut app = setup();
        let e_hacking_tool = setup_hacking_tool(&mut app);
        let e_player_ec = setup_player(&mut app, vec![e_hacking_tool], Faction::EC, Rank::R1);
        let e_door_vi_medium = setup_door(&mut app, Faction::VI, SecurityLevel::Medium);

        // when - EC player hacks VI door.
        app.world_mut()
            .trigger_targets(UseCommand::new(e_player_ec), e_door_vi_medium);
        app.update();
        assert!(
            matches!(
                *app.get::<DoorState>(e_door_vi_medium),
                DoorState::Open { .. }
            ),
            "Door is hacked and open."
        );

        // and when - EC player opens VI door after 2 minutes.
        app.update_after(Duration::from_secs_f32(2.0 * 60.0));

        assert_eq!(
            *app.get::<DoorState>(e_door_vi_medium),
            DoorState::Closed,
            "Door is autoclosed after some time."
        );

        app.world_mut()
            .trigger_targets(UseCommand::new(e_player_ec), e_door_vi_medium);

        // then
        assert!(
            matches!(
                *app.get::<DoorState>(e_door_vi_medium),
                DoorState::Open { .. }
            ),
            "Re-open hacked doors after they autoclosed."
        );
    }

    #[test]
    fn vi_player_hacks_ec_door_and_can_not_open_them_after_5_minutes() {
        // given
        let mut app = setup();
        let hacking_tool_entity = setup_hacking_tool(&mut app);
        let e_player_vi = setup_player(&mut app, vec![hacking_tool_entity], Faction::VI, Rank::R0);
        let e_door_ec = setup_door(&mut app, Faction::EC, SecurityLevel::Low);
        app.world_mut()
            .trigger_targets(UseCommand::new(e_player_vi), e_door_ec);
        app.update_after(Duration::from_secs_f32(DoorState::HACK_DURATION_SECS));

        // when
        app.world_mut()
            .trigger_targets(UseCommand::new(e_player_vi), e_door_ec);

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
            .trigger_targets(UseCommand::new(e_player_ec), e_door_ec);

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
            .trigger_targets(UseCommand::new(e_player_ec), e_door_cmg);
        app.update_after(Duration::from_secs_f32(10.0));
        let e_other_player_ec = setup_player(&mut app, vec![], Faction::EC, Rank::R0);

        // when
        app.world_mut()
            .trigger_targets(UseCommand::new(e_other_player_ec), e_door_cmg);

        // then
        assert!(matches!(
            *app.get::<DoorState>(e_door_cmg),
            DoorState::Open { .. }
        ));
    }

    fn setup() -> App {
        let mut app = App::new();
        app.init_time();
        app.add_event::<LootCommand>();
        app.add_event::<DoorHackCommand>();
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
            .spawn((Player, Inventory, PlayerFactionInfo { faction, rank }))
            .id();
        player_entity
    }

    fn setup_hacking_tool(app: &mut App) -> Entity {
        let item_entity = app.world_mut().spawn(HackingTool).id();
        item_entity
    }

    fn setup_door(app: &mut App, faction: Faction, security: SecurityLevel) -> Entity {
        let mut ownership_registry = OwnershipRegistry::default();
        ownership_registry.add_permanent(faction);
        let door_entity = app
            .world_mut()
            .spawn((Door, security, ownership_registry))
            .observe(on_use_command)
            .id();
        door_entity
    }
}
