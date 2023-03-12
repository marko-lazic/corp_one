use bevy::ecs::system::EntityCommands;
use bevy::log::info;
use bevy::prelude::Component;
use bevy_mod_picking::PickableBundle;
use bevy_rapier3d::geometry::{Collider, Sensor};

use crate::world::colony::barrier::{BarrierControl, BarrierField};
use crate::world::colony::vortex::{VortexGate, VortexNode};
use crate::world::physics;

#[derive(Component)]
pub struct UnpickableGLTF;

pub(crate) fn scene_hook_insert_components(name: &str, commands: &mut EntityCommands) {
    match name {
        n if n.starts_with("VortexGate") => {
            info!("Insert vortex gate");
            commands.insert((
                VortexGate,
                Sensor,
                Collider::cuboid(0.5, 1.0, 0.5),
                physics::CollideGroups::vortex_gate(),
            ))
        }
        n if n.starts_with("VortexNode") => {
            info!("Insert vortex node");
            commands.insert(VortexNode)
        }
        "BarrierField1" => {
            info!("Insert barrier field B1");
            commands.insert(BarrierField::new("B1"))
        }
        "BarrierControl11" | "BarrierControl12" => {
            info!("Insert barrier control B1");
            commands.insert((BarrierControl::new("B1"), PickableBundle::default()))
        }

        "BarrierField2" => {
            info!("Insert barrier field B2");
            commands.insert(BarrierField::new("B2"))
        }
        "BarrierControl21" | "BarrierControl22" => {
            info!("Insert barrier control B2");
            commands.insert((BarrierControl::new("B2"), PickableBundle::default()))
        }
        "Plant Tree" => {
            info!("Plant tree");
            commands.insert(UnpickableGLTF)
        }
        _ => commands,
    };
}
