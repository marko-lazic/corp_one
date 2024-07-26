use bevy::{log::LogPlugin, prelude::*};

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_systems(Startup, startup)
        .observe(wave_spawner)
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .add_systems(FixedUpdate, update_loop)
        .run();
}

fn startup(mut commands: Commands) {
    let door = commands.spawn_empty().observe(on_use_event_door).id();
    let energy_node = commands
        .spawn_empty()
        .observe(on_use_event_energy_node)
        .id();
    let market_terminal = commands
        .spawn_empty()
        .observe(on_use_event_market_terminal)
        .id();

    commands.trigger_targets(UseEvent, door);
    commands.trigger_targets(UseEvent, energy_node);
    commands.trigger_targets(UseEvent, market_terminal);
    commands.trigger_targets(UseEvent, vec![door, energy_node, market_terminal]);
    commands.trigger(NextWave);
    commands.trigger(NextWave);
    commands.trigger(NextWave);
}

#[derive(Debug, Event)]
struct UseEvent;

fn on_use_event_door(trigger: Trigger<UseEvent>) {
    info!(
        "Using energy door {:?} {:?}",
        trigger.entity(),
        trigger.event()
    );
}

fn on_use_event_energy_node(trigger: Trigger<UseEvent>) {
    info!(
        "Using energy node {:?} {:?}",
        trigger.entity(),
        trigger.event()
    );
}

fn on_use_event_market_terminal(trigger: Trigger<UseEvent>) {
    info!(
        "Using market terminal {:?} {:?}",
        trigger.entity(),
        trigger.event()
    );
}

#[derive(Event)]
struct NextWave;

#[derive(Component)]
struct Monster;
fn wave_spawner(_trigger: Trigger<NextWave>, mut wave_number: Local<i32>, mut commands: Commands) {
    *wave_number += 1;
    info!("Spawning wave: {}", *wave_number);
    commands.spawn(Monster);
}

fn update_loop(mut commands: Commands) {
    info!("Update loop");
    commands.trigger(NextWave);
}
