use avian3d::prelude::PhysicsLayer;
use serde::{Deserialize, Serialize};

#[derive(PhysicsLayer, Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Zone,
    Sensor,
    Fixed,
}
