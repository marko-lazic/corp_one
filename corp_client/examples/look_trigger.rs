use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(Game::default())
        .add_plugins((DefaultPlugins, DebugLinesPlugin::default(), ShapePlugin))
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                my_cursor_system,
                draw_player_to_trigger_line,
                draw_player_to_mouse_line,
                player_look_trigger,
            ),
        )
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource, Default)]
struct MousePosWorld {
    x: f32,
    y: f32,
}

#[derive(Component, Default)]
struct Trigger {
    player_to_trigger_dir: Vec3,
}

#[derive(Component)]
struct Player {
    preciceness: f32,
    look_dir: Vec3,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            preciceness: 0.8,
            look_dir: Vec3::X,
        }
    }
}

#[derive(Resource, Default)]
struct Game {
    trigger: Option<Entity>,
    player: Option<Entity>,
}

fn my_cursor_system(
    // need to get window dimensions
    primary_windows: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    // store to resource
    mut mouse_pos_world: ResMut<MousePosWorld>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let Ok(primary) = primary_windows.get_single() else {
        return;
    };
    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = primary
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_pos_world.x = world_position.x;
        mouse_pos_world.y = world_position.y;
    }
}

fn player_look_trigger(
    transforms: Query<&Transform>,
    players: Query<&Player>,
    triggers: Query<&Trigger>,
    game: Res<Game>,
    mut lines: ResMut<DebugLines>,
) {
    let player_com = players.get(game.player.unwrap()).unwrap();
    let trigger_com = triggers.get(game.trigger.unwrap()).unwrap();
    let lookness = trigger_com.player_to_trigger_dir.dot(player_com.look_dir);

    let is_looking = lookness >= player_com.preciceness;

    if is_looking {
        let player_pos = transforms.get(game.player.unwrap()).unwrap().translation;
        lines.line_colored(
            player_pos,
            player_pos + trigger_com.player_to_trigger_dir * 200.0,
            0.,
            Color::GREEN,
        );
    }
}

fn draw_player_to_mouse_line(
    mut query: Query<(&Transform, &mut Player)>,
    game: ResMut<Game>,
    mut lines: ResMut<DebugLines>,
    mouse_pos_world: Res<MousePosWorld>,
) {
    let (player_tf, mut player_com) = query.get_mut(game.player.unwrap()).unwrap();
    let player_pos = player_tf.translation;
    let mouse_world = Vec3::new(mouse_pos_world.x, mouse_pos_world.y, 0.0);
    let look_dir = (mouse_world - player_pos).normalize();
    player_com.look_dir = look_dir;
    lines.line_colored(player_pos, player_pos + look_dir * 200.0, 0., Color::RED);
}

fn draw_player_to_trigger_line(
    mut triggers: Query<&mut Trigger>,
    query: Query<&Transform>,
    game: Res<Game>,
    mut lines: ResMut<DebugLines>,
) {
    let trigger_pos = query.get(game.trigger.unwrap()).unwrap().translation;
    let player_pos = query.get(game.player.unwrap()).unwrap().translation;
    let player_to_trigger_dir = (trigger_pos - player_pos).normalize();
    let mut trigger_com = triggers.get_mut(game.trigger.unwrap()).unwrap();
    trigger_com.player_to_trigger_dir = player_to_trigger_dir;
    lines.line_colored(
        player_pos,
        player_pos + player_to_trigger_dir * 200.0,
        0.,
        Color::WHITE,
    );
}

fn setup_system(mut commands: Commands, mut game: ResMut<Game>) {
    let octagon = RegularPolygon {
        sides: 6,
        feature: RegularPolygonFeature::Radius(40.0),
        ..RegularPolygon::default()
    };

    commands.spawn((Camera2dBundle::default(), MainCamera));
    let trigger = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&octagon),
                ..default()
            },
            Fill::color(Color::CYAN),
            Stroke::new(Color::BLACK, 4.0),
            Transform::default(),
        ))
        .insert(Trigger::default())
        .id();

    game.trigger = Some(trigger);

    let circle = shapes::Circle {
        radius: 30.0,
        ..shapes::Circle::default()
    };

    let player = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&circle),
                ..default()
            },
            Fill::color(Color::YELLOW),
            Stroke::new(Color::ORANGE, 4.0),
            Transform::from_xyz(300.0, 200.0, 0.0),
        ))
        .insert(Player::default())
        .id();

    game.player = Some(player);
}
