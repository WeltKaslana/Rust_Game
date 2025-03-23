use bevy::{
    dev_tools::states::*, 
    audio::{AudioPlayer, PlaybackSettings},
    prelude::*,
};
use crate::gamestate::GameState;
pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), audio_play__MainMenu)
            .add_systems(OnExit(GameState::MainMenu), pause)
            .add_systems(OnEnter(GameState::Home), audio_play_Home)
            .add_systems(OnExit(GameState::Home), pause)
            .add_systems(Update, log_transitions::<GameState>);
    }
}

fn audio_play__MainMenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("AudioClip/MainMenu - Takaramonogatari.wav")),
        PlaybackSettings::LOOP,
    ));
}

fn audio_play_Home(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("AudioClip/Angel24 - Cotton Candy Island.wav")),
        PlaybackSettings::LOOP,
    ));
}

fn pause(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity)>,
){
    for (mut audio, entity) in audio_sink.iter_mut() {
        commands.entity(entity).despawn();
    }
}