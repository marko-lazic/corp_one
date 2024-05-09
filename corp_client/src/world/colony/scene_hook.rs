use bevy::{ecs::system::EntityCommands, log::info, prelude::Entity};
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use corp_shared::{
    prelude::{ControlRegistry, Door, Faction, InteractionObjectType, TerritoryNodeType},
    world::{objects::territory::TerritoryNode, security::Security},
};

use crate::{
    state::Despawn,
    world::{
        colony::{
            barrier::{BarrierControl, BarrierField},
            vortex::{VortexGate, VortexNode},
        },
        physics,
    },
};

pub fn scene_hook_insert_components(entity: Entity, name: &str, commands: &mut EntityCommands) {
    match name {
        n if n.starts_with("VortexGate") => commands.insert((
            VortexGate,
            Sensor,
            Collider::cuboid(0.5, 1.0, 0.5),
            physics::CollideGroups::vortex_gate(),
            Despawn,
        )),
        n if n.starts_with("VortexNode") => {
            info!("Insert vortex node: {}", n);
            commands.insert((VortexNode, Despawn))
        }
        n if n.starts_with("BarrierField1") => {
            let mut registry = ControlRegistry::default();
            registry.add_permanent(Faction::EC);
            commands.insert((
                BarrierField::new(entity, "B1"),
                Door::new(Security::Low),
                registry,
            ))
        }
        n if n.starts_with("BarrierControl11") | n.starts_with("BarrierControl12") => commands
            .insert((
                BarrierControl::new("B1"),
                PickableBundle::default(),
                InteractionObjectType::DoorControl,
                Despawn,
            )),

        n if n.starts_with("BarrierField2") => {
            let mut registry = ControlRegistry::default();
            registry.add_permanent(Faction::EC);
            commands.insert((
                BarrierField::new(entity, "B2"),
                Door::new(Security::Low),
                registry,
            ))
        }
        n if n.starts_with("BarrierControl21") | n.starts_with("BarrierControl22") => commands
            .insert((
                BarrierControl::new("B2"),
                PickableBundle::default(),
                InteractionObjectType::DoorControl,
                Despawn,
            )),
        n if n.starts_with("EnergyNode1") => commands.insert((
            TerritoryNode {
                r#type: TerritoryNodeType::EnergyNode,
                security: Security::Low,
            },
            PickableBundle::default(),
            InteractionObjectType::TerritoryNode,
            Despawn,
        )),
        n if n.starts_with("Plant Tree") => commands.insert(Despawn),
        _ => commands,
    };
}
