use bevy::{
    dev_tools::states::*, 
    prelude::*, 
    color::palettes::css::{BLUE, GREEN, RED},};

use crate::{
    gamestate::GameState,
    character::{
        Character,
        Health,
    },
};

pub struct UIPlugin;

#[derive(Component)]
pub struct Bar;
#[derive(Component)]
pub struct UI;
#[derive(Component)]
pub struct Hurtui;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Home), setup_ui_all)
        .add_systems(Update, (
            hurtui,
            update_ui,))
        .add_systems(Update, log_transitions::<GameState>);
    }
}
//ui相对于摄像头的偏移量
const UI_OFFSET: Vec3 = Vec3::new(-590.0, -240.0, 0.0);

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
    commands.spawn((
        Sprite {
            image: asset_server.load("UI_Hub_PlayerHealth_Bar.png"),
            ..Default::default()
        },
        // Transform::from_scale(Vec3::splat(0.5))
        //     .with_translation(Vec3::new(-630.0, -350.0, 110.0)),
        Transform::from_scale(Vec3::splat(0.5))
        .with_translation(Vec3::new(loc.x ,loc.y ,110.0) + UI_OFFSET),
        UI,
    ));
    commands.spawn((
        Sprite {
            image:asset_server.load("UI_Hub_PlayerHealth.png"),
            ..Default::default()
        },
        // Transform::from_scale(Vec3::splat(0.5))
        // .with_translation(Vec3::new(-630.0, -350.0, 109.0)),
        Transform::from_scale(Vec3::splat(0.5))
        .with_translation(Vec3::new(loc.x, loc.y, 110.0) + UI_OFFSET),
        Bar,
        UI,
    ));
}

fn update_ui (
    // health_query: Query<&Health, With<Character>>,
    loc_query: Query<&Transform, (With<Camera2d>, Without<UI>)>,
    mut ui_query: Query<&mut Transform, (With<UI>, Without<Camera2d>)>,
) {
    if loc_query.is_empty() || ui_query.is_empty() {
        return;
    }
    let loc = loc_query.single().translation.truncate();
    for mut trans in ui_query.iter_mut() {
        trans.translation = Vec3::new(loc.x ,loc.y ,110.0) + UI_OFFSET;
    }
    //to do: 根据角色血量更新血条

}

fn hurtui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, (With<Camera2d>, Without<Hurtui>)>,
    mut query2: Query<Entity, (With<Hurtui>, Without<Camera2d>)>,
    // event: EventReader<PlayerHurtEvent>, //后续角色受伤时发出事件，ui响应
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if query.is_empty() {
        return;
    }
    
    //模拟受伤
    if keyboard_input.pressed(KeyCode::KeyH) {
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
    //测试消失
    if keyboard_input.pressed(KeyCode::KeyJ) {
        for entity in query2.iter() {
            commands.entity(entity).despawn();
        }
    }
    
}