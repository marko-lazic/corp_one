use crate::TargetEntity;
use avian3d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

pub fn cast_ray_system(
    q_spatial: SpatialQuery,
    mut r_target_entity: ResMut<TargetEntity>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) -> Result {
    let window = q_windows.single()?;

    let Some(cursor_position) = window.cursor_position() else {
        return Ok(());
    };

    // We will color in read the colliders hovered by the mouse.
    for (camera, camera_transform) in &q_camera {
        // First, compute a ray from the mouse position.
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return Ok(());
        };

        // Then cast the ray.
        if let Some(hit_data) = q_spatial.cast_ray(
            ray.origin,
            ray.direction.into(),
            f32::MAX,
            true,
            &SpatialQueryFilter::default(),
        ) {
            r_target_entity.0 = Some(hit_data.entity);
        } else {
            r_target_entity.0 = None;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use avian3d::PhysicsPlugins;
    use bevy::{
        render::{
            settings::{RenderCreation, WgpuSettings},
            RenderPlugin,
        },
        scene::ScenePlugin,
        time::TimePlugin,
    };

    use corp_shared::prelude::{Player, TestUtils};

    use super::*;

    #[test]
    fn test_ray_intersect_door() {
        let mut app = setup();
        setup_camera(&mut app);
        setup_player(&mut app);
        let door_entity = setup_door(&mut app);

        let mut query = app.world().query::<&mut Window>();
        query
            .single_mut(&mut app.world())
            .unwrap()
            .set_cursor_position(Some(Vec2::new(0.0, 0.0)));

        app.update();
        app.update();

        let target_entity = app.get_resource::<TargetEntity>();
        assert_eq!(target_entity.0, Some(door_entity));
    }

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((
            HeadlessRenderPlugin,
            TransformPlugin,
            TimePlugin,
            PhysicsPlugins::default(),
        ))
        .init_resource::<TargetEntity>()
        .add_systems(Update, cast_ray_system);
        app
    }

    fn setup_camera(app: &mut App) -> Entity {
        let entity = app
            .world_mut()
            .spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ))
            .id();
        entity
    }

    fn setup_player(app: &mut App) -> Entity {
        let player_entity = app.world().spawn(Player).id();
        player_entity
    }

    fn setup_door(app: &mut App) -> Entity {
        let length = 1.0;
        let entity = app
            .world_mut()
            .spawn((
                Transform::from(Transform::from_xyz(0.0, 0.0, 0.0)),
                RigidBody::Static,
                Collider::cuboid(length, length, length),
            ))
            .id();
        entity
    }

    struct HeadlessRenderPlugin;

    impl Plugin for HeadlessRenderPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((
                WindowPlugin::default(),
                AssetPlugin::default(),
                ScenePlugin::default(),
                RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: None,
                        ..default()
                    }),
                    ..default()
                },
                ImagePlugin::default(),
            ));
        }
    }
}
