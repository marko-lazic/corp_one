use bevy::prelude::*;
use bevy_replicon::prelude::{
    DisconnectRequest, FromClient, SendMode, ServerTriggerExt, ToClients,
};
use corp_shared::prelude::*;

#[derive(Component)]
struct DeathTimer(Entity, Timer);

const DEATH_SECS: f32 = 5.0;

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_death_timer)
            .add_observer(on_add_dead)
            .add_observer(on_kill_me_command);
    }
}
fn update_death_timer(
    mut q_death_timer: Query<&mut DeathTimer>,
    r_time: Res<Time<Fixed>>,
    mut events: EventWriter<DisconnectRequest>,
    mut commands: Commands,
) {
    for mut death_timer in &mut q_death_timer {
        death_timer.1.tick(r_time.delta());
        if death_timer.1.finished() {
            let dead_player_e = death_timer.0;
            commands.server_trigger(ToClients {
                mode: SendMode::Direct(dead_player_e),
                event: YouDied,
            });
            events.write(DisconnectRequest {
                client_entity: dead_player_e,
            });
        }
    }
}

fn on_add_dead(trigger: Trigger<OnAdd, Dead>, mut commands: Commands) {
    let player_e = trigger.target();
    info!("On add Dead for entity {:?} triggered", player_e);
    commands
        .entity(player_e)
        .insert(Immobilized)
        .insert(DeathTimer(
            player_e,
            Timer::from_seconds(DEATH_SECS, TimerMode::Once),
        ));
}

fn on_kill_me_command(
    trigger: Trigger<FromClient<KillMeCommand>>,
    mut player_health: Query<&mut Health, With<Player>>,
) -> Result {
    let player = trigger.target();
    let mut health = player_health.get_mut(player)?;
    health.kill_mut();
    Ok(())
}
