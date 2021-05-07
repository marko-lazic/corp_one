use crate::gui::metrics::Metrics;
use bevy::prelude::*;
use bevy_mod_raycast::{RayCastMethod, RayCastSource};

pub struct MyRaycastSet;

// Update our `RayCastSource` with the current cursor position every frame.
pub fn update_raycast_with_cursor(
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
