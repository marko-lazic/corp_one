pub use crate::world::security::*;
pub use backpack::*;
use bevy::prelude::*;
pub use door::*;
pub use plant::*;
pub use prop::*;
pub use vortex::*;

pub mod backpack;
pub mod door;
pub mod plant;
pub mod prop;
pub mod vortex;

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EnergyNode>()
            .register_type::<Tree>()
            .register_type::<Wall>()
            .register_type::<Ground>()
            .register_type::<Floor>()
            .register_type::<VortexGate>()
            .register_type::<VortexNode>()
            .register_type::<DoorId>()
            .register_type::<Door>()
            .register_type::<DoorTerminal>()
            .add_systems(
                FixedUpdate,
                (
                    despawn_empty_backpack_system,
                    attach_energy_node_observer,
                    attach_door_observer,
                    attach_door_terminal_observer,
                ),
            );
    }
}

fn attach_energy_node_observer(mut commands: Commands, query: Query<Entity, Added<EnergyNode>>) {
    for e in &query {
        commands.entity(e).observe(on_use_territory_node_event);
    }
}

fn attach_door_observer(mut commands: Commands, query: Query<Entity, Added<Door>>) {
    for e in &query {
        commands
            .entity(e)
            .observe(on_use_door_event)
            .observe(on_use_door_hack_event);
    }
}

fn attach_door_terminal_observer(
    mut commands: Commands,
    query: Query<Entity, Added<DoorTerminal>>,
) {
    for e in &query {
        commands.entity(e).observe(on_use_door_terminal);
    }
}
