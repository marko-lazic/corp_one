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

#[derive(Default, Component)]
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

#[derive(Component)]
pub struct Zone {
    zone_type: ZoneType,
    value: f32,
    timer: Timer,
}

impl Zone {
    pub fn from(asset: ZoneAsset) -> Self {
        Zone {
            zone_type: asset.zone_type,
            value: asset.value,
            timer: Timer::from_seconds(asset.second, true),
        }
    }
}

pub struct ZonePlugin;

impl ZonePlugin {
    fn collision_events(
        mut collision_events: EventReader<CollisionEvent>,
        mut vortex_events: EventWriter<VortexEvent>,
        mut zone_entities: Query<&mut ZoneEntities>,
    ) {
        for event in collision_events.iter() {
            match event {
                CollisionEvent::Started(d1, d2) => {
                    if Self::player_in_vortex_gate(d1, d2) {
                        vortex_events.send(VortexEvent::vort(Colony::StarMap));
                    } else if let Some((player, zone)) = Self::player_in_zone(d1, d2) {
                        let mut zone_entities = zone_entities.get_mut(zone).unwrap();
                        zone_entities.add(player)
                    }
                }
                CollisionEvent::Stopped(d1, d2) => {
                    if let Some((player, zone)) = Self::player_in_zone(d1, d2) {
                        let mut zone_entities = zone_entities.get_mut(zone).unwrap();
                        zone_entities.remove(player)
                    }
                }
            }
        }
    }

    fn player_in_zone(d1: &CollisionData, d2: &CollisionData) -> Option<(Entity, Entity)> {
        if Self::is_zone(d1) && Self::is_player(d2) {
            Some((d2.rigid_body_entity(), d1.collision_shape_entity()))
        } else if Self::is_player(d1) && Self::is_zone(d2) {
            Some((d1.rigid_body_entity(), d2.collision_shape_entity()))
        } else {
            None
        }
    }

    fn player_in_vortex_gate(d1: &CollisionData, d2: &CollisionData) -> bool {
        Self::is_vortex_gate(d1) && Self::is_player(d2)
            || Self::is_player(d1) && Self::is_vortex_gate(d2)
    }

    fn is_player(data: &CollisionData) -> bool {
        data.collision_layers().contains_group(Layer::Player)
    }

    fn is_zone(data: &CollisionData) -> bool {
        data.collision_layers().contains_group(Layer::Zone)
    }

    fn is_vortex_gate(data: &CollisionData) -> bool {
        data.collision_layers().contains_group(Layer::VortexGate)
    }

    fn handle_health_in_zones(
        time: Res<Time>,
        mut zones: Query<(&mut Zone, &ZoneEntities)>,
        mut healths: Query<&mut Health>,
    ) {
        for (mut zone, zone_entities) in zones.iter_mut() {
            zone.timer.tick(time.delta());
            if zone.timer.finished() {
                for entity in zone_entities.entities.iter() {
                    let mut health = healths.get_mut(*entity).unwrap();
                    match zone.zone_type {
                        ZoneType::Damage => health.take_damage(zone.value),
                        ZoneType::Heal => health.heal(zone.value),
                        _ => {}
                    }
                }
            }
        }
    }
}

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::collision_events.system())
                .with_system(Self::handle_health_in_zones.system()),
        );
    }
}
