use std::{fmt::Display, ops::Deref};

use bevy::prelude::*;
use bevy::render::camera::{Camera, OrthographicProjection};

/// The location of the mouse in screenspace.
#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
pub struct MousePos(pub Vec2);

impl Deref for MousePos {
    type Target = Vec2;
    fn deref(&self) -> &Vec2 {
        &self.0
    }
}

impl Display for MousePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn update_pos(mut mouse_loc: ResMut<MousePos>, mut reader: EventReader<CursorMoved>) {
    for event in reader.iter() {
        mouse_loc.0 = event.position;
    }
}

/// The location of the mouse in worldspace.
#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
pub struct MousePosWorld(pub Vec3);

impl Display for MousePosWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for MousePosWorld {
    type Target = Vec3;
    fn deref(&self) -> &Vec3 {
        &self.0
    }
}

fn update_pos_ortho(
    mut mouse_world: ResMut<MousePosWorld>,
    mut cursor: EventReader<CursorMoved>,
    cameras: Query<(&GlobalTransform, &OrthographicProjection), With<Camera>>,
) {
    if let Some(cursor_latest) = cursor.iter().last() {
        let (camera, proj) = cameras
            .iter()
            .next()
            .expect("could not find an orthographic camera");
        mouse_world.0 = camera.mul_vec3(
            cursor_latest.position.extend(0.0) + Vec3::new(proj.left, proj.bottom, proj.near),
        );
    }
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MousePos>()
            .add_system(update_pos.system())
            .init_resource::<MousePosWorld>()
            .add_system_to_stage(CoreStage::PreUpdate, update_pos_ortho.system());
    }
}
