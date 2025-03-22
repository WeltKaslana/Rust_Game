use bevy::{dev_tools::states::*, prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};

use crate::gamestate::GameState;
// use crate::world::GameEntity;

pub struct GuiPlugin;

#[derive(Component)]
struct MainMenuItem;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(Update, (animation1::<LeftSlide>, animation2::<RightSlide>))
            .add_systems(Update, log_transitions::<GameState>);
    }
}

fn animation1<S:Component>(
    mut query: Query<&mut Transform, With<S>>,
) {
    let mut transform = query.single_mut();
    transform.translation.y += 0.1;
    if transform.translation.y > 800.0 {
        transform.translation.y = 50.0;
    }
}
fn animation2<S:Component>(
    mut query: Query<&mut Transform, With<S>>,
) {
    let mut transform = query.single_mut();
    transform.translation.y -= 0.1;
    if transform.translation.y < -800.0 {
        transform.translation.y = 50.0;
    }
}
#[derive(Component)]
struct LeftSlide;

#[derive(Component)]
struct RightSlide;
fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,) {
    
    commands.spawn((
        Sprite {
            image: asset_server.load("Menu1.png"),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(0.9)).with_translation(Vec3::new(0.0, 0.0, 25.0)),
    )).insert(MainMenuItem);//背景图
    commands.spawn((
        Sprite {
            image: asset_server.load("Menu_Mask.png"),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(0.9)).with_translation(Vec3::new(0.0, 0.0, 0.0)),
    )).insert(MainMenuItem);//背景图2
    commands.spawn((
        Sprite {
            image: asset_server.load("Menu_Slide.png"),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(4.0)).with_translation(Vec3::new(-400.0, 50.0, 1.0)),
        LeftSlide,
    )).insert(MainMenuItem);//左条条
    commands.spawn((
        Sprite {
            image: asset_server.load("Menu_Slide.png"),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(4.0)).with_translation(Vec3::new(400.0, 50.0, 1.0)),
        RightSlide,
    )).insert(MainMenuItem);//右条条
    commands.spawn((
        Sprite {
            image: asset_server.load("Logo.png"),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(0.5)).with_translation(Vec3::new(0.0, 100.0, 1.0)),
    )).insert(MainMenuItem);//标题
}

fn handle_main_menu_buttons(
    // interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if  keyboard_input.pressed(KeyCode::KeyK) {
        println!("Working!");
        next_state.set(GameState::Home);
    }
}

fn despawn_main_menu(mut commands: Commands, menu_items_query: Query<Entity, With<MainMenuItem>>) {
    // for e in menu_items_query.iter() {
    //     commands.entity(e).despawn_recursive();
    // }
}
