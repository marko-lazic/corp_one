use bevy::prelude::*;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastMethod, RayCastSource};
use bevy_orbit_controls::{OrbitCamera, OrbitCameraPlugin};

use crate::gui::metrics::Metrics;
use crate::world::player::Player;
use crate::world::WorldSystem;
use crate::GameState;

pub struct MyRaycastSet;

pub struct TopDownCameraPlugin;

impl TopDownCameraPlugin {
    fn setup(mut commands: Commands) {
        commands
            .spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_translation(Vec3::new(-3.0, 3.0, 5.0))
                    .looking_at(Vec3::default(), Vec3::Y),
                ..Default::default()
            })
            .insert(OrbitCamera::new(20.0, Vec3::ZERO))
            .insert(RayCastSource::<MyRaycastSet>::new());
    }

    fn update_camera_center(
        mut camera_query: Query<&mut OrbitCamera>,
        mut player_query: Query<(&Player, &Transform)>,
    ) {
        if let Ok((_, transform)) = player_query.single_mut() {
            if let Ok(mut camera) = camera_query.single_mut() {
                camera.center.x = transform.translation.x;
                camera.center.z = transform.translation.z;
            }
        }
    }

    // Update our `RayCastSource` with the current cursor position every frame.
    fn update_raycast_with_cursor(
        mut cursor: EventReader<CursorMoved>,
        mut query: Query<&mut RayCastSource<MyRaycastSet>>,
        mut metrics: ResMut<Metrics>,
    ) {
        for mut pick_source in &mut query.iter_mut() {
            // Grab the most recent cursor event if it exists:
            if let Some(cursor_latest) = cursor.iter().last() {
                pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
                if let Some((_entity, intersect)) = pick_source.intersect_top() {
                    metrics.mouse_world_position = intersect.position();
                    metrics.mouse_screen_position = cursor_latest.position;
                }
            }
        }
    }
}

impl Plugin for TopDownCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(OrbitCameraPlugin);
        app.add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default());
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(
                Self::setup
                    .system()
                    .label(WorldSystem::CameraSetup)
                    .after(WorldSystem::PlayerSetup),
            ),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(Self::update_camera_center.system()),
        );
        app.add_system(Self::update_raycast_with_cursor.system());
    }
}
