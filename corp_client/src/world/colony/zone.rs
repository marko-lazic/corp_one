use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_rapier3d::prelude::*;

use corp_shared::prelude::*;

use crate::{
    asset::{ZoneConfig, ZoneType},
    state::GameState
    ,
};

#[derive(Component)]
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
            Update,
            (handle_health_in_zones, zone_collider)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn handle_health_in_zones(
    time: Res<Time>,
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

fn zone_collider(
    mut zones: Query<(&mut Zone, &Transform, &Collider)>,
    rapier_context: Res<RapierContext>,
) {
    let filter = QueryFilter::only_kinematic();

    for (mut zone, t_zone, collider) in zones.iter_mut() {
        zone.entities.clear();
        rapier_context.intersections_with_shape(
            t_zone.translation,
            t_zone.rotation,
            collider,
            filter,
            |entity| {
                zone.add(entity);
                // Match all intersections, not just the first one
                true
            },
        );
    }
}
