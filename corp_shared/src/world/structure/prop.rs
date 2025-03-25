use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(
    Name(|| Name::new("Tree")),
    MeshCollider
)]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState>(|| StateScoped(GameState::Playing)))
)]
pub struct Tree;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(
    Name(|| Name::new("Wall")),
    MeshCollider
)]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState>(|| StateScoped(GameState::Playing)))
)]
pub struct Wall;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(
    Name(|| Name::new("Ground")),
    MeshCollider
)]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState>(|| StateScoped(GameState::Playing)))
)]
pub struct Ground;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(
    Name(|| Name::new("Floor")),
    MeshCollider
)]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState>(|| StateScoped(GameState::Playing)))
)]
pub struct Floor;
