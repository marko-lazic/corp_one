use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;

use corp_shared::prelude::{Interactor, Player};

pub fn cast_ray_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut interactor_query: Query<&mut Interactor, With<Player>>,
) {
    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else { return; };

    // We will color in read the colliders hovered by the mouse.
    for (camera, camera_transform) in &cameras {
        // First, compute a ray from the mouse position.
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return; };

        // Then cast the ray.
        let hit = rapier_context.cast_ray(
            ray.origin,
            ray.direction,
            f32::MAX,
            true,
            QueryFilter::only_fixed(),
        );

        if let Some((entity, _toi)) = hit {
            let Ok(mut interactor) = interactor_query.get_single_mut() else { return; };
            interactor.interact(entity);
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::render::settings::WgpuSettings;
    use bevy::render::RenderPlugin;
    use bevy::scene::ScenePlugin;
    use bevy::time::TimePlugin;

    use corp_shared::prelude::TestUtils;

    use super::*;

    #[test]
    fn test_ray_intersect_door() {
        let mut app = setup();
        let _camera_entity = setup_camera(&mut app);
        let player_entity = setup_player(&mut app);
        let door_entity = setup_door(&mut app);

        let mut query = app.world.query::<&mut Window>();
        query
            .single_mut(&mut app.world)
            .set_cursor_position(Some(Vec2::new(0.0, 0.0)));

        app.update();
        app.update();

        let interactor = app.get::<Interactor>(player_entity);
        assert!(interactor.target_entity.is_some());
        assert_eq!(interactor.target_entity.unwrap(), door_entity);
    }

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugin(HeadlessRenderPlugin);
        app.add_plugin(TransformPlugin);
        app.add_plugin(TimePlugin);
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_system(cast_ray_system);
        app
    }

    fn setup_camera(app: &mut App) -> Entity {
        let entity = app
            .world
            .spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1.0)
                    .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
                ..Default::default()
            })
            .id();
        entity
    }

    fn setup_player(app: &mut App) -> Entity {
        let player_entity = app.world.spawn((Player, Interactor::default())).id();
        player_entity
    }

    fn setup_door(app: &mut App) -> Entity {
        let half_size: Real = 1.0;
        let entity = app
            .world
            .spawn((
                TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
                RigidBody::Fixed,
                Collider::cuboid(half_size, half_size, half_size),
            ))
            .id();
        entity
    }

    struct HeadlessRenderPlugin;

    impl Plugin for HeadlessRenderPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugin(WindowPlugin::default())
                .add_plugin(AssetPlugin::default())
                .add_plugin(ScenePlugin::default())
                .add_plugin(RenderPlugin {
                    wgpu_settings: WgpuSettings {
                        backends: None,
                        ..Default::default()
                    },
                })
                .add_plugin(ImagePlugin::default());
        }
    }
}
