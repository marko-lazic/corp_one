pub(crate) mod input {

    use bevy::prelude::*;

    pub struct InputPlugin;

    impl Plugin for InputPlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_startup_system(setup.system());
        }
    }

    fn setup() {}
}
