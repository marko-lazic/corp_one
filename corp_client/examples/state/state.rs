use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_trait_query::RegisterExt;

use corp_shared::prelude::*;

use crate::ray::cast_ray_system;

mod endesga;
mod ray;

#[derive(Component)]
struct InventoryText(Entity);

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_event::<DoorStateEvent>()
        .add_event::<BackpackInteractionEvent>()
        .add_event::<DoorInteractionEvent>()
        .register_component_as::<dyn Interactive, Backpack>()
        .add_event::<DoorInteractionEvent>()
        .add_event::<BackpackInteractionEvent>()
        .add_event::<DoorHackEvent>()
        .register_component_as::<dyn Interactive, Door>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                door_cooldown_system,
                process_temporary_faction_ownership_timers_system,
                cast_ray_system,
                interaction_system,
                door_interaction_event_system,
                backpack_interaction_event_system,
                despawn_backpack_system,
                door_hack_event_system,
                show_inventory_system,
                door_color_change_state_system,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 8.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    // spawn door
    let _ec_door = setup_door(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(0.0, 0.5, 0.0),
        Faction::EC,
        Security::High,
    );

    let _vi_door = setup_door(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(-10.0, 0.5, 0.0),
        Faction::VI,
        Security::Low,
    );

    let _cmg_door = setup_door(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(5.0, 0.5, -5.0),
        Faction::CMG,
        Security::Medium,
    );

    let hacking_tool_entity = commands.spawn(HackingToolBundle::default()).id();

    // spawn player
    let player_entity = commands
        .spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::default(),
            Player,
            Interactor::default(),
            Inventory::new(vec![hacking_tool_entity]),
            MemberOf {
                faction: Faction::EC,
                rank: Rank::R7,
            },
        ))
        .id();

    // print inventory
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "null",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
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
        InventoryText(player_entity),
    ));
}

fn setup_door(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    door_position: Transform,
    faction: Faction,
    security: Security,
) -> Entity {
    let door_size = 1.0;
    let door_hs = door_size / 2.0;
    let ec_door = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: door_size })),
                material: materials.add(endesga::SKY.into()).into(),
                transform: door_position,
                ..default()
            },
            Door::new(security),
            ControlRegistry::new_permanent(faction),
            RigidBody::Fixed,
            Collider::cuboid(door_hs, door_hs, door_hs),
        ))
        .id();
    ec_door
}

fn show_inventory_system(
    mut inventory_text_query: Query<(&mut Text, &InventoryText)>,
    mut inventories: Query<&mut Inventory, Changed<Inventory>>,
    item_query: Query<&Item>,
) {
    for (mut text, inventory_text) in &mut inventory_text_query {
        if let Ok(inventory) = inventories.get_mut(inventory_text.0) {
            let mut items: Vec<String> = Vec::new();
            for inventory_item in inventory.items() {
                items.push(item_query.get(*inventory_item).unwrap().name.clone());
            }
            text.sections[0].value = format!("Inventory {:?}", items);
        }
    }
}

fn door_color_change_state_system(
    mut doors: Query<(&Door, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (door, material) in &mut doors {
        match door.state() {
            DoorState::Open => {
                materials.get_mut(material).unwrap().base_color = endesga::FOG.into()
            }
            DoorState::Closed => {
                materials.get_mut(material).unwrap().base_color = endesga::SKY.into()
            }
        }
    }
}
