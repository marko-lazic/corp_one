use bevy::{ecs::system::EntityCommands, log::info};
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
            barrier::{BarrierControl, BarrierField, BarrierPickingEvent},
            object_interaction::PickingEvent,
            territory::TerritoryNodePickingEvent,
            vortex::{VortexGate, VortexNode},
        },
        physics,
    },
};

pub fn scene_hook_insert_components(name: &str, commands: &mut EntityCommands) {
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
        "BarrierField1" => {
            let mut registry = ControlRegistry::default();
            registry.add_permanent(Faction::EC);
            commands.insert((
                BarrierField::new("B1"),
                Door::new(Security::Low),
                registry,
                InteractionObjectType::Door,
            ))
        }
        "BarrierControl11" | "BarrierControl12" => commands.insert((
            BarrierControl::new("B1"),
            PickableBundle::default(),
            On::<Pointer<Over>>::send_event::<PickingEvent<BarrierPickingEvent>>(),
            On::<Pointer<Out>>::send_event::<PickingEvent<BarrierPickingEvent>>(),
            Despawn,
        )),

        "BarrierField2" => {
            let mut registry = ControlRegistry::default();
            registry.add_permanent(Faction::EC);
            commands.insert((
                BarrierField::new("B2"),
                Door::new(Security::Low),
                InteractionObjectType::Door,
                registry,
            ))
        }
        "BarrierControl21" | "BarrierControl22" => commands.insert((
            BarrierControl::new("B2"),
            PickableBundle::default(),
            On::<Pointer<Over>>::send_event::<PickingEvent<BarrierPickingEvent>>(),
            On::<Pointer<Out>>::send_event::<PickingEvent<BarrierPickingEvent>>(),
            Despawn,
        )),
        "EnergyNode1" => commands.insert((
            TerritoryNode {
                r#type: TerritoryNodeType::EnergyNode,
                security: Security::Low,
            },
            PickableBundle::default(),
            On::<Pointer<Over>>::send_event::<PickingEvent<TerritoryNodePickingEvent>>(),
            On::<Pointer<Out>>::send_event::<PickingEvent<TerritoryNodePickingEvent>>(),
            InteractionObjectType::TerritoryNode,
            Despawn,
        )),
        "Plant Tree" => commands.insert(Despawn),
        _ => commands,
    };
}
