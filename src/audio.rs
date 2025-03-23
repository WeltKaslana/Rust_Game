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
            .add_systems(OnEnter(GameState::MainMenu), audio_play)
            .add_systems(Update, log_transitions::<GameState>);
    }
}

fn audio_play(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("AudioClip/MainMenu - Takaramonogatari.wav")),
        PlaybackSettings::LOOP,
    ));
}