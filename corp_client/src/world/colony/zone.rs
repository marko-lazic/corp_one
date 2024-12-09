use crate::{
    asset::{ZoneConfig, ZoneType},
    state::GameState,
    world::prelude::*,
};
use avian3d::prelude::*;
use bevy::{prelude::*, utils::hashbrown::HashSet};
use corp_shared::prelude::*;

#[derive(Component, Debug)]
pub struct Zone {
    zone_type: ZoneType,
    value: f32,
    timer: Timer,
    entities: HashSet<Entity>,
}

impl Zone {
    pub fn from(zone_config: ZoneConfig) -> Self {
        Zone {
            zone_type: zone_config.zone_type,
            value: zone_config.value,
            timer: Timer::from_seconds(zone_config.second, TimerMode::Repeating),
            entities: HashSet::new(),
        }
    }

    fn add(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }
}

pub struct ZonePlugin;

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (handle_health_in_zones, zone_collider)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn handle_health_in_zones(
    time: Res<Time<Fixed>>,
    mut zones: Query<&mut Zone>,
    mut healths: Query<&mut Health>,
) {
    for mut zone in zones.iter_mut() {
        zone.timer.tick(time.delta());
        if zone.timer.finished() {
            for entity in zone.entities.iter() {
                let Ok(mut health) = healths.get_mut(*entity) else {
                    return;
                };
                match zone.zone_type {
                    ZoneType::Damage => health.take_damage(zone.value.clone()),
                    ZoneType::Heal => health.heal(zone.value.clone()),
                    _ => {}
                }
            }
        }
    }
}

fn zone_collider(mut zones: Query<(&mut Zone, &Transform, &Collider)>, q_spatial: SpatialQuery) {
    for (mut zone, t_zone, c_zone) in zones.iter_mut() {
        zone.entities.clear();
        q_spatial.shape_intersections_callback(
            c_zone,
            t_zone.translation,
            t_zone.rotation,
            SpatialQueryFilter::default().with_mask(Layer::Player),
            |entity| {
                zone.add(entity);
                // Match all intersections, not just the first one
                true
            },
        );
    }
}
