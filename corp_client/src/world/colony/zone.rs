use bevy::core::prelude::Timer;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use heron::{CollisionData, CollisionEvent};
use serde::Deserialize;

use corp_shared::prelude::*;

use crate::constants::state::GameState;
use crate::constants::tick;
use crate::world::colony::colony_assets::ZoneAsset;
use crate::world::colony::vortex::VortexEvent;
use crate::world::colony::{Colony, Layer};

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum ZoneType {
    Heal,
    Damage,
    Unknown,
}

#[derive(Default)]
pub struct ZoneEntities {
    entities: Vec<Entity>,
}

impl ZoneEntities {
    fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    fn remove(&mut self, entity: Entity) {
        if let Some(entity) = self.entities.iter().position(|e| *e == entity) {
            self.entities.remove(entity);
        }
    }
}

impl Default for ZoneType {
    fn default() -> Self {
        Self::Unknown
    }
}

pub struct Zone {
    zone_type: ZoneType,
    value: f32,
    second: f32,
}

impl Zone {
    pub fn from_asset(asset: ZoneAsset) -> Self {
        Zone {
            zone_type: asset.zone_type,
            value: asset.value,
            second: asset.second,
        }
    }
    pub fn new(zone_type: ZoneType, value: f32, second: f32) -> Self {
        Zone {
            zone_type,
            value,
            second,
        }
    }
}

pub struct ZonePlugin;

impl ZonePlugin {
    fn vortex_gate_collision(
        mut collision_events: EventReader<CollisionEvent>,
        mut vortex_events: EventWriter<VortexEvent>,
        mut zone_entities: Query<&mut ZoneEntities>,
    ) {
        for event in collision_events.iter() {
            match event {
                CollisionEvent::Started(d1, d2) => {
                    if Self::check_collision_data(&d1, &d2, [Layer::Player, Layer::VortexGate]) {
                        vortex_events.send(VortexEvent::vort(Colony::StarMap));
                    } else if let Some((player, zone)) = Self::player_on_zone(&d1, &d2) {
                        let mut zone_entities = zone_entities.get_mut(zone).unwrap();
                        zone_entities.add(player)
                    }
                }
                CollisionEvent::Stopped(d1, d2) => {
                    if let Some((player, zone)) = Self::player_on_zone(&d1, &d2) {
                        let mut zone_entities = zone_entities.get_mut(zone).unwrap();
                        zone_entities.remove(player)
                    }
                }
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

    fn handle_entities_in_zones(
        zones: Query<(&Zone, &ZoneEntities)>,
        mut healths: Query<&mut Health>,
    ) {
        for (zone, zone_entities) in zones.iter() {
            for entity in zone_entities.entities.iter() {
                let mut health = healths.get_mut(*entity).unwrap();
                println!("healht {:?}", health);
                // match zone.zone_type {
                //     ZoneType::Damage => health.take_damage(zone.value),
                //     ZoneType::Heal => health.heal(zone.value),
                //     _ => {}
                // }
            }
        }
    }
}

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::vortex_gate_collision.system())
                .with_system(Self::handle_entities_in_zones.system()),
        );
    }
}
