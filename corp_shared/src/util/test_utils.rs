use std::time::Duration;

use bevy::{
    app::App,
    prelude::{Component, Entity, Mut, NextState, Resource, States, Time},
};

pub trait TestUtils {
    fn init_time(&mut self) -> &mut Self;
    fn update_after(&mut self, duration: Duration) -> &mut Self;
    fn get<C: Component>(&self, entity: Entity) -> &C;
    fn get_mut<C: Component>(&mut self, entity: Entity) -> Mut<C>;
    fn has_component<C: Component>(&self, entity: Entity) -> bool;
    fn get_resource_mut<R: Resource>(&mut self) -> Mut<R>;
    fn set_state<S: States>(&mut self, state: S) -> &mut Self;
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

    fn get<C: Component>(&self, entity: Entity) -> &C {
        self.world.get::<C>(entity).unwrap_or_else(|| {
            panic!(
                "Component {} not found on entity {}",
                std::any::type_name::<C>(),
                entity.index()
            )
        })
    }

    fn get_mut<C: Component>(&mut self, entity: Entity) -> Mut<C> {
        self.world.get_mut::<C>(entity).unwrap_or_else(|| {
            panic!(
                "Component {} not found on entity {}",
                std::any::type_name::<C>(),
                entity.index()
            )
        })
    }

    fn has_component<C: Component>(&self, entity: Entity) -> bool {
        self.world.get::<C>(entity).is_some()
    }

    fn get_resource_mut<R: Resource>(&mut self) -> Mut<R> {
        self.world.get_resource_mut::<R>().unwrap()
    }

    fn set_state<S: States>(&mut self, state: S) -> &mut Self {
        self.world
            .get_resource_mut::<NextState<S>>()
            .unwrap()
            .set(state);
        self.update();
        self
    }
}
