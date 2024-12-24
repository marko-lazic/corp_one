use crate::prelude::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use corp_shared::prelude::*;

pub fn components(entity: Entity, name: &str, commands: &mut EntityCommands) {
    match name {
        n if n.starts_with("VortexGate") => commands.insert((
            VortexGate,
            Sensor,
            Collider::cuboid(1.0, 1.0, 1.0),
            CollisionLayers::new([GameLayer::Sensor], [GameLayer::Player]),
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
                InteractionObjectType::DoorControl,
                StateScoped(GameState::Playing),
            ))
            .observe(|over: Trigger<Pointer<Over>>| {
                info!("Over barrier control: {:?}", over);
            }),

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
                InteractionObjectType::DoorControl,
                StateScoped(GameState::Playing),
            ))
            .observe(|over: Trigger<Pointer<Over>>| {
                info!("Over barrier control: {:?}", over);
            }),
        n if n.starts_with("EnergyNode1") => commands
            .insert((
                TerritoryNodeBundle {
                    territory_node: TerritoryNode,
                    node_type: TerritoryNodeType::EnergyNode,
                    security_level: SecurityLevel::Low,
                    ownership: OwnershipRegistry::new_permanent(Faction::EC),
                },
                InteractionObjectType::TerritoryNode,
                StateScoped(GameState::Playing),
            ))
            .observe(|over: Trigger<Pointer<Over>>| {
                info!("Over energy node: {:?}", over);
            })
            .observe(on_use_territory_node_event),
        n if n.starts_with("Plant Tree") => commands.insert(StateScoped(GameState::Playing)),
        _ => commands,
    };
}
