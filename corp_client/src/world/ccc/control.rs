use std::f32::consts::PI;

use bevy::{app::AppExit, prelude::*};
use bevy_rapier3d::{pipeline::QueryFilter, plugin::RapierContext};
use leafwing_input_manager::prelude::*;

use corp_shared::prelude::{
    Health, InteractionEvent, InteractionObjectType, Player, UseDoorEvent, UseTerritoryNodeEvent,
};

use crate::{
    asset::Colony,
    gui::prelude::{DebugGizmos, DebugGuiEvent},
    sound::InteractionSoundEvent,
    state::GameState,
    world::{
        ccc::{ControlMovement, DoubleTap, MainCamera, MainCameraFollow, OrientationMode},
        colony::prelude::{BarrierControl, BarrierField, VortInEvent},
    },
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ControlSet {
    PlayingInput,
    StarmapInput,
}

#[derive(Resource, Default, Deref, DerefMut)]
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

pub struct UsableEntity {
    entity: Entity,
    hit_point: Vec3,
}

impl UsableEntity {
    pub fn new(entity: Entity, hit_point: Vec3) -> Self {
        Self { entity, hit_point }
    }

    pub fn get(&self) -> (Entity, Vec3) {
        (self.entity, self.hit_point)
    }
}

#[derive(Resource, Default)]
pub struct UseEntity(pub Vec<UsableEntity>);

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
        let mut input = InputMap::default();
        input
            .insert(UIAction::Escape, KeyCode::Escape)
            .insert(UIAction::ColonyIris, KeyCode::KeyI)
            .insert(UIAction::ColonyPlayground, KeyCode::KeyP)
            .insert(UIAction::ColonyLiberte, KeyCode::KeyL);
        input
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Forward,
    Backward,
    Left,
    Right,
    Aim,
    OrientationMode,
    Use,
    Shoot,
    CameraZoomIn,
    CameraZoomOut,
    CameraRotateClockwise,
    CameraRotateCounterClockwise,
    Kill,
}

impl PlayerAction {
    pub fn player_input_map() -> InputMap<PlayerAction> {
        let mut input = InputMap::default();
        input
            // Movement
            .insert(PlayerAction::Forward, KeyCode::KeyW)
            .insert(PlayerAction::Backward, KeyCode::KeyS)
            .insert(PlayerAction::Left, KeyCode::KeyA)
            .insert(PlayerAction::Right, KeyCode::KeyD)
            // Weapon
            .insert(PlayerAction::Aim, MouseButton::Right)
            // Abilities
            .insert(PlayerAction::Use, KeyCode::KeyE)
            .insert(PlayerAction::Kill, KeyCode::KeyK)
            .insert(PlayerAction::Shoot, MouseButton::Left)
            // Options
            .insert(PlayerAction::OrientationMode, KeyCode::Space)
            .insert(PlayerAction::CameraZoomIn, KeyCode::Equal)
            .insert(PlayerAction::CameraZoomOut, KeyCode::Minus)
            .insert(PlayerAction::CameraRotateClockwise, KeyCode::KeyZ)
            .insert(PlayerAction::CameraRotateCounterClockwise, KeyCode::KeyC);

        input
    }
}

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .init_resource::<PlayerEntity>()
            .init_resource::<CursorWorld>()
            .init_resource::<UseEntity>()
            .add_plugins(InputManagerPlugin::<UIAction>::default())
            .init_resource::<ActionState<UIAction>>()
            .insert_resource(UIAction::ui_input_map())
            .add_systems(Update, double_tap_to_exit)
            .add_systems(
                Update,
                (
                    update_cursor_world,
                    player_control_movement,
                    player_control_orientation,
                    detect_interactable_objects,
                    create_use_event,
                    kill,
                    toggle_window_cursor_visible,
                )
                    .chain()
                    .in_set(ControlSet::PlayingInput)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), enable_cursor_visible)
            .add_systems(
                Update,
                starmap_keyboard
                    .in_set(ControlSet::StarmapInput)
                    .run_if(in_state(GameState::StarMap)),
            );
    }
}

fn double_tap_to_exit(
    action_state: Res<ActionState<UIAction>>,
    r_time: Res<Time>,
    mut ev_exit_app: EventWriter<AppExit>,
    mut l_double_tap: Local<DoubleTap>,
) {
    if action_state.just_pressed(&UIAction::Escape) {
        l_double_tap.increment();
    }
    l_double_tap.tick(r_time.delta()).on_complete(|| {
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
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
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
    q_action_state: Query<&ActionState<PlayerAction>, With<Player>>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mut q_movement: Query<&mut ControlMovement, With<Player>>,
) {
    let Ok(cam) = q_camera.get_single() else {
        warn!("Can not find Transform, With<MainCamera>");
        return;
    };

    let cam_forward = Vec3::new(
        cam.rotation.mul_vec3(Vec3::Z).x,
        0.0,
        cam.rotation.mul_vec3(Vec3::Z).z,
    )
    .normalize_or_zero();
    let cam_right = Vec3::new(
        cam.rotation.mul_vec3(Vec3::X).x,
        0.0,
        cam.rotation.mul_vec3(Vec3::X).z,
    )
    .normalize_or_zero();

    let Ok(mut movement) = q_movement.get_single_mut() else {
        warn!("Can not find ControlMovement, With<Player>");
        return;
    };

    let Ok(action_state) = q_action_state.get_single() else {
        warn!("Can not find ActionState<PlayerAction>, With<Player>");
        return;
    };

    let mut direction = Vec3::ZERO;
    if action_state.pressed(&PlayerAction::Forward) {
        direction -= cam_forward;
    }
    if action_state.pressed(&PlayerAction::Backward) {
        direction += cam_forward;
    }
    if action_state.pressed(&PlayerAction::Left) {
        direction -= cam_right;
    }
    if action_state.pressed(&PlayerAction::Right) {
        direction += cam_right;
    }

    movement.direction = direction;
}

fn player_control_orientation(
    r_cursor_world: Res<CursorWorld>,
    q_action_state: Query<&ActionState<PlayerAction>, With<Player>>,
    mut q_orientation: Query<&mut OrientationMode, With<Player>>,
) {
    let Ok(action_state) = q_action_state.get_single() else {
        warn!("PlayerAction state is missing.");
        return;
    };
    if action_state.just_pressed(&PlayerAction::OrientationMode) {
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

fn detect_interactable_objects(
    r_player_entity: Res<PlayerEntity>,
    mut r_use_entity: ResMut<UseEntity>,
    q_transform: Query<&Transform>,
    q_parent: Query<&Parent>,
    r_rapier_context: Res<RapierContext>,
    q_object_type: Query<&InteractionObjectType>,
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
    let origin = t_player.translation;
    for i in 0..NUM_RAYS {
        // Calculate the horizontal angle for the current ray
        let ray_angle = (i as f32 * RAY_SPACING) - (HORIZONTAL_FOV / 2.0);

        // Create a rotation quaternion for the current ray angle
        let ray_rotation = Quat::from_rotation_y(ray_angle);

        // Rotate the player's forward vector to get the ray's direction
        let ray_direction = ray_rotation.mul_vec3(t_player.forward().into());

        // Calculate the endpoint of the ray
        let ray_end = origin + ray_direction * LOOKUP_RANGE;

        let direction = ray_end - origin;

        if direction == Vec3::ZERO {
            warn!("FOV ray direction is zero");
            return;
        }

        let ray = Ray3d::new(origin, direction);
        rays.push(ray);
    }
    // Cast cursor ray
    let cursor = r_cursor_world.0;
    let direction = Vec3::new(cursor.x, origin.y, cursor.z) - origin;
    if direction == Vec3::ZERO {
        return;
    }
    let ray = Ray3d::new(origin, direction);
    rays.push(ray);

    // Clear any previously selected entities
    r_use_entity.0.clear();
    // Collect usable objects
    for ray in rays {
        if let Some((entity, real)) = r_rapier_context.cast_ray(
            ray.origin,
            ray.direction.into(),
            LOOKUP_RANGE,
            true,
            QueryFilter::only_fixed().exclude_sensors(),
        ) {
            if let Ok(_) = q_object_type.get(entity.clone()) {
                gizmos.ray(
                    ray.origin,
                    (ray.origin + ray.direction * real) - ray.origin,
                    bevy::color::palettes::tailwind::RED_700,
                );

                let Ok(parent) = q_parent.get(entity) else {
                    warn!("Failed to retrieve parent for entity {entity:?}");
                    return;
                };

                let Ok(transform) = q_transform.get(parent.get()) else {
                    warn!("Failed to retrieve transform for entity {entity:?}");
                    return;
                };
                r_use_entity
                    .0
                    .push(UsableEntity::new(entity, transform.translation));
                e_debug_gui.send(DebugGuiEvent::Interaction(entity));
            } else {
                gizmos.ray(
                    ray.origin,
                    (ray.origin + ray.direction * real) - ray.origin,
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
    r_use_entity: Res<UseEntity>,
    r_player_entity: Res<PlayerEntity>,
    q_action_state: Query<&ActionState<PlayerAction>, With<Player>>,
    q_interaction_object: Query<&InteractionObjectType>,
    mut ev_interaction_sound: EventWriter<InteractionSoundEvent>,
) {
    let Ok(action_state) = q_action_state.get_single() else {
        warn!("PlayerAction state is missing.");
        return;
    };
    if action_state.just_pressed(&PlayerAction::Use) {
        let Some(player) = r_player_entity.get() else {
            return;
        };
        let Some(usable_entity) = r_use_entity.0.iter().last() else {
            return;
        };
        let usable_entity = usable_entity.entity;

        let Ok(interaction_object) = q_interaction_object.get(usable_entity) else {
            return;
        };

        match interaction_object {
            InteractionObjectType::DoorControl => {
                commands.add(move |w: &mut World| {
                    let barrier_fields = {
                        let fields = w.query::<&BarrierField>().iter(&w).collect::<Vec<_>>();
                        fields
                    };

                    if let Some(barrier_control) = w.get::<BarrierControl>(usable_entity) {
                        let target_entity = barrier_fields
                            .iter()
                            .find(|&&bf| bf.name == barrier_control.barrier_field_name)
                            .map(|&bf| bf.entity);

                        if let Some(target_entity) = target_entity {
                            w.send_event(InteractionEvent::new(
                                player,
                                target_entity,
                                UseDoorEvent,
                            ));
                        } else {
                            warn!(
                                "Didn't find any barrier field with name: {}",
                                barrier_control.barrier_field_name
                            );
                        }
                    } else {
                        warn!("Barrier control not found for entity: {:?}", usable_entity);
                    }
                });
            }
            InteractionObjectType::TerritoryNode => {
                commands.add(move |w: &mut World| {
                    w.send_event(InteractionEvent::new(
                        player,
                        usable_entity,
                        UseTerritoryNodeEvent,
                    ));
                });
            }
        }

        ev_interaction_sound.send(InteractionSoundEvent);
    }
}

fn kill(
    q_action_state: Query<&ActionState<PlayerAction>, With<Player>>,
    mut q_player_health: Query<&mut Health, With<Player>>,
) {
    let Ok(action_state) = q_action_state.get_single() else {
        warn!("PlayerAction state is missing.");
        return;
    };
    if action_state.just_pressed(&PlayerAction::Kill) {
        if let Some(mut health) = q_player_health.iter_mut().next() {
            health.kill_mut();
        }
    }
}

fn toggle_window_cursor_visible(
    action_state: Res<ActionState<UIAction>>,
    mut q_windows: Query<&mut Window>,
) {
    if action_state.just_pressed(&UIAction::Escape) {
        let mut window = q_windows.single_mut();
        window.cursor.visible = !window.cursor.visible;
    }
}

fn enable_cursor_visible(mut q_windows: Query<&mut Window>) {
    let mut window = q_windows.single_mut();
    window.cursor.visible = true;
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
        app.send_input(KeyCode::Escape);
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
            .init_state::<GameState>()
            .add_plugins(MinimalPlugins)
            .add_plugins((InputPlugin, ControlPlugin));
        app
    }
}
