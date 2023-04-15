use std::collections::HashMap;
use std::time::Duration;

use bevy::prelude::*;
use lazy_static::lazy_static;

use crate::faction::{ControlRegistry, ControlType, MemberOf, Rank};
use crate::interactive::{InteractionType, Interactive};
use crate::inventory::Inventory;
use crate::item::HackingTool;

const FIVE_MINUTES: f32 = 5.0 * 60.0;

lazy_static! {
    static ref REQUIRED_RANK_BY_DOOR_SECURITY: HashMap<Security, Rank> = {
        let mut map = HashMap::new();
        map.insert(Security::Low, Rank::R4);
        map.insert(Security::Medium, Rank::R5);
        map.insert(Security::High, Rank::R6);
        map
    };
}

#[derive(Default, Eq, PartialEq, Debug)]
pub enum DoorState {
    Open,
    #[default]
    Closed,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Security {
    Low,
    Medium,
    High,
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

    pub fn state(&self) -> &DoorState {
        &self.state
    }

    pub fn toggle(&mut self) {
        if self.toggle_cooldown.finished() {
            self.toggle_cooldown.reset();
            self.toggle_state();
        }
    }

    fn toggle_state(&mut self) {
        if self.state == DoorState::Open {
            self.state = DoorState::Closed;
        } else {
            self.state = DoorState::Open;
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

impl Interactive for Door {
    fn interaction_type(&self) -> InteractionType {
        InteractionType::Door
    }
}

pub fn door_cooldown_system(mut door_query: Query<&mut Door>, time: Res<Time>) {
    for mut door in &mut door_query {
        // If the door is currently open and the cooldown timer has expired, set the state to Closed
        if door.state == DoorState::Open && door.open_cooldown.tick(time.delta()).just_finished() {
            door.state = DoorState::Closed;
        }

        // If the door toggle cooldown timer has expired, allow the player to interact with the door again
        if !door.toggle_cooldown.finished() {
            door.toggle_cooldown.tick(time.delta());
        }
    }
}

pub struct DoorInteractionEvent {
    pub door_entity: Entity,
    pub interactor_entity: Entity,
}

pub struct DoorHackEvent {
    pub door_entity: Entity,
    pub interactor_entity: Entity,
}

pub fn door_interaction_event_system(
    mut door_interaction_event_reader: EventReader<DoorInteractionEvent>,
    mut door_hack_event_writer: EventWriter<DoorHackEvent>,
    interactor_query: Query<&MemberOf>,
    mut door_query: Query<(&mut Door, &ControlRegistry)>,
) {
    for event in door_interaction_event_reader.iter() {
        if let Ok(member_of) = interactor_query.get(event.interactor_entity) {
            if let Ok((mut door, control_registry)) = door_query.get_mut(event.door_entity) {
                if let Some(control_type) = control_registry.get_control_type(&member_of.faction) {
                    match control_type {
                        ControlType::Permanent(_) => {
                            if Some(&member_of.rank)
                                >= REQUIRED_RANK_BY_DOOR_SECURITY.get(&door.security())
                            {
                                door.toggle();
                            }
                        }
                        ControlType::Hacked(_, _) => {
                            door.toggle();
                        }
                    }
                } else {
                    door_hack_event_writer.send(DoorHackEvent {
                        door_entity: event.door_entity,
                        interactor_entity: event.interactor_entity,
                    });
                }
            }
        }
    }
}

pub fn door_hack_event_system(
    mut door_hack_event_reader: EventReader<DoorHackEvent>,
    mut door_query: Query<(&mut Door, &mut ControlRegistry)>,
    mut interactor_query: Query<(&mut Inventory, &MemberOf)>,
    hacking_tool_query: Query<&HackingTool>,
    mut commands: Commands,
) {
    for event in door_hack_event_reader.iter() {
        if let Ok((mut door, mut faction_ownership_registry)) =
            door_query.get_mut(event.door_entity)
        {
            if let Ok((mut inventory, member_of)) =
                interactor_query.get_mut(event.interactor_entity)
            {
                if let Some(hacking_tool_entity) = inventory
                    .items
                    .iter()
                    .find(|&&item_entity| hacking_tool_query.get(item_entity).is_ok())
                    .copied()
                {
                    inventory.remove_item(hacking_tool_entity);
                    commands.entity(hacking_tool_entity).despawn_recursive();
                    faction_ownership_registry.add_temporary(
                        member_of.faction,
                        Timer::new(Duration::from_secs_f32(FIVE_MINUTES), TimerMode::Once),
                    );
                    door.toggle();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::prelude::*;
    use bevy_trait_query::RegisterExt;

    use crate::door::{
        door_cooldown_system, door_hack_event_system, door_interaction_event_system, Door,
        DoorHackEvent, DoorInteractionEvent, DoorState, Security, FIVE_MINUTES,
    };
    use crate::faction::{
        process_temporary_faction_ownership_timers_system, ControlRegistry, Faction, MemberOf, Rank,
    };
    use crate::interactive::{interaction_system, Interactive, Interactor};
    use crate::inventory::Inventory;
    use crate::item::HackingToolBundle;
    use crate::player::Player;
    use crate::test_utils::TestUtils;

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
    }

    #[test]
    fn player_open_door() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R5);
        let door_entity = setup_door(&mut app, Faction::EC, Security::Low);

        // when
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
    }

    #[test]
    fn player_close_open_door() {
        // given
        let mut app = setup();
        let player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R4);
        let door_entity = setup_door(&mut app, Faction::EC, Security::Low);
        app.get_mut::<Door>(door_entity).state = DoorState::Open;

        // when
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
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
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update_after(Duration::from_secs_f32(3.0));
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
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
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update_after(Duration::from_secs_f32(0.5));
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update_after(Duration::from_secs_f32(0.1));

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
        assert!(!result.toggle_cooldown.finished());
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
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
        assert_eq!(app.get::<Inventory>(player_entity).items.len(), 0);
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
        app.get_mut::<Interactor>(cmg_player_entity)
            .interact(door_entity);
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
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
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
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
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
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update();
        app.update_after(Duration::from_secs_f32(2.0 * 60.0));

        // when
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
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
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update();
        app.update_after(Duration::from_secs_f32(FIVE_MINUTES));

        // when
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
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
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
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
        app.get_mut::<Interactor>(player_entity)
            .interact(door_entity);
        app.update();
        app.update_after(Duration::from_secs_f32(10.0));
        let another_player_entity = setup_player(&mut app, vec![], Faction::EC, Rank::R0);

        // when
        app.get_mut::<Interactor>(another_player_entity)
            .interact(door_entity);
        app.update();

        // then
        let result = app.get::<Door>(door_entity);
        assert_eq!(result.state, DoorState::Open);
    }

    fn setup() -> App {
        let mut app = App::new();
        app.init_time();
        app.add_event::<DoorInteractionEvent>();
        app.add_event::<DoorHackEvent>();
        app.register_component_as::<dyn Interactive, Door>();
        app.add_systems(
            (
                door_cooldown_system,
                process_temporary_faction_ownership_timers_system,
                interaction_system,
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
            .spawn((
                Player,
                Interactor::default(),
                Inventory::new(items),
                MemberOf { faction, rank },
            ))
            .id();
        player_entity
    }

    fn setup_hacking_tool(app: &mut App) -> Entity {
        let item_entity = app.world.spawn(HackingToolBundle::default()).id();
        item_entity
    }

    fn setup_door(app: &mut App, faction: Faction, security: Security) -> Entity {
        let mut registry = ControlRegistry::default();
        registry.add_permanent(faction);
        let door_entity = app.world.spawn((Door::new(security), registry)).id();
        door_entity
    }
}
