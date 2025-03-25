use crate::prelude::*;
use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
#[require(
    Name(|| Name::new("Vortex Node")),
)]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState>(|| StateScoped(GameState::Playing)))
)]
pub struct VortexNode;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
#[require(
    Name(|| Name::new("Vortex Gate")),
    Sensor,
    Collider(vortex_gate_collider),
    CollisionLayers(vortex_gate_collision_layers)
)]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState>(|| StateScoped(GameState::Playing)))
)]
pub struct VortexGate;

fn vortex_gate_collider() -> Collider {
    Collider::cuboid(1.0, 1.0, 1.0)
}

fn vortex_gate_collision_layers() -> CollisionLayers {
    CollisionLayers::new([GameLayer::Sensor], [GameLayer::Player])
}
