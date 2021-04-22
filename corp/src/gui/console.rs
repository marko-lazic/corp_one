use bevy::prelude::*;

struct Person;

struct GreetTimer {
    timer: Timer,
}

struct Name(String);

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer {
            timer: Timer::from_seconds(60.0, true),
        })
        .add_startup_system(add_people.system())
        .add_system(greet_people.system());
    }
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert((Person, Name("Marko Lazic".to_string())));
    commands
        .spawn()
        .insert((Person, Name("Ilija Nikolic".to_string())));
    commands
        .spawn()
        .insert((Person, Name("Borka Lazic".to_string())));
}

fn greet_people(time: Res<Time>, mut my_timer: ResMut<GreetTimer>, query: Query<(&Person, &Name)>) {
    if my_timer.timer.tick(time.delta()).just_finished() {
        for (_person, name) in &mut query.iter() {
            info!("hello {}!", name.0);
        }
    }
}
