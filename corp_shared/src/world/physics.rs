use avian3d::prelude::{PhysicsLayer, RigidBody};
use bevy::prelude::Component;
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

#[derive(Component, Default, Copy, Clone)]
pub enum MeshCollider {
    #[default]
    Static,
    Kinematic,
}

impl From<MeshCollider> for RigidBody {
    fn from(value: MeshCollider) -> Self {
        match value {
            MeshCollider::Static => RigidBody::Static,
            MeshCollider::Kinematic => RigidBody::Kinematic,
        }
    }
}
