use crate::prelude::*;
use avian3d::prelude::*;
use bevy::{prelude::*, time::common_conditions::on_timer};
use std::time::Duration;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
#[require(PassStructure, Transform, CollisionLayers(init_area_collision_layers))]
#[cfg_attr(feature = "client", require(
    StateScoped<GameState>(|| StateScoped(GameState::Playing)))
)]
pub struct Area;

pub fn init_area_collision_layers() -> CollisionLayers {
    CollisionLayers::new([GameLayer::Area], [GameLayer::Player])
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Area)]
pub struct FireArea;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Area)]
pub struct HealArea;

pub struct AreaPlugin;

impl Plugin for AreaPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FireArea>()
            .register_type::<HealArea>()
            .add_systems(
                FixedUpdate,
                (
                    fire_area_collider.run_if(on_timer(Duration::from_secs_f32(1.5))),
                    heal_area_collider.run_if(on_timer(Duration::from_secs_f32(0.8))),
                ),
            );
    }
}

fn fire_area_collider(
    mut commands: Commands,
    mut q_area: Query<(&Transform, &Collider), With<FireArea>>,
    q_spatial: SpatialQuery,
) {
    for (transform, collider) in q_area.iter_mut() {
        q_spatial.shape_intersections_callback(
            collider,
            transform.translation,
            transform.rotation,
            &SpatialQueryFilter::default().with_mask(GameLayer::Player),
            |entity| {
                commands.trigger(DamageActionEvent {
                    receiver: entity,
                    range: Range::new(8, 16),
                });
                // Match all intersections, not just the first one
                true
            },
        );
    }
}

fn heal_area_collider(
    mut commands: Commands,
    mut q_area: Query<(&Transform, &Collider), With<HealArea>>,
    q_spatial: SpatialQuery,
) {
    for (transform, collider) in q_area.iter_mut() {
        q_spatial.shape_intersections_callback(
            collider,
            transform.translation,
            transform.rotation,
            &SpatialQueryFilter::default().with_mask(GameLayer::Player),
            |entity| {
                commands.trigger(HealActionEvent {
                    receiver: entity,
                    range: Range::new(1, 12),
                });
                // Match all intersections, not just the first one
                true
            },
        );
    }
}
