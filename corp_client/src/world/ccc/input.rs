use crate::prelude::{
    CameraModifier, CursorWorld, HoverEntities, LocalPlayer, OrientationMode, RequestConnect,
    RequestExit, did_fixed_timestep_run_this_frame,
};
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    math::Vec2,
    prelude::*,
    window::{CursorOptions, PrimaryWindow},
};
use bevy_dolly::{
    dolly_type::Rig,
    prelude::{Arm, YawPitch},
};
use bevy_enhanced_input::prelude::*;
use bevy_replicon::prelude::ClientTriggerExt;
use corp_shared::prelude::{Contains, Item, KillMeCommand, Use, UseCommand};
use corp_types::prelude::Colony;

pub fn plugin(app: &mut App) {
    app.add_input_context::<PlayerInput>()
        .add_input_context::<StarMapInput>()
        .add_input_context::<UiInput>()
        .add_observer(apply_movement)
        .add_observer(apply_orientation_mode)
        .add_observer(apply_use)
        .add_observer(apply_kill)
        .add_observer(apply_inventory)
        .add_observer(apply_exit)
        .add_observer(apply_window_cursor_visible)
        .add_observer(apply_star_map_iris)
        .add_observer(apply_star_map_liberte)
        .add_observer(apply_rotate_camera_clockwise)
        .add_observer(apply_rotate_camera_counter_clockwise)
        .add_observer(apply_camera_zoom_in)
        .add_observer(apply_camera_zoom_out)
        .add_observer(apply_aim)
        .add_observer(apply_aim_completed)
        .add_systems(
            RunFixedMainLoop,
            clear_accumulated_input
                .run_if(did_fixed_timestep_run_this_frame)
                .in_set(RunFixedMainLoopSystems::AfterFixedMainLoop),
        );
}

#[derive(Component, Default)]
#[component(on_add = PlayerInput::on_add)]
pub struct PlayerInput;

impl PlayerInput {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world.commands().entity(ctx.entity).apply_scene(bsn! {
            Actions<PlayerInput> [
                Action::<Movement>::new()
                DeadZone
                Bindings [
                    {preset::cardinal::wasd_keys()}
                    --
                    {preset::axial::left_stick()}
                ]
                --
                Action::<AimWeapon>::new()
                Bindings [
                    Binding(MouseButton::Right)
                    --
                    Binding(GamepadButton::LeftTrigger)
                ]
                --
                Action::<UseObject>::new()
                Bindings [
                    Binding(KeyCode::KeyE)
                    --
                    Binding(GamepadButton::South)
                ]
                --
                Action::<KillMe>::new()
                Bindings [
                    Binding(KeyCode::KeyK)
                ]
                --
                Action::<Shoot>::new()
                Bindings [
                    Binding(MouseButton::Left)
                    --
                    Binding(GamepadButton::RightTrigger)
                ]
                --
                Action::<Inventory>::new()
                Bindings [
                    Binding(KeyCode::KeyI)
                    --
                    Binding(GamepadButton::North)
                ]
                --
                Action::<OrientationModeSwitch>::new()
                Bindings [
                    Binding(KeyCode::Space)
                    --
                    Binding(GamepadButton::RightThumb)
                ]
                --
                Action::<ZoomIn>::new()
                Bindings [
                    Binding(KeyCode::Equal)
                    --
                    Binding(GamepadButton::DPadUp)
                ]
                --
                Action::<ZoomOut>::new()
                Bindings [
                    Binding(KeyCode::Minus)
                    --
                    Binding(GamepadButton::DPadDown)
                ]
                --
                Action::<RotateClockwise>::new()
                Bindings [
                    Binding(KeyCode::KeyZ)
                    --
                    Binding(GamepadButton::DPadRight)
                ]
                --
                Action::<RotateCounterClockwise>::new()
                Bindings [
                    Binding(KeyCode::KeyC)
                    --
                    Binding(GamepadButton::DPadLeft)
                ]
            ]
        });
    }
}

#[derive(Component, Default)]
#[component(on_add = StarMapInput::on_add)]
pub struct StarMapInput;

impl StarMapInput {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world.commands().entity(ctx.entity).apply_scene(bsn! {
            Actions<StarMapInput> [
                Action::<ColonyIris>::new()
                Bindings [Binding(KeyCode::KeyI)]
                --
                Action::<ColonyLiberte>::new()
                Bindings [Binding(KeyCode::KeyL)]
            ]
        });
    }
}

#[derive(Component, Default)]
#[component(on_add = UiInput::on_add)]
pub struct UiInput;

impl UiInput {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world.commands().entity(ctx.entity).apply_scene(bsn! {
            Actions<UiInput> [
                Action::<ExitHold>::new()
                ~Hold::new(0.5)
                Bindings [
                    Binding(KeyCode::Escape)
                    --
                    Binding(GamepadButton::Start)
                ]
            ]
        });
    }
}

#[derive(Component, Clone, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct AccumulatedInput {
    pub last_movement: Option<Vec2>,
}

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

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct ColonyIris;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct ColonyLiberte;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct ExitHold;

fn apply_movement(
    movement: On<Fire<Movement>>,
    mut accumulated_inputs: Query<&mut AccumulatedInput>,
) {
    if let Ok(mut accumulated_inputs) = accumulated_inputs.get_mut(movement.context) {
        accumulated_inputs.last_movement = Some(movement.value);
    }
}

fn apply_orientation_mode(
    _orientation_mode: On<Start<OrientationModeSwitch>>,
    r_cursor_world: Res<CursorWorld>,
    mut player_orientation_mode: Single<&mut OrientationMode, With<LocalPlayer>>,
) {
    **player_orientation_mode = match **player_orientation_mode {
        OrientationMode::Direction => {
            OrientationMode::Location(Vec2::new(r_cursor_world.x, r_cursor_world.z))
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

fn apply_star_map_iris(_colony_iris: On<Start<ColonyIris>>, mut commands: Commands) {
    info!("apply_star_map_iris");
    commands.trigger(RequestConnect(Colony::Iris));
}

fn apply_star_map_liberte(_colony_liberte: On<Start<ColonyLiberte>>, mut commands: Commands) {
    info!("apply_star_map_liberte");
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

fn clear_accumulated_input(mut accumulated_inputs: Query<&mut AccumulatedInput>) {
    for mut accumulated_input in &mut accumulated_inputs {
        *accumulated_input = AccumulatedInput {
            last_movement: default(),
        }
    }
}
