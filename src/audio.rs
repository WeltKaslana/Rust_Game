use bevy::{
    audio::{AudioPlayer, AudioSource, PlaybackSettings, Volume}, dev_tools::states::*, prelude::*, state::commands
};
use crate::{
    animation::{BossRoarEvent, ChestOpenEvent, DoorEvent}, character::{Character, PlayerHurtEvent, PlayerJumpEvent, PlayerParryEvent, PlayerRunEvent, PlayerSkill2Event, PlayerSkill3Event, PlayerSkill4Event, PlayerTimer}, gamestate::GameState, gun::{PlayerFireEvent, PlayerSkill4FireEvent}, room::{self, AssetsManager, RoomCleanEvent}, GlobalCharacterTextureAtlas, AUDIOVOLUME, ROOMS
};
use rand::Rng;
pub struct GameAudioPlugin;

#[derive(Component)]
pub struct FireAudio;
#[derive(Component)]
pub struct RunAudio;
#[derive(Component)]
pub struct JumpAudio;
#[derive(Component)]
pub struct HurtAudio;
#[derive(Component)]
pub struct InGameBGM;

#[derive(Resource,Default)]
pub struct GameAudioSource {
    // common
    pub menu_bgm: Handle<AudioSource>,
    pub home_bgm: Handle<AudioSource>,
    pub in_game_bgm: Handle<AudioSource>,
    pub boss_bgm: Handle<AudioSource>,
    pub walk: Handle<AudioSource>,
    pub jump: Handle<AudioSource>,

    pub fridge_open: Handle<AudioSource>,
    pub fridge_close: Handle<AudioSource>,

    pub chest_open: Handle<AudioSource>,
    pub boss_roar: Handle<AudioSource>,
    pub door_open: Handle<AudioSource>,
    pub door_close: Handle<AudioSource>,

    pub room_clean: Handle<AudioSource>,

    // shiroko
    pub shiroko_hurt: [Handle<AudioSource>;3],
    pub shiroko_shout: [Handle<AudioSource>;3],
    pub shiroko_gun_fire: Handle<AudioSource>,
    pub shiroko_gameover: Handle<AudioSource>,
    pub shiroko_skill: [Handle<AudioSource>;3],
    pub shiroko_skill_talk: [Handle<AudioSource>;3],


    // arisu
    pub arisu_hurt: [Handle<AudioSource>;3],
    pub arisu_shout: [Handle<AudioSource>;5],
    pub arisu_gun_fire: Handle<AudioSource>,
    pub arisu_gun_fire_special: [Handle<AudioSource>;2],
    pub arisu_skill_talk: [Handle<AudioSource>;3],
    pub arisu_gun_shieldblock: Handle<AudioSource>,
    pub arisu_gameover: Handle<AudioSource>,

    // utaha
    pub utaha_hurt: [Handle<AudioSource>;3],
    pub utaha_attack: Handle<AudioSource>,
    pub utaha_shout: [Handle<AudioSource>;3],
    pub utaha_skill_talk: [Handle<AudioSource>;3],
    pub utaha_skill: [Handle<AudioSource>;3],
}

impl GameAudioSource {
    pub fn init(asset_server: &Res<AssetServer>,)->Self {
        Self {
            // common
            menu_bgm: asset_server.load("AudioClip/MainMenu - Takaramonogatari.wav"),
            home_bgm: asset_server.load("AudioClip/Angel24 - Cotton Candy Island.wav"),
            in_game_bgm: asset_server.load("AudioClip/Level1 - Let me think about it.wav"),
            boss_bgm: asset_server.load("AudioClip/Boss1 - KARAKURhythm.wav"),
            walk: asset_server.load("AudioClip/SE_EntityRun.wav"),
            jump: asset_server.load("AudioClip/SE_EntityJump.wav"),
            
            fridge_open: asset_server.load("AudioClip/SE_Door_Open.wav"),
            fridge_close: asset_server.load("AudioClip/SE_Door_Close.wav"),

            chest_open: asset_server.load("AudioClip/SE_Chest_Open.wav"),
            boss_roar: asset_server.load("AudioClip/SE_Boss_Roar.wav"),
            door_open: asset_server.load("AudioClip/SE_BossDoor_Open.wav"),
            door_close: asset_server.load("AudioClip/SE_BossDoor_Close.wav"),

            room_clean: asset_server.load("AudioClip/SE_Boss_Death.wav"),
            // shiroko
            shiroko_hurt: [
                asset_server.load("AudioClip/Shiroko_Battle_Damage_1.wav"),
                asset_server.load("AudioClip/Shiroko_Battle_Damage_2.wav"),
                asset_server.load("AudioClip/Shiroko_Battle_Damage_3.wav"),
            ],
            shiroko_shout: [
                asset_server.load("AudioClip/Shiroko_Battle_Shout_1.wav"),
                asset_server.load("AudioClip/Shiroko_Battle_Shout_2.wav"),
                asset_server.load("AudioClip/Shiroko_Battle_Shout_3.wav"),
            ],
            shiroko_gun_fire: asset_server.load("AudioClip/SE_Shiroko_Attack.wav"),
            shiroko_gameover: asset_server.load("AudioClip/Shiroko_Battle_Retire.wav"),
            shiroko_skill: [
                asset_server.load("AudioClip/Shiroko_CommonSkill.wav"),
                asset_server.load("AudioClip/SE_Shiroko_Explosion.wav"),
                asset_server.load("AudioClip/SE_Shiroko_Ex.wav"),
            ],
            shiroko_skill_talk: [
                asset_server.load("AudioClip/Shiroko_ExSkill_Level_1.wav"),
                asset_server.load("AudioClip/Shiroko_ExSkill_Level_2.wav"),
                asset_server.load("AudioClip/Shiroko_ExSkill_Level_3.wav"),
            ],
            // arisu
            arisu_hurt: [
                asset_server.load("AudioClip/Arisu_(Maid)_Battle_Damage_1.ogg.wav"),
                asset_server.load("AudioClip/Arisu_(Maid)_Battle_Damage_2.ogg.wav"),
                asset_server.load("AudioClip/Arisu_(Maid)_Battle_Damage_3.ogg.wav"),
            ],
            arisu_shout: [
                asset_server.load("AudioClip/Arisu_(Maid)_Battle_Shout_1.ogg.wav"),
                asset_server.load("AudioClip/Arisu_(Maid)_Battle_Shout_2.ogg.wav"),
                asset_server.load("AudioClip/Arisu_(Maid)_MiniGame_Shout_1.ogg.wav"),
                asset_server.load("AudioClip/Arisu_(Maid)_MiniGame_Shout_2.ogg.wav"),
                asset_server.load("AudioClip/Arisu_(Maid)_MiniGame_Shout_3.ogg.wav"),  
            ],
            arisu_gun_fire: asset_server.load("AudioClip/SE_Aris_Attack.wav"),
            arisu_gun_fire_special: [
                asset_server.load("AudioClip/SE_Aris_Ex_Start.wav"),
                asset_server.load("AudioClip/SE_Aris_Ex_End.wav"),
            ],
            arisu_skill_talk: [
                asset_server.load("AudioClip/Arisu_(Maid)_ExSkill_Level_1.ogg.wav"),
                asset_server.load("AudioClip/Arisu_(Maid)_ExSkill_Level_2.ogg.wav"),
                asset_server.load("AudioClip/Arisu_(Maid)_ExSkill_Level_3.ogg.wav"),
            ],
            arisu_gun_shieldblock: asset_server.load("AudioClip/SE_Aris_ShieldBlock.wav"),
            arisu_gameover: asset_server.load("AudioClip/Arisu_(Maid)_Battle_Damage_3.ogg.wav"),
            // utaha
            utaha_hurt: [
                asset_server.load("AudioClip/Utaha_(Cheerleader)_Battle_Damage_1.ogg.wav"),
                asset_server.load("AudioClip/Utaha_(Cheerleader)_Battle_Damage_2.ogg.wav"),
                asset_server.load("AudioClip/Utaha_(Cheerleader)_Battle_Damage_3.ogg.wav"),
            ],
            utaha_attack: asset_server.load("AudioClip/SE_Utaha_Attack.wav"),
            utaha_shout: [
                asset_server.load("AudioClip/Utaha_(Cheerleader)_Battle_Shout_1.ogg.wav"),
                asset_server.load("AudioClip/Utaha_(Cheerleader)_Battle_Shout_2.ogg.wav"),
                asset_server.load("AudioClip/Utaha_(Cheerleader)_Battle_Shout_3.ogg.wav"),
            ],
            utaha_skill_talk: [
                asset_server.load("AudioClip/Utaha_(Cheerleader)_ExSkill_3.wav"),
                asset_server.load("AudioClip/Utaha_ExSkill_1.ogg.wav"),
                asset_server.load("AudioClip/Utaha_ExSkill_3.ogg.wav"),
            ],
            utaha_skill: [
                asset_server.load("AudioClip/SE_Utaha_Secondary_Rotate.wav"),
                asset_server.load("AudioClip/SE_Small_Explosion.wav"),
                asset_server.load("AudioClip/SE_Utaha_Special_Landing.wav"),
            ]
        }
    }
}


impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerFireEvent>()
            .add_event::<PlayerRunEvent>()
            .add_event::<PlayerJumpEvent>()
            .add_event::<PlayerParryEvent>()
            .add_event::<RoomCleanEvent>()
            .add_systems(Startup, load_audio)
            .add_systems(OnEnter(GameState::MainMenu), audio_play__MainMenu)
            .add_systems(OnExit(GameState::MainMenu), pause)
            .add_systems(OnEnter(GameState::Home), audio_play_Home)
            .add_systems(OnExit(GameState::Home), pause)
            .add_systems(OnEnter(GameState::InGame), audio_play_Ingame)
            // .add_systems(OnExit(GameState::InGame), pause)

            .add_systems(Update,(
                        audio_fire,
                        player_jump,
                        player_run,
                        player_hurt,
                        player_skill2,
                        player_skill3,
                        player_skill4,
                        home_effect_audio,
                        ).run_if(in_state(GameState::Home)))
            .add_systems(Update,(
                audio_fire,
                player_jump,
                player_run,
                player_hurt,
                player_skill2,
                player_skill3,
                player_skill4,
                room_clean,
                ingame_effect_audio,
                ).run_if(in_state(GameState::InGame)))
            // .add_systems(Update, log_transitions::<GameState>)
            ;
    }
}

fn load_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let audio = GameAudioSource::init(&asset_server);
    commands.insert_resource(audio);
    info!("Audio loaded!");
}
// 判断当前游戏中音乐是否在播放，避免进到下一房间时重复播放
static mut hasplay: bool = false;
fn audio_play__MainMenu(
    mut commands: Commands,
    bgm_query: Query<Entity, With<InGameBGM>>,
    asset_server: Res<AssetServer>,
) {
    for e in bgm_query.iter() {
        commands.entity(e).despawn_recursive();
    }
    unsafe {
        hasplay = false;
    }
    commands.spawn((
        AudioPlayer::new(asset_server.load("AudioClip/MainMenu - Takaramonogatari.wav")),
        PlaybackSettings::LOOP.with_volume(Volume::new(AUDIOVOLUME)),
    ));
}

fn audio_play_Home(
    mut commands: Commands,
    bgm_query: Query<Entity, With<InGameBGM>>,
    source:  Res<GameAudioSource>,
) {
    for e in bgm_query.iter() {
        commands.entity(e).despawn_recursive();
    }
    unsafe {
        hasplay = false;
    }
    commands.spawn((
        AudioPlayer::new(source.home_bgm.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::new(AUDIOVOLUME)),
    ));

}


fn audio_play_Ingame(
    mut commands: Commands,
    mgr: Res<AssetsManager>,
    bgm_query: Query<Entity, With<InGameBGM>>, 
    source:  Res<GameAudioSource>,
) {
    let mut ifplay = false;
    unsafe {
        if hasplay == false {
            hasplay = true;
            ifplay = true;
        }
    }
    if ifplay && bgm_query.is_empty() {
        commands.spawn((
            AudioPlayer::new(source.in_game_bgm.clone()),
            PlaybackSettings::LOOP.with_volume(Volume::new(AUDIOVOLUME)),
            InGameBGM,
        ));
        return;
    }
    if mgr.map_index == 0 {
        for e in bgm_query.iter() {
            commands.entity(e).despawn();
        }
        commands.spawn((
            AudioPlayer::new(source.boss_bgm.clone()),
            PlaybackSettings::LOOP.with_volume(Volume::new(AUDIOVOLUME)),
            InGameBGM,
        ));
        println!("BGM: BossInGameBGM");
        return;
    }
    if mgr.map_index == 1 {
        for e in bgm_query.iter() {
            commands.entity(e).despawn();
        }
        commands.spawn((
            AudioPlayer::new(source.in_game_bgm.clone()),
            PlaybackSettings::LOOP.with_volume(Volume::new(AUDIOVOLUME)),
            InGameBGM,
        ));
        println!("BGM: InGameBGM");
        return;
    }
}

fn room_clean (
    mut events: EventReader<RoomCleanEvent>,
    mut commands: Commands,
    source: Res<GameAudioSource>,
) {
    for _ in events.read() {
        println!("room clean audio");
        commands.spawn((
            AudioPlayer::new(source.room_clean.clone()),
            PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME * 1.5)),
        )); 
        break;
    }
}

fn audio_fire (
    mut events: EventReader<PlayerFireEvent>,
    mut commands: Commands,
    source:  Res<GameAudioSource>,
    player: Res<GlobalCharacterTextureAtlas>,
) {
    for PlayerFireEvent(id) in events.read() {
        let audio = match player.id {
            1 => source.shiroko_gun_fire.clone(),
            2 => source.arisu_gun_fire.clone(),
            3 => match id {
                0 => source.shiroko_gun_fire.clone(),
                1 => source.utaha_attack.clone(),
                _ => source.shiroko_gun_fire.clone(),
            },
            _ => {
                dbg!("wrong player id!");
                source.shiroko_gun_fire.clone()
            }
        };
        commands.spawn((
            AudioPlayer::new(audio),
            PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
            FireAudio,
        ));
    }
}
fn player_run (
    mut events: EventReader<PlayerRunEvent>,
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<&mut PlayerTimer, With<Character>>,
    mut query: Query<Entity, With<RunAudio>>,
    source:  Res<GameAudioSource>
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
            AudioPlayer::new(source.walk.clone()),
            PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
            RunAudio,
        ));
    }
}
fn player_jump(
    mut commands: Commands,
    mut events: EventReader<PlayerJumpEvent>,
    mut query: Query<Entity, With<JumpAudio>>,
    source:  Res<GameAudioSource>
) {
    for _ in events.read() {
        for e in query.iter_mut() {
            // println!("despawn jump audio");
            commands.entity(e).despawn();
        }
        commands.spawn((
            AudioPlayer::new(source.jump.clone()),
            PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
            JumpAudio,
        ));
    }
}

fn player_hurt(
    mut commands: Commands,
    mut events: EventReader<PlayerHurtEvent>,
    query: Query<Entity, With<HurtAudio>>,
    source:  Res<GameAudioSource>,
    player: Res<GlobalCharacterTextureAtlas>,
) {
    for _ in events.read() {
        for e in query.iter() {
            // println!("despawn jump audio");
            commands.entity(e).despawn();
        }
        let possible = rand::rng().random_range(0..9);
        let index = rand::rng().random_range(0..3);
        let audio = match player.id {
            1 => source.shiroko_hurt.clone(),
            2 => source.arisu_hurt.clone(),
            3 => source.utaha_hurt.clone(),
            _ => {
                dbg!("wrong player id!");
                source.shiroko_hurt.clone()
            }
        };
        if possible>5 {
            commands.spawn((
                AudioPlayer::new(audio[index].clone()),
                PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                HurtAudio,
            ));
        }

    }
    
}

fn player_skill2(
    mut events: EventReader<PlayerSkill2Event>,
    mut events2: EventReader<PlayerParryEvent>,
    mut commands: Commands,
    source:  Res<GameAudioSource>,
    player: Res<GlobalCharacterTextureAtlas>,
) {
    for _ in events.read() {
        match player.id {
            1 => {
                // shiroko
                commands.spawn((
                    AudioPlayer::new(source.shiroko_shout[rand::rng().random_range(0..3)].clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
            },
            2 =>{
                // arisu
                commands.spawn((
                    AudioPlayer::new(source.arisu_skill_talk[1].clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
            },
            3 =>{
                // utaha
                commands.spawn((
                    AudioPlayer::new(source.utaha_skill_talk[0].clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
                commands.spawn((
                    AudioPlayer::new(source.utaha_skill[0].clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
            },
            _ => {}
        }
    }
    for _ in events2.read() {
        // arisu
        commands.spawn((
            AudioPlayer::new(source.arisu_gun_shieldblock.clone()),
            PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
        ));
    }
}

fn player_skill3(
    mut events: EventReader<PlayerSkill3Event>,
    mut commands: Commands,
    source:  Res<GameAudioSource>,
) {
    for _ in events.read() {
        commands.spawn((
            AudioPlayer::new(source.shiroko_skill[1].clone()),
            PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
        ));
    }
}
fn player_skill4(
    mut events: EventReader<PlayerSkill4Event>,
    mut events2: EventReader<PlayerSkill4FireEvent>,
    mut commands: Commands,
    source:  Res<GameAudioSource>,
    player: Res<GlobalCharacterTextureAtlas>,
) {
    for _ in events.read() {
        match player.id {
            1 => {
                // shiroko
                commands.spawn((
                    AudioPlayer::new(source.shiroko_skill[2].clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
                commands.spawn((
                    AudioPlayer::new(source.shiroko_skill_talk[2].clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
            },
            2 =>{
                // arisu
                commands.spawn((
                    AudioPlayer::new(source.arisu_skill_talk[0].clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
                commands.spawn((
                    AudioPlayer::new(source.arisu_gun_fire_special[0].clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
            },
            3 =>{
                // utaha
                commands.spawn((
                    AudioPlayer::new(source.utaha_skill_talk[2].clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
                commands.spawn((
                    AudioPlayer::new(source.utaha_skill[2].clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
            }
            _ => {}
        }
    }
    for _ in events2.read() {
        match player.id {
            1 => {},
            2 =>{
                // arisu
                commands.spawn((
                    AudioPlayer::new(source.arisu_gun_fire_special[1].clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
            }
            _ => {}
        }
    }
}


fn home_effect_audio (
    mut commands: Commands,
    mut fridge: EventReader<DoorEvent>,
    source: Res<GameAudioSource>,
) {
    for DoorEvent(id) in fridge.read() {
        match id {
            3 => {
                commands.spawn((
                    AudioPlayer::new(source.fridge_open.clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
            },
            4 => {
                commands.spawn((
                    AudioPlayer::new(source.fridge_close.clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
            },
            _ => {}
        }
    }
}

fn ingame_effect_audio  (
    mut commands: Commands,
    mut door: EventReader<DoorEvent>,
    mut bossroar: EventReader<BossRoarEvent>,
    mut chest: EventReader<ChestOpenEvent>,
    source: Res<GameAudioSource>,
) {
    for DoorEvent(id) in door.read() {
        match id {
            1 => {
                commands.spawn((
                    AudioPlayer::new(source.door_open.clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
            },
            2 => {
                commands.spawn((
                    AudioPlayer::new(source.door_close.clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
                ));
            },
            _ => {}
        }
    }
    for _ in bossroar.read() {
        commands.spawn((
            AudioPlayer::new(source.boss_roar.clone()),
            PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
        ));
    }
    for _ in chest.read() {
        commands.spawn((
            AudioPlayer::new(source.chest_open.clone()),
            PlaybackSettings::DESPAWN.with_volume(Volume::new(AUDIOVOLUME)),
        ));
    }
}

fn pause(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity)>,
){
    for (_, entity) in audio_sink.iter_mut() {
        commands.entity(entity).despawn();
    }
}