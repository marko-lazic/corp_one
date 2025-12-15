use crate::prelude::*;
use avian3d::prelude::*;
use bevy::{
    platform::collections::HashSet,
    prelude::*,
    window::{CursorOptions, PrimaryWindow},
};
use bevy_dolly::{
    dolly_type::Rig,
    prelude::{Arm, *},
};
use bevy_enhanced_input::prelude::*;
use bevy_replicon::prelude::ClientTriggerExt;
use bevy_tnua::{builtins::TnuaBuiltinWalk, controller::TnuaController};
use corp_shared::prelude::*;
use std::{f32::consts::PI, hash::Hash};

#[derive(Debug, Clone, Copy, Event)]
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

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .init_resource::<CursorWorld>()
            .init_resource::<HoverEntities>()
            .add_input_context::<PlayingInput>()
            .add_input_context::<StarMapInput>()
            .add_input_context::<UiInput>()
            .add_systems(OnEnter(GameState::StarMap), setup_star_map_input_controls)
            .add_systems(OnEnter(GameState::Playing), setup_playing_input_controls)
            .add_systems(OnExit(GameState::Playing), reset_cursor_visible)
            .add_systems(
                FixedUpdate,
                (
                    can_move,
                    detect_usable_targets,
                    update_cursor_world,
                    rotate_character,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_observer(apply_movement)
            .add_observer(apply_stop_movement)
            .add_observer(apply_orientation_mode)
            .add_observer(apply_use)
            .add_observer(apply_kill)
            .add_observer(apply_inventory)
            .add_observer(apply_exit)
            .add_observer(apply_window_cursor_visible)
            .add_observer(apply_starmap_iris)
            .add_observer(apply_starmap_liberte)
            .add_observer(apply_rotate_camera_clockwise)
            .add_observer(apply_rotate_camera_counter_clockwise)
            .add_observer(apply_camera_zoom_in)
            .add_observer(apply_camera_zoom_out)
            .add_observer(apply_aim)
            .add_observer(apply_aim_completed);
    }
}

fn setup_star_map_input_controls(mut commands: Commands) {
    commands.spawn((
        Name::new("Star Map Input Controls"),
        star_map_bindings(),
        ui_bindings(),
        DespawnOnExit(GameState::StarMap),
    ));
}

fn setup_playing_input_controls(mut commands: Commands) {
    commands.spawn((
        Name::new("Playing Input Controls"),
        playing_bindings(),
        ui_bindings(),
        DespawnOnExit(GameState::Playing),
    ));
}

fn playing_bindings() -> impl Bundle {
    (
        PlayingInput,
        actions!(
        PlayingInput[
            (
            Action::<Movement>::new(),
            DeadZone::default(),
            Bindings::spawn((Cardinal::wasd_keys(), Axial::left_stick())),
            ),
            (
                Action::<AimWeapon>::new(),
                bindings![MouseButton::Right, GamepadButton::LeftTrigger], // LT to aim (hold)
            ),
            (
                Action::<UseObject>::new(),
                bindings![KeyCode::KeyE, GamepadButton::South], // A to interact
            ),
            (
                Action::<KillMe>::new(),
                bindings![KeyCode::KeyK]
            ),
            (
                Action::<Shoot>::new(),
                bindings![MouseButton::Left, GamepadButton::RightTrigger], // RT to shoot
            ),
            (
                Action::<Inventory>::new(),
                bindings![KeyCode::KeyI, GamepadButton::North], // Y to open inventory
            ),
            (
                Action::<OrientationModeSwitch>::new(),
                bindings![KeyCode::Space, GamepadButton::RightThumb], // R3 toggles orientation mode
            ),
            (
                Action::<ZoomIn>::new(),
                bindings![KeyCode::Equal, GamepadButton::DPadUp], // D-Pad Up zoom in
            ),
            (
                Action::<ZoomOut>::new(),
                bindings![KeyCode::Minus, GamepadButton::DPadDown], // D-Pad Down zoom out
            ),
            (
                Action::<RotateClockwise>::new(),
                bindings![KeyCode::KeyZ, GamepadButton::DPadRight], // D-Pad Right rotate camera CW
            ),
            (
                Action::<RotateCounterClockwise>::new(),
                bindings![KeyCode::KeyC, GamepadButton::DPadLeft], // D-Pad Left rotate camera CCW
            )
        ]),
    )
}

fn star_map_bindings() -> impl Bundle {
    (
        StarMapInput,
        actions!(StarMapInput[
            (
                Action::<ColonyIris>::new(),
                bindings![ KeyCode::KeyI ]
            ),
            (
                Action::<ColonyLiberte>::new(),
                bindings![ KeyCode::KeyL ]
            )
        ]),
    )
}

fn ui_bindings() -> impl Bundle {
    (
        UiInput,
        actions!(
            UiInput[(
                Action::<ExitHold>::new(),
                Hold::new(0.5),
                bindings![KeyCode::Escape, GamepadButton::Start] // Start held to exit/confirm
            )]
        ),
    )
}

fn reset_cursor_visible(
    mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    primary_cursor_options.visible = true;
}

fn can_move(mut query: Query<(&mut CharacterMovement, &Health), Changed<Health>>) {
    for (mut character_movement, health) in &mut query {
        character_movement.can_move = health.is_alive();
    }
}

const HORIZONTAL_FOV: f32 = 2.0 * PI / 3.0; // 120 degrees in radians
const NUM_RAYS: usize = 10; // Number of rays to evenly distribute within the FOV
const RAY_SPACING: f32 = HORIZONTAL_FOV / (NUM_RAYS - 1) as f32; // Angle between each ray
const LOOKUP_RANGE: f32 = 2.0; // Range of rays

fn detect_usable_targets(
    player_entity: Single<Entity, With<LocalPlayer>>,
    mut r_hover_entities: ResMut<HoverEntities>,
    q_transform: Query<&Transform>,
    q_spatial: SpatialQuery,
    q_use: Query<&Use>,
    mut commands: Commands,
    r_cursor_world: Res<CursorWorld>,
    mut gizmos: Gizmos<DebugGizmos>,
) -> Result {
    let t_player = q_transform.get(*player_entity)?;

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
        return Ok(());
    };
    if direction.as_vec3() == Vec3::ZERO {
        return Ok(());
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
                    return Ok(());
                };
                let usable_target = UsableTarget {
                    entity: ray_hit_data.entity,
                    hit_point: transform.translation,
                };
                r_hover_entities.insert(usable_target);
                commands.trigger(usable_target)
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
    Ok(())
}

fn update_cursor_world(
    mut r_cursor_world: ResMut<CursorWorld>,
    mut orientation_mode: Single<&mut OrientationMode, With<LocalPlayer>>,
    window: Single<&Window>,
    follow_pos: Single<&Transform, With<MainCameraFollow>>,
    s_camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = *s_camera;
    let belt_level = Vec3::new(0.0, 1.0, 0.0);

    let ray = window
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

    if let OrientationMode::Location(_) = **orientation_mode {
        **orientation_mode =
            OrientationMode::Location(Vec2::new(mouse_ground_pos.x, mouse_ground_pos.z));
    }
}

fn rotate_character(
    mut query: Query<
        (&mut Transform, &CharacterMovement, &OrientationMode),
        Or<(Changed<OrientationMode>, Changed<CharacterMovement>)>,
    >,
) {
    for (mut transform, character_movement, orientation) in &mut query {
        match orientation {
            OrientationMode::Direction => {
                if character_movement.direction == Vec3::ZERO {
                    continue;
                }
                let direction_2d = Vec2::new(
                    character_movement.direction.x,
                    character_movement.direction.z,
                );
                let rotation_angle = std::f32::consts::PI + direction_2d.angle_to(Vec2::Y);

                let current_rotation = transform.rotation;
                let target_rotation = Quat::from_rotation_y(rotation_angle);
                let interpolated_rotation = current_rotation.lerp(target_rotation, 0.1);

                transform.rotation = interpolated_rotation;
            }
            OrientationMode::Location(location_2d) => {
                let target_position =
                    Vec3::new(location_2d.x, transform.translation.y, location_2d.y);
                let look_direction = transform.translation - target_position; // Reverse direction vector

                if look_direction.length_squared() > 0.0 {
                    let rotation_angle = look_direction.x.atan2(look_direction.z);

                    let current_rotation = transform.rotation;
                    let target_rotation = Quat::from_rotation_y(rotation_angle);
                    let interpolated_rotation = current_rotation.lerp(target_rotation, 0.1);

                    transform.rotation = interpolated_rotation;
                }
            }
        }
    }
}

fn apply_movement(
    fire_movement: On<Fire<Movement>>,
    cam_transform: Single<&Transform, With<MainCamera>>,
    mut movement: Single<&mut CharacterMovement, With<LocalPlayer>>,
    mut tnua: Single<&mut TnuaController>,
) {
    let cam_forward = {
        let f = cam_transform.rotation.mul_vec3(Vec3::Z);
        // Invert it to account for camera looking down -Z
        -Vec3::new(f.x, 0.0, f.z).normalize_or_zero()
    };
    let cam_right = {
        let r = cam_transform.rotation.mul_vec3(Vec3::X);
        Vec3::new(r.x, 0.0, r.z).normalize_or_zero()
    };

    let input_axis = fire_movement.value;
    let input_forward = cam_forward * input_axis.y;
    let input_strafe = cam_right * input_axis.x;

    movement.direction = (input_forward + input_strafe).normalize_or_zero();
    if movement.can_move {
        movement.velocity = movement.direction * movement.speed;
        tnua.basis(TnuaBuiltinWalk {
            // The `desired_velocity` determines how the character will move.
            desired_velocity: movement.velocity,
            // The `float_height` must be greater (even if by little) from the distance between the
            // character's center and the lowest point of its collider.
            float_height: 1.5,
            ..default()
        });
    }
}

fn apply_stop_movement(
    _completed_move: On<Complete<Movement>>,
    mut tnua: Single<&mut TnuaController>,
    mut player_movement: Single<&mut CharacterMovement, With<LocalPlayer>>,
) {
    player_movement.velocity = Vec3::ZERO;
    tnua.basis(TnuaBuiltinWalk {
        desired_velocity: Vec3::ZERO,
        float_height: 1.5,
        ..default()
    });
}

fn apply_orientation_mode(
    _orientation_mode: On<Start<OrientationModeSwitch>>,
    r_cursor_world: Res<CursorWorld>,
    mut player_orientation_mode: Single<&mut OrientationMode, With<LocalPlayer>>,
) {
    **player_orientation_mode = match **player_orientation_mode {
        OrientationMode::Direction => {
            OrientationMode::Location(Vec2::new(r_cursor_world.0.x, r_cursor_world.0.z))
        }
        OrientationMode::Location(_) => OrientationMode::Direction,
    }
}

fn apply_use(
    _use_action: On<Start<UseObject>>,
    mut commands: Commands,
    r_use_entity: Res<HoverEntities>,
    player_entity: Single<Entity, With<LocalPlayer>>,
    q_use: Query<&Use>,
) -> Result {
    for entity_target in r_use_entity.iter() {
        if q_use.contains(entity_target.entity) {
            commands.trigger(UseCommand::new(entity_target.entity, *player_entity));
        }
    }
    Ok(())
}

fn apply_kill(_kill_action: On<Start<KillMe>>, mut commands: Commands) -> Result {
    commands.client_trigger(KillMeCommand);
    Ok(())
}

fn apply_inventory(
    _inventory_action: On<Start<Inventory>>,
    container_query: Query<(&Name, &Contains), With<LocalPlayer>>,
    q_item_name: Query<&Name, With<Item>>,
) {
    for (container_name, contains) in &container_query {
        println!("{:?} contains:", container_name);
        for item_entity in contains.iter() {
            if let Ok(name) = q_item_name.get(item_entity) {
                println!("  - {}", name);
            }
        }
    }
}

fn apply_exit(excape_action: On<Ongoing<ExitHold>>, mut commands: Commands) {
    if excape_action.elapsed_secs > 0.4 {
        commands.trigger(RequestExit);
    }
}

fn apply_window_cursor_visible(
    _escape_action: On<Start<ExitHold>>,
    mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    primary_cursor_options.visible = !primary_cursor_options.visible;
}

fn apply_starmap_iris(_colony_iris: On<Start<ColonyIris>>, mut commands: Commands) {
    info!("apply_starmap_iris");
    commands.trigger(RequestConnect(Colony::Iris));
}

fn apply_starmap_liberte(_colony_liberte: On<Start<ColonyLiberte>>, mut commands: Commands) {
    info!("apply_starmap_liberte");
    commands.trigger(RequestConnect(Colony::Liberte));
}

fn apply_rotate_camera_clockwise(
    _rotate_clockwise: On<Start<RotateClockwise>>,
    mut rig: Single<&mut Rig>,
) {
    let camera_yp = rig.driver_mut::<YawPitch>();
    camera_yp.rotate_yaw_pitch(-45.0, 0.0);
}

fn apply_rotate_camera_counter_clockwise(
    _rotate_counter_clockwise: On<Start<RotateCounterClockwise>>,
    mut rig: Single<&mut Rig>,
) {
    let camera_yp = rig.driver_mut::<YawPitch>();
    camera_yp.rotate_yaw_pitch(45.0, 0.0);
}

fn apply_camera_zoom_in(
    _zoom_in: On<Fire<ZoomIn>>,
    mut rig: Single<&mut Rig>,
    time: Res<Time<Fixed>>,
) {
    if let Some(arm) = rig.try_driver_mut::<Arm>() {
        let mut xz = arm.offset;
        xz.z = (xz.z - 4.0 * time.delta_secs()).abs();
        arm.offset = xz.clamp_length_min(6.0);
    }
}

fn apply_camera_zoom_out(
    _zoom_out: On<Fire<ZoomOut>>,
    mut rig: Single<&mut Rig>,
    time: Res<Time<Fixed>>,
) {
    if let Some(arm) = rig.try_driver_mut::<Arm>() {
        let mut xz = arm.offset;
        xz.z = (xz.z + 4.0 * time.delta_secs()).abs();
        arm.offset = xz.clamp_length_max(18.0);
    }
}

fn apply_aim(_aim_action: On<Start<AimWeapon>>, mut r_camera_modifier: ResMut<CameraModifier>) {
    r_camera_modifier.aim_zoom_factor = 1.8;
}

fn apply_aim_completed(
    _aim_action: On<Complete<AimWeapon>>,
    mut r_camera_modifier: ResMut<CameraModifier>,
) {
    r_camera_modifier.aim_zoom_factor = 1.0;
}

#[derive(Component)]
struct PlayingInput;

#[derive(Debug, InputAction)]
#[action_output(Vec2)]
struct Movement;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct AimWeapon;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct OrientationModeSwitch;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct UseObject;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct Inventory;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct Shoot;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct KillMe;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct ZoomIn;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct ZoomOut;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct RotateClockwise;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct RotateCounterClockwise;

#[derive(Component)]
struct StarMapInput;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct ColonyIris;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct ColonyLiberte;

#[derive(Component)]
struct UiInput;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct ExitHold;
