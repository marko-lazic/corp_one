use bevy::prelude::*;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Label {
    Input,
    Movement,
}
