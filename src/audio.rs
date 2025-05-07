use bevy::{
    dev_tools::states::*, 
    audio::{AudioPlayer, PlaybackSettings, Volume},
    prelude::*,
};
use crate::{
    character::{Character, 
                PlayerRunEvent, 
                PlayerJumpEvent, 
                PlayerTimer,}, 
    gamestate::GameState, 
    gun::PlayerFireEvent};
pub struct GameAudioPlugin;

#[derive(Component)]
pub struct FireAudio;
#[derive(Component)]
pub struct RunAudio;
#[derive(Component)]
pub struct JumpAudio;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerFireEvent>()
            .add_event::<PlayerRunEvent>()
            .add_event::<PlayerJumpEvent>()
            .add_systems(OnEnter(GameState::MainMenu), audio_play__MainMenu)
            .add_systems(OnExit(GameState::MainMenu), pause)
            .add_systems(OnEnter(GameState::Home), audio_play_Home)
            .add_systems(OnExit(GameState::Home), pause)
            .add_systems(OnEnter(GameState::InGame), audio_play_Ingame)
            .add_systems(OnExit(GameState::InGame), pause)

            .add_systems(Update,(
                        audio_fire,
                        player_jump,
                        player_run,
                        ).run_if(in_state(GameState::Home)))
            .add_systems(Update,(
                audio_fire,
                player_jump,
                player_run,
                ).run_if(in_state(GameState::InGame)))
            // .add_systems(Update, log_transitions::<GameState>)
            ;
    }
}

fn audio_play__MainMenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("AudioClip/MainMenu - Takaramonogatari.wav")),
        PlaybackSettings::LOOP.with_volume(Volume::new(0.3)),
    ));
}

fn audio_play_Home(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("AudioClip/Angel24 - Cotton Candy Island.wav")),
        PlaybackSettings::LOOP.with_volume(Volume::new(0.3)),
    ));

}

fn audio_play_Ingame(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("AudioClip/Level1 - Let me think about it.wav")),
        PlaybackSettings::LOOP.with_volume(Volume::new(0.3)),
    ));
}

fn audio_fire (
    mut events: EventReader<PlayerFireEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<Entity, With<FireAudio>>,
) {
    for _ in events.read() {
        for e in query.iter_mut() {
            // println!("despawn fire audio");
            commands.entity(e).despawn();
        }
        commands.spawn((
            AudioPlayer::new(asset_server.load("AudioClip/SE_Shiroko_Attack.wav")),
            PlaybackSettings::default(),
            FireAudio,
        ));
    }
}
fn player_run (
    mut events: EventReader<PlayerRunEvent>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<&mut PlayerTimer, With<Character>>,
    mut query: Query<Entity, With<RunAudio>>,
) {
    if player_query.is_empty() {
        return;
    }
    let mut timer = player_query.single_mut();
    for _ in events.read() {
        timer.0.tick(time.delta());
        if timer.0.elapsed_secs() < 0.4 {
            return;
        }

        for e in query.iter_mut() {
            // println!("despawn run audio");
            commands.entity(e).despawn();
        }
        timer.0.reset();
        commands.spawn((
            AudioPlayer::new(asset_server.load("AudioClip/SE_EntityRun.wav")),
            PlaybackSettings::default().with_volume(Volume::new(0.9)),
            RunAudio,
        ));
    }
}
fn player_jump(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<PlayerJumpEvent>,
    mut query: Query<Entity, With<JumpAudio>>,
) {
    for _ in events.read() {
        for e in query.iter_mut() {
            // println!("despawn jump audio");
            commands.entity(e).despawn();
        }
        commands.spawn((
            AudioPlayer::new(asset_server.load("AudioClip/SE_EntityJump.wav")),
            PlaybackSettings::default().with_volume(Volume::new(0.9)),
            JumpAudio,
        ));
    }
}
fn pause(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity)>,
){
    for (audio, entity) in audio_sink.iter_mut() {
        commands.entity(entity).despawn();
    }
}