use std::fs;

use bevy::app::AppExit;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use kurinji::{Kurinji, KurinjiPlugin, OnActionActive, OnActionEnd};

use input_command::PlayerAction;

use crate::asset::asset_loading::ColonyAssets;
use crate::constants::state::GameState;
use crate::constants::tick;
use crate::Game;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystem {
    Playing,
    Starmap,
}

pub mod input_command;

pub struct InputControlPlugin;

impl InputControlPlugin {
    fn setup_kurinji_binding(mut kurinji: ResMut<Kurinji>) {
        let binding_json = fs::read_to_string("corp_client/config/binding.json")
            .expect("Error! could not open config file");
        kurinji.set_bindings_with_json(&binding_json);
    }

    fn quit_app(mut reader: EventReader<OnActionEnd>, mut writer: EventWriter<AppExit>) {
        for event in reader.iter() {
            if event.action == "QUIT_APP" {
                println!("Quitting...");
                writer.send(AppExit);
            }
        }
    }

    fn player_keyboard_action(
        mut player_action: ResMut<PlayerAction>,
        mut reader: EventReader<OnActionActive>,
    ) {
        for event in reader.iter() {
            player_action.key_action(&event.action);
            player_action.mouse_action(&event.action);
        }
    }

    fn starmap_keyboard(
        mut game: ResMut<Game>,
        colony_assets: Res<ColonyAssets>,
        keyboard_input: Res<Input<KeyCode>>,
        mut game_state: ResMut<State<GameState>>,
    ) {
        if !keyboard_input.is_changed() {
            return;
        }

        if keyboard_input.just_pressed(KeyCode::I) {
            info!("Moonbase: Station Iris");
            game.current_colony_asset = colony_assets.iris.clone();
            let _result = game_state.set(GameState::Playing);
        } else if keyboard_input.just_pressed(KeyCode::L) {
            info!("Mars: Colony Liberte");
            game.current_colony_asset = colony_assets.liberte.clone();
            let _result = game_state.set(GameState::Playing);
        }
    }
}

impl Plugin for InputControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(KurinjiPlugin::default());
        app.add_system(Self::setup_kurinji_binding.system());
        app.add_system(Self::quit_app.system());

        app.add_system_set(
            SystemSet::on_update(GameState::StarMap)
                .label(InputSystem::Starmap)
                .with_system(Self::starmap_keyboard.system()),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label(InputSystem::Playing)
                .with_run_criteria(FixedTimestep::steps_per_second(tick::FRAME_RATE))
                .with_system(Self::player_keyboard_action.system()),
        );
    }
}
