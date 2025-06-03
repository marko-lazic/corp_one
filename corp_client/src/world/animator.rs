use crate::prelude::*;
use bevy::{animation::animate_targets, prelude::*};
use corp_shared::prelude::*;
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
        app.add_systems(OnExit(GameState::Init), setup_animation_graph)
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
    animation_player_entity: Single<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    let (entity, mut player) = animation_player_entity.into_inner();
    let mut transitions = AnimationTransitions::new();
    transitions
        .play(&mut player, animations.node.idle, Duration::ZERO)
        .repeat();
    commands
        .entity(entity)
        .insert(animations.graph.clone())
        .insert(transitions);
}

fn animation_control(
    movement: Single<&CharacterMovement, With<Player>>,
    q_animation_players: Single<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<MannequinAnimations>,
) {
    let (mut player, mut transitions) = q_animation_players.into_inner();
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
