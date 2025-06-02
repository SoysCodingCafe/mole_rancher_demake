use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(OnEnter(GameState::Playing), start_audio);
    }
}

#[derive(Resource)]
struct FlyingAudio(Handle<AudioInstance>);

// #[derive(Resource)]
// struct BGM(Handle<AudioInstance>);

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.flying.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(FlyingAudio(handle));

	audio.resume();
	let _handle = audio
        .play(audio_assets.bgm.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    // commands.insert_resource(BGM(handle));
}