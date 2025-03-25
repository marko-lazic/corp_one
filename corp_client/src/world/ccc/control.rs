use crate::prelude::*;
use avian3d::prelude::*;
use bevy::{app::AppExit, prelude::*, utils::HashSet};
use corp_shared::prelude::*;
use leafwing_input_manager::prelude::*;
use std::{f32::consts::PI, hash::Hash};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ControlSet {
    PlayingInput,
    StarMapInput,
}

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct PlayerEntity(pub Option<Entity>);

impl PlayerEntity {
    pub fn get(&self) -> Option<Entity> {
        self.0
    }
}

impl From<Entity> for PlayerEntity {
    fn from(entity: Entity) -> Self {
        Self(Some(entity))
    }
}

#[derive(Debug, Clone)]
pub struct UsableTarget {
    pub entity: Entity,
    /// Position of targeted usable entity.
    pub hit_point: Vec3,
}

impl PartialEq for UsableTarget {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

impl Eq for UsableTarget {}

impl Hash for UsableTarget {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entity.hash(state);
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct HoverEntities(pub HashSet<UsableTarget>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CursorWorld(Vec3);

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum UIAction {
    Escape,
    ColonyIris,
    ColonyLiberte,
    ColonyPlayground,
}

impl UIAction {
    fn ui_input_map() -> InputMap<UIAction> {
        use UIAction::*;
        let mut input = InputMap::default();
        input
            .insert(Escape, KeyCode::Escape)
            .insert(ColonyIris, KeyCode::KeyI)
            .insert(ColonyPlayground, KeyCode::KeyP)
            .insert(ColonyLiberte, KeyCode::KeyL);
        input
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CharacterAction {
    Move,
    Aim,
    OrientationMode,
    Use,
    Inventory,
    Shoot,
    CameraZoomIn,
    CameraZoomOut,
    CameraRotateClockwise,
    CameraRotateCounterClockwise,
    Kill,
}

impl Actionlike for CharacterAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::DualAxis,
            _ => InputControlKind::Button,
        }
    }
}

impl CharacterAction {
    pub fn player_input_map() -> InputMap<CharacterAction> {
        use CharacterAction::*;
        let mut input = InputMap::default();
        input
            // Movement
            .insert_dual_axis(Move, VirtualDPad::wasd())
            // Weapon
            .insert(Aim, MouseButton::Right)
            // Abilities
            .insert(Use, KeyCode::KeyE)
            .insert(Kill, KeyCode::KeyK)
            .insert(Shoot, MouseButton::Left)
            // User Interface
            .insert(Inventory, KeyCode::KeyI)
            // Options
            .insert(OrientationMode, KeyCode::Space)
            .insert(CameraZoomIn, KeyCode::Equal)
            .insert(CameraZoomOut, KeyCode::Minus)
            .insert(CameraRotateClockwise, KeyCode::KeyZ)
            .insert(CameraRotateCounterClockwise, KeyCode::KeyC);

        input
    }
}

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CharacterAction>::default())
            .init_resource::<PlayerEntity>()
            .init_resource::<CursorWorld>()
            .init_resource::<HoverEntities>()
            .add_plugins(InputManagerPlugin::<UIAction>::default())
            .init_resource::<ActionState<UIAction>>()
            .insert_resource(UIAction::ui_input_map())
            .add_systems(FixedUpdate, double_tap_to_exit)
            .add_systems(
                FixedUpdate,
                (
                    detect_usable_targets,
                    update_cursor_world,
                    player_control_movement,
                    player_control_orientation,
                    create_use_event,
                    kill,
                    log_inventory,
                    toggle_window_cursor_visible,
                )
                    .chain()
                    .in_set(ControlSet::PlayingInput)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), enable_cursor_visible)
            .add_systems(
                FixedUpdate,
                starmap_keyboard
                    .in_set(ControlSet::StarMapInput)
                    .run_if(in_state(GameState::StarMap)),
            );
    }
}

fn double_tap_to_exit(
    mut commands: Commands,
    action_state: Res<ActionState<UIAction>>,
    r_time: Res<Time<Fixed>>,
    mut ev_exit_app: EventWriter<AppExit>,
    mut l_double_tap: Local<DoubleTap>,
) {
    if action_state.just_pressed(&UIAction::Escape) {
        l_double_tap.increment();
    }
    l_double_tap.tick(r_time.delta()).on_complete(|| {
        commands.disconnect_client();
        ev_exit_app.send(AppExit::Success);
    });
}

fn update_cursor_world(
    q_windows: Query<&Window>,
    mut r_cursor_world: ResMut<CursorWorld>,
    q_follow_cam: Query<&Transform, With<MainCameraFollow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_orientation: Query<&mut OrientationMode, With<Player>>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        return;
    };
    let belt_level = Vec3::new(0.0, 1.0, 0.0);
    let Ok(follow_pos) = q_follow_cam.get_single() else {
        return;
    };

    let ray = q_windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .unwrap_or_else(|| Ray3d {
            origin: follow_pos.translation,
            direction: follow_pos.down(),
        });

    // Calculate if and where the ray is hitting the belt (of the character height) level.
    let Some(distance) = ray.intersect_plane(belt_level, InfinitePlane3d::new(Vec3::Y)) else {
        return;
    };
    let mouse_ground_pos = ray.get_point(distance);
    r_cursor_world.0 = mouse_ground_pos;

    for mut orientation_mode in &mut q_orientation {
        if let OrientationMode::Location(_) = *orientation_mode {
            *orientation_mode =
                OrientationMode::Location(Vec2::new(mouse_ground_pos.x, mouse_ground_pos.z));
        }
    }
}

fn player_control_movement(
    q_action_state: Query<&ActionState<CharacterAction>, With<Player>>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mut q_movement: Query<&mut ControlMovement, With<Player>>,
) {
    let Ok(cam_transform) = q_camera.get_single() else {
        warn!("Can not find Transform, With<MainCamera>");
        return;
    };

    let cam_forward = {
        let f = cam_transform.rotation.mul_vec3(Vec3::Z);
        // Invert it to account for camera looking down -Z
        -Vec3::new(f.x, 0.0, f.z).normalize_or_zero()
    };
    let cam_right = {
        let r = cam_transform.rotation.mul_vec3(Vec3::X);
        Vec3::new(r.x, 0.0, r.z).normalize_or_zero()
    };

    let Ok(action_state) = q_action_state.get_single() else {
        warn!("Can not find ActionState<PlayerAction>, With<Player>");
        return;
    };

    let input_axis = action_state
        .axis_pair(&CharacterAction::Move)
        .clamp_length_max(1.0);

    let input_forward = cam_forward * input_axis.y;
    let input_strafe = cam_right * input_axis.x;

    let Ok(mut movement) = q_movement.get_single_mut() else {
        warn!("Can not find ControlMovement, With<Player>");
        return;
    };

    movement.direction = (input_forward + input_strafe).normalize_or_zero();
}

fn player_control_orientation(
    r_cursor_world: Res<CursorWorld>,
    q_action_state: Query<&ActionState<CharacterAction>, With<Player>>,
    mut q_orientation: Query<&mut OrientationMode, With<Player>>,
) {
    let Ok(action_state) = q_action_state.get_single() else {
        warn!("PlayerAction state is missing.");
        return;
    };
    if action_state.just_pressed(&CharacterAction::OrientationMode) {
        for mut orientation_mode in &mut q_orientation {
            *orientation_mode = match *orientation_mode {
                OrientationMode::Direction => {
                    OrientationMode::Location(Vec2::new(r_cursor_world.x, r_cursor_world.z))
                }
                OrientationMode::Location(_) => OrientationMode::Direction,
            }
        }
    }
}

const HORIZONTAL_FOV: f32 = 2.0 * PI / 3.0; // 120 degrees in radians
const NUM_RAYS: usize = 10; // Number of rays to evenly distribute within the FOV
const RAY_SPACING: f32 = HORIZONTAL_FOV / (NUM_RAYS - 1) as f32; // Angle between each ray
const LOOKUP_RANGE: f32 = 2.0; // Range of rays

fn detect_usable_targets(
    r_player_entity: Res<PlayerEntity>,
    mut r_hover_entities: ResMut<HoverEntities>,
    q_transform: Query<&Transform>,
    q_spatial: SpatialQuery,
    q_use: Query<&Use>,
    mut e_debug_gui: EventWriter<DebugGuiEvent>,
    r_cursor_world: Res<CursorWorld>,
    mut gizmos: Gizmos<DebugGizmos>,
) {
    let Some(e_player) = r_player_entity.get() else {
        return;
    };

    let Ok(t_player) = q_transform.get(e_player) else {
        return;
    };

    let mut rays = Vec::new();
    // Cast FOV rays
    let offset_height = Vec3::Y / 2.0; // Adjustable height of rays origin
    let origin = t_player.translation - offset_height;
    for i in 0..NUM_RAYS {
        // Calculate the horizontal angle for the current ray
        let ray_angle = (i as f32 * RAY_SPACING) - (HORIZONTAL_FOV / 2.0);

        // Create a rotation quaternion for the current ray angle
        let ray_rotation = Quat::from_rotation_y(ray_angle);

        // Rotate the player's forward vector to get the ray's direction
        let ray_direction = ray_rotation.mul_vec3(t_player.forward().into());

        // Calculate the endpoint of the ray
        let ray_end = origin + ray_direction * LOOKUP_RANGE;

        let Ok(direction) = Dir3::new(ray_end - origin) else {
            continue;
        };

        if direction.as_vec3() == Vec3::ZERO {
            warn!("FOV ray direction is zero");
            continue;
        }

        let ray = Ray3d::new(origin, direction);
        rays.push(ray);
    }
    // Cast cursor ray
    let cursor = r_cursor_world.0;
    let Ok(direction) = Dir3::new(Vec3::new(cursor.x, origin.y, cursor.z) - origin) else {
        return;
    };
    if direction.as_vec3() == Vec3::ZERO {
        return;
    }
    let ray = Ray3d::new(origin, direction);
    rays.push(ray);

    // Clear any previously selected entities
    r_hover_entities.clear();
    // Collect usable structures
    for ray in rays {
        if let Some(ray_hit_data) = q_spatial.cast_ray(
            ray.origin,
            ray.direction.into(),
            LOOKUP_RANGE,
            true,
            &SpatialQueryFilter::from_mask([GameLayer::Structure, GameLayer::Sensor]),
        ) {
            if let Ok(_) = q_use.get(ray_hit_data.entity) {
                gizmos.ray(
                    ray.origin,
                    (ray.origin + ray.direction * ray_hit_data.distance) - ray.origin,
                    bevy::color::palettes::tailwind::RED_700,
                );

                let Ok(transform) = q_transform.get(ray_hit_data.entity) else {
                    warn!("Err get transform for entity {:?}", ray_hit_data.entity);
                    return;
                };
                r_hover_entities.insert(UsableTarget {
                    entity: ray_hit_data.entity,
                    hit_point: transform.translation,
                });
                e_debug_gui.send(DebugGuiEvent::Interaction(ray_hit_data.entity));
            } else {
                gizmos.ray(
                    ray.origin,
                    (ray.origin + ray.direction * ray_hit_data.distance) - ray.origin,
                    bevy::color::palettes::tailwind::RED_700,
                );
            }
        } else {
            gizmos.ray(
                ray.origin,
                (ray.origin + ray.direction * 2.0) - ray.origin,
                bevy::color::palettes::tailwind::RED_700,
            );
        }
    }
}

fn create_use_event(
    mut commands: Commands,
    r_use_entity: Res<HoverEntities>,
    r_player_entity: Res<PlayerEntity>,
    q_action_state: Query<&ActionState<CharacterAction>, With<Player>>,
    q_use: Query<&Use>,
    mut ev_interaction_sound: EventWriter<InteractionSoundEvent>,
) {
    let Ok(action_state) = q_action_state.get_single() else {
        warn!("PlayerAction state is missing.");
        return;
    };

    if action_state.just_pressed(&CharacterAction::Use) {
        let Some(player) = r_player_entity.get() else {
            return;
        };

        for entity_target in r_use_entity.0.iter() {
            if let Ok(_) = q_use.get(entity_target.entity) {
                commands.trigger_targets(UseEvent::new(player), entity_target.entity);
                ev_interaction_sound.send(InteractionSoundEvent);
            }
        }
    }
}

fn kill(
    q_action_state: Query<&ActionState<CharacterAction>, With<Player>>,
    mut q_player_health: Query<&mut Health, With<Player>>,
) {
    let Ok(action_state) = q_action_state.get_single() else {
        warn!("PlayerAction state is missing.");
        return;
    };
    if action_state.just_pressed(&CharacterAction::Kill) {
        if let Some(mut health) = q_player_health.iter_mut().next() {
            health.kill_mut();
        }
    }
}

fn log_inventory(
    q_inventory: Query<&Inventory, With<Player>>,
    q_name: Query<&Name>,
    q_action_state: Query<&ActionState<CharacterAction>, With<Player>>,
) {
    let Ok(action_state) = q_action_state.get_single() else {
        warn!("PlayerAction state is missing.");
        return;
    };
    if action_state.just_pressed(&CharacterAction::Inventory) {
        if let Ok(inventory) = q_inventory.get_single() {
            let item_names: Vec<String> = inventory
                .items
                .iter()
                .filter_map(|&item| q_name.get(item).ok().map(|name| name.to_string()))
                .collect();

            let output = format!("Inventory: [{}]", item_names.join(", "));
            info!("{}", output);
        }
    }
}

fn toggle_window_cursor_visible(
    action_state: Res<ActionState<UIAction>>,
    mut q_windows: Query<&mut Window>,
) {
    if action_state.just_pressed(&UIAction::Escape) {
        let mut window = q_windows.single_mut();
        window.cursor_options.visible = !window.cursor_options.visible;
    }
}

fn enable_cursor_visible(mut q_windows: Query<&mut Window>) {
    let mut window = q_windows.single_mut();
    window.cursor_options.visible = true;
}

fn starmap_keyboard(
    action_state: Res<ActionState<UIAction>>,
    mut ev_vort_in: EventWriter<VortInEvent>,
) {
    if action_state.just_pressed(&UIAction::ColonyIris) {
        ev_vort_in.send(VortInEvent::vort(Colony::Iris));
    } else if action_state.just_pressed(&UIAction::ColonyLiberte) {
        ev_vort_in.send(VortInEvent::vort(Colony::Liberte));
    } else if action_state.just_pressed(&UIAction::ColonyPlayground) {
        ev_vort_in.send(VortInEvent::vort(Colony::Playground));
    }
}

#[cfg(test)]
mod tests {
    use bevy::input::InputPlugin;

    use corp_shared::prelude::*;

    use super::*;

    #[test]
    fn send_input() {
        // given
        let mut app = setup();

        // when
        KeyCode::Escape.press(app.world_mut());

        app.update();

        // then
        assert!(app
            .world()
            .resource::<ActionState<UIAction>>()
            .pressed(&UIAction::Escape));
    }

    fn setup() -> App {
        let mut app = App::new();
        app.init_time()
            .add_plugins(MinimalPlugins)
            .add_plugins((InputPlugin, ControlPlugin));
        app
    }
}
