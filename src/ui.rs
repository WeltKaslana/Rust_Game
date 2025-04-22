use bevy::{
    dev_tools::states::*, 
    prelude::*, 
    color::palettes::css::{BLUE, GREEN, RED},};

use crate::{
    character::{
        Character,
        Health,
        PlayerHurtEvent
    }, 
    gamestate::GameState, PLAYER_HEALTH
};

pub struct UIPlugin;

#[derive(Component)]
pub struct Bar;
#[derive(Component)]
pub struct BufferBar;
#[derive(Component)]
pub struct UI;
#[derive(Component)]
pub struct Hurtui;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<PlayerHurtEvent>()
        .add_systems(OnEnter(GameState::Home), setup_ui_all)
        .add_systems(Update, (
            hurtui,
            update_ui,
            handle_state_bar,
        ))
        .add_systems(Update, log_transitions::<GameState>);
    }
}
//ui相对于摄像头的偏移量
const UI_OFFSET: Vec3 = Vec3::new(-590.0, 240.0, 0.0);
static mut buffer_offset:f32 = 0.0;
static mut bar_offset:f32 = 0.0;

fn setup_ui_all (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    loc_query: Query<&Transform, With<Camera2d>>,
) {
    if loc_query.is_empty() {
        return;
    }
    let loc = loc_query.single().translation.truncate();
    //将UI设置的坐标生成为相对相机坐标
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
        UI,
    ))
    .with_child((
        Text2d::new(format!("{}/{}",PLAYER_HEALTH,PLAYER_HEALTH)),
        TextFont {
            font: asset_server.load("Fonts/FIXEDSYS-EXCELSIOR-301.ttf"),
            font_size: 35.0,
            ..default()
        },  
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
        // UI,
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
        // UI,
    ));
}

fn update_ui (
    loc_query: Query<&Transform, (With<Camera2d>, Without<UI>, Without<Hurtui>)>,
    mut ui_query: Query<&mut Transform, (With<UI>, Without<Camera2d>, Without<Hurtui>)>,
    mut hurtui_query: Query<&mut Transform, (With<Hurtui>, Without<Camera2d>, Without<UI>)>,
) {
    if loc_query.is_empty() || ui_query.is_empty() {
        return;
    }
    let loc = loc_query.single().translation.truncate();
    for mut trans in ui_query.iter_mut() {
        trans.translation = Vec3::new(loc.x ,loc.y ,trans.translation.z) + UI_OFFSET;
    }
    for mut trans in hurtui_query.iter_mut() {
        trans.translation = Vec3::new(loc.x ,loc.y ,111.0);
    }
}
fn handle_state_bar(
    mut commands: Commands,
    loc_query: Query<&Transform, (With<Camera2d>, Without<Bar>, Without<Character>, Without<BufferBar>)>,
    health_query: Query<&mut Health, (With<Character>, Without<Bar>, Without<BufferBar>, Without<Camera2d>)>,
    mut buffer_query: Query<&mut Transform, (With<BufferBar>, Without<Character>, Without<Bar>, Without<Camera2d>)>,
    mut bar_query: Query<&mut Transform, (With<Bar>, Without<BufferBar>, Without<Character>, Without<Camera2d>)>,
    mut text_query: Query<&mut Text2d>,//后续可能文本框不止这一个，需要加限制过滤
    query2: Query<Entity, (With<Hurtui>, Without<Camera2d>)>,

) {
    if health_query.is_empty() ||buffer_query.is_empty() || bar_query.is_empty() || loc_query.is_empty(){
        return;
    }
    


    let health =health_query.single();

    if !text_query.is_empty() {
        for mut text in text_query.iter_mut() {
            text.0 = format!("{}/{}",health.0,PLAYER_HEALTH);
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
        buffer.scale.x -= 0.003;

        let temp = 0.003 * 0.5 * barwidth;
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

fn hurtui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, (With<Camera2d>, Without<Hurtui>)>,
    query2: Query<Entity, (With<Hurtui>, Without<Camera2d>)>,
    mut event: EventReader<PlayerHurtEvent>, //后续角色受伤时发出事件，ui响应
    //test
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if query.is_empty() {
        return;
    }
    for _ in event.read() {
        let trans = query.single().translation.truncate();
        commands.spawn((
            Sprite {
                image: asset_server.load("UI_Hit.png"),
                ..Default::default()
            },
            Transform::from_scale(Vec3::new(1.9, 1.4, 1.9))
                .with_translation(Vec3::new(trans.x, trans.y, 111.0)),
            Hurtui,
        ));     
    }
    //模拟受伤
    // if keyboard_input.pressed(KeyCode::KeyH) {
    //     let trans = query.single().translation.truncate();
    //     commands.spawn((
    //         Sprite {
    //             image: asset_server.load("UI_Hit.png"),
    //             ..Default::default()
    //         },
    //         Transform::from_scale(Vec3::new(1.9, 1.4, 1.9))
    //             .with_translation(Vec3::new(trans.x, trans.y, 111.0)),
    //         Hurtui,
    //     ));
    // }
    //测试消失
    // if keyboard_input.pressed(KeyCode::KeyJ) {
    //     for entity in query2.iter() {
    //         commands.entity(entity).despawn();
    //     }
    // }
    
}