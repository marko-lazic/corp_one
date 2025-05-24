use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Name::new("Tree"), Structure)]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState> = StateScoped(GameState::Playing))
)]
pub struct Tree;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Name::new("Wall"), Structure)]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState> = StateScoped(GameState::Playing))
)]
pub struct Wall;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Name::new("Ground"), Structure)]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState> = StateScoped(GameState::Playing))
)]
pub struct Ground;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Name::new("Floor"), Structure)]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState> = StateScoped(GameState::Playing))
)]
pub struct Floor;
