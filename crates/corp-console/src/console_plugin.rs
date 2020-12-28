pub mod console {
    use bevy::prelude::*;

    struct Person;

    struct GreetTimer {
        timer: Timer,
    }

    struct Name(String);

    pub struct ConsolePlugin;

    impl Plugin for ConsolePlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_resource(GreetTimer {
                timer: Timer::from_seconds(60.0, true),
            })
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
        }
    }

    fn add_people(commands: &mut Commands) {
        commands
            .spawn((Person, Name("Marko Lazic".to_string())))
            .spawn((Person, Name("Ilija Nikolic".to_string())))
            .spawn((Person, Name("Borka Lazic".to_string())));
    }

    fn greet_people(
        time: Res<Time>,
        mut my_timer: ResMut<GreetTimer>,
        query: Query<(&Person, &Name)>,
    ) {
        if my_timer.timer.tick(time.delta_seconds()).just_finished() {
            for (_person, name) in &mut query.iter() {
                info!("hello {}!", name.0);
            }
        }
    }
}
