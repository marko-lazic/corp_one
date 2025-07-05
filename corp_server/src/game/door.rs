use bevy::prelude::*;
use bevy_replicon::prelude::*;
use corp_shared::prelude::*;
use std::time::Duration;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_hack_door_command);
    }
}

fn on_hack_door_command(
    trigger: Trigger<FromClient<DoorHackCommand>>,
    mut commands: Commands,
    contains_query: Query<&Contains, With<Inventory>>,
    hacking_tools: Query<&HackingTool>,
    mut doors: Query<(&mut DoorState, &mut OwnershipRegistry)>,
    faction_info_query: Query<&PlayerFactionInfo, With<Player>>,
) {
    let door_e = trigger.target();
    let from_client = trigger.event();
    let client_e = from_client.client_entity;
    if let Ok(inventory_content) = contains_query.get(client_e) {
        if let Some(hacking_tool_e) = inventory_content
            .into_iter()
            .find(|item| hacking_tools.get(**item).is_ok())
        {
            if let Ok((mut door_state, mut door_owners)) = doors.get_mut(door_e) {
                if let Ok(faction_info) = faction_info_query.get(client_e) {
                    commands.entity(*hacking_tool_e).despawn();
                    door_owners.add(Ownership::Hacked(
                        faction_info.faction,
                        Timer::new(
                            Duration::from_secs_f32(DoorState::HACK_DURATION_SECS),
                            TimerMode::Once,
                        ),
                    ));
                    door_state.toggle();
                    let to_clients = ToClients {
                        mode: SendMode::Broadcast,
                        event: DoorHackedEvent::Successful,
                    };
                    commands.server_trigger_targets(to_clients, door_e);
                }
            }
        }
    } else {
        warn!(
            "Client {} tried to hack a door without a inventory",
            client_e
        );
        let to_clients = ToClients {
            mode: SendMode::Broadcast,
            event: DoorHackedEvent::Failure,
        };
        commands.server_trigger_targets(to_clients, door_e);
    }
}
