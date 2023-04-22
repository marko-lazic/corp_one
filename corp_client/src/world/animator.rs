use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::asset::asset_loading::PlayerAssets;
use crate::GameState;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum PlayerAnimationAction {
    Run = 0,
    Idle = 1,
}

impl Default for PlayerAnimationAction {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Component, Default)]
pub struct AnimationComponent {
    pub next: Option<PlayerAnimationAction>,
    pub current: PlayerAnimationAction,
}

impl AnimationComponent {
    pub fn new(action: PlayerAnimationAction) -> Self {
        Self {
            next: Some(action),
            ..default()
        }
    }
}

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        // Warning: Re-insertion happens every time game enters playing state
        app.add_system(Self::insert_animation_resources.in_schedule(OnEnter(GameState::Playing)));
        app.add_system(Self::play_animations.in_set(OnUpdate(GameState::Playing)));
    }
}

impl AnimatorPlugin {
    fn insert_animation_resources(mut commands: Commands, player_assets: Res<PlayerAssets>) {
        let mut hm: HashMap<PlayerAnimationAction, Handle<AnimationClip>> = HashMap::new();
        hm.insert(PlayerAnimationAction::Run, player_assets.run.clone());
        hm.insert(PlayerAnimationAction::Idle, player_assets.idle.clone());
        commands.insert_resource(PlayerAnimations(hm));
    }
    fn play_animations(
        player_animations: Res<PlayerAnimations>,
        mut animation_player_query: Query<&mut AnimationPlayer>,
        mut animation_components: Query<&mut AnimationComponent>,
    ) {
        if let Ok(mut animation_player) = animation_player_query.get_single_mut() {
            for mut animation_component in animation_components.iter_mut() {
                if let Some(next) = animation_component.next {
                    animation_player.set_speed(1.2);
                    animation_player
                        .play(player_animations.get(&next).unwrap().clone_weak())
                        .repeat();
                    animation_component.current = next;
                    animation_component.next = None;
                }
            }
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
struct PlayerAnimations(pub HashMap<PlayerAnimationAction, Handle<AnimationClip>>);
