use crate::ray::cast_ray_system;
use avian3d::prelude::*;
use bevy::prelude::*;
use corp_shared::prelude::*;

mod ray;

#[derive(Resource, Deref)]
struct PlayerEntity(Entity);

#[derive(Resource, Default)]
pub struct TargetEntity(pub Option<Entity>);

#[derive(Component)]
struct InventoryText;

fn main() {
    App::new()
        .init_resource::<TargetEntity>()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .insert_resource(AmbientLight::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                door_cooldown_system,
                process_temporary_faction_ownership_timers_system,
                cast_ray_system,
                interaction_system,
                show_inventory_system,
                door_color_change_state_system,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    r_asset_server: Res<AssetServer>,
    mut r_assets_mesh: ResMut<Assets<Mesh>>,
    mut r_assets_material: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 8.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

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

    let hacking_tool_entity = commands.spawn(HackingTool).id();

    // spawn player
    let player_entity = commands
        .spawn((
            Player,
            Inventory,
            PlayerFactionInfo {
                faction: Faction::EC,
                rank: Rank::R7,
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();

    commands
        .entity(player_entity)
        .insert(StoredIn(hacking_tool_entity));

    commands.insert_resource(PlayerEntity(player_entity));

    // print inventory
    commands.spawn((
        InventoryText,
        Text::new("null"),
        Node {
            top: Val::Px(100.0),
            left: Val::Px(50.0),
            ..default()
        },
        TextColor::from(endesga::AQUA),
        TextFont::from_font(r_asset_server.load("fonts/FiraMono-Medium.ttf")).with_font_size(30.0),
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
            Mesh3d(meshes.add(Cuboid::new(door_size, door_size, door_size))),
            MeshMaterial3d(materials.add(endesga::SKY)),
            door_position,
            Door,
            level,
            OwnershipRegistry::new_permanent(owner),
            RigidBody::Static,
            Collider::cuboid(door_hs, door_hs, door_hs),
        ))
        .observe(on_use_command)
        .id();
    ec_door
}

fn interaction_system(
    r_keyboard_input: Res<ButtonInput<KeyCode>>,
    r_player_entity: Res<PlayerEntity>,
    r_target_entity: Res<TargetEntity>,
    mut commands: Commands,
) {
    if r_keyboard_input.just_pressed(KeyCode::KeyE) {
        if let Some(target) = r_target_entity.0 {
            commands.trigger_targets(
                UseCommand {
                    user: **r_player_entity,
                },
                target,
            );
        }
    }
}

fn show_inventory_system(
    inventory_text_entity: Single<Entity, With<InventoryText>>,
    mut writer: TextUiWriter,
    container_query: Query<&Contains, With<Player>>,
    q_name: Query<&Name>,
) {
    let mut string_list: Vec<String> = Vec::new();
    for contains in &container_query {
        for e_item in contains.iter() {
            if let Ok(name) = q_name.get(e_item) {
                string_list.push(name.to_string());
            }
        }
    }
    *writer.text(*inventory_text_entity, 0) = format!("Inventory {:?}", string_list);
}

fn door_color_change_state_system(
    mut doors: Query<(&DoorState, &MeshMaterial3d<StandardMaterial>), With<Door>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (door_state, material) in &mut doors {
        match door_state {
            DoorState::Open { .. } => {
                materials.get_mut(material).unwrap().base_color = endesga::FOG
            }
            DoorState::Closed => materials.get_mut(material).unwrap().base_color = endesga::SKY,
        }
    }
}
