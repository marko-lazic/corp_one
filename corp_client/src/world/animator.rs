use crate::prelude::*;
use bevy::{animation::animate_targets, prelude::*};
use corp_shared::prelude::Player;
use std::time::Duration;

struct MannequinAnimationNodeIndex {
    idle: AnimationNodeIndex,
    run: AnimationNodeIndex,
}

#[derive(Resource)]
struct MannequinAnimations {
    node: MannequinAnimationNodeIndex,
    graph: AnimationGraphHandle,
}

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        // Warning: Re-insertion happens every time game enters playing state
        // Loading needs to be split into loading for resources and data setup
        app.add_systems(OnExit(GameState::Loading), setup_animation_graph)
            .add_systems(
                FixedUpdate,
                setup_scene_once_loaded
                    .before(animate_targets)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                animation_control.run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_animation_graph(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Build the animation graph
    let mut animation_graph = AnimationGraph::new();

    let idle = animation_graph.add_clip(player_assets.idle.clone(), 1.0, animation_graph.root);
    let run = animation_graph.add_clip(player_assets.run.clone(), 1.0, animation_graph.root);

    // Insert a resource with the current scene information
    let handle = animation_graphs.add(animation_graph);

    commands.insert_resource(MannequinAnimations {
        node: MannequinAnimationNodeIndex { idle, run },
        graph: AnimationGraphHandle(handle),
    });
}

fn setup_scene_once_loaded(
    mut commands: Commands,
    animations: Res<MannequinAnimations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();
        transitions
            .play(&mut player, animations.node.idle, Duration::ZERO)
            .repeat();
        commands
            .entity(entity)
            .insert(animations.graph.clone())
            .insert(transitions);
    }
}

fn animation_control(
    q_player_movement: Query<&CharacterMovement, With<Player>>,
    mut q_animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<MannequinAnimations>,
) {
    for (mut player, mut transitions) in &mut q_animation_players {
        for movement in &q_player_movement {
            if movement.is_moving() {
                if !player.is_playing_animation(animations.node.run) {
                    transitions
                        .play(&mut player, animations.node.run, Duration::ZERO)
                        .repeat();
                }
            } else {
                if !player.is_playing_animation(animations.node.idle) {
                    transitions
                        .play(&mut player, animations.node.idle, Duration::ZERO)
                        .repeat();
                }
            }
        }
    }
}
