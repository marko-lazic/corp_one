use std::time::Duration;

use bevy::{app::App, ecs::component::Mutable, prelude::*};

pub trait TestUtils {
    fn init_time(&mut self) -> &mut Self;
    fn update_after(&mut self, duration: Duration) -> &mut Self;
    fn get<C: Component>(&self, entity: Entity) -> &C;
    fn get_mut<C: Component<Mutability = Mutable>>(&mut self, entity: Entity) -> Mut<C>;
    fn has_component<C: Component>(&self, entity: Entity) -> bool;

    fn get_resource<R: Resource>(&self) -> &R;
    fn get_resource_mut<R: Resource>(&mut self) -> Mut<R>;
}

impl TestUtils for App {
    fn init_time(&mut self) -> &mut Self {
        self.init_resource::<Time>();
        let mut time = Time::default();
        time.update();
        self.world_mut().insert_resource(time);
        self
    }

    fn update_after(&mut self, duration: Duration) -> &mut Self {
        let mut time = self.world_mut().resource_mut::<Time>();
        time.advance_by(duration);
        self.update();
        self
    }

    fn get<C: Component>(&self, entity: Entity) -> &C {
        self.world().get::<C>(entity).unwrap_or_else(|| {
            panic!(
                "Component {} not found on entity {}",
                std::any::type_name::<C>(),
                entity.index()
            )
        })
    }

    fn get_mut<C: Component<Mutability = Mutable>>(&mut self, entity: Entity) -> Mut<C> {
        self.world_mut().get_mut::<C>(entity).unwrap_or_else(|| {
            panic!(
                "Component {} not found on entity {}",
                std::any::type_name::<C>(),
                entity.index()
            )
        })
    }

    fn has_component<C: Component>(&self, entity: Entity) -> bool {
        self.world().get::<C>(entity).is_some()
    }

    fn get_resource<R: Resource>(&self) -> &R {
        self.world().get_resource::<R>().unwrap()
    }

    fn get_resource_mut<R: Resource>(&mut self) -> Mut<R> {
        self.world_mut().get_resource_mut::<R>().unwrap()
    }
}
