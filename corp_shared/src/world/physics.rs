use avian3d::{collision::CollisionLayers, prelude::*};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(PhysicsLayer, Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Zone,
    Sensor,
    Structure,
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
#[require(CollisionLayers(structure_collision_layers), RigidBody(|| RigidBody::Static))]
pub struct Structure;

pub fn structure_collision_layers() -> CollisionLayers {
    CollisionLayers::new([GameLayer::Structure], [GameLayer::Player])
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
#[require(RigidBody(|| RigidBody::Kinematic))]
pub struct DynamicStructure;
