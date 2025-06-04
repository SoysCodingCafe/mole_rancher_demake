#![allow(clippy::type_complexity)]

mod audio;
mod loading;
mod menu;
mod molecules;
mod player;
mod postprocess;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::molecules::MoleculesPlugin;
use crate::player::PlayerPlugin;
use crate::postprocess::PostProcessPlugin;

use bevy::app::App;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            PostProcessPlugin,
            LoadingPlugin,
            MenuPlugin,
            InternalAudioPlugin,
			MoleculesPlugin,
            PlayerPlugin,
        ));

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugins((
        //         FrameTimeDiagnosticsPlugin::default(),
        //         LogDiagnosticsPlugin::default(),
        //     ));
        // }
    }
}
