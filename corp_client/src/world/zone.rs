use bam3d::{Aabb3, Discrete};
use bevy::core::prelude::Timer;
use bevy::prelude::*;
use glam::Vec3;

use bevy_mod_bounding::aabb;
use corp_shared::components::Player;
use corp_shared::events::DealDamageEvent;

pub struct DamageZone {
    damage: u32,
}

impl DamageZone {
    pub fn new(damage: u32) -> Self {
        DamageZone { damage }
    }
}

pub struct ZonePlugin;

impl ZonePlugin {
    fn check_player_in_zone(
        mut players: Query<(&GlobalTransform, &aabb::Aabb), With<Player>>,
        bounds: Query<(&GlobalTransform, &aabb::Aabb, &DamageZone)>,
        mut ev_damage: EventWriter<DealDamageEvent>,
        time: Res<Time>,
        mut timer: ResMut<DamageTimer>,
    ) {
        for (player_global, player_bounding) in players.iter_mut() {
            let player_vertices = player_bounding.vertices(*player_global);

            let player_aabb = Self::convert_to_aabb3(player_vertices);

            for (zone_global, zone_bounding, damage_zone) in bounds.iter() {
                let zone_vertices = zone_bounding.vertices(*zone_global);
                let zone_aabb = Self::convert_to_aabb3(zone_vertices);

                if zone_aabb.intersects(&player_aabb) {
                    if timer.timer.tick(time.delta()).just_finished() {
                        ev_damage.send(DealDamageEvent(damage_zone.damage));
                    }
                }
            }
        }
    }

    fn convert_to_aabb3(vertices: [bevy::math::Vec3; 8]) -> Aabb3 {
        Aabb3::new(
            Vec3::new(vertices[0].x, vertices[0].y, vertices[0].z),
            Vec3::new(vertices[6].x, vertices[6].y, vertices[6].z),
        )
    }
}

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(DamageTimer::default());
        app.add_system(Self::check_player_in_zone.system());
    }
}

struct DamageTimer {
    timer: Timer,
}

impl Default for DamageTimer {
    fn default() -> Self {
        DamageTimer {
            timer: Timer::from_seconds(0.5, true),
        }
    }
}
