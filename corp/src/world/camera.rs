use bevy::prelude::*;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastMethod, RayCastSource};

use crate::world::WorldSystem;
use crate::{Game, GameState};

pub struct MyRaycastSet;

pub struct TopDownCamera {
    _zoom_sensitivity: f32,
}

impl Default for TopDownCamera {
    fn default() -> Self {
        TopDownCamera {
            _zoom_sensitivity: 1.,
        }
    }
}

pub struct TopDownCameraPlugin;

impl Plugin for TopDownCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default());
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(
                setup_camera
                    .system()
                    .label(WorldSystem::CameraSetup)
                    .after(WorldSystem::PlayerSetup),
            ),
        );
        app.add_system(update_raycast_with_cursor.system());
    }
}

struct CorpCameraBundle;

impl CorpCameraBundle {
    fn create() -> PerspectiveCameraBundle {
        let mat4 = Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        );
        PerspectiveCameraBundle {
            transform: Transform::from_matrix(mat4),
            ..Default::default()
        }
    }
}

fn setup_camera(mut commands: Commands, game: Res<Game>) {
    let camera = commands
        .spawn_bundle(CorpCameraBundle::create())
        .insert(TopDownCamera::default())
        .insert(RayCastSource::<MyRaycastSet>::new())
        .id();

    commands
        .entity(game.player.unwrap())
        .push_children(&[camera]);
}

// Update our `RayCastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<MyRaycastSet>>,
) {
    for mut pick_source in &mut query.iter_mut() {
        // Grab the most recent cursor event if it exists:
        if let Some(cursor_latest) = cursor.iter().last() {
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
        }
    }
}
