pub mod health;

#[derive(Default, bevy::ecs::component::Component)]
pub struct Player {
    pub is_moving: bool,
}
