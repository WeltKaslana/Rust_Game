use bevy::{dev_tools::states::*, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, state::{self, commands}, transform};

use crate::{gamestate::GameState,
            character::Character,};

pub struct GuiPlugin;
#[derive(Component)]
pub struct Transition;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
        .add_systems(
            Update,
            handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(Update, (animation1::<LeftSlide1>, animation1::<LeftSlide2>, animation2::<RightSlide1>, animation2::<RightSlide2>).run_if(in_state(GameState::MainMenu)))
        .add_systems(Update, statetransition.run_if(in_state(GameState::MainMenu)))
        .add_systems(Update, statetransition.run_if(in_state(GameState::Home)))
        .add_systems(Update, log_transitions::<GameState>);
    }
}

fn animation1<S:Component>(
    mut query: Query<&mut Transform, With<S>>,
) {
    let mut transform = query.single_mut();
    transform.translation.y += 4.0;
    if transform.translation.y > 930.0 {
        transform.translation.y = -935.0;
    }
}
fn animation2<S:Component>(
    mut query: Query<&mut Transform, With<S>>,
) {
    let mut transform = query.single_mut();
    transform.translation.y -= 4.0;
    if transform.translation.y < -930.0 {
        transform.translation.y = 935.0;
    }
}
#[derive(Component)]
struct LeftSlide1;
#[derive(Component)]
struct LeftSlide2;

#[derive(Component)]
struct RightSlide1;
#[derive(Component)]
struct RightSlide2;
fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,) {
    
    commands.spawn((
        Sprite {
            image: asset_server.load("Menu1.png"),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(0.9)).with_translation(Vec3::new(0.0, 0.0, 25.0)),
    ));//背景图
    

    commands.spawn((
        Sprite {
            image: asset_server.load("Menu_Mask.png"),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(0.9)).with_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));//背景图2

    commands.spawn((
        Sprite {
            image: asset_server.load("Menu_Slide.png"),
            ..Default::default()
        },
        // Transform::from_scale(Vec3::splat(4.0)).with_translation(Vec3::new(-400.0, 0.0, 1.0)),
        Transform::from_scale(Vec3::new(4.0,5.0,0.0)).with_translation(Vec3::new(-400.0, 0.0, 1.0)),
        LeftSlide1,
    ));//左条条1
    commands.spawn((
        Sprite {
            image: asset_server.load("Menu_Slide.png"),
            ..Default::default()
        },
        // Transform::from_scale(Vec3::splat(4.0)).with_translation(Vec3::new(-400.0, -500.0, 1.0)),
        Transform::from_scale(Vec3::new(4.0,5.0,0.0)).with_translation(Vec3::new(-400.0, -625.0, 1.0)),
        LeftSlide2,
    ));//左条条2
    commands.spawn((
        Sprite {
            image: asset_server.load("Menu_Slide.png"),
            ..Default::default()
        },
        Transform::from_scale(Vec3::new(4.0,5.0,0.0)).with_translation(Vec3::new(400.0, 0.0, 1.0)),
        RightSlide1,
    ));//右条条1
    commands.spawn((
        Sprite {
            image: asset_server.load("Menu_Slide.png"),
            ..Default::default()
        },
        Transform::from_scale(Vec3::new(4.0,5.0,0.0)).with_translation(Vec3::new(400.0, 625.0, 1.0)),
        RightSlide2,
    ));//右条条2
    commands.spawn((
        Sprite {
            image: asset_server.load("Logo.png"),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(0.5)).with_translation(Vec3::new(0.0, 100.0, 1.0)),
    ));//标题
}


fn handle_main_menu_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    transition_query: Query<Entity, With<Transition>>,
) {
    if  keyboard_input.pressed(KeyCode::KeyK) && transition_query.is_empty() {
        println!("Working!");
        commands.spawn((
            Sprite {
                image: asset_server.load("Menu_Transition1.png"),
                ..Default::default()
            },
            Transform::from_scale(Vec3::new(0.7,0.7,0.5)).with_translation(Vec3::new(-3200.0, 0.0, 100.0)),
            Transition,
        )); 
    }
}

fn statetransition(
    mut commands: Commands, 
    mut transition_query: Query<(&mut Transform, Entity), With<Transition>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if transition_query.is_empty() {
        return;
    }
    let (mut transform, e) = transition_query.single_mut();
    transform.translation.x += 20.0;
    if transform.translation.x == 400.0 {
        transform.translation.y -= 100.0;
        next_state.set(GameState::Home);
    }
    if transform.translation.x > 2400.0 {
        commands.entity(e).despawn_recursive();
    }
}

fn despawn_main_menu(
    mut commands: Commands, 
    mut menu_items_query: Query<Entity, (With<Sprite>, Without<Transition>)>) {
    for parent in &mut menu_items_query {
        commands.entity(parent).despawn_recursive();
    }
}

