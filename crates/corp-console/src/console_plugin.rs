pub mod console {
    use bevy::app::{AppBuilder, Plugin};
    use bevy::core::{Time, Timer};
    use bevy::ecs::{Commands, IntoQuerySystem, Query, Res, ResMut};

    struct Person;

    struct Name(String);

    struct GreetTimer(Timer);

    pub struct ConsolePlugin;

    impl Plugin for ConsolePlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
                .add_startup_system(add_people.system())
                .add_system(greet_people.system());
        }
    }

    fn add_people(mut commands: Commands) {
        commands
            .spawn((Person, Name("Marko Lazic".to_string())))
            .spawn((Person, Name("Ilija Nikolic".to_string())))
            .spawn((Person, Name("Borka Lazic".to_string())));
    }

    fn greet_people(
        time: Res<Time>,
        mut timer: ResMut<GreetTimer>,
        mut query: Query<(&Person, &Name)>,
    ) {
        timer.0.tick(time.delta_seconds);
        if timer.0.finished {
            for (_person, name) in &mut query.iter() {
                println!("hello {}!", name.0);
            }
        }
    }
}
