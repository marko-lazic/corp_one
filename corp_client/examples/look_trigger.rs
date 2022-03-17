use bevy::prelude::*;
use bevy_mouse_tracking_plugin::{MousePosPlugin, MousePosWorld};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Game::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .insert_resource(WindowDescriptor::default())
        .add_plugin(MousePosPlugin::Orthographic)
        .add_plugin(ShapePlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(setup_system)
        .add_system(draw_player_to_trigger_line)
        .add_system(draw_player_to_mouse_line)
        .add_system(player_look_trigger)
        .run();
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
    mouse_world_pos: Res<MousePosWorld>,
) {
    let (player_tf, mut player_com) = query.get_mut(game.player.unwrap()).unwrap();
    let player_pos = player_tf.translation;
    let mouse_world = Vec3::new(mouse_world_pos.x, mouse_world_pos.y, 0.0);
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
    let octagon = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(40.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let trigger = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &octagon,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 4.0),
            },
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
        .spawn_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::YELLOW),
                outline_mode: StrokeMode::new(Color::ORANGE, 4.0),
            },
            Transform::from_xyz(300.0, 200.0, 0.0),
        ))
        .insert(Player::default())
        .id();

    game.player = Some(player);
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

#[derive(Default)]
struct Game {
    trigger: Option<Entity>,
    player: Option<Entity>,
}
