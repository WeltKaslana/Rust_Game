use bevy::{
    color::palettes::css::{BLUE, GREEN, RED}, dev_tools::states::*, prelude::*, scene::ron::de, text::cosmic_text::ttf_parser::name};
use bevy_rapier2d::na::ComplexField;

use crate::{
    boss::{Boss, BossDeathEvent, BossSetupEvent}, 
    character::{
        Character, Health, Player, PlayerHurtEvent, ReloadPlayerEvent, Skill2Timer, Skill3Timer, Skill4Timer
    }, 
    enemy::BaseSetupEvent, 
    gamestate::GameState, room::{Map, Progress}, 
    GlobalCharacterTextureAtlas, GlobalMenuTextureAtlas,
    configs::*,
};

pub struct UIPlugin;

#[derive(Component)]
pub struct PlayerHealthBarBottom;
#[derive(Component)]
pub struct Bar;
#[derive(Component)]
pub struct BufferBar;
#[derive(Component)]
pub struct SkillIcon;
#[derive(Component)]
pub struct UI;
#[derive(Component)]
pub struct Hurtui;

#[derive(Component)]
pub struct BossUI;

#[derive(Component)]
pub struct Bossbar;

#[derive(Component)]
pub struct Bossbufferbar;

#[derive(Component)]
pub struct TimerUI;

#[derive(Component)]
pub struct Timerbar;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<PlayerHurtEvent>()
        .add_systems(OnEnter(GameState::Home), setup_ui_all)
        .add_systems(Update, (
            hurtui,
            // update_ui,
            handle_state_bar,
            handle_player_skill,
            handle_boss_ui_setup,
            handle_boss_ui_update,
            handle_boss_ui_delete,
            handle_timer_ui_setup,
            handle_timer_ui_update,
            handle_timer_ui_delete,
        ))
        ;
    }
}
//ui相对于摄像头的偏移量
const UI_OFFSET: Vec3 = Vec3::new(-590.0, 240.0, 0.0);
const SKILL_OFFSET: [Vec3; 3]= [
    Vec3::new(530.0, -250.0, 0.0), 
    Vec3::new(630.0, -250.0, 0.0), 
    Vec3::new(730.0, -250.0, 0.0), 
];
static mut buffer_offset:f32 = 0.0;
static mut bar_offset:f32 = 0.0;

const  BOSSUI_OFFSET: Vec3 = Vec3::new(0.0, 300.0, 0.0);
static mut boss_buffer_offset:f32 = 0.0;
static mut boss_bar_offset:f32 = 0.0;

const TIMER_OFFSET: Vec3 = Vec3::new(0.0, 300.0, 0.0);
static mut timer_offset:f32 = -291.0;

fn setup_ui_all (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    loc_query: Query<&Transform, With<Camera2d>>,
    source: Res<GlobalCharacterTextureAtlas>,
    source1: Res<GlobalMenuTextureAtlas>,
) {
    unsafe {
        // 初始化血条偏移量
        bar_offset = 0.0;
        buffer_offset = 0.0;
    }
    if loc_query.is_empty() {
        return;
    }
    //将UI设置的坐标生成为相对相机坐标
    let loc = loc_query.single().translation.truncate();
    
    let font = asset_server.load("Fonts/FIXEDSYS-EXCELSIOR-301.ttf");
    //血条框
    commands.spawn((
        Sprite {
            image: asset_server.load("UI_Hub_PlayerHealth_Bar1.png"),
            ..Default::default()
        },
        // Transform::from_scale(Vec3::splat(0.5))
        //     .with_translation(Vec3::new(-630.0, -350.0, 110.0)),
        Transform::from_scale(Vec3::splat(0.5))
        .with_translation(Vec3::new(loc.x ,loc.y ,90.0) + UI_OFFSET),
        PlayerHealthBarBottom,
        UI,
    ))
    .with_child((
        Text2d::new(format!("{}/{}",PLAYER_HEALTH,PLAYER_HEALTH)),
        TextFont {
            font: font.clone(),
            font_size: 35.0,
            ..default()
        }, 
        Health(0.0),
        TextColor(Color::rgb(123.0, 157.0, 131.0)),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
    ));
    //缓冲血条
    commands.spawn((
        Sprite {
            image:asset_server.load("UI_Hub_PlayerHealth_1.png"),
            ..Default::default()
        },
        // Transform::from_scale(Vec3::splat(0.5))
        // .with_translation(Vec3::new(-630.0, -350.0, 109.0)),
        Transform::from_scale(Vec3::splat(0.5))
        .with_translation(Vec3::new(loc.x, loc.y, 70.0) + UI_OFFSET),
        BufferBar,
        UI,
    ));
    //血条
    commands.spawn((
        Sprite {
            image:asset_server.load("UI_Hub_PlayerHealth_3.png"),
            ..Default::default()
        },
        // Transform::from_scale(Vec3::splat(0.5))
        // .with_translation(Vec3::new(-630.0, -350.0, 109.0)),
        Transform::from_scale(Vec3::splat(0.5))
        .with_translation(Vec3::new(loc.x, loc.y, 80.0) + UI_OFFSET),
        Bar,
        UI,
    ));
    // 三个技能图标
    commands.spawn((
        Name::new("Skill2"),
        Sprite {
                image: match source.id {
                    1 => source1.shiroko_skill2.clone(),
                    2 => source1.arisu_skill2.clone(),
                    3 => source1.utaha_skill2.clone(),
                    _ => {
                        println!("Wrong id, connot set skill icon");
                        source1.shiroko_skill2.clone()
                    }
                },
                ..Default::default()
            },
        Transform::from_scale(Vec3::splat(0.8))
            .with_translation(SKILL_OFFSET[0] + Vec3::new(loc.x, loc.y, 90.0)),
        SkillIcon,
        UI, 
    )).with_child((
        Name::new("Skill2"),
        Text2d::new(format!("{}", SKILL2_CD as i32)),
        TextFont {
            font: font.clone(),
            font_size: 60.0,
            ..default()
        }, 
        TextColor(Color::rgb(123.0, 157.0, 131.0)),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
    ));

    commands.spawn((
        Name::new("Skill3"),
        Sprite {
                image: match source.id {
                    1 => source1.shiroko_skill3.clone(),
                    2 => source1.arisu_skill3.clone(),
                    3 => source1.utaha_skill3.clone(),
                    _ => {
                        println!("Wrong id, connot set skill icon");
                        source1.shiroko_skill3.clone()
                    }
                },
                ..Default::default()
            },
        Transform::from_scale(Vec3::splat(0.8))
            .with_translation(SKILL_OFFSET[1] + Vec3::new(loc.x, loc.y, 90.0)),
        SkillIcon,
        UI, 
    )).with_child((
        Name::new("Skill3"),
        Text2d::new(format!("{}", SKILL3_CD as i32)),
        TextFont {
            font: font.clone(),
            font_size: 60.0,
            ..default()
        }, 
        TextColor(Color::rgb(123.0, 157.0, 131.0)),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
    ));

    commands.spawn((
        Name::new("Skill4"),
        Sprite {
                image: match source.id {
                    1 => source1.shiroko_skill4.clone(),
                    2 => source1.arisu_skill4.clone(),
                    3 => source1.utaha_skill4.clone(),
                    _ => {
                        println!("Wrong id, connot set skill icon");
                        source1.shiroko_skill4.clone()
                    }
                },
                ..Default::default()
            },
        Transform::from_scale(Vec3::splat(0.8))
            .with_translation(SKILL_OFFSET[2] + Vec3::new(loc.x, loc.y, 90.0)),
        SkillIcon,
        UI, 
    )).with_child((
        Name::new("Skill4"),
        Text2d::new(format!("{}", SKILL4_CD as i32)),
        TextFont {
            font: font.clone(),
            font_size: 60.0,
            ..default()
        }, 
        TextColor(Color::rgb(123.0, 157.0, 131.0)),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
    ));

}


fn handle_state_bar(
    mut commands: Commands,
    loc_query: Query<&Transform, (With<Camera2d>, Without<Bar>, Without<Character>, Without<BufferBar>)>,
    health_query: Query<&mut Health, (With<Character>, Without<Bar>, Without<BufferBar>, Without<Camera2d>)>,
    mut buffer_query: Query<&mut Transform, (With<BufferBar>, Without<Character>, Without<Bar>, Without<Camera2d>)>,
    mut bar_query: Query<&mut Transform, (With<Bar>, Without<BufferBar>, Without<Character>, Without<Camera2d>)>,
    mut text_query: Query<&mut Text2d, With<Health>>,//后续可能文本框不止这一个，需要加限制过滤
    query2: Query<Entity, (With<Hurtui>, Without<Camera2d>)>,
    mut ui_query: Query<&mut Transform, (With<PlayerHealthBarBottom>, Without<Camera2d>, Without<BufferBar>, Without<Bar>, Without<Character>)>,
) {
    if health_query.is_empty() ||buffer_query.is_empty() || bar_query.is_empty() || loc_query.is_empty(){
        return;
    }
    let health =health_query.single();

    if !text_query.is_empty() {
        for mut text in text_query.iter_mut() {
            text.0 = format!("{}/{}",health.0 as i32,PLAYER_HEALTH);
        }
    }

    let mut buffer = buffer_query.single_mut();
    let mut bar = bar_query.single_mut();
    //控制血条位置
    let loc = loc_query.single().translation.truncate();

    unsafe {
        buffer.translation = Vec3::new(loc.x + buffer_offset, loc.y , buffer.translation.z) + UI_OFFSET;
        bar.translation = Vec3::new(loc.x + bar_offset, loc.y , bar.translation.z) + UI_OFFSET;  
    }

    //血条框位置
    for mut trans in ui_query.iter_mut() {
        trans.translation = Vec3::new(loc.x ,loc.y ,trans.translation.z) + UI_OFFSET;
    }

    //血条控制
    let mut delta = bar.scale.x;
    let barwidth = 582.0; //582为血条宽度
    bar.scale.x = health.0 / PLAYER_HEALTH * 0.5;//0.5是最开始血条的缩放比例
    //血条要位移，因为缩放是两边向中间缩放
    delta -= bar.scale.x;

    unsafe {
        let temp = delta * 0.5 * barwidth;
        bar_offset -= temp;
        bar.translation.x -= temp;//提前响应，不然左侧会有瑕疵
    }

    //缓冲血条控制
    if buffer.scale.x > bar.scale.x {
        let bufferspeed = 0.001;
        buffer.scale.x -= bufferspeed;

        let temp = bufferspeed * 0.5 * barwidth;
        unsafe {
            buffer_offset -= temp;
            buffer.translation.x -= temp;//提前响应，不然左侧会有瑕疵
        }
        
    } else {
        let delta2 = bar.scale.x - buffer.scale.x;
        buffer.scale.x = bar.scale.x;
        let temp = delta2 * 0.5 * barwidth;
        unsafe {
            buffer_offset += temp;
            buffer.translation.x += temp;
        }

        for entity in query2.iter() {
            commands.entity(entity).despawn();
        }

    }
}

fn handle_player_skill(
    mut events: EventReader<ReloadPlayerEvent>,
    timer_query: Query<(&Skill2Timer, &Skill3Timer, &Skill4Timer), (With<Character>, Without<Camera2d>, Without<SkillIcon>)>,
    loc_query: Query<&Transform, (With<Camera2d>, Without<SkillIcon>)>,
    mut skill_query: Query<(&mut Transform, &mut Sprite, &Name), (With<SkillIcon>, Without<Camera2d>)>,
    mut text_query: Query<(&mut Text2d, &Name, &mut Visibility)>,
    player_source: Res<GlobalCharacterTextureAtlas>,
    source: Res<GlobalMenuTextureAtlas>,
) {
    if loc_query.is_empty() || skill_query.is_empty() {
        return;
    }
    // 获取技能冷却时间,有后续多角色访问崩溃的风险
    let (timer2, timer3, timer4) = timer_query.single();
    // println!("Skill2: {}, Skill3: {}, Skill4: {}", timer2.0.elapsed_secs().ceil() as i32, timer3.0.elapsed_secs().ceil() as i32, timer4.0.elapsed_secs().ceil() as i32);
    
    // 处理角色切换
    let mut reload = false;
    for _ in events.read() {
        reload = true;
        break;
    }

    let loc = loc_query.single().translation.truncate();
    for (mut trans, mut icon, name) in skill_query.iter_mut() {
        match name.as_str() {
            "Skill2" => {
                trans.translation = Vec3::new(loc.x, loc.y, 90.0) + SKILL_OFFSET[0];
                let cd = timer2.0.elapsed_secs();
                if cd < 0.017 || (reload && cd < SKILL2_CD) { // 1/60
                    icon.image = match player_source.id {
                        1 => {
                            source.shiroko_skill2_cool.clone()
                        },
                        2 => {
                            source.arisu_skill2_cool.clone()
                        },
                        3 => {
                           source.utaha_skill2_cool.clone()
                        },
                        _ => {
                            println!("Wrong id, connot set skill icon");
                            source.shiroko_skill2_cool.clone()
                        }
                    };
                } else if cd > SKILL2_CD && cd <= SKILL2_CD + 0.017 || (reload && cd >= SKILL2_CD) { 
                    icon.image = match player_source.id {
                        1 => {
                            source.shiroko_skill2.clone()
                        },
                        2 => {
                            source.arisu_skill2.clone()
                        },
                        3 => {
                           source.utaha_skill2.clone()
                        },
                        _ => {
                            println!("Wrong id, connot set skill icon");
                            source.shiroko_skill2.clone()
                        }
                    };
                }
            },
            "Skill3" => {
                trans.translation = Vec3::new(loc.x, loc.y, 90.0) + SKILL_OFFSET[1];
                let cd = timer3.0.elapsed_secs();
                if cd < 0.017 || (reload && cd < SKILL3_CD) { // 1/60
                    icon.image = match player_source.id {
                        1 => {
                            source.shiroko_skill3_cool.clone()
                        },
                        2 => {
                            source.arisu_skill3_cool.clone()
                        },
                        3 => {
                           source.utaha_skill3_cool.clone()
                        },
                        _ => {
                            println!("Wrong id, connot set skill icon");
                            source.shiroko_skill3_cool.clone()
                        }
                    };
                } else if cd > SKILL3_CD && cd <= SKILL3_CD + 0.017 || (reload && cd >= SKILL3_CD) { 
                    icon.image = match player_source.id {
                        1 => {
                            source.shiroko_skill3.clone()
                        },
                        2 => {
                            source.arisu_skill3.clone()
                        },
                        3 => {
                           source.utaha_skill3.clone()
                        },
                        _ => {
                            println!("Wrong id, connot set skill icon");
                            source.shiroko_skill3.clone()
                        }
                    };
                }
            },
            "Skill4" => {
                trans.translation = Vec3::new(loc.x, loc.y, 90.0) + SKILL_OFFSET[2];
                let cd = timer4.0.elapsed_secs();
                if cd < 0.017 || (reload && cd < SKILL4_CD) { // 1/60
                    icon.image = match player_source.id {
                        1 => {
                            source.shiroko_skill4_cool.clone()
                        },
                        2 => {
                            source.arisu_skill4_cool.clone()
                        },
                        3 => {
                           source.utaha_skill4_cool.clone()
                        },
                        _ => {
                            println!("Wrong id, connot set skill icon");
                            source.shiroko_skill4_cool.clone()
                        }
                    };
                } else if cd > SKILL4_CD && cd <= SKILL4_CD + 0.017 || (reload && cd >= SKILL4_CD) { 
                    icon.image = match player_source.id {
                        1 => {
                            source.shiroko_skill4.clone()
                        },
                        2 => {
                            source.arisu_skill4.clone()
                        },
                        3 => {
                           source.utaha_skill4.clone()
                        },
                        _ => {
                            println!("Wrong id, connot set skill icon");
                            source.shiroko_skill4.clone()
                        }
                    };
                }
            },
            _ => {
                println!("Wrong icon, connot update skill icon");
            }
        }
    }
    for (mut cd, name, mut vis) in text_query.iter_mut() {
        match name.as_str() {
            "Skill2" => {
                let temp = timer2.0.elapsed_secs();
                cd.0 = format!("{}", (SKILL2_CD - temp).ceil() as i32);
                if temp > SKILL2_CD {
                    *vis = Visibility::Hidden;
                } else {
                    *vis = Visibility::Visible;
                }
            },
            "Skill3" => {
                let temp = timer3.0.elapsed_secs();
                cd.0 = format!("{}", (SKILL3_CD - temp).ceil() as i32);
                if temp > SKILL3_CD {
                    *vis = Visibility::Hidden;
                } else {
                    *vis = Visibility::Visible;
                }
            },
            "Skill4" => {
                let temp = timer4.0.elapsed_secs();
                cd.0 = format!("{}", (SKILL4_CD - temp).ceil() as i32);
                if temp > SKILL4_CD {
                    *vis = Visibility::Hidden;
                } else {
                    *vis = Visibility::Visible;
                }
            },
            _ => {}
        }
    }
}

fn hurtui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    loc_query: Query<&Transform, (With<Camera2d>, Without<Hurtui>)>,
    mut hurtui_query: Query<&mut Transform, (With<Hurtui>, Without<Camera2d>)>,
    mut event: EventReader<PlayerHurtEvent>, //后续角色受伤时发出事件，ui响应
) {
    if loc_query.is_empty() {
        return;
    }
    let loc = loc_query.single().translation.truncate();
    for _ in event.read() {
        // println!("Hurt!");//test
        commands.spawn((
            Sprite {
                image: asset_server.load("UI_Hit.png"),
                ..Default::default()
            },
            Transform::from_scale(Vec3::new(1.9, 1.4, 1.9))
                .with_translation(Vec3::new(loc.x, loc.y, 111.0)),
            Hurtui,
            Player,
        ));     
    }
    for mut trans in hurtui_query.iter_mut() {
        trans.translation = Vec3::new(loc.x ,loc.y ,111.0);
    }
}

fn handle_boss_ui_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut setupevent: EventReader<BossSetupEvent>,
    camera_query: Query<&Transform, With<Camera2d>>,
    // health_query: Query<&Health, (With<Boss>, Without<BossUI>, Without<Camera2d>)>,
    // mut bar_query: Query<&mut Transform, (With<Bossbar>, Without<Boss>, Without<Bossbufferbar>, Without<Camera2d>)>,
    // mut buffer_query: Query<&mut Transform, (With<Bossbufferbar>, Without<Camera2d>, Without<Bossbar>, Without<Boss>)>,
    // mut ui_query: Query<&mut Transform, (With<BossUI>, Without<Camera2d>, Without<Boss>, Without<Bossbar>, Without<Bossbufferbar>)>,
) {
    if camera_query.is_empty() {
        return;
    }
    let loc = camera_query.single().translation.truncate();
    for _ in setupevent.read() {
        unsafe {
            boss_bar_offset = 0.0;
            boss_buffer_offset = 0.0;
        }
        commands.spawn((
            Sprite {
                image: asset_server.load("UI_Hub_PlayerHealth_Bar.png"),
                ..default()
            },
            Transform::from_scale(Vec3::splat(1.0))
            .with_translation(Vec3::new(loc.x, loc.y, 90.0) + BOSSUI_OFFSET ),
            BossUI,
            Map,
        ));
        commands.spawn((
            Sprite {
                image: asset_server.load("UI_Hub_PlayerHealth_1.png"),
                ..default()
            },
            Transform::from_scale(Vec3::splat(1.0))
            .with_translation(Vec3::new(loc.x, loc.y, 70.0) + BOSSUI_OFFSET ),
            BossUI,
            Bossbufferbar,
            Map,
        ));
        commands.spawn((
            Sprite {
                image: asset_server.load("UI_Hub_PlayerHealth_2.png"),
                ..default()
            },
            Transform::from_scale(Vec3::splat(1.0))
            .with_translation(Vec3::new(loc.x, loc.y, 80.0) + BOSSUI_OFFSET ),
            BossUI,
            Bossbar,
            Map,
        ));
        break;
    }
}

fn handle_boss_ui_update(
    camera_query: Query<&Transform, (With<Camera2d>, Without<BossUI>, Without<Boss>)>,
    health_query: Query<&Health, (With<Boss>, Without<BossUI>, Without<Camera2d>)>,
    mut bar_query: Query<&mut Transform, (With<Bossbar>, Without<Boss>, Without<Bossbufferbar>, Without<Camera2d>)>,
    mut buffer_query: Query<&mut Transform, (With<Bossbufferbar>, Without<Camera2d>, Without<Bossbar>, Without<Boss>)>,
    mut ui_query: Query<&mut Transform, (With<BossUI>, Without<Camera2d>, Without<Boss>, Without<Bossbar>, Without<Bossbufferbar>)>,
) {
    if camera_query.is_empty() ||  health_query.is_empty() || ui_query.is_empty() || buffer_query.is_empty() || bar_query.is_empty() {
        return ;
    }
    let health =  health_query.single();
    let mut buffer = buffer_query.single_mut();
    let mut bar = bar_query.single_mut();
    let loc = camera_query.single().translation.truncate();

    unsafe {
        buffer.translation = Vec3::new(loc.x + boss_buffer_offset, loc.y, buffer.translation.z) + BOSSUI_OFFSET;
        bar.translation = Vec3::new(loc.x + boss_bar_offset, loc.y, bar.translation.z) + BOSSUI_OFFSET;
    }

    let mut trans = ui_query.single_mut();
    trans.translation = Vec3::new(loc.x, loc.y, trans.translation.z) + BOSSUI_OFFSET;

    let mut delta = bar.scale.x;
    let barwidth = 582.0; //582为血条宽度
    bar.scale.x = health.0 / BOSS_HEALTH * 1.0;//0.5是最开始血条的缩放比例
    //血条要位移，因为缩放是两边向中间缩放
    delta -= bar.scale.x;

    unsafe {
        let temp = delta * 0.5 * barwidth;
        boss_bar_offset -= temp;
        bar.translation.x -= temp;//提前响应，不然左侧会有瑕疵
    }

    //缓冲血条控制
    if buffer.scale.x > bar.scale.x {
        let bufferspeed = 0.001;
        buffer.scale.x -= bufferspeed;

        let temp = bufferspeed * 0.5 * barwidth;
        unsafe {
            boss_buffer_offset -= temp;
            buffer.translation.x -= temp;//提前响应，不然左侧会有瑕疵
        }
        
    } else {
        let delta2 = bar.scale.x - buffer.scale.x;
        buffer.scale.x = bar.scale.x;
        let temp = delta2 * 0.5 * barwidth;
        unsafe {
            boss_buffer_offset += temp;
            buffer.translation.x += temp;
        }
    }

}

fn handle_boss_ui_delete(
    mut commands: Commands,
    mut death_event:  EventReader<BossDeathEvent>,
    boss_ui_query: Query<Entity, With<BossUI>>,
) {
    if boss_ui_query.is_empty() {
        return;
    }
    for _ in death_event.read() {
        for entity in boss_ui_query.iter() {
            commands.entity(entity).despawn();
        }
        break;
    }
}


fn handle_timer_ui_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut setupevent: EventReader<BaseSetupEvent>,
    camera_query: Query<&Transform, With<Camera2d>>,
) {
    if camera_query.is_empty() {
        return;
    }
    let loc = camera_query.single().translation.truncate();
    for _ in setupevent.read() {
        unsafe {
            timer_offset = 0.0;
        }
        commands.spawn((
            Sprite {
                image: asset_server.load("UI_Hub_PlayerHealth_Bar.png"),
                ..default()
            },
            Transform::from_scale(Vec3::splat(1.0))
            .with_translation(Vec3::new(loc.x, loc.y, 90.0) + TIMER_OFFSET ),
            TimerUI,
            Map,
        ));
        commands.spawn((
            Sprite {
                image: asset_server.load("UI_Hub_PlayerHealth_2.png"),
                ..default()
            },
            Transform::from_scale(Vec3::splat(1.0))
            .with_translation(Vec3::new(loc.x, loc.y, 80.0) + TIMER_OFFSET ),
            TimerUI,
            Timerbar,
            Map,
        ));
        break;
    }
}

fn handle_timer_ui_update(
    camera_query: Query<&Transform, (With<Camera2d>, Without<TimerUI>)>,
    progress_query: Query<&Progress, With<Progress>>,
    mut bar_query: Query<&mut Transform, (With<Timerbar>, With<TimerUI>, Without<Camera2d>)>,
    mut ui_query: Query<&mut Transform, (Without<Timerbar>, With<TimerUI>, Without<Camera2d>)>
) {
    if camera_query.is_empty() ||progress_query.is_empty() || bar_query.is_empty() || ui_query.is_empty() {
        return;
    }
    let progress = progress_query.single();
    let mut bar = bar_query.single_mut();
    let loc = camera_query.single().translation.truncate();

    unsafe {
        bar.translation = Vec3::new(loc.x + timer_offset, loc.y, bar.translation.z) + TIMER_OFFSET;
    }

    let mut trans = ui_query.single_mut();
    trans.translation = Vec3::new(loc.x, loc.y, trans.translation.z) + TIMER_OFFSET;
    
    let mut delta = bar.scale.x;
    let barwidth = 582.0;
    bar.scale.x = progress.0 / 90.0;
    delta -= bar.scale.x;

    unsafe {
        let temp = delta * 0.5 * barwidth;
        timer_offset -= temp;
        bar.translation.x -= temp;//提前响应，不然左侧会有瑕疵
    }
}

fn handle_timer_ui_delete(
    mut commands: Commands,
    timer_query: Query<Entity, With<Progress>>,
    ui_query: Query<Entity, With<TimerUI>>,
) {
    if ui_query.is_empty() {
        return;
    }
    if timer_query.is_empty() {
        let entity = ui_query.single();
        commands.entity(entity).despawn();
    }
}