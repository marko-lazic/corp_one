use bevy::prelude::*;
use bevy_mod_raycast::{RayCastMethod, RayCastSource};

use crate::gui::metrics::Metrics;

pub struct MyRaycastSet;

// Update our `RayCastSource` with the current cursor position every frame.
pub fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut ray_cast_source: Query<&mut RayCastSource<MyRaycastSet>>,
    mut metrics: ResMut<Metrics>,
) {
    for mut pick_source in &mut ray_cast_source.iter_mut() {
        // Grab the most recent cursor event if it exists:
        if let Some(cursor_latest) = cursor.iter().last() {
            metrics.mouse_screen_position = cursor_latest.position;
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
            if let Some((_entity, intersect)) = pick_source.intersect_top() {
                metrics.mouse_world_position = intersect.position();
            }
        }
    }
}
