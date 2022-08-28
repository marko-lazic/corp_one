use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use bitflags::bitflags;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    }
}

bitflags! {
    pub struct CollideGroups: u32 {
       const PLAYER = (1 << 30) - 1; // 1 << 0 clusterjunk
       const ZONE = 1 << 1;
       const VORTEX_GATE  = 1 << 2;
    }
}

impl CollideGroups {
    pub fn player() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::PLAYER.bits(),
            filters: Self::PLAYER.bits() | Self::VORTEX_GATE.bits(),
        }
    }

    pub fn zone() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::ZONE.bits(),
            filters: Self::ZONE.bits() | Self::VORTEX_GATE.bits(),
        }
    }

    pub fn vortex_gate() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::VORTEX_GATE.bits(),
            filters: Self::all().bits(),
        }
    }
}
