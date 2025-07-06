use crate::prelude::*;
use avian3d::prelude::*;
use bevy::{platform::collections::HashSet, prelude::*};
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
            .add_input_context::<OnFoot>()
            .add_input_context::<OnStarMap>()
            .add_input_context::<OnUi>()
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
            .add_observer(foot_binding)
            .add_observer(star_map_binding)
            .add_observer(ui_binding)
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
        Actions::<OnStarMap>::default(),
        Actions::<OnUi>::default(),
        StateScoped(GameState::StarMap),
    ));
}

fn setup_playing_input_controls(mut commands: Commands) {
    commands.spawn((
        Name::new("Playing Input Controls"),
        Actions::<OnFoot>::default(),
        Actions::<OnUi>::default(),
        StateScoped(GameState::Playing),
    ));
}

fn reset_cursor_visible(mut window: Single<&mut Window>) {
    window.cursor_options.visible = true;
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
    player_entity: Single<Entity, With<Player>>,
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
    mut orientation_mode: Single<&mut OrientationMode, With<Player>>,
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

fn foot_binding(trigger: Trigger<Binding<OnFoot>>, mut players: Query<&mut Actions<OnFoot>>) {
    let mut actions = players.get_mut(trigger.target()).unwrap();
    actions
        .bind::<Move>()
        .to(Cardinal::wasd_keys())
        .with_modifiers(DeadZone::default());
    actions.bind::<AimAction>().to(MouseButton::Right);
    actions.bind::<UseAction>().to(KeyCode::KeyE);
    actions.bind::<KillAction>().to(KeyCode::KeyK);
    actions.bind::<ShootAction>().to(MouseButton::Left);
    actions.bind::<InventoryAction>().to(KeyCode::KeyI);
    actions.bind::<OrientationModeAction>().to(KeyCode::Space);
    actions.bind::<ZoomInAction>().to(KeyCode::Equal);
    actions.bind::<ZoomOutAction>().to(KeyCode::Minus);
    actions.bind::<RotateClockwiseAction>().to(KeyCode::KeyZ);
    actions
        .bind::<RotateCounterClockwiseAction>()
        .to(KeyCode::KeyC);
}

fn star_map_binding(
    trigger: Trigger<Binding<OnStarMap>>,
    mut players: Query<&mut Actions<OnStarMap>>,
) {
    let mut actions = players.get_mut(trigger.target()).unwrap();
    actions.bind::<ColonyIrisAction>().to(KeyCode::KeyI);
    actions.bind::<ColonyLiberteAction>().to(KeyCode::KeyL);
}

fn ui_binding(trigger: Trigger<Binding<OnUi>>, mut players: Query<&mut Actions<OnUi>>) {
    let mut actions = players.get_mut(trigger.target()).unwrap();
    actions
        .bind::<EscapeAction>()
        .to(KeyCode::Escape)
        .with_conditions(Hold::new(0.5));
}

fn apply_movement(
    trigger: Trigger<Fired<Move>>,
    cam_transform: Single<&Transform, With<MainCamera>>,
    mut movement: Single<&mut CharacterMovement, With<Player>>,
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

    let input_axis = trigger.value;
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
            ..Default::default()
        });
    }
}

fn apply_stop_movement(
    _trigger: Trigger<Completed<Move>>,
    mut tnua: Single<&mut TnuaController>,
    mut player_movement: Single<&mut CharacterMovement, With<Player>>,
) {
    player_movement.velocity = Vec3::ZERO;
    tnua.basis(TnuaBuiltinWalk {
        desired_velocity: Vec3::ZERO,
        float_height: 1.5,
        ..Default::default()
    });
}

fn apply_orientation_mode(
    _trigger: Trigger<Started<OrientationModeAction>>,
    r_cursor_world: Res<CursorWorld>,
    mut player_orientation_mode: Single<&mut OrientationMode, With<Player>>,
) {
    **player_orientation_mode = match **player_orientation_mode {
        OrientationMode::Direction => {
            OrientationMode::Location(Vec2::new(r_cursor_world.0.x, r_cursor_world.0.z))
        }
        OrientationMode::Location(_) => OrientationMode::Direction,
    }
}

fn apply_use(
    _trigger: Trigger<Started<UseAction>>,
    mut commands: Commands,
    r_use_entity: Res<HoverEntities>,
    player_entity: Single<Entity, With<Player>>,
    q_use: Query<&Use>,
) -> Result {
    for entity_target in r_use_entity.iter() {
        if q_use.contains(entity_target.entity) {
            commands.trigger_targets(UseCommand::new(*player_entity), entity_target.entity);
        }
    }
    Ok(())
}

fn apply_kill(_trigger: Trigger<Started<KillAction>>, mut commands: Commands) -> Result {
    commands.client_trigger(KillMeCommand);
    Ok(())
}

fn apply_inventory(
    _trigger: Trigger<Started<InventoryAction>>,
    container_query: Query<(&Name, &Contains), With<Player>>,
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

fn apply_exit(trigger: Trigger<Ongoing<EscapeAction>>, mut commands: Commands) {
    if trigger.elapsed_secs > 0.4 {
        commands.trigger(RequestExit);
    }
}

fn apply_window_cursor_visible(
    _trigger: Trigger<Started<EscapeAction>>,
    mut window: Single<&mut Window>,
) {
    window.cursor_options.visible = !window.cursor_options.visible;
}

fn apply_starmap_iris(_trigger: Trigger<Started<ColonyIrisAction>>, mut commands: Commands) {
    info!("apply_starmap_iris");
    commands.trigger(RequestConnect(Colony::Iris));
}

fn apply_starmap_liberte(_trigger: Trigger<Started<ColonyLiberteAction>>, mut commands: Commands) {
    commands.trigger(RequestConnect(Colony::Liberte));
}

fn apply_rotate_camera_clockwise(
    _trigger: Trigger<Started<RotateClockwiseAction>>,
    mut rig: Single<&mut Rig>,
) {
    let camera_yp = rig.driver_mut::<YawPitch>();
    camera_yp.rotate_yaw_pitch(-45.0, 0.0);
}

fn apply_rotate_camera_counter_clockwise(
    _trigger: Trigger<Started<RotateCounterClockwiseAction>>,
    mut rig: Single<&mut Rig>,
) {
    let camera_yp = rig.driver_mut::<YawPitch>();
    camera_yp.rotate_yaw_pitch(45.0, 0.0);
}

fn apply_camera_zoom_in(
    _trigger: Trigger<Fired<ZoomInAction>>,
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
    _trigger: Trigger<Fired<ZoomOutAction>>,
    mut rig: Single<&mut Rig>,
    time: Res<Time<Fixed>>,
) {
    if let Some(arm) = rig.try_driver_mut::<Arm>() {
        let mut xz = arm.offset;
        xz.z = (xz.z + 4.0 * time.delta_secs()).abs();
        arm.offset = xz.clamp_length_max(18.0);
    }
}

fn apply_aim(_trigger: Trigger<Started<AimAction>>, mut r_camera_modifier: ResMut<CameraModifier>) {
    r_camera_modifier.aim_zoom_factor = 1.8;
}

fn apply_aim_completed(
    _trigger: Trigger<Completed<AimAction>>,
    mut r_camera_modifier: ResMut<CameraModifier>,
) {
    r_camera_modifier.aim_zoom_factor = 1.0;
}

#[derive(InputContext)]
struct OnFoot;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
struct Move;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct AimAction;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct OrientationModeAction;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct UseAction;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct InventoryAction;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct ShootAction;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct KillAction;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct ZoomInAction;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct ZoomOutAction;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct RotateClockwiseAction;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct RotateCounterClockwiseAction;

#[derive(InputContext)]
struct OnStarMap;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct ColonyIrisAction;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct ColonyLiberteAction;

#[derive(InputContext)]
struct OnUi;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct EscapeAction;
