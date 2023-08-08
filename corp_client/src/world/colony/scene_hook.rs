use bevy::{ecs::system::EntityCommands, log::info};
use bevy_mod_picking::{backends::rapier::RapierPickTarget, prelude::*};
use bevy_rapier3d::prelude::*;

use corp_shared::prelude::{ControlRegistry, Door, Faction, Security};

use crate::{
    state::Despawn,
    world::{
        colony::{
            barrier::{BarrierControl, BarrierField, BarrierPickingEvent},
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
            info!("Insert vortex node");
            commands.insert((VortexNode, Despawn))
        }
        "BarrierField1" => {
            let mut registry = ControlRegistry::default();
            registry.add_permanent(Faction::EC);
            commands.insert((BarrierField::new("B1"), Door::new(Security::Low), registry))
        }
        "BarrierControl11" | "BarrierControl12" => commands.insert((
            BarrierControl::new("B1"),
            PickableBundle::default(),
            RapierPickTarget::default(),
            On::<Pointer<Over>>::send_event::<BarrierPickingEvent>(),
            On::<Pointer<Out>>::send_event::<BarrierPickingEvent>(),
            Despawn,
        )),

        "BarrierField2" => {
            let mut registry = ControlRegistry::default();
            registry.add_permanent(Faction::EC);
            commands.insert((BarrierField::new("B2"), Door::new(Security::Low), registry))
        }
        "BarrierControl21" | "BarrierControl22" => commands.insert((
            BarrierControl::new("B2"),
            PickableBundle::default(),
            RapierPickTarget::default(),
            On::<Pointer<Over>>::send_event::<BarrierPickingEvent>(),
            On::<Pointer<Out>>::send_event::<BarrierPickingEvent>(),
            Despawn,
        )),
        "Plant Tree" => commands.insert(Despawn),
        _ => commands,
    };
}
