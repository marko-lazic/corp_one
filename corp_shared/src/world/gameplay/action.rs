use crate::{prelude::Health, world::gameplay::roll::Range};
use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand::Rng;

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
    trigger: Trigger<HealActionEvent>,
    mut q_health: Query<&mut Health>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    if let Ok(mut health) = q_health.get_mut(trigger.receiver) {
        let amount = rng.gen_range((*trigger.range).clone()) as f32;
        health.heal(amount);
    }
}

fn take_damage(
    trigger: Trigger<DamageActionEvent>,
    mut q_health: Query<&mut Health>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    if let Ok(mut health) = q_health.get_mut(trigger.receiver) {
        let amount = rng.gen_range((*trigger.range).clone()) as f32;
        health.take_damage(amount);
    }
}
