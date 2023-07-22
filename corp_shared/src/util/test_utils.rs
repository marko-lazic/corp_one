use std::time::Duration;

use bevy::{
    app::App,
    prelude::{Component, Entity, Mut, Time},
};

pub trait TestUtils {
    fn init_time(&mut self) -> &mut Self;
    fn update_after(&mut self, duration: Duration) -> &mut Self;
    fn get<T: Component>(&self, entity: Entity) -> &T;
    fn get_mut<T: Component>(&mut self, entity: Entity) -> Mut<T>;
    fn has_component<T: Component>(&self, entity: Entity) -> bool;
}

impl TestUtils for App {
    fn init_time(&mut self) -> &mut Self {
        self.init_resource::<Time>();
        let mut time = Time::default();
        time.update();
        self.world.insert_resource(time);
        self
    }

    fn update_after(&mut self, duration: Duration) -> &mut Self {
        let mut time = self.world.resource_mut::<Time>();
        let last_update = time.last_update().unwrap();
        time.update_with_instant(last_update + duration);
        self.update();
        self
    }

    fn get<T: Component>(&self, entity: Entity) -> &T {
        self.world.get::<T>(entity).unwrap_or_else(|| {
            panic!(
                "Component {} not found on entity {}",
                std::any::type_name::<T>(),
                entity.index()
            )
        })
    }

    fn get_mut<T: Component>(&mut self, entity: Entity) -> Mut<T> {
        self.world.get_mut::<T>(entity).unwrap_or_else(|| {
            panic!(
                "Component {} not found on entity {}",
                std::any::type_name::<T>(),
                entity.index()
            )
        })
    }

    fn has_component<T: Component>(&self, entity: Entity) -> bool {
        self.world.get::<T>(entity).is_some()
    }
}
