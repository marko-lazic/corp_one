use bevy::prelude::{Component, Entity, FromWorld, World, *};
use std::{ops::Deref, slice};

#[derive(
    Component,
    Clone,
    PartialEq,
    Eq,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    bevy::reflect::Reflect,
)]
#[reflect(Component, PartialEq, Debug, FromWorld, Clone, Serialize, Deserialize)]
#[relationship(relationship_target = Contains)]
#[doc(alias = "IsStored", alias = "Container")]
pub struct StoredIn(#[entities] pub Entity);

impl StoredIn {
    pub fn container(&self) -> Entity {
        self.0
    }
}

impl FromWorld for StoredIn {
    #[inline(always)]
    fn from_world(_world: &mut World) -> Self {
        StoredIn(Entity::PLACEHOLDER)
    }
}

/// All items held by this component (the inventory)
#[derive(Component, Debug)]
#[relationship_target(relationship = StoredIn)]
#[doc(alias = "IsContainer")]
pub struct Contains(Vec<Entity>);

impl<'a> IntoIterator for &'a Contains {
    type Item = <Self::IntoIter as Iterator>::Item;

    type IntoIter = slice::Iter<'a, Entity>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Deref for Contains {
    type Target = [Entity];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
