pub(crate) mod live {
    use bevy::app::{AppBuilder, Plugin};
    use bevy::asset::{AssetServer, Handle};
    use bevy::audio::{Audio, AudioSource};
    use bevy::ecs::{Commands, IntoQuerySystem, Query, Res};
    use corp_scene::player::{MovementSpeed, Player};

    pub struct LivePlugin;

    pub struct MusicRes {
        music: Handle<AudioSource>,
    }

    pub struct SoundRes {
        footsteps_concrete: Handle<AudioSource>,
    }

    impl Plugin for LivePlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_startup_system(setup.system());
            app.add_startup_system_to_stage("game_setup", play_startup_music.system());
            app.add_system(player_sound.system());
        }
    }

    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let music = asset_server.load("sounds/music.mp3");
        commands.insert_resource(MusicRes { music });
        let footsteps_concrete = asset_server.load("sounds/footsteps_concrete.mp3");
        commands.insert_resource(SoundRes { footsteps_concrete });
    }

    fn play_startup_music(audio: Res<Audio>, music_res: Res<MusicRes>) {
        audio.play(music_res.music.clone());
    }

    fn player_sound(
        audio: Res<Audio>,
        sound_res: Res<SoundRes>,
        mut player_position: Query<(&Player, &mut MovementSpeed)>,
    ) {
        for (_player, mut movement) in player_position.iter_mut() {
            if !movement.moving_happen && movement.is_moving {
                movement.moving_happen = true;
                audio.play(sound_res.footsteps_concrete.clone());
            }
        }
    }
}