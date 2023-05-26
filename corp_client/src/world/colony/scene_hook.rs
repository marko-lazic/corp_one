use bevy::ecs::system::EntityCommands;
use bevy::log::info;
use bevy_mod_picking::backends::rapier::RapierPickTarget;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::{Collider, Sensor};

use crate::state::Despawn;
use crate::world::colony::barrier::{BarrierControl, BarrierField, BarrierPickingEvent};
use crate::world::colony::vortex::{VortexGate, VortexNode};
use crate::world::physics;

pub(crate) fn scene_hook_insert_components(name: &str, commands: &mut EntityCommands) {
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
        "BarrierField1" => commands.insert(BarrierField::new("B1")),
        "BarrierControl11" | "BarrierControl12" => commands.insert((
            BarrierControl::new("B1"),
            RigidBody::Fixed,
            Collider::cuboid(0.5, 0.5, 0.5),
            PickableBundle::default(),
            RapierPickTarget::default(),
            OnPointer::<Over>::send_event::<BarrierPickingEvent>(),
            OnPointer::<Out>::send_event::<BarrierPickingEvent>(),
            Despawn,
        )),

        "BarrierField2" => commands.insert(BarrierField::new("B2")),
        "BarrierControl21" | "BarrierControl22" => commands.insert((
            BarrierControl::new("B2"),
            RigidBody::Fixed,
            Collider::cuboid(0.5, 0.5, 0.5),
            PickableBundle::default(),
            RapierPickTarget::default(),
            OnPointer::<Over>::send_event::<BarrierPickingEvent>(),
            OnPointer::<Out>::send_event::<BarrierPickingEvent>(),
            Despawn,
        )),
        "Plant Tree" => commands.insert(Despawn),
        _ => commands,
    };
}
