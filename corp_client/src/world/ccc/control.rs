use crate::{prelude::*, world::ccc::input};
use avian3d::prelude::*;
use bevy::{
    ecs::{
        lifecycle::HookContext,
        query::QueryData,
        relationship::RelationshipSourceCollection,
        system::lifetimeless::{Read, Write},
        world::DeferredWorld,
    },
    platform::collections::HashSet,
    prelude::*,
    window::{CursorOptions, PrimaryWindow},
};
use bevy_enhanced_input::prelude::*;
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
        app.add_plugins((EnhancedInputPlugin, input::plugin))
            .init_resource::<CursorWorld>()
            .init_resource::<HoverEntities>()
            .add_systems(OnEnter(GameState::StarMap), setup_star_map_input_controls)
            .add_systems(OnEnter(GameState::Playing), setup_ui_input_controls)
            .add_systems(OnExit(GameState::Playing), reset_cursor_visible)
            .add_systems(
                FixedUpdate,
                (
                    can_move,
                    detect_usable_targets,
                    update_cursor_world,
                    rotate_character,
                    run_move_and_slide,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component, Clone, Reflect, Debug)]
#[reflect(Component)]
#[require(
    AccumulatedInput,
    CharacterControllerState,
    TranslationInterpolation,
    CustomPositionIntegration,
    RigidBody = RigidBody::Kinematic,
    SpeculativeMargin::ZERO,
    CollidingEntities
)]
#[component(on_add=CharacterController::on_add)]
pub struct CharacterController {
    pub gravity: f32,
    pub filter: SpatialQueryFilter,
}

impl Default for CharacterController {
    fn default() -> Self {
        CharacterController {
            gravity: 9.81,
            filter: SpatialQueryFilter::default(),
        }
    }
}

impl CharacterController {
    pub fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;
        world
            .commands()
            .queue(move |world: &mut World| world.run_system_cached_with(setup_collider, entity));
    }
}

fn setup_collider(
    In(entity): In<Entity>,
    mut kcc: Query<(
        &mut CharacterController,
        &mut CharacterControllerState,
        &Collider,
    )>,
) {
    let Ok((mut cfg, mut state, collider)) = kcc.get_mut(entity) else {
        return;
    };
    cfg.filter.excluded_entities.add(entity);
    state.collider = collider.clone();
}

#[derive(Component, Clone, Reflect, Debug)]
#[require(CharacterMovement)]
#[reflect(Component)]
pub struct CharacterControllerState {
    pub cam_transform: Transform,
    #[reflect(ignore)]
    pub collider: Collider,
    pub touching_entities: Vec<TouchingEntity>,
}

impl Default for CharacterControllerState {
    fn default() -> Self {
        CharacterControllerState {
            cam_transform: Transform::IDENTITY,
            collider: Collider::default(),
            touching_entities: Vec::new(),
        }
    }
}

#[derive(Clone, Reflect, PartialEq, Debug)]
pub struct TouchingEntity {
    /// The entity of the collider that was hit by the shape.
    pub entity: Entity,

    /// The maximum distance that is safe to move in the given direction so that the collider
    /// still keeps a distance of `skin_width` to the other colliders.
    ///
    /// This is `0.0` when any of the following is true:
    ///
    /// - The collider started off intersecting another collider.
    /// - The collider is moving toward another collider that is already closer than `skin_width`.
    ///
    /// If you want to know the real distance to the next collision, use [`Self::collision_distance`].
    pub distance: f32,

    /// The hit point on the shape that was hit, expressed in world space.
    pub point: Vec3,

    /// The outward surface normal on the hit shape at `point`, expressed in world space.
    pub normal: Dir3,

    /// The position of the collider at the time of the move and slide iteration.
    pub character_position: Vec3,

    /// The velocity of the collider at the time of the move and slide iteration.
    pub character_velocity: Vec3,

    /// The raw distance to the next collision, not respecting skin width.
    /// To move the shape, use [`Self::distance`] instead.
    #[doc(alias = "time_of_impact")]
    pub collision_distance: f32,
}
impl From<MoveAndSlideHitData<'_>> for TouchingEntity {
    fn from(value: MoveAndSlideHitData<'_>) -> Self {
        Self {
            entity: value.entity,
            distance: value.distance,
            point: value.point,
            normal: *value.normal,
            character_position: *value.position,
            character_velocity: *value.velocity,
            collision_distance: value.collision_distance,
        }
    }
}

#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
struct Ctx {
    velocity: Write<LinearVelocity>,
    movement: Write<CharacterMovement>,
    state: Write<CharacterControllerState>,
    transform: Write<Transform>,
    input: Write<AccumulatedInput>,
    cfg: Read<CharacterController>,
}

fn setup_star_map_input_controls(mut commands: Commands) {
    commands.spawn((
        Name::new("Star Map Input Controls"),
        StarMapInput,
        UiInput,
        DespawnOnExit(GameState::StarMap),
    ));
}

fn setup_ui_input_controls(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Input Controls"),
        UiInput,
        DespawnOnExit(GameState::Playing),
    ));
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

fn run_move_and_slide(mut kccs: Query<Ctx>, move_and_slide: MoveAndSlide, time: Res<Time>) {
    for mut ctx in &mut kccs {
        let cam_forward = {
            let f = ctx.state.cam_transform.rotation.mul_vec3(Vec3::Z);
            // Invert it to account for camera looking down -Z
            -Vec3::new(f.x, 0.0, f.z).normalize_or_zero()
        };
        let cam_right = {
            let r = ctx.state.cam_transform.rotation.mul_vec3(Vec3::X);
            Vec3::new(r.x, 0.0, r.z).normalize_or_zero()
        };

        let input_axis = ctx.input.last_movement.unwrap_or_default();
        let input_forward = cam_forward * input_axis.y;
        let input_strafe = cam_right * input_axis.x;

        ctx.movement.direction = (input_forward + input_strafe).normalize_or_zero();

        if !ctx.movement.can_move {
            return;
        }

        ctx.movement.velocity = ctx.movement.direction * ctx.movement.speed;

        // Apply velocity to position
        ctx.transform.translation += ctx.movement.velocity * time.delta_secs();

        let mut touching_entities = std::mem::take(&mut ctx.state.touching_entities);
        // Perform move and slide
        let MoveAndSlideOutput {
            position,
            projected_velocity,
        } = move_and_slide.move_and_slide(
            &ctx.state.collider,
            ctx.transform.translation,
            ctx.transform.rotation,
            ctx.movement.velocity,
            time.delta(),
            &MoveAndSlideConfig::default(),
            &ctx.cfg.filter,
            |hit| {
                touching_entities.push(hit.into());
                true
            },
        );
        ctx.transform.translation = position;
        ctx.velocity.0 = projected_velocity;
    }
}
