use bevy::prelude::*;
use bevy_replicon::prelude::{ClientId, FromClient, SendTargets, ServerTriggerExt, ToClients};
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
        if death_timer.is_finished() {
            info!(
                "Sending SendDeadPlayerToCloningCommand entity {:?}",
                dead_player_e
            );
            commands.server_trigger(ToClients {
                targets: SendTargets::Single(ClientId::from(dead_player_e)),
                message: SendDeadPlayerToCloningCommand,
            });
            commands.entity(dead_player_e).remove::<DeathTimer>();
        }
    }
}

fn on_add_dead(trigger: On<Add<Dead>>, mut commands: Commands) {
    let player_e = trigger.event_target();
    info!("On add Dead for entity {:?} triggered", player_e);

    commands.entity(player_e).insert((
        Immobilized,
        DeathTimer(Timer::from_seconds(DEATH_SECS, TimerMode::Once)),
    ));
}

fn on_kill_me_command(
    kill_me_command: On<FromClient<KillMeCommand>>,
    mut player_health: Query<&mut Health, With<Player>>,
) -> Result {
    if let ClientId::Client(player_e) = kill_me_command.client_id {
        let mut health = player_health.get_mut(player_e)?;
        health.kill_mut();
    } else {
        warn!("Received KillMeCommand from server, but it's not from a client");
    }

    Ok(())
}
