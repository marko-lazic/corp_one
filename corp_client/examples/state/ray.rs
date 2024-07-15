use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use crate::TargetEntity;

pub fn cast_ray_system(
    r_rapier_context: Res<RapierContext>,
    mut r_target_entity: ResMut<TargetEntity>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = q_windows.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // We will color in read the colliders hovered by the mouse.
    for (camera, camera_transform) in &q_camera {
        // First, compute a ray from the mouse position.
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };

        // Then cast the ray.
        let hit = r_rapier_context.cast_ray(
            ray.origin,
            ray.direction.into(),
            f32::MAX,
            true,
            QueryFilter::only_fixed(),
        );

        if let Some((entity, _toi)) = hit {
            r_target_entity.0 = Some(entity);
        } else {
            r_target_entity.0 = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::{
        render::{
            RenderPlugin,
            settings::{RenderCreation, WgpuSettings},
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
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .init_resource::<TargetEntity>()
        .add_systems(Update, cast_ray_system);
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
        let player_entity = app.world().spawn(Player).id();
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
