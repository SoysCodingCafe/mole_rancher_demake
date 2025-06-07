#![allow(clippy::type_complexity)]

mod audio;
mod loading;
mod menu;
mod retry;
mod molecules;
mod player;
mod postprocess;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::retry::RetryPlugin;
use crate::molecules::MoleculesPlugin;
use crate::player::PlayerPlugin;
use crate::postprocess::PostProcessPlugin;

use bevy::app::App;
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
    Menu,
	Retry,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            PostProcessPlugin,
            LoadingPlugin,
            MenuPlugin,
			RetryPlugin,
            InternalAudioPlugin,
			MoleculesPlugin,
            PlayerPlugin,
        ));
    }
}
