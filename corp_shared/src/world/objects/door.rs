use std::time::Duration;

use bevy::prelude::*;

use crate::prelude::*;

pub const HACK_DURATION_FIVE_MIN: f32 = 5.0 * 60.0;

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone)]
pub enum DoorState {
    Open,
    #[default]
    Closed,
}

#[derive(Component)]
pub struct Door {
    state: DoorState,
    security: Security,
    open_cooldown: Timer,
    toggle_cooldown: Timer,
}

impl Door {
    const OPEN_TIME: f32 = 10.0;
    const TOGGLE_TIME: f32 = 1.0;

    pub fn new(security: Security) -> Self {
        Self {
            security,
            ..Default::default()
        }
    }

    pub fn state(&self) -> DoorState {
        self.state
    }

    pub fn toggle(&mut self) {
        if self.toggle_cooldown.finished() {
            self.toggle_cooldown.reset();
            self.open_cooldown.reset();
            if self.state == DoorState::Open {
                self.state = DoorState::Closed;
            } else {
                self.state = DoorState::Open;
            }
        }
    }

    pub fn security(&self) -> &Security {
        &self.security
    }
}

impl Default for Door {
    fn default() -> Self {
        let mut toggle_cooldown = Timer::from_seconds(Self::TOGGLE_TIME, TimerMode::Once);
        toggle_cooldown.tick(Duration::from_secs_f32(Self::TOGGLE_TIME));
        Self {
            state: DoorState::Closed,
            security: Security::Low,
            open_cooldown: Timer::from_seconds(Self::OPEN_TIME, TimerMode::Once),
            toggle_cooldown,
        }
    }
}

pub struct UseDoorEvent;

#[derive(Event)]
pub struct DoorHackEvent {
    pub door_entity: Entity,
    pub interactor_entity: Entity,
}

#[derive(Event, Debug)]
pub struct DoorStateEvent(Entity, DoorState);

impl DoorStateEvent {
    pub fn entity(&self) -> Entity {
        self.0
    }

    pub fn state(&self) -> DoorState {
        self.1
    }
}

pub fn door_cooldown_system(
    mut ev_door_state: EventWriter<DoorStateEvent>,
    mut q_entity_door: Query<(Entity, &mut Door)>,
    r_time: Res<Time>,
) {
    for (entity, mut door) in &mut q_entity_door {
        // If the door is currently open and the cooldown timer has expired, set the state to Closed
        if door.state == DoorState::Open && door.open_cooldown.tick(r_time.delta()).just_finished()
        {
            door.state = DoorState::Closed;
            ev_door_state.send(DoorStateEvent(entity, DoorState::Closed));
            door.open_cooldown.reset();
        }

        // If the door toggle cooldown timer has expired, allow the player to interact with the door again
        if !door.toggle_cooldown.finished() {
            door.toggle_cooldown.tick(r_time.delta());
        }
    }
}

pub fn door_interaction_event_system(
    mut ev_door_interaction: EventReader<InteractionEvent<UseDoorEvent>>,
    mut ev_door_hack: EventWriter<DoorHackEvent>,
    mut ev_door_state: EventWriter<DoorStateEvent>,
    q_member_of: Query<&MemberOf>,
    mut q_door_control_registry: Query<(&mut Door, &ControlRegistry)>,
) {
    for event in ev_door_interaction.read() {
        if let Ok(member_of) = q_member_of.get(event.interactor) {
            if let Ok((mut door, control_registry)) = q_door_control_registry.get_mut(event.target)
            {
                if let Some(control_type) = control_registry.get_control_type(&member_of.faction) {
                    match control_type {
                        ControlType::Permanent(_) => {
                            if Some(&member_of.rank)
                                >= REQUIRED_RANK_BY_SECURITY.get(door.security())
                            {
                                door.toggle();
                                ev_door_state.send(DoorStateEvent(event.target, door.state()));
                            }
                        }
                        ControlType::Hacked(_, _) => {
                            door.toggle();
                            ev_door_state.send(DoorStateEvent(event.target, door.state()));
                        }
                    }
                } else {
                    ev_door_hack.send(DoorHackEvent {
                        door_entity: event.target,
                        interactor_entity: event.interactor,
                    });
                }
            }
        }
    }
}

pub fn door_hack_event_system(
    mut commands: Commands,
    mut ev_door_hack: EventReader<DoorHackEvent>,
    mut ev_door_state: EventWriter<DoorStateEvent>,
    mut q_door_control_registry: Query<(&mut Door, &mut ControlRegistry)>,
    mut q_player_inventory_member_of: Query<(&mut Inventory, &MemberOf), With<Player>>,
    q_hacking_tool: Query<&HackingTool>,
) {
    for event in ev_door_hack.read() {
        if let Ok((mut door, mut faction_ownership_registry)) =
            q_door_control_registry.get_mut(event.door_entity)
        {
            if let Ok((mut inventory, member_of)) =
                q_player_inventory_member_of.get_mut(event.interactor_entity)
            {
                if let Some(hacking_tool_entity) = inventory
                    .items
                    .iter()
                    .find(|&&item_entity| q_hacking_tool.get(item_entity).is_ok())
                    .copied()
                {
                    inventory.remove_item(hacking_tool_entity);
                    commands.entity(hacking_tool_entity).despawn_recursive();
                    faction_ownership_registry.add_temporary(
                        member_of.faction,
                        Timer::new(
                            Duration::from_secs_f32(HACK_DURATION_FIVE_MIN),
                            TimerMode::Once,
                        ),
                    );
                    door.toggle();
                    ev_door_state.send(DoorStateEvent(event.door_entity, door.state()));
                }
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
        let door_entity = setup_door(&mut app, Faction::EC, Security::Low);
        setup_player(&mut app, vec![], Faction::EC, Rank::R0);

        // when
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Closed);
    }

    #[test]
    fn open_door_closed_after_10s() {
        // given
        let mut app = setup();
        setup_player(&mut app, vec![], Faction::EC, Rank::R0);
        let door_entity = setup_door(&mut app, Faction::EC, Security::Low);
        app.get_mut::<Door>(door_entity).state = DoorState::Open;

        // when
        app.update_after(Duration::from_secs_f32(10.0));

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Closed);
        assert!(!result.open_cooldown.finished());
        assert_eq!(result.open_cooldown.remaining_secs(), 10.0);
    }

    #[test]
    fn player_open_door() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R5);
        let door_entity = setup_door(&mut app, Faction::EC, Security::Low);

        // when
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
    }

    #[test]
    fn two_doors_one_open_other_closed() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R5);
        let door_entity_1 = setup_door(&mut app, Faction::EC, Security::Low);
        let door_entity_2 = setup_door(&mut app, Faction::EC, Security::Low);

        // when
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity_1,
            UseDoorEvent,
        ));
        app.update();

        // then
        let result_1 = app.get::<Door>(door_entity_1);
        assert_eq!(result_1.state, DoorState::Open);
        let result_2 = app.get::<Door>(door_entity_2);
        assert_eq!(result_2.state, DoorState::Closed);
    }

    #[test]
    fn player_close_open_door() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R4);
        let door_entity = setup_door(&mut app, Faction::EC, Security::Low);
        app.get_mut::<Door>(door_entity).state = DoorState::Open;

        // when
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Closed);
    }

    #[test]
    fn player_open_door_wait_3_seconds_and_close_door() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R0);
        let door_entity = setup_door(&mut app, Faction::EC, Security::Low);

        // when
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update_after(Duration::from_secs_f32(3.0));
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Closed);
    }

    #[test]
    fn player_open_door_two_times_before_toggle_cooldown_finished() {
        // given
        let mut app = setup();
        let door_entity = setup_door(&mut app, Faction::EC, Security::High);
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R6);

        // when
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update_after(Duration::from_secs_f32(0.5));
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update_after(Duration::from_secs_f32(0.1));

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
        assert!(!result.toggle_cooldown.finished());
    }

    #[test]
    fn player_open_door_two_times_open_cooldown_resets() {
        // given
        let mut app = setup();
        let door_entity = setup_door(&mut app, Faction::EC, Security::High);
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R6);

        // when
        for _ in 0..3 {
            app.world().send_event(InteractionEvent::new(
                player_entity,
                door_entity,
                UseDoorEvent,
            ));
            app.update_after(Duration::from_secs_f32(2.0));
        }

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
        assert_eq!(result.open_cooldown.remaining_secs(), 10.0);
    }

    #[test]
    fn cmg_player_hacks_ec_door_and_lose_hacking_tool() {
        // given
        let mut app = setup();
        let hacking_tool_entity = setup_hacking_tool(&mut app);
        let player_entity =
            setup_player(&mut app, vec![hacking_tool_entity], Faction::CMG, Rank::R0);
        let door_entity = setup_door(&mut app, Faction::EC, Security::Low);

        // when
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
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
        let door_entity = setup_door(&mut app, Faction::EC, Security::Low);

        // when
        app.world().send_event(InteractionEvent::new(
            cmg_player_entity,
            door_entity,
            UseDoorEvent,
        ));
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
        let door_entity = setup_door(&mut app, Faction::CMG, Security::High);

        // when
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Closed);
    }

    #[test]
    fn ec_player_hacks_cmg_door_with_hacking_tool() {
        // given
        let mut app = setup();
        let hacking_tool_entity = setup_hacking_tool(&mut app);
        let player_entity =
            setup_player(&mut app, vec![hacking_tool_entity], Faction::EC, Rank::R3);
        let door_entity = setup_door(&mut app, Faction::CMG, Security::Low);

        // when
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
    }

    #[test]
    fn ec_player_hacks_vi_door_and_can_open_them_after_2_minutes() {
        // given
        let mut app = setup();
        let hacking_tool_entity = setup_hacking_tool(&mut app);
        let player_entity =
            setup_player(&mut app, vec![hacking_tool_entity], Faction::EC, Rank::R1);
        let door_entity = setup_door(&mut app, Faction::VI, Security::Medium);
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update();
        app.update_after(Duration::from_secs_f32(2.0 * 60.0));

        // when
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
    }

    #[test]
    fn vi_player_hacks_ec_door_and_can_not_open_them_after_5_minutes() {
        // given
        let mut app = setup();
        let hacking_tool_entity = setup_hacking_tool(&mut app);
        let player_entity =
            setup_player(&mut app, vec![hacking_tool_entity], Faction::VI, Rank::R0);
        let door_entity = setup_door(&mut app, Faction::EC, Security::Low);
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update();
        app.update_after(Duration::from_secs_f32(HACK_DURATION_FIVE_MIN));

        // when
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Closed);
    }

    #[test]
    fn ec_player_r3_can_not_open_ec_door_with_security_low() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R3);
        let door_entity = setup_door(&mut app, Faction::EC, Security::Low);

        // when
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Closed);
    }

    #[test]
    fn same_faction_player_can_open_hacked_door() {
        // given
        let mut app = setup();
        let hacking_tool_entity = setup_hacking_tool(&mut app);
        let player_entity =
            setup_player(&mut app, vec![hacking_tool_entity], Faction::EC, Rank::R2);
        let door_entity = setup_door(&mut app, Faction::CMG, Security::Low);
        app.world().send_event(InteractionEvent::new(
            player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update();
        app.update_after(Duration::from_secs_f32(10.0));
        let another_player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R0);

        // when
        app.world().send_event(InteractionEvent::new(
            another_player_entity,
            door_entity,
            UseDoorEvent,
        ));
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
    }

    fn setup() -> App {
        let mut app = App::new();
        app.init_time();
        app.add_event::<InteractionEvent<UseDoorEvent>>();
        app.add_event::<BackpackInteractionEvent>();
        app.add_event::<DoorHackEvent>();
        app.add_event::<DoorStateEvent>();
        app.add_systems(
            Update,
            (
                door_cooldown_system,
                process_temporary_faction_ownership_timers_system,
                door_interaction_event_system,
                door_hack_event_system,
            )
                .chain(),
        );
        app
    }

    fn setup_player(app: &mut App, items: Vec<Entity>, faction: Faction, rank: Rank) -> Entity {
        let player_entity = app
            .world
            .spawn((Player, Inventory::new(items), MemberOf { faction, rank }))
            .id();
        player_entity
    }

    fn setup_hacking_tool(app: &mut App) -> Entity {
        let item_entity = app.world().spawn(HackingToolBundle::default()).id();
        item_entity
    }

    fn setup_door(app: &mut App, faction: Faction, security: Security) -> Entity {
        let mut registry = ControlRegistry::default();
        registry.add_permanent(faction);
        let door_entity = app.world().spawn((Door::new(security), registry)).id();
        door_entity
    }
}
