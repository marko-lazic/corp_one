use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use bevy_rapier3d::prelude::*;
use iyes_loopless::condition::ConditionSet;
use serde::Deserialize;

use corp_shared::prelude::*;

use crate::constants::state::GameState;
use crate::world::colony::colony_assets::ZoneAsset;
use crate::world::physics;

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum ZoneType {
    Heal,
    Damage,
    Unknown,
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
    entities: HashSet<Entity>,
}

impl Zone {
    pub fn from(asset: ZoneAsset) -> Self {
        Zone {
            zone_type: asset.zone_type,
            value: asset.value,
            timer: Timer::from_seconds(asset.second, true),
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
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(Self::handle_health_in_zones)
                .with_system(Self::zone_collider)
                .into(),
        );
    }
}

impl ZonePlugin {
    fn handle_health_in_zones(
        time: Res<Time>,
        mut zones: Query<&mut Zone>,
        mut healths: Query<&mut Health>,
    ) {
        for mut zone in zones.iter_mut() {
            zone.timer.tick(time.delta());
            if zone.timer.finished() {
                for entity in zone.entities.iter() {
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

    fn zone_collider(
        mut zones: Query<(&mut Zone, &Transform, &Collider)>,
        rapier_context: Res<RapierContext>,
    ) {
        let filter = QueryFilter::only_dynamic().groups(physics::CollideGroups::zone().into());

        for (mut zone, transform, collider) in zones.iter_mut() {
            zone.entities.clear();
            let shape_rot = transform.rotation;
            let shape_pos = transform.translation;
            rapier_context.intersections_with_shape(
                shape_pos,
                shape_rot,
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
}
