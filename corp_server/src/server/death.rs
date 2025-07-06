use bevy::prelude::*;
use bevy_replicon::prelude::{FromClient, SendMode, ServerTriggerExt, ToClients};
use corp_shared::prelude::*;

#[derive(Component, Deref, DerefMut)]
struct DeathTimer(Timer);

const DEATH_SECS: f32 = 5.0;

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, despawn_dead_player)
            .add_observer(on_add_dead)
            .add_observer(on_kill_me_command);
    }
}
fn despawn_dead_player(
    mut q_death_timer: Query<(Entity, &mut DeathTimer)>,
    r_time: Res<Time<Fixed>>,
    mut commands: Commands,
) {
    for (dead_player_e, mut death_timer) in &mut q_death_timer {
        death_timer.tick(r_time.delta());
        if death_timer.finished() {
            info!(
                "Sending SendDeadPlayerToCloningCommand entity {:?}",
                dead_player_e
            );
            commands.server_trigger(ToClients {
                mode: SendMode::Direct(dead_player_e),
                event: SendDeadPlayerToCloningCommand,
            });
            commands.entity(dead_player_e).remove::<DeathTimer>();
        }
    }
}

fn on_add_dead(trigger: Trigger<OnAdd, Dead>, mut commands: Commands) {
    let player_e = trigger.target();
    info!("On add Dead for entity {:?} triggered", player_e);
    commands
        .entity(player_e)
        .insert(Immobilized)
        .insert(DeathTimer(Timer::from_seconds(DEATH_SECS, TimerMode::Once)));
}

fn on_kill_me_command(
    trigger: Trigger<FromClient<KillMeCommand>>,
    mut player_health: Query<&mut Health, With<Player>>,
) -> Result {
    let player_e = trigger.client_entity;
    let mut health = player_health.get_mut(player_e)?;
    health.kill_mut();
    Ok(())
}
