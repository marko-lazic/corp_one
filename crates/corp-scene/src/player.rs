use bevy::ecs::Entity;

pub struct Player {
    pub yaw: f32,
    pub camera_distance: f32,
    pub camera_pitch: f32,
    pub camera_entity: Option<Entity>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            yaw: 0.,
            camera_distance: 20.,
            camera_pitch: 30.0f32.to_radians(),
            camera_entity: None,
        }
    }
}
