use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(OnEnter(GameState::Playing), start_audio);
    }
}

fn start_audio(
	audio_assets: Res<AudioAssets>, 
	audio: Res<Audio>,
) {
	audio.resume();
	let _handle = audio
        .play(audio_assets.bgm.clone())
        .looped()
        .with_volume(0.3)
        .handle();
}