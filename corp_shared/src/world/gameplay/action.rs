use crate::{prelude::Health, world::gameplay::roll::Range};
use bevy::prelude::*;
use bevy_rand::prelude::{GlobalRng, WyRand};
use rand::RngExt;

#[derive(Event)]
pub struct HealActionEvent {
    pub receiver: Entity,
    pub range: Range,
}

#[derive(Event)]
pub struct DamageActionEvent {
    pub receiver: Entity,
    pub range: Range,
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(heal).add_observer(take_damage);
    }
}

fn heal(
    heal_action: On<HealActionEvent>,
    mut q_health: Query<&mut Health>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    if let Ok(mut health) = q_health.get_mut(heal_action.receiver) {
        let amount = rng.random_range((*heal_action.range).clone()) as f32;
        health.heal(amount);
    }
}

fn take_damage(
    damage_action: On<DamageActionEvent>,
    mut q_health: Query<&mut Health>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    if let Ok(mut health) = q_health.get_mut(damage_action.receiver) {
        let amount = rng.random_range((*damage_action.range).clone()) as f32;
        health.take_damage(amount);
    }
}
