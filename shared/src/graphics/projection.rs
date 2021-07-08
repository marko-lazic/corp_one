use bevy::math::{Mat4, Vec3, Vec4};

pub fn project_screen_to_world(
    screen: Vec3,
    view_projection: Mat4,
    viewport: Vec4,
) -> Option<Vec3> {
    let world = Vec4::new(
        (screen.x - (viewport.x as f32)) / (viewport.z as f32) * 2.0 - 1.0,
        // Screen Origin is Top Left    (Mouse Origin is Top Left)
        //          (screen.y - (viewport.y as f32)) / (viewport.w as f32) * 2.0 - 1.0,
        // Screen Origin is Bottom Left (Mouse Origin is Top Left)
        (1.0 - (screen.y - (viewport.y as f32)) / (viewport.w as f32)) * 2.0 - 1.0,
        screen.z * 2.0 - 1.0,
        1.0,
    );
    let world = view_projection.inverse() * world;

    if world.w != 0.0 {
        Some(world.truncate() * (1.0 / world.w))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Mat4;
    use bevy::math::{Vec3, Vec4};
    use corp_shared::projection::project_screen_to_world;

    #[test]
    fn test_projection() {
        let screen = Vec3::new(1.0, 1.0, 1.0);
        let view_projection = Mat4::from_scale(Vec3::new(1.0, 1.0, 1.0));
        let viewport = Vec4::new(1.0, 1.0, 1.0, 1.0);
        let result = pdomain.xmlroject_screen_to_world(screen, view_projection, viewport);

        println!("{}", result.unwrap())
    }
}
