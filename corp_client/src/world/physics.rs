use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    }
}

pub struct CollideGroups;

impl CollideGroups {
    const PLAYER: Group = Group::GROUP_1;
    const ZONE: Group = Group::GROUP_2;
    const VORTEX_GATE: Group = Group::GROUP_3;

    pub fn player() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::PLAYER,
            filters: Self::VORTEX_GATE | Self::ZONE,
        }
    }

    pub fn zone() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::ZONE,
            filters: Self::PLAYER,
        }
    }

    pub fn vortex_gate() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::VORTEX_GATE,
            filters: Self::PLAYER,
        }
    }
}
