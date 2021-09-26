use bevy::core::prelude::Timer;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use heron::{CollisionData, CollisionEvent};
use serde::Deserialize;

use corp_shared::prelude::*;

use crate::constants::state::GameState;
use crate::constants::tick;
use crate::world::colony::vortex::VortexEvent;
use crate::world::colony::{Colony, Layer};
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
    fn vortex_gate_collision(
        mut collision_events: EventReader<CollisionEvent>,
        mut vortex_events: EventWriter<VortexEvent>,
        mut player_zone_events: EventWriter<PlayerZoneEvent>,
        zones: Query<&Zone>,
        mut healths: Query<&mut Health>,
    ) {
        for event in collision_events.iter() {
            match event {
                CollisionEvent::Started(d1, d2) => {
                    if Self::check_collision_data(d1, d2, [Layer::Player, Layer::VortexGate]) {
                        vortex_events.send(VortexEvent::vort(Colony::StarMap));
                    } else if let Some((player, zone)) = Self::player_on_zone(&d1, &d2) {
                        let zone_type = zones.get(zone).unwrap().zone_type;
                        player_zone_events.send(PlayerZoneEvent(zone_type));
                        let mut health = healths.get_mut(player).unwrap();
                        match zone_type {
                            ZoneType::Damage(amount) => health.take_damage(amount),
                            ZoneType::Heal(amount) => health.heal(amount),
                            _ => {}
                        }
                    }
                }
                CollisionEvent::Stopped(_, _) => {}
            }
        }
    }

    fn player_on_zone(d1: &CollisionData, d2: &CollisionData) -> Option<(Entity, Entity)> {
        if Self::is_zone(d1) && Self::is_player(d2) {
            Some((d2.rigid_body_entity(), d1.collision_shape_entity()))
        } else if Self::is_player(d1) && Self::is_zone(d2) {
            Some((d1.rigid_body_entity(), d2.collision_shape_entity()))
        } else {
            None
        }
    }

    fn check_collision_data(d1: &CollisionData, d2: &CollisionData, l: [Layer; 2]) -> bool {
        d1.collision_layers().contains_group(l[0]) && d2.collision_layers().contains_group(l[1])
            || d1.collision_layers().contains_group(l[1])
                && d2.collision_layers().contains_group(l[0])
    }

    fn is_player(data: &CollisionData) -> bool {
        data.collision_layers().contains_group(Layer::Player)
    }

    fn is_zone(data: &CollisionData) -> bool {
        data.collision_layers().contains_group(Layer::Zone)
    }

    fn player_in_zone_event(
        game: Res<Game>,
        mut healths: Query<&mut Health>,
        mut player_zone_events: EventReader<PlayerZoneEvent>,
        mut vortex_events: EventWriter<VortexEvent>,
    ) {
        for event in player_zone_events.iter() {
            let mut health = healths.get_mut(game.player_entity.unwrap()).unwrap();

            match event.0 {
                ZoneType::Damage(amount) => health.take_damage(amount),
                ZoneType::Heal(amount) => health.heal(amount),
                ZoneType::VortexGate => vortex_events.send(VortexEvent::vort(Colony::StarMap)),
                _ => {}
            }
        }
    }
}

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<PlayerZoneEvent>();
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::vortex_gate_collision.system())
                .with_system(Self::player_in_zone_event.system()),
        );
    }
}

struct PlayerZoneEvent(ZoneType);
