use bevy::{
    ecs::system::EntityCommands,
    log::info,
    prelude::{Entity, StateScoped},
    utils::default,
};
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    state::GameState,
    world::{
        colony::{
            barrier::{BarrierControl, BarrierField},
            vortex::{VortexGate, VortexNode},
        },
        physics,
    },
};
use corp_shared::{
    prelude::{
        on_use_door_event, on_use_door_hack_event, on_use_territory_node_event, DoorBundle,
        Faction, InteractionObjectType, OwnershipRegistry, TerritoryNode, TerritoryNodeType,
    },
    world::{objects::territory::TerritoryNodeBundle, security::SecurityLevel},
};

pub fn scene_hook_insert_components(entity: Entity, name: &str, commands: &mut EntityCommands) {
    match name {
        n if n.starts_with("VortexGate") => commands.insert((
            VortexGate,
            Sensor,
            Collider::cuboid(0.5, 1.0, 0.5),
            physics::CollideGroups::vortex_gate(),
            StateScoped(GameState::Playing),
        )),
        n if n.starts_with("VortexNode") => {
            info!("Insert vortex node: {}", n);
            commands.insert((VortexNode, StateScoped(GameState::Playing)))
        }
        n if n.starts_with("BarrierField1") => commands
            .insert((
                BarrierField::new(entity, "B1"),
                DoorBundle {
                    security_level: SecurityLevel::Low,
                    ownership: OwnershipRegistry::new_permanent(Faction::EC),
                    ..default()
                },
            ))
            .observe(on_use_door_event)
            .observe(on_use_door_hack_event),
        n if n.starts_with("BarrierControl11") | n.starts_with("BarrierControl12") => commands
            .insert((
                BarrierControl::new("B1"),
                PickableBundle::default(),
                InteractionObjectType::DoorControl,
                StateScoped(GameState::Playing),
            )),

        n if n.starts_with("BarrierField2") => commands
            .insert((
                BarrierField::new(entity, "B2"),
                DoorBundle {
                    security_level: SecurityLevel::Low,
                    ownership: OwnershipRegistry::new_permanent(Faction::EC),
                    ..default()
                },
            ))
            .observe(on_use_door_event)
            .observe(on_use_door_hack_event),
        n if n.starts_with("BarrierControl21") | n.starts_with("BarrierControl22") => commands
            .insert((
                BarrierControl::new("B2"),
                PickableBundle::default(),
                InteractionObjectType::DoorControl,
                StateScoped(GameState::Playing),
            )),
        n if n.starts_with("EnergyNode1") => commands
            .insert((
                TerritoryNodeBundle {
                    territory_node: TerritoryNode,
                    node_type: TerritoryNodeType::EnergyNode,
                    security_level: SecurityLevel::Low,
                    ownership: OwnershipRegistry::new_permanent(Faction::EC),
                },
                PickableBundle::default(),
                InteractionObjectType::TerritoryNode,
                StateScoped(GameState::Playing),
            ))
            .observe(on_use_territory_node_event),
        n if n.starts_with("Plant Tree") => commands.insert(StateScoped(GameState::Playing)),
        _ => commands,
    };
}
