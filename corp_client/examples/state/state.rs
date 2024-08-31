use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use corp_shared::{asset::endesga, prelude::*};

use crate::ray::cast_ray_system;

mod ray;

#[derive(Resource, Default)]
struct PlayerEntity(Option<Entity>);

#[derive(Resource, Default)]
pub struct TargetEntity(pub Option<Entity>);

#[derive(Component)]
struct InventoryText;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .init_resource::<PlayerEntity>()
        .init_resource::<TargetEntity>()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(AmbientLight::default())
        .add_event::<BackpackInteractionEvent>()
        .add_event::<InteractionEvent<UseDoorEvent>>()
        .add_event::<BackpackInteractionEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                door_cooldown_system,
                process_temporary_faction_ownership_timers_system,
                cast_ray_system,
                interaction_system,
                backpack_interaction_event_system,
                despawn_backpack_system,
                show_inventory_system,
                door_color_change_state_system,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut r_player_entity: ResMut<PlayerEntity>,
    r_asset_server: Res<AssetServer>,
    mut r_assets_mesh: ResMut<Assets<Mesh>>,
    mut r_assets_material: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 8.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    // spawn door
    let _ec_door = setup_door(
        &mut commands,
        &mut r_assets_mesh,
        &mut r_assets_material,
        Transform::from_xyz(0.0, 0.5, 0.0),
        Faction::EC,
        SecurityLevel::High,
    );

    let _vi_door = setup_door(
        &mut commands,
        &mut r_assets_mesh,
        &mut r_assets_material,
        Transform::from_xyz(-10.0, 0.5, 0.0),
        Faction::VI,
        SecurityLevel::Low,
    );

    let _cmg_door = setup_door(
        &mut commands,
        &mut r_assets_mesh,
        &mut r_assets_material,
        Transform::from_xyz(5.0, 0.5, -5.0),
        Faction::CMG,
        SecurityLevel::Medium,
    );

    let hacking_tool_entity = commands.spawn(HackingToolBundle::default()).id();

    // spawn player
    let player_entity = commands
        .spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::default(),
            Player,
            Inventory::new(vec![hacking_tool_entity]),
            MemberOf {
                faction: Faction::EC,
                rank: Rank::R7,
            },
        ))
        .id();
    r_player_entity.0 = Some(player_entity);

    // print inventory
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "null",
                TextStyle {
                    font: r_asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 30.0,
                    color: endesga::AQUA,
                },
            ),
            style: Style {
                left: Val::Px(100.0),
                top: Val::Px(50.0),
                ..default()
            },
            ..Default::default()
        },
        InventoryText,
    ));
}

fn setup_door(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    door_position: Transform,
    owner: Faction,
    level: SecurityLevel,
) -> Entity {
    let door_size = 1.0;
    let door_hs = door_size / 2.0;
    let ec_door = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(door_size, door_size, door_size)),
                material: materials.add(endesga::SKY),
                transform: door_position,
                ..default()
            },
            DoorBundle {
                security_level: level,
                ownership_registry: OwnershipRegistry::new_permanent(owner),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(door_hs, door_hs, door_hs),
        ))
        .observe(on_use_door_event_door)
        .observe(on_use_door_hack_event)
        .id();
    ec_door
}

fn interaction_system(
    r_keyboard_input: Res<ButtonInput<KeyCode>>,
    r_player_entity: Res<PlayerEntity>,
    r_target_entity: Res<TargetEntity>,
    mut ev_use_door: EventWriter<InteractionEvent<UseDoorEvent>>,
    mut commands: Commands,
) {
    if r_keyboard_input.just_pressed(KeyCode::KeyE) {
        if let Some(interactor) = r_player_entity.0 {
            if let Some(target) = r_target_entity.0 {
                ev_use_door.send(InteractionEvent::new(interactor, target, UseDoorEvent));
                commands.trigger_targets(UseEvent { user: interactor }, target);
            }
        }
    }
}

fn show_inventory_system(
    mut inventory_text_query: Query<&mut Text, With<InventoryText>>,
    mut inventories: Query<&mut Inventory, Changed<Inventory>>,
    item_query: Query<&Item>,
    r_player_entity: Res<PlayerEntity>,
) {
    for mut text in &mut inventory_text_query {
        if let Ok(inventory) = inventories.get_mut(r_player_entity.0.unwrap()) {
            let mut items: Vec<String> = Vec::new();
            for inventory_item in inventory.items() {
                items.push(item_query.get(*inventory_item).unwrap().name.clone());
            }
            text.sections[0].value = format!("Inventory {:?}", items);
        }
    }
}

fn door_color_change_state_system(
    mut doors: Query<(&DoorState, &Handle<StandardMaterial>), With<Door>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (door_state, material) in &mut doors {
        match door_state {
            DoorState::Open => materials.get_mut(material).unwrap().base_color = endesga::FOG,
            DoorState::Closed => materials.get_mut(material).unwrap().base_color = endesga::SKY,
        }
    }
}
