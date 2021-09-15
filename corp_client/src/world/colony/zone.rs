use bam3d::{Aabb3, Discrete};
use bevy::core::prelude::Timer;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_mod_bounding::aabb;
use glam::Vec3;
use serde::Deserialize;

use corp_shared::prelude::*;

use crate::constants::state::GameState;
use crate::constants::tick;
use crate::world::colony::vortex::VortexGateEvent;
use crate::Game;

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum ZoneType {
    Heal(f64),
    Damage(f64),
    VortexGate,
    Unknown,
}

impl Default for ZoneType {
    fn default() -> Self {
        Self::Unknown
    }
}

pub struct Zone {
    zone_type: ZoneType,
}

impl Zone {
    pub fn new(zone_type: ZoneType) -> Self {
        Zone { zone_type }
    }
}

pub struct ZonePlugin;

impl ZonePlugin {
    fn player_in_zone(
        mut players: Query<(&GlobalTransform, &aabb::Aabb), With<Player>>,
        bounds: Query<(&GlobalTransform, &aabb::Aabb, &Zone)>,
        mut ev_zone: EventWriter<ZoneEvent>,
        time: Res<Time>,
        mut timer: ResMut<DamageTimer>,
    ) {
        for (player_global, player_bounding) in players.iter_mut() {
            let player_vertices = player_bounding.vertices(*player_global);

            let player_aabb = Self::convert_to_aabb3(player_vertices);

            for (zone_global, zone_bounding, zone) in bounds.iter() {
                let zone_vertices = zone_bounding.vertices(*zone_global);
                let zone_aabb = Self::convert_to_aabb3(zone_vertices);

                if zone_aabb.intersects(&player_aabb) {
                    if timer.timer.tick(time.delta()).just_finished() {
                        ev_zone.send(ZoneEvent(zone.zone_type));
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

    fn zone_event(
        mut ev_zone: EventReader<ZoneEvent>,
        mut ev_vortex_gate: EventWriter<VortexGateEvent>,
        game: Res<Game>,
        mut healths: Query<&mut Health>,
    ) {
        for event in ev_zone.iter() {
            let mut health = healths.get_mut(game.player_entity.unwrap()).unwrap();

            match event.0 {
                ZoneType::Damage(amount) => health.take_damage(amount),
                ZoneType::Heal(amount) => health.heal(amount),
                ZoneType::VortexGate => ev_vortex_gate.send(VortexGateEvent),
                _ => {}
            }
        }
    }
}

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(DamageTimer::default());
        app.add_event::<ZoneEvent>();
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::player_in_zone.system())
                .with_system(Self::zone_event.system()),
        );
    }
}

struct ZoneEvent(ZoneType);

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
