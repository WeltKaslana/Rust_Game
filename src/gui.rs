use bevy::{asset, ecs::query, prelude::*, render::camera, text::cosmic_text::ttf_parser::Style, utils::info};
use encoding_rs::{GBK, UTF_8};
use rand::Rng;
use std::collections::HashSet;

use crate::{
    character::{
        Buff, Character, Health, Player, ReloadPlayerEvent
    }, gamestate::*, gun::{Cursor, Gun}, home::Home, resources::*, room::{AssetsManager, Chest, ChestType, Map}, ui::UI, configs::*,
};

pub struct GuiPlugin;
#[derive(Component)]
pub struct Transition;

#[derive(Component)]
pub enum PauseMenu {
    Body,
    BorderUp,
    BorderDown,
}

#[derive(Component)]
pub struct SoraMenu;

#[derive(Component)]
pub struct CharacterSelectButton;

#[derive(Component)]
pub struct ChoosingBuffMenu;

#[derive(Component)]
pub struct GameOverMenu;
// #[derive(Component)]
// pub enum ButtonType {
//     Close,
//     ReturntoMainMenu,
//     ReturntoHome,
//     Settings,
//     Quit,
// }

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::MainMenu), (
            clear_all.before(setup_main_menu),
            setup_main_menu
        ))
        .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
        .add_systems(
            Update,
            handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update, 
            handle_home_menu.run_if(in_state(GameState::Home)))
        .add_systems(
            Update, 
            handle_ingame_menu.run_if(in_state(GameState::InGame)))

        // .enable_state_scoped_entities::<HomeState>()//test

        .add_systems(OnEnter(HomeState::Pause), setup_stopmenu)
        .add_systems(Update, handle_stopmenu1.run_if(in_state(HomeState::Pause)))
        .add_systems(OnExit(HomeState::Pause), cleanup_stopmenu)

        .add_systems(OnEnter(HomeState::Reloading), setup_soramenu)
        .add_systems(Update, (
            handle_soramenu.run_if(in_state(HomeState::Reloading)),
            reload_button_change.run_if(in_state(HomeState::Reloading)),
            handle_player_messages.run_if(in_state(HomeState::Reloading)),
        ))
        .add_systems(OnExit(HomeState::Reloading), cleanup_soramenu)

        .add_systems(OnEnter(InGameState::Pause), setup_stopmenu)
        .add_systems(Update, handle_stopmenu2.run_if(in_state(InGameState::Pause)))
        .add_systems(OnExit(InGameState::Pause), cleanup_stopmenu)

        .add_systems(OnEnter(InGameState::ChoosingBuff), setup_choosingbuffmenu)
        .add_systems(Update, handle_choosingbuffmenu.run_if(in_state(InGameState::ChoosingBuff)))
        .add_systems(OnExit(InGameState::ChoosingBuff), cleanup_choosingbuffmenu)

        .add_systems(OnEnter(InGameState::GameOver), setup_gameovermenu)
        .add_systems(Update, handle_gameovermenu.run_if(in_state(InGameState::GameOver)))
        .add_systems(OnExit(InGameState::GameOver), cleanup_gameovermenu)

        .add_systems(Update, (animation1::<LeftSlide1>, animation1::<LeftSlide2>, animation2::<RightSlide1>, animation2::<RightSlide2>).run_if(in_state(GameState::MainMenu)))
        .add_systems(Update, statetransition)

        //test
        .add_systems(Update, print_node_loc)
        //

        ;
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


fn clear_all(
    mut commands: Commands,
    query1: Query<Entity, (With<Player>, (Without<UI>, Without<Map>))>,
    query2: Query<Entity, (With<UI>, (Without<Player>, Without<Map>))>,
    query3: Query<Entity, (With<Map>, (Without<UI>, Without<Player>))>,
) {
    for entity in query1.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in query2.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in query3.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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

    commands.spawn((
        Sprite {..Default::default()},
        Text2d::new("Press any key to start!"),
        TextFont {
            font: asset_server.load("Fonts/FIXEDSYS-EXCELSIOR-301.ttf"),
            font_size: 60.0,
            ..default()
        },  
        TextColor(Color::rgb(255.0, 0.0, 255.0)),
        Transform::from_translation(Vec3::new(0.0, -200.0, 99.0)),
    ));

}


fn handle_main_menu_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    transition_query: Query<Entity, (With<Transition>, Without<Camera2d>)>,
    mut camera_query: Query<(&Transform, &mut GameState), (With<Camera2d>, Without<Transition>)>,
) {

    if camera_query.is_empty() {
        return;
    }

    if  (keyboard_input.get_just_pressed().count() > 0) && transition_query.is_empty() {
        // println!("Working!");

        let (trans_transform, mut nextstate) = camera_query.single_mut();
        let trans = trans_transform.translation;

        *nextstate = GameState::Home;
        commands.spawn((
            Sprite {
                image: asset_server.load("Menu_Transition1.png"),
                ..Default::default()
            },
            // Transform::from_scale(Vec3::new(0.7,0.7,0.5)).with_translation(Vec3::new(-3200.0, 0.0, 100.0)),
            Transform::from_scale(Vec3::new(0.7,0.7,0.5))
                .with_translation(Vec3::new(trans.x-3200.0, trans.y, 100.0)),
            Transition,
            Home,
        )); 
    }
}



// 状态切换和切换动画控制
fn statetransition(
    mut commands: Commands, 
    mut transition_query: Query<(&mut Transform, Entity), (With<Transition>, Without<Camera2d>)>,
    mut camera_query: Query<(&Transform, &mut GameState), (With<Camera2d>, Without<Transition>)>,
    clear_query: Query<Entity, (With<Player>, Without<Gun>, Without<Character>, Without<Cursor>, Without<Camera2d>)>,
    state: Res<State<GameState>>,
    mut mgr: ResMut<AssetsManager>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if transition_query.is_empty() || camera_query.is_empty() {
        return;
    }
    let (mut transform, e) = transition_query.single_mut();
    let (camera_transform, mut nextstate) = camera_query.single_mut();
    let trans = camera_transform.translation;
    let x = trans.x;
    transform.translation.x += 20.0;
    transform.translation.y = trans.y;
    let delta = transform.translation.x - x;

    // println!("delta: {}", delta);
    // 根据nextstate来判断下一个要切换到的状态，同时将nextstate初始化为MainMenu
    match *state.get() {
        GameState::MainMenu if delta >= 400.0 && delta < 420.0 => {
            match *nextstate {
                GameState::Home => {
                    next_state.set(GameState::Home);
                    // info!("transition to game!");
                },
                _ => {},
            }
        },
        GameState::Home if delta >= 400.0 && delta < 420.0 => {
            println!("Home transition to {:?}!", *nextstate);
            match *nextstate {
                GameState::MainMenu => {
                    *nextstate = GameState::None;
                    next_state.set(GameState::MainMenu);
                },
                GameState::Loading => {
                    *nextstate = GameState::None;
                    mgr.cycle_map(&mut commands);
                    for (e) in clear_query.iter() {
                        commands.entity(e).despawn_recursive();
                    }
                    next_state.set(GameState::Loading);
                },
                GameState::None  => {},
                _ => {
                    info!("Wrong state trasition!");
                    *nextstate = GameState::None;
                    next_state.set(GameState::MainMenu);
                }
            }
        },
        GameState::Loading if delta >= 800.0 => {
            next_state.set(GameState::InGame);
        },

        GameState::InGame if delta >= 400.0 && delta < 420.0 => {
            println!("InGame transition to {:?}!", *nextstate);
            match *nextstate {
                GameState::MainMenu => {
                    *nextstate = GameState::None;
                    mgr.del_map(&mut commands);
                    next_state.set(GameState::MainMenu);
                },
                GameState::Loading => {
                    *nextstate = GameState::None;
                    mgr.cycle_map(&mut commands);
                    for (e) in clear_query.iter() {
                        commands.entity(e).despawn_recursive();
                    }
                    next_state.set(GameState::Loading);
                },
                GameState::Home => {
                    mgr.del_map(&mut commands);
                    *nextstate = GameState::None;
                    next_state.set(GameState::Home);
                },
                GameState::None  => {},
                _ => {
                    info!("Wrong state trasition!");
                    *nextstate = GameState::None;
                    next_state.set(GameState::MainMenu);
                }
            }
        },
        _ => {}
    }

    if delta > 2400.0 {
        commands.entity(e).despawn_recursive();
        // info!("translation deleted!");
    }
}
fn despawn_main_menu(
    mut commands: Commands, 
    mut menu_items_query: Query<Entity, (With<Sprite>, Without<Transition>)>) {
    for parent in &mut menu_items_query {
        commands.entity(parent).despawn_recursive();
    }
}


fn handle_home_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<HomeState>>,
    mut next_state: ResMut<NextState<HomeState>>,
    mut windows: Query<&mut Window>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = windows.get_single_mut() {
        next_state.set(match *state.get() {
            HomeState::Running => {
                window.cursor_options.visible = true;
                HomeState::Pause
            },
            HomeState::Pause => {
                window.cursor_options.visible = false;
                HomeState::Running
            },
            HomeState::Reloading => {
                window.cursor_options.visible = false;
                HomeState::Running
            },
        });
        }


    }
}

fn handle_ingame_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<InGameState>>,
    mut windows: Query<&mut Window>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = windows.get_single_mut() {
            next_state.set(match *state.get() {
                InGameState::Running => {
                    window.cursor_options.visible = true;
                    InGameState::Pause
                },
                InGameState::Pause => {
                    window.cursor_options.visible = false;
                    InGameState::Running
                },
                InGameState::ChoosingBuff => {InGameState::ChoosingBuff},
                InGameState::GameOver=>{InGameState::GameOver},
            }); 
        }
    }
}

// fn utf8_to_gb2312(input: &str) -> String {
//     // 将 UTF-8 字符串编码为 GBK（兼容 GB2312）
//     let (encoded, _, had_errors) = UTF_8.encode(input);
    
//     if had_errors {
//         eprintln!("警告：转换过程中出现无法映射的字符");
//     }
//     let (utf8_decoded, _, _) = GBK.decode(&encoded);
//     utf8_decoded.into_owned()
// }

fn setup_stopmenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, (With<Camera2d>)>,
    source: Res<GlobalMenuTextureAtlas>,
) {
    if camera_query.is_empty() {
        return;
    }
    let loc = camera_query.single().translation.truncate();
    commands.spawn((
        Sprite {
            image: source.border.clone(),
            // image: asset_server.load("BookMenu_PauseBorderSmall.png"),
            flip_x: true,
            flip_y: true,
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(0.8)).with_translation(Vec3::new(loc.x - 530.0, loc.y + 510.0, 100.0)),
            PauseMenu::BorderUp,
    ));
    commands.spawn((
        Sprite {
            image: source.border.clone(),
            // image: asset_server.load("BookMenu_PauseBorderSmall.png"),
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(0.8)).with_translation(Vec3::new(loc.x + 530.0, loc.y - 510.0, 100.0)),
            PauseMenu::BorderDown,
    )); 


    // let font: Handle<Font> = asset_server.load("Fonts/FIXEDSYS-EXCELSIOR-301.ttf");
    let font = asset_server.load("fonts/pixel_font.ttf");
    
    commands.spawn((
        ImageNode::new(source.list.clone()),
        Node {
            width: Val::Percent(30.0),
            height: Val::Percent(50.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(35.0),
            top: Val::Percent(25.0 +  40.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        PauseMenu::Body,))
    .with_children(|parent| {
            parent.spawn((
                Text::new("选项菜单"),
                // Text::new(utf8_to_gb2312("选项菜单")),
                TextFont {
                    font: font.clone(),
                    font_size: 48.0,
                    ..default()
                },  
                TextColor(Color::rgb(123.0, 0.0, 131.0)),
                Node {
                    top: Val::Percent(0.0),
                    left: Val::Percent(40.0),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                }, 
            ));
            parent.spawn((
                Name::new("back to game"),
                ImageNode::new(source.button.clone()),
                // ImageNode::new(image_button.clone()),
                Node {
                    width: Val::Percent(60.0),
                    height: Val::Percent(10.0),
                    top: Val::Percent(25.0),
                    left: Val::Percent(20.0),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                Button,
            ))
            .with_child((
                Text::new("继续"),
                TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                },  
                TextColor(Color::rgb(0.0, 0.0, 0.0)),
                Node {
                    align_items: AlignItems::Center,
                    left: Val::Percent(15.0),
                    ..default()
                },   
            ));
            parent.spawn((
                Name::new("back to menu"),
                ImageNode::new(source.button.clone()),
                Node {
                    width: Val::Percent(60.0),
                    height: Val::Percent(10.0),
                    top: Val::Percent(42.0),
                    left: Val::Percent(20.0),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                Button,
            ))
            .with_child((
                Text::new("返回封面"),
                TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                },  
                TextColor(Color::rgb(0.0, 0.0, 0.0)), 
                Node {
                    align_items: AlignItems::Center,
                    left: Val::Percent(15.0),
                    ..default()
                },   
            ));
            parent.spawn((
                Name::new("back to home"),
                ImageNode::new(source.button.clone()),
                Node {
                    width: Val::Percent(60.0),
                    height: Val::Percent(10.0),
                    top: Val::Percent(60.3),
                    left: Val::Percent(20.0),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                Button,
            ))
            .with_child((
                Text::new("返回大厅"),
                TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                },  
                TextColor(Color::rgb(0.0, 0.0, 0.0)), 
                Node {
                    align_items: AlignItems::Center,
                    left: Val::Percent(15.0),
                    ..default()
                },   
            ));
            parent.spawn((
                Name::new("exit"),
                ImageNode::new(source.button.clone()),
                Node {
                    width: Val::Percent(60.0),
                    height: Val::Percent(10.0),
                    top: Val::Percent(79.0),
                    left: Val::Percent(20.0),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                Button,
                test,
            ))
            .with_child((
                Text::new("退出游戏"),
                TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                },  
                TextColor(Color::rgb(0.0, 0.0, 0.0)), 
                Node {
                    align_items: AlignItems::Center,
                    left: Val::Percent(15.0),
                    ..default()
                },   
            ));
        });
}

fn handle_stopmenu1 (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<(&Transform, &mut GameState), (With<Camera2d>, Without<PauseMenu>)>,
    mut menu_query: Query<(&mut Transform, &PauseMenu,), (Without<Camera2d>, Without<Node>)>,
    mut interaction_query: Query<(
            &mut ImageNode,
            &Interaction,
            &Name,),
        (Changed<Interaction>, With<Button>),
    >,
    mut windows: Query<&mut Window>,
    mut query: Query<&mut Node, (With<PauseMenu>, Without<Camera2d>)>,
    source: Res<GlobalMenuTextureAtlas>,
    mut next_state: ResMut<NextState<HomeState>>,
    mut app_exit_events: EventWriter<AppExit>,
    // mut next_state2: ResMut<NextState<GameState>>,
) {
    if camera_query.is_empty() || menu_query.is_empty() {
        return;
    }
    let (camera_trans, mut nextstate) = camera_query.single_mut();
    let loc = camera_trans.translation.truncate();
    for (mut trans, obj) in menu_query.iter_mut() {
        match *obj {
            PauseMenu::BorderUp => {
                if trans.translation.y > loc.y + 250.0 {
                    trans.translation.y -= 10.0;
                }
            },
            PauseMenu::BorderDown => {
                if trans.translation.y < loc.y - 250.0 {
                    trans.translation.y += 10.0;
                }
            },
            _ => {},
        }
    }
    for mut node in query.iter_mut() {
        match node.top {
            Val::Percent(v) => {
                if v > 25.0 {
                    node.top = Val::Percent(v - 2.0);
                } else{
                    for (mut image, interaction, name) in &mut interaction_query {
                        info!("interaction: ");
                        match *interaction {
                            Interaction::Pressed => {
                                println!("{}Clicked!", name);
                                match name.as_str() {
                                    "back to game" | "back to home" => {
                                        if let Ok(mut window) = windows.get_single_mut() {
                                            window.cursor_options.visible = false;
                                        }
                                        next_state.set(HomeState::Running);
                                    },
                                    "back to menu" => {
                                        if let Ok(mut window) = windows.get_single_mut() {
                                            window.cursor_options.visible = false;
                                        }
                                        next_state.set(HomeState::Running);
                                        *nextstate = GameState::MainMenu;
                                        commands.spawn((
                                            Sprite {
                                                image: asset_server.load("Menu_Transition1.png"),
                                                ..Default::default()
                                            },
                                            Transform::from_scale(Vec3::new(0.7,0.7,0.5))
                                                .with_translation(Vec3::new(loc.x-3200.0, loc.y, 100.0)),
                                            Transition,
                                        ));  
                                        // next_state2.set(GameState::MainMenu);
                                    },
                                    "exit" => {
                                        app_exit_events.send(AppExit::Success);
                                    },
                                    _ => {}
                                }
                            },
                            Interaction::Hovered => {
                                println!("Hovered!");
                                image.image = source.button_hover.clone();
                            },
                            Interaction::None => {
                                image.image = source.button.clone();
                            },
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

fn handle_stopmenu2 (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<(&Transform, &mut GameState), (With<Camera2d>, Without<PauseMenu>)>,
    mut menu_query: Query<(&mut Transform, &PauseMenu,), (Without<Camera2d>, Without<Node>)>,
    mut interaction_query: Query<(
            &mut ImageNode,
            &Interaction,
            &Name,),
        (Changed<Interaction>, With<Button>),
    >,
    mut windows: Query<&mut Window>,
    mut query: Query<&mut Node, (With<PauseMenu>, Without<Camera2d>)>,
    source: Res<GlobalMenuTextureAtlas>,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if camera_query.is_empty() || menu_query.is_empty() {
        return;
    }
    let (camera_trans, mut nextstate) = camera_query.single_mut();
    let loc = camera_trans.translation.truncate();
    for (mut trans, obj) in menu_query.iter_mut() {
        match *obj {
            PauseMenu::BorderUp => {
                if trans.translation.y > loc.y + 250.0 {
                    trans.translation.y -= 10.0;
                }
            },
            PauseMenu::BorderDown => {
                if trans.translation.y < loc.y - 250.0 {
                    trans.translation.y += 10.0;
                }
            },
            _ => {},
        }
    }
    for mut node in query.iter_mut() {
        match node.top {
            Val::Percent(v) => {
                if v > 25.0 {
                    node.top = Val::Percent(v - 2.0);
                } else{
                    for (mut image, interaction, name) in &mut interaction_query {
                        info!("interaction: ");
                        match *interaction {
                            Interaction::Pressed => {
                                println!("{}Clicked!", name);
                                match name.as_str() {
                                    "back to game" => {
                                        if let Ok(mut window) = windows.get_single_mut() {
                                            window.cursor_options.visible = false;
                                        }
                                        next_state.set(InGameState::Running);
                                    },
                                    "back to menu" => {
                                        if let Ok(mut window) = windows.get_single_mut() {
                                            window.cursor_options.visible = false;
                                        }
                                        next_state.set(InGameState::Running);
                                        *nextstate = GameState::MainMenu;
                                        commands.spawn((
                                            Sprite {
                                                image: asset_server.load("Menu_Transition1.png"),
                                                ..Default::default()
                                            },
                                            Transform::from_scale(Vec3::new(0.7,0.7,0.5))
                                                .with_translation(Vec3::new(loc.x-3200.0, loc.y, 100.0)),
                                            Transition,
                                        ));
                                        // 游戏内最好改成返回大厅
                                        // next_state2.set(GameState::MainMenu);
                                    },
                                    "back to home" => {
                                        if let Ok(mut window) = windows.get_single_mut() {
                                            window.cursor_options.visible = false;
                                        }
                                        next_state.set(InGameState::Running);
                                        *nextstate = GameState::Home;
                                        commands.spawn((
                                            Sprite {
                                                image: asset_server.load("Menu_Transition1.png"),
                                                ..Default::default()
                                            },
                                            Transform::from_scale(Vec3::new(0.7,0.7,0.5))
                                                .with_translation(Vec3::new(loc.x-3200.0, loc.y, 100.0)),
                                            Transition,
                                        ));
                                    },
                                    "exit" => {
                                        app_exit_events.send(AppExit::Success);
                                    },
                                    _ => {}
                                }
                            },
                            Interaction::Hovered => {
                                println!("Hovered!");
                                image.image = source.button_hover.clone();
                            },
                            Interaction::None => {
                                image.image = source.button.clone();
                            },
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

fn cleanup_stopmenu(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenu>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_soramenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    source: Res<GlobalMenuTextureAtlas>,
    source1: Res<GlobalCharacterTextureAtlas>,
) {
    let font: Handle<Font> = asset_server.load("Fonts/FIXEDSYS-EXCELSIOR-301.ttf");
    let font2: Handle<Font> = asset_server.load("fonts/pixel_font.ttf");
    commands.spawn((
        //底
        ImageNode::new(source.menu.clone()),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(130.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(0.0),
            top: Val::Percent(-10.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        SoraMenu,
    ))
    .with_children(|parent| {
            //顶
            parent.spawn((
                ImageNode::new(source.top.clone()),
                Node {
                    width: Val::Percent(70.0),
                    height: Val::Percent(59.0),
                    top: Val::Percent(20.7),
                    left: Val::Percent(15.0),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
            ))
            .with_children(|parent| {
                // 角色选择标题
                parent.spawn(( 
                    ImageNode::new(source.sub_title.clone()),
                    Node {
                        width: Val::Percent(48.2),
                        height: Val::Percent(18.5),
                        top: Val::Percent(5.9),
                        left: Val::Percent(50.3),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    }, 
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Shiroko_choose"),
                        // ImageNode::new(asset_server.load("UI_Hub_Portrait_Shiroko.png")),
                        ImageNode::new(
                            if source1.id == 1 {
                                source.shiroko.clone()
                            } else {
                                source.shiroko_unselect.clone()
                            }),
                        Node {
                            width: Val::Percent(20.5),
                            height: Val::Percent(87.2),
                            top: Val::Percent(9.4),
                            left: Val::Percent(9.2),
                            position_type: PositionType::Absolute,
                            ..Default::default()
                        },
                        CharacterSelectButton,
                        Button,
                    ));
                    parent.spawn((
                        Name::new("Arisu_choose"),
                        // ImageNode::new(asset_server.load("UI_Hub_Portrait_Arisu.png")),
                        ImageNode::new(
                            if source1.id == 2 {
                                source.arisu.clone()
                            } else {
                                source.arisu_unselect.clone()
                            }),
                        Node {
                            width: Val::Percent(20.5),
                            height: Val::Percent(87.2),
                            top: Val::Percent(7.7),
                            left: Val::Percent(44.4),
                            position_type: PositionType::Absolute,
                            ..Default::default()
                        },
                        CharacterSelectButton,
                        Button,
                    ));
                    parent.spawn((
                        Name::new("Utaha_choose"),
                        // ImageNode::new(asset_server.load("UI_Hub_Portrait_Utaha.png")),
                        ImageNode::new(
                            if source1.id == 3 {
                                source.utaha.clone()
                            } else {
                                source.utaha_unselect.clone()
                            }),
                        Node {
                            width: Val::Percent(20.5),
                            height: Val::Percent(87.2),
                            top: Val::Percent(7.7),
                            left: Val::Percent(77.0),
                            position_type: PositionType::Absolute,
                            ..Default::default()
                        },
                        CharacterSelectButton,
                        Button,
                    ));
                });
                // 角色简介底
                let image = match source1.id {
                    1 => {
                        [
                            source.shiroko_skill1.clone(),
                            source.shiroko_skill2.clone(),
                            source.shiroko_skill3.clone(),
                            source.shiroko_skill4.clone(),
                        ]
                    },
                    2 => {
                        [
                            source.arisu_skill1.clone(),
                            source.arisu_skill2.clone(),
                            source.arisu_skill3.clone(),
                            source.arisu_skill4.clone(),
                        ]
                    },
                    3 => {
                        [
                            source.utaha_skill1.clone(),
                            source.utaha_skill2.clone(),
                            source.utaha_skill3.clone(),
                            source.utaha_skill4.clone(),
                        ]
                    },
                    _ => {
                        println!("Invalid charcter id!");
                        [
                            source.shiroko_skill1.clone(),
                            source.shiroko_skill2.clone(),
                            source.shiroko_skill3.clone(),
                            source.shiroko_skill4.clone(),
                        ]
                    }
                };
                let skill_text = match source1.id {
                    1 => {
                        source.shiroko_skill_text.clone()
                    },
                    2 => {
                        source.arisu_skill_text.clone()
                    },
                    3  => { 
                        source.utaha_skill_text.clone()
                    },
                    _ => {
                        println!("Invalid charcter id!");
                        source.shiroko_skill_text.clone()
                    }
                };
                parent.spawn(( 
                    ImageNode::new(source.tips.clone()),
                    Node {
                        width: Val::Percent(46.7),
                        height: Val::Percent(72.0),
                        top: Val::Percent(25.1),
                        left: Val::Percent(51.3),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    }, 
                )).with_children( |parent| {
                    parent.spawn((
                        Name::new("1"),
                        ImageNode::new(image[0].clone()),
                        Node {
                            width: Val::Percent(13.3),
                            height: Val::Percent(20.0),
                            top: Val::Percent(6.0),
                            left: Val::Percent(14.0),
                            ..Default::default()
                        },
                        CharacterSelectButton,
                    )).with_child((
                        Text::new(skill_text[0].to_string()),
                        TextFont {
                                font: font2.clone(),
                                font_size: 15.0,
                                ..default()
                        },  
                        TextColor(Color::rgb(0.0, 0.0, 0.0)),
                        TextLayout::new_with_no_wrap(), 
                        Node {
                            top: Val::Percent(7.6),
                            left: Val::Percent(108.1),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Name::new("2"),
                        ImageNode::new(image[1].clone()),
                        Node {
                            width: Val::Percent(13.3),
                            height: Val::Percent(20.0),
                            top: Val::Percent(28.6),
                            left: Val::Percent(-1.8),
                            ..Default::default()
                        },
                        CharacterSelectButton,
                    )).with_child((
                        Text::new(skill_text[1].to_string()),
                        TextFont {
                                font: font2.clone(),
                                font_size: 15.0,
                                ..default()
                        },  
                        TextColor(Color::rgb(0.0, 0.0, 0.0)),
                        TextLayout::new_with_no_wrap(), 
                        Node {
                            top: Val::Percent(7.6),
                            left: Val::Percent(108.1),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Name::new("3"),
                        ImageNode::new(image[2].clone()),
                        Node {
                            width: Val::Percent(13.3),
                            height: Val::Percent(20.0),
                            top: Val::Percent(51.9),
                            left: Val::Percent(-17.4),
                            ..Default::default()
                        },
                        CharacterSelectButton,
                    )).with_child((
                        Text::new(skill_text[2].to_string()),
                        TextFont {
                                font: font2.clone(),
                                font_size: 15.0,
                                ..default()
                        },  
                        TextColor(Color::rgb(0.0, 0.0, 0.0)), 
                        TextLayout::new_with_no_wrap(),
                        Node {
                            top: Val::Percent(7.6),
                            left: Val::Percent(108.1),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Name::new("4"),
                        ImageNode::new(image[3].clone()),
                        Node {
                            width: Val::Percent(13.3),
                            height: Val::Percent(20.0),
                            top: Val::Percent(74.3),
                            left: Val::Percent(-33.2),
                            // max_width: Val::Percent(40.0),
                            ..Default::default()
                        },
                        CharacterSelectButton,
                    )).with_child((
                        Text::new(skill_text[3].to_string()),
                        TextFont {
                                font: font2.clone(),
                                font_size: 15.0,
                                ..default()
                        },  
                        TextColor(Color::rgb(0.0, 0.0, 0.0)),
                        TextLayout::new_with_no_wrap(),
                        Node {
                            top: Val::Percent(7.6),
                            left: Val::Percent(108.1),
                            ..default()
                        },
                        // test,
                    ));
                });
                // 关闭
                parent.spawn(( 
                    Name::new("close"),
                    ImageNode::new(source.close.clone()),
                    Node {
                        width: Val::Percent(5.6),
                        height: Val::Percent(9.0),
                        top: Val::Percent(6.7),
                        left: Val::Percent(99.8),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    }, 
                    Button,
                ));
                // 别针
                parent.spawn(( 
                    ImageNode::new(source.title.clone()),
                    Node {
                        width: Val::Percent(23.3),
                        height: Val::Percent(16.5),
                        top: Val::Percent(-4.2),
                        left: Val::Percent(13.5),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    }, 
                ))                
                .with_child((
                    Text::new("setting"),
                    TextFont {
                        // font: font.clone(),
                        font_size: 35.0,
                        ..default()
                    },
                    TextColor(Color::rgb(1.0, 1.0, 1.0)),
                    Node {
                        top: Val::Percent(0.0),
                        left: Val::Percent(20.0),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                ));
                // 难度选择标题
                parent.spawn(( 
                    ImageNode::new(source.sub_title.clone()),
                    Node {
                        width: Val::Percent(37.1),
                        height: Val::Percent(11.4),
                        top: Val::Percent(22.7),
                        left: Val::Percent(6.4),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    }, 
                ))
                .with_child((
                    Text::new("Choose difficulty"),
                    TextFont {
                        // font: font.clone(),
                        font_size: 35.0,
                        ..default()
                    },
                    TextColor(Color::rgb(1.0, 1.0, 1.0)),
                    Node {
                        top: Val::Percent(0.0),
                        left: Val::Percent(20.0),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                ));
                // 按钮
                parent.spawn(( 
                    Name::new("easy"),
                    ImageNode::new(source.button.clone()),
                    Node {
                        width: Val::Percent(10.9),
                        height: Val::Percent(8.3),
                        top: Val::Percent(40.9),
                        left: Val::Percent(5.1),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                   }, 
                   Button,
                ))
                .with_child((
                    Text::new("easy"),
                    TextFont {
                        // font: font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::rgb(0.0, 10.0, 1.0)),
                    Node {
                        top: Val::Percent(0.0),
                        left: Val::Percent(20.0),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                ));
                parent.spawn(( 
                    Name::new("normal"),
                    ImageNode::new(source.button.clone()),
                    Node {
                        width: Val::Percent(10.9),
                        height: Val::Percent(8.3),
                        top: Val::Percent(40.9),
                        left: Val::Percent(20.7),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                   }, 
                   Button,
                ))
                .with_child((
                    Text::new("normal"),
                    TextFont {
                        // font: font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::rgb(0.0, 10.0, 1.0)),
                    Node {
                        top: Val::Percent(0.0),
                        left: Val::Percent(20.0),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                ));
                parent.spawn(( 
                    Name::new("hard"),
                    ImageNode::new(source.button.clone()),
                    Node {
                        width: Val::Percent(10.9),
                        height: Val::Percent(8.3),
                        top: Val::Percent(40.9),
                        left: Val::Percent(36.1),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                   }, 
                   Button,
                ))
                .with_child((
                    Text::new("hard"),
                    TextFont {
                        // font: font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::rgb(0.0, 10.0, 1.0)),
                    Node {
                        top: Val::Percent(0.0),
                        left: Val::Percent(20.0),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                ));
                // 挑战因子
                parent.spawn(( 
                    ImageNode::new(source.sub_title.clone()),
                    Node {
                        width: Val::Percent(37.1),
                        height: Val::Percent(11.4),
                        top: Val::Percent(58.5),
                        left: Val::Percent(6.4),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    }, 
                ))
                .with_child((
                    Text::new("difficulty factor"),
                    TextFont {
                        // font: font.clone(),
                        font_size: 35.0,
                        ..default()
                    },
                    TextColor(Color::rgb(1.0, 1.0, 1.0)),
                    Node {
                        top: Val::Percent(0.0),
                        left: Val::Percent(20.0),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                ));
                // parent.spawn(( 
                //     Name::new("easy"),
                //     ImageNode::new(source.button.clone()),
                //     Node {
                //         width: Val::Percent(10.9),
                //         height: Val::Percent(8.3),
                //         top: Val::Percent(40.9),
                //         left: Val::Percent(5.1),
                //         position_type: PositionType::Absolute,
                //         ..Default::default()
                //    }, 
                //    Button,
                // ))
                // .with_child((
                //     Text::new("easy"),
                //     TextFont {
                //         // font: font.clone(),
                //         font_size: 20.0,
                //         ..default()
                //     },
                //     TextColor(Color::rgb(0.0, 10.0, 1.0)),
                //     Node {
                //         top: Val::Percent(0.0),
                //         left: Val::Percent(20.0),
                //         position_type: PositionType::Absolute,
                //         ..Default::default()
                //     },
                // ));
                // parent.spawn(( 
                //     Name::new("normal"),
                //     ImageNode::new(source.button.clone()),
                //     Node {
                //         width: Val::Percent(10.9),
                //         height: Val::Percent(8.3),
                //         top: Val::Percent(40.9),
                //         left: Val::Percent(20.7),
                //         position_type: PositionType::Absolute,
                //         ..Default::default()
                //    }, 
                //    Button,
                // ))
                // .with_child((
                //     Text::new("normal"),
                //     TextFont {
                //         // font: font.clone(),
                //         font_size: 20.0,
                //         ..default()
                //     },
                //     TextColor(Color::rgb(0.0, 10.0, 1.0)),
                //     Node {
                //         top: Val::Percent(0.0),
                //         left: Val::Percent(20.0),
                //         position_type: PositionType::Absolute,
                //         ..Default::default()
                //     },
                // ));
                // parent.spawn(( 
                //     Name::new("hard"),
                //     ImageNode::new(source.button.clone()),
                //     Node {
                //         width: Val::Percent(10.9),
                //         height: Val::Percent(8.3),
                //         top: Val::Percent(40.9),
                //         left: Val::Percent(36.1),
                //         position_type: PositionType::Absolute,
                //         ..Default::default()
                //    }, 
                //    Button,
                // ))
                // .with_child((
                //     Text::new("hard"),
                //     TextFont {
                //         // font: font.clone(),
                //         font_size: 20.0,
                //         ..default()
                //     },
                //     TextColor(Color::rgb(0.0, 10.0, 1.0)),
                //     Node {
                //         top: Val::Percent(0.0),
                //         left: Val::Percent(20.0),
                //         position_type: PositionType::Absolute,
                //         ..Default::default()
                //     },
                // ));

                // 书签
                parent.spawn((
                    Name::new("bookmark_settings"),
                    ImageNode::new(source.bookmark.clone()),
                    Node {
                        width: Val::Percent(7.7),
                        height: Val::Percent(10.7),
                        top: Val::Percent(6.6),
                        left: Val::Percent(-7.6),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },  
                    Button,
                ))
                .with_child((
                    Text::new("setting"),
                    TextFont {
                        // font: font.clone(),
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(Color::rgb(10.0, 0.0, 7.0)),
                    Node {
                        top: Val::Percent(0.0),
                        left: Val::Percent(20.0),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                ));
                // parent.spawn((
                //     Name::new("bookmark_atlas"),
                //     ImageNode::new(source.bookmark.clone()),
                //     Node {
                //         width: Val::Percent(7.7),
                //         height: Val::Percent(10.7),
                //         top: Val::Percent(21.0),
                //         left: Val::Percent(-7.6),
                //         position_type: PositionType::Absolute,
                //         ..Default::default()
                //     },  
                //     Button,
                // ))
                // .with_child((
                //     Text::new("Atlas"),
                //     TextFont {
                //         // font: font.clone(),
                //         font_size: 15.0,
                //         ..default()
                //     },
                //     TextColor(Color::rgb(10.0, 0.0, 7.0)),
                //     Node {
                //         top: Val::Percent(0.0),
                //         left: Val::Percent(20.0),
                //         position_type: PositionType::Absolute,
                //         ..Default::default()
                //     },
                // ));
                // parent.spawn((
                //     Name::new("bookmark_room"),
                //     ImageNode::new(source.bookmark.clone()),
                //     Node {
                //         width: Val::Percent(7.7),
                //         height: Val::Percent(10.7),
                //         top: Val::Percent(35.2),
                //         left: Val::Percent(-7.6),
                //         position_type: PositionType::Absolute,
                //         ..Default::default()
                //     },  
                //     Button,
                // ))
                // .with_child((
                //     Text::new("Room"),
                //     TextFont {
                //         // font: font.clone(),
                //         font_size: 15.0,
                //         ..default()
                //     },
                //     TextColor(Color::rgb(10.0, 0.0, 7.0)),
                //     Node {
                //         top: Val::Percent(0.0),
                //         left: Val::Percent(20.0),
                //         position_type: PositionType::Absolute,
                //         ..Default::default()
                //     },
                // ));
            });
        });
}

//更改角色
fn reload_player (
    id: u8,
    asset_server: &Res<AssetServer>,
    mut texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    mut source: ResMut<GlobalCharacterTextureAtlas>,
) {
    //根据id选择角色
    *source = GlobalCharacterTextureAtlas::init(id, &asset_server, &mut texture_atlas_layouts);
    info!("Player Reloading!");
}

fn reload_button_change (
    mut query: Query<(
            &mut ImageNode,
            &Name,),
        (With<Button>, With<CharacterSelectButton>,),
    >,
    mut query2: Query<(&mut ImageNode, &Name, &Children,), (With<CharacterSelectButton>, Without<Button>)>,
    mut text_query: Query<&mut Text>,
    source: Res<GlobalMenuTextureAtlas>,
    mut events: EventReader<ReloadPlayerEvent>,
) {
    for event in events.read() {
        for (mut image, name) in query.iter_mut() {
            match name.as_str() {
                "Shiroko_choose" => {
                    if event.0 == 1 {
                        image.image = source.shiroko.clone();
                    } else {
                        image.image = source.shiroko_unselect.clone();
                    }
                },
                "Arisu_choose" => {
                    if event.0 == 2 {
                        image.image = source.arisu.clone();
                    } else {
                        image.image = source.arisu_unselect.clone();
                    }
                },
                "Utaha_choose" => {
                    if event.0 == 3 {
                        image.image = source.utaha.clone();
                    } else {
                        image.image = source.utaha_unselect.clone();
                    }
                },
                _ => {}
            }
        }
        for (mut image, name, t) in &mut query2 {
            // 获取技能文本实体
            let mut stext: Option<Entity> = None;
            for te in t.iter() {
                stext = Some(*te);
                break;
            }
            let mut skilltext = text_query.get_mut(stext.unwrap()).unwrap();
            match event.0 {
                1 => {
                    match name.as_str() {
                        "1" => {
                            image.image = source.shiroko_skill1.clone();
                            skilltext.0 = source.shiroko_skill_text[0].clone();
                        },
                        "2" => {
                            image.image = source.shiroko_skill2.clone();
                            skilltext.0 = source.shiroko_skill_text[1].clone();
                        },
                        "3" => {
                            image.image = source.shiroko_skill3.clone();
                            skilltext.0 = source.shiroko_skill_text[2].clone();
                        },
                        "4" => {
                            image.image = source.shiroko_skill4.clone();
                            skilltext.0 = source.shiroko_skill_text[3].clone();
                        },
                        _ => {}
                    }
                },
                2 => {
                    match name.as_str() {
                        "1" => {
                            image.image = source.arisu_skill1.clone();
                            skilltext.0 = source.arisu_skill_text[0].clone();
                        },
                        "2" => {
                            image.image = source.arisu_skill2.clone();
                            skilltext.0 = source.arisu_skill_text[1].clone();
                        },
                        "3" => {
                            image.image = source.arisu_skill3.clone();
                            skilltext.0 = source.arisu_skill_text[2].clone();
                        },
                        "4" => {
                            image.image = source.arisu_skill4.clone();
                            skilltext.0 = source.arisu_skill_text[3].clone();
                        },
                        _ => {}
                    }
                },
                3 => {
                    match name.as_str() {
                        "1" => {
                            image.image = source.utaha_skill1.clone();
                            skilltext.0 = source.utaha_skill_text[0].clone();
                        },
                        "2" => {
                            image.image = source.utaha_skill2.clone();
                            skilltext.0 = source.utaha_skill_text[1].clone();
                        },
                        "3" => {
                            image.image = source.utaha_skill3.clone();
                            skilltext.0 = source.utaha_skill_text[2].clone();
                        },
                        "4" => {
                            image.image = source.utaha_skill4.clone();
                            skilltext.0 = source.utaha_skill_text[3].clone();
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }
}
fn handle_player_messages (
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut interaction_query: Query<(
            &mut ImageNode,
            &Interaction,
            &Name,),
        (Changed<Interaction>, With<Button>, With<CharacterSelectButton>, ),
    >,
    source: Res<GlobalMenuTextureAtlas>,
    character_source: ResMut<GlobalCharacterTextureAtlas>,
    mut events: EventWriter<ReloadPlayerEvent>,
) {
    for (mut image, interaction, name) in &mut interaction_query  {
        match name.as_str() {
            "Shiroko_choose" => {
                match *interaction {
                    Interaction::Pressed => {
                        if character_source.id != 1 {
                            reload_player(1, &asset_server, &mut texture_atlas_layouts, character_source);
                            events.send(ReloadPlayerEvent(1));
                            break;
                        }
                    },
                    Interaction::Hovered => {
                        image.image = source.shiroko_hover.clone();
                    }
                    Interaction::None => {
                        println!("Shiroko yo!");
                        if character_source.id == 1 {
                            image.image = source.shiroko.clone();
                        } else {
                            image.image = source.shiroko_unselect.clone();
                        }
                    }
                }
            },
            "Arisu_choose" => {
                match *interaction {
                    Interaction::Pressed => {
                        if character_source.id != 2 {
                            reload_player(2, &asset_server, &mut texture_atlas_layouts, character_source);
                            events.send(ReloadPlayerEvent(2));
                            break;
                        }
                    },
                    Interaction::Hovered => {
                        image.image = source.arisu_hover.clone();
                    }
                    Interaction::None => {
                        if character_source.id == 2 {
                            image.image = source.arisu.clone();
                        } else {
                            image.image = source.arisu_unselect.clone();
                        }
                    }
                }
            },
            "Utaha_choose" => {
                match *interaction {
                    Interaction::Pressed => {
                        if character_source.id != 3 {
                            reload_player(3, &asset_server, &mut texture_atlas_layouts, character_source);
                            events.send(ReloadPlayerEvent(3));
                            break;
                        }
                    },
                    Interaction::Hovered => {
                        image.image = source.utaha_hover.clone();
                    }
                    Interaction::None => {
                        if character_source.id == 3 {
                            image.image = source.utaha.clone();
                        } else {
                            image.image = source.utaha_unselect.clone();
                        }
                    }
                } 
            }
            _ => {}
        }
    }
}

fn handle_soramenu (
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut interaction_query: Query<(
            &mut ImageNode,
            &Interaction,
            &Name,),
        (Changed<Interaction>, With<Button>),
    >,
    mut windows: Query<&mut Window>,
    source: Res<GlobalMenuTextureAtlas>,
    mut character_source: ResMut<GlobalCharacterTextureAtlas>,
    mut events: EventWriter<ReloadPlayerEvent>,
    mut next_state: ResMut<NextState<HomeState>>,
) {
    for (mut image, interaction, name) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // println!("{}Clicked!", name);
                match name.as_str() {
                    "close" => {
                        if let Ok(mut window) = windows.get_single_mut() {
                            window.cursor_options.visible = false;
                        }
                        next_state.set(HomeState::Running);
                    },
                    _ => {}
                }
            },
            Interaction::Hovered => {
                // println!("Hovered!");

            },
            Interaction::None => {
            },
        }
    }
}

fn cleanup_soramenu(
    mut commands: Commands,
    query: Query<Entity, With<SoraMenu>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_choosingbuffmenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, (With<Camera2d>, Without<Chest>)>,
    chest_query: Query<&ChestType, (With<Chest>, Without<Camera2d>)>,
    source: Res<GlobalMenuTextureAtlas>,
) {
    if camera_query.is_empty() {
        return;
    }
    let loc = camera_query.single().translation.truncate();
    commands.spawn((
        Sprite {
            image: source.border.clone(),
            flip_x: true,
            flip_y: true,
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(0.8)).with_translation(Vec3::new(loc.x - 530.0, loc.y + 510.0, 100.0)),
            PauseMenu::BorderUp,
            ChoosingBuffMenu,
    ));
    commands.spawn((
        Sprite {
            image: source.border.clone(),
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(0.8)).with_translation(Vec3::new(loc.x + 530.0, loc.y - 510.0, 100.0)),
            PauseMenu::BorderDown,
            ChoosingBuffMenu,
    )); 


    let font = asset_server.load("fonts/pixel_font.ttf");
    
    commands.spawn((
        ImageNode::new(source.tips.clone()),
        Node {
            width: Val::Percent(30.0),
            height: Val::Percent(50.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(35.0),
            top: Val::Percent(25.0 +  40.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ChoosingBuffMenu,
    ))
    .with_children(|parent| {
            parent.spawn((
                Text::new("增益菜单"),
                TextFont {
                    font: font.clone(),
                    font_size: 48.0,
                    ..default()
                },  
                TextColor(Color::rgb(123.0, 0.0, 131.0)),
                Node {
                    top: Val::Percent(0.0),
                    left: Val::Percent(40.0),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
            ));
            let ctype = chest_query.single().0;
            let mut rng = rand::rng();

            match ctype {
                0 => {
                    // 最好的宝箱选buff
                    // 随机生成三个buff
                    let mut unique_numbers = HashSet::new();
                    while unique_numbers.len() < 3 {
                        let num: i32 = rng.gen_range(0..5); // 生成1到100之间的整数
                        unique_numbers.insert(num);
                    }
                    let numbers: Vec<i32> = unique_numbers.into_iter().collect();

                    parent.spawn((
                        // Name::new("buff1"),
                        Name::new(format!("{}", numbers[0])),
                        // ImageNode::new(asset_server.load("Icon_Buff_AmmoUp.png")),
                        ImageNode::new(source.buff_icon[numbers[0] as usize].clone()),
                        Node {
                            width: Val::Percent(34.0),
                            height: Val::Percent(33.6),
                            top: Val::Percent(33.3),
                            left: Val::Percent(4.3),
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        Button,
                    ));
                    parent.spawn((
                        // Name::new("buff2"),
                        Name::new(format!("{}", numbers[1])),
                        // ImageNode::new(asset_server.load("Icon_Buff_AttackUp.png")),
                        ImageNode::new(source.buff_icon[numbers[1] as usize].clone()),
                        Node {
                            width: Val::Percent(34.0),
                            height: Val::Percent(33.6),
                            top: Val::Percent(33.1),
                            left: Val::Percent(36.1),
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        Button,
                    ));
                    parent.spawn((
                        // Name::new("buff3"),
                        Name::new(format!("{}", numbers[2])),
                        // ImageNode::new(asset_server.load("Icon_Buff_AbnormalUp.png")),
                        ImageNode::new(source.buff_icon[numbers[2] as usize].clone()),
                        Node {
                            width: Val::Percent(34.0),
                            height: Val::Percent(33.6),
                            top: Val::Percent(33.1),
                            left: Val::Percent(67.9),
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        Button,
                    ));
                },
                1 => {
                    // 差一点的宝箱选mod
                    // 随机生成三个buff
                    let mut unique_numbers = HashSet::new();
                    while unique_numbers.len() < 3 {
                        let num: i32 = rng.gen_range(0..3); // 生成1到100之间的整数
                        unique_numbers.insert(num);
                    }
                    let numbers: Vec<i32> = unique_numbers.into_iter().collect();

                    parent.spawn((
                        // Name::new("buff1"),
                        Name::new(format!("{}", numbers[0])),
                        // ImageNode::new(asset_server.load("Icon_Buff_AmmoUp.png")),
                        ImageNode::new(source.mod_icon[numbers[0] as usize].clone()),
                        Node {
                            width: Val::Percent(34.0),
                            height: Val::Percent(33.6),
                            top: Val::Percent(33.3),
                            left: Val::Percent(4.3),
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        Button,
                    ));
                    parent.spawn((
                        // Name::new("buff2"),
                        Name::new(format!("{}", numbers[1])),
                        // ImageNode::new(asset_server.load("Icon_Buff_AttackUp.png")),
                        ImageNode::new(source.mod_icon[numbers[1] as usize].clone()),
                        Node {
                            width: Val::Percent(34.0),
                            height: Val::Percent(33.6),
                            top: Val::Percent(33.1),
                            left: Val::Percent(36.1),
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        Button,
                    ));
                    parent.spawn((
                        // Name::new("buff3"),
                        Name::new(format!("{}", numbers[2])),
                        // ImageNode::new(asset_server.load("Icon_Buff_AbnormalUp.png")),
                        ImageNode::new(source.mod_icon[numbers[2] as usize].clone()),
                        Node {
                            width: Val::Percent(34.0),
                            height: Val::Percent(33.6),
                            top: Val::Percent(33.1),
                            left: Val::Percent(67.9),
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        Button,
                    ));
                },
                2 => {
                    // 最小的宝箱选消耗品
                    // 随机生成三个consump
                    let mut unique_numbers = HashSet::new();
                    while unique_numbers.len() < 3 {
                        let num: i32 = rng.gen_range(0..3); // 生成1到100之间的整数
                        unique_numbers.insert(num);
                    }
                    let numbers: Vec<i32> = unique_numbers.into_iter().collect();

                    parent.spawn((
                        // Name::new("buff1"),
                        Name::new(format!("{}", numbers[0])),
                        // ImageNode::new(asset_server.load("Icon_Buff_AmmoUp.png")),
                        ImageNode::new(source.buff_consumptions[numbers[0] as usize].clone()),
                        Node {
                            width: Val::Percent(34.0),
                            height: Val::Percent(33.6),
                            top: Val::Percent(33.3),
                            left: Val::Percent(4.3),
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        Button,
                    ));
                    parent.spawn((
                        // Name::new("buff2"),
                        Name::new(format!("{}", numbers[1])),
                        // ImageNode::new(asset_server.load("Icon_Buff_AttackUp.png")),
                        ImageNode::new(source.buff_consumptions[numbers[1] as usize].clone()),
                        Node {
                            width: Val::Percent(34.0),
                            height: Val::Percent(33.6),
                            top: Val::Percent(33.1),
                            left: Val::Percent(36.1),
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        Button,
                    ));
                    parent.spawn((
                        // Name::new("buff3"),
                        Name::new(format!("{}", numbers[2])),
                        // ImageNode::new(asset_server.load("Icon_Buff_AbnormalUp.png")),
                        ImageNode::new(source.buff_consumptions[numbers[2] as usize].clone()),
                        Node {
                            width: Val::Percent(34.0),
                            height: Val::Percent(33.6),
                            top: Val::Percent(33.1),
                            left: Val::Percent(67.9),
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        Button,
                    ));
                },
                _ => {}
            }

        });
}

fn handle_choosingbuffmenu (
    camera_query: Query<&Transform, (With<Camera2d>, Without<PauseMenu>)>,
    mut menu_query: Query<(&mut Transform, &PauseMenu,), (Without<Camera2d>, Without<Node>)>,
    mut interaction_query: Query<(
            &mut ImageNode,
            &Interaction,
            &Name,),
        (Changed<Interaction>, With<Button>),
    >,
    chest_query: Query<&ChestType, With<Chest>>,
    mut buff_query: Query<(&mut Buff, &mut Health), With<Character>>,
    mut windows: Query<&mut Window>,
    mut query: Query<&mut Node, (With<ChoosingBuffMenu>, Without<Camera2d>)>,
    // source: Res<GlobalMenuTextureAtlas>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if camera_query.is_empty() || menu_query.is_empty() {
        return;
    }
    let loc = camera_query.single().translation.truncate();
    for (mut trans, obj) in menu_query.iter_mut() {
        match *obj {
            PauseMenu::BorderUp => {
                if trans.translation.y > loc.y + 250.0 {
                    trans.translation.y -= 10.0;
                }
            },
            PauseMenu::BorderDown => {
                if trans.translation.y < loc.y - 250.0 {
                    trans.translation.y += 10.0;
                }
            },
            _ => {},
        }
    }
    for mut node in query.iter_mut() {
        match node.top {
            Val::Percent(v) => {
                if v > 25.0 {
                    node.top = Val::Percent(v - 2.0);
                } else{
                    for (mut image, interaction, name) in &mut interaction_query {
                        info!("interaction: ");
                        match *interaction {
                            Interaction::Pressed => {
                                println!("{}Clicked!", name);
                                if buff_query.is_empty() {
                                    return;
                                }
                                let (mut buff, mut health) = buff_query.single_mut();

                                let ctype = chest_query.single().0;

                                if let Ok(mut window) = windows.get_single_mut() {
                                    window.cursor_options.visible = false;
                                }

                                match ctype {
                                    0 => {
                                        match name.as_str() {
                                            "0" => {
                                                // 子弹分裂
                                                // if let Ok(mut window) = windows.get_single_mut() {
                                                //     window.cursor_options.visible = false;
                                                // }
                                                buff.0 += 2;
                                            },
                                            "1" => {
                                                // 伤害增加
                                                // if let Ok(mut window) = windows.get_single_mut() {
                                                //     window.cursor_options.visible = false;
                                                // }
                                                buff.4 += 2;
                                            },
                                            "2" => {
                                                // 射速提高
                                                // if let Ok(mut window) = windows.get_single_mut() {
                                                //     window.cursor_options.visible = false;
                                                // }
                                                buff.1 += 1;
                                            },
                                            "3" => {
                                                // 技能冷却加快
                                                // if let Ok(mut window) = windows.get_single_mut() {
                                                //     window.cursor_options.visible = false;
                                                // }
                                                buff.6 += 1;
                                            },
                                            "4" => {
                                                // 抗性增加
                                                // if let Ok(mut window) = windows.get_single_mut() {
                                                //     window.cursor_options.visible = false;
                                                // }
                                                buff.7 += 1;
                                            },
                                            _ => {}
                                        }
                                    },
                                    1 => {
                                        match name.as_str() {
                                            "0" => {
                                                // if let Ok(mut window) = windows.get_single_mut() {
                                                //     window.cursor_options.visible = false;
                                                // }
                                                buff.0+=2;
                                            },
                                            "1" => {
                                                // if let Ok(mut window) = windows.get_single_mut() {
                                                //     window.cursor_options.visible = false;
                                                // }
                                                buff.4 += 2;
                                            },
                                            "2" => {
                                                // if let Ok(mut window) = windows.get_single_mut() {
                                                //     window.cursor_options.visible = false;
                                                // }
                                                buff.1 += 1;
                                            },
                                            _ => {}
                                        }
                                    },
                                    2 => {
                                        match name.as_str() {
                                            "0" => {
                                                // 回生命
                                                // if let Ok(mut window) = windows.get_single_mut() {
                                                //     window.cursor_options.visible = false;
                                                // }
                                                health.0 += PLAYER_HEALTH * 0.15;
                                                if health.0 > PLAYER_HEALTH {
                                                    health.0 = PLAYER_HEALTH;
                                                }
                                            },
                                            "1" => {
                                                // if let Ok(mut window) = windows.get_single_mut() {
                                                //     window.cursor_options.visible = false;
                                                // }
                                                health.0 += PLAYER_HEALTH * 0.15;
                                                if health.0 > PLAYER_HEALTH {
                                                    health.0 = PLAYER_HEALTH;
                                                }
                                            },
                                            "2" => {
                                                // if let Ok(mut window) = windows.get_single_mut() {
                                                //     window.cursor_options.visible = false;
                                                // }
                                                health.0 += PLAYER_HEALTH * 0.15;
                                                if health.0 > PLAYER_HEALTH {
                                                    health.0 = PLAYER_HEALTH;
                                                }
                                            },
                                            _ => {}
                                        }
                                    },
                                    _ => {}
                                }

                                next_state.set(InGameState::Running);
                            },
                            Interaction::Hovered => {
                                println!("Hovered!");
                                // image.image = source.button_hover.clone();
                            },
                            Interaction::None => {
                                // image.image = source.button.clone();
                            },
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

fn cleanup_choosingbuffmenu (
    mut commands: Commands,
    query: Query<Entity, With<ChoosingBuffMenu>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_gameovermenu (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    buff_query: Query<&Buff, With<Character>>,
    source: Res<GlobalMenuTextureAtlas>,
    source1: Res<ScoreResource>,
) {
   let font: Handle<Font> = asset_server.load("Fonts/FIXEDSYS-EXCELSIOR-301.ttf");
   let font2 : Handle<Font> = asset_server.load("Fonts/pixel_font.ttf");
   if buff_query.is_empty() {
       return;
   }
   let mut level = -8;
   for buff in buff_query.iter() {
    level += buff.sum();
   }
    commands.spawn((
        //底
        ImageNode::new(source.menu.clone()),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(130.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(0.0),
            top: Val::Percent(-10.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        GameOverMenu,
    ))
    .with_children(|parent| {
            //顶
            parent.spawn((
                ImageNode::new(source.top.clone()),
                Node {
                    width: Val::Percent(70.0),
                    height: Val::Percent(59.0),
                    top: Val::Percent(20.7),
                    left: Val::Percent(15.0),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
            ))
            .with_children(|parent| {
                // 角色简介底
                parent.spawn(( 
                    ImageNode::new(source.list.clone()),
                    Node {
                        width: Val::Percent(45.0),
                        height: Val::Percent(68.8),
                        top: Val::Percent(10.7),
                        left: Val::Percent(52.5),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    }, 
                ));
                // 别针
                parent.spawn(( 
                    ImageNode::new(source.title.clone()),
                    Node {
                        width: Val::Percent(23.3),
                        height: Val::Percent(16.5),
                        top: Val::Percent(-4.2),
                        left: Val::Percent(13.5),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    }, 
                ))                
                .with_child((
                    Text::new("Game Over!"),
                    TextFont {
                        font: font.clone(),
                        font_size: 35.0,
                        ..default()
                    },
                    TextColor(Color::rgb(0.0, 0.0, 0.0)),
                    Node {
                        top: Val::Percent(0.0),
                        left: Val::Percent(20.0),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                ));
                parent.spawn((
                    Text::new(format!("小机器人击杀数:{}", source1.enemy_score)),
                    TextFont {
                        font: font2.clone(),
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::rgb(0.0, 0.0, 0.0)),
                    Node {
                        top: Val::Percent(39.2),
                        left: Val::Percent(6.6),
                        ..Default::default()
                    },
                    test,
                ));
                parent.spawn((
                    Text::new(format!("Boss击杀数:{}", source1.boss_score)),
                    TextFont {
                        font: font2.clone(),
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::rgb(0.0, 0.0, 0.0)),
                    Node {
                        top: Val::Percent(19.2),
                        left: Val::Percent(6.6),
                        ..Default::default()
                    },
                    test,
                ));
                parent.spawn((
                    Text::new(format!("存活时间:  {}:{}", source1.time_min, source1.time_sec)),
                    TextFont {
                        font: font2.clone(),
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::rgb(0.0, 0.0, 0.0)),
                    Node {
                        top: Val::Percent(-0.8),
                        left: Val::Percent(6.6),
                        ..Default::default()
                    },
                    test,
                ));
                parent.spawn((
                    Text::new(format!("通过关卡:{}", source1.map_index)),
                    TextFont {
                        font: font2.clone(),
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::rgb(0.0, 0.0, 0.0)),
                    Node {
                        top: Val::Percent(-20.8),
                        left: Val::Percent(6.6),
                        ..Default::default()
                    },
                    test,
                ));
                parent.spawn((
                    Text::new(format!("角色等级:{}", level)),
                    TextFont {
                        font: font2.clone(),
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::rgb(0.0, 0.0, 0.0)),
                    Node {
                        top: Val::Percent(-40.8),
                        left: Val::Percent(6.6),
                        ..Default::default()
                    },
                    test,
                ));

                // 按钮
                parent.spawn(( 
                    Name::new("back to home"),
                    ImageNode::new(source.button.clone()),
                    Node {
                        width: Val::Percent(17.5),
                        height: Val::Percent(7.4),
                        top: Val::Percent(67.5),
                        left: Val::Percent(66.7),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                   }, 
                   Button,
                ))
                .with_child((
                    Text::new("back to home"),
                    TextFont {
                        font: font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::rgb(0.0, 0.0, 0.0)),
                    Node {
                        top: Val::Percent(0.0),
                        left: Val::Percent(20.0),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                ));
            });
        });
}

fn handle_gameovermenu (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<(&Transform, &mut GameState), (With<Camera2d>, Without<PauseMenu>)>,
    mut menu_query: Query<(&mut Transform, &PauseMenu,), (Without<Camera2d>, Without<Node>)>,
    mut interaction_query: Query<(
            &mut ImageNode,
            &Interaction,
            &Name,),
        (Changed<Interaction>, With<Button>),
    >,
    mut windows: Query<&mut Window>,
    mut query: Query<&mut Node, (With<PauseMenu>, Without<Camera2d>)>,
    source: Res<GlobalMenuTextureAtlas>,
    // mut mgr: ResMut<AssetsManager>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if camera_query.is_empty() {
        return;
    }
    let (camera_trans, mut nextstate) = camera_query.single_mut();
    let loc = camera_trans.translation.truncate();
    for (mut image, interaction, name) in &mut interaction_query {
        info!("interaction: ");
        match *interaction {
            Interaction::Pressed => {
                println!("{}Clicked!", name);
                match name.as_str() {
                    "back to home" => {
                        if let Ok(mut window) = windows.get_single_mut() {
                            window.cursor_options.visible = false;
                        }
                        next_state.set(InGameState::Running);
                        *nextstate = GameState::Home;
                        commands.spawn((
                            Sprite {
                                image: asset_server.load("Menu_Transition1.png"),
                                ..Default::default()
                            },
                            Transform::from_scale(Vec3::new(0.7,0.7,0.5))
                                .with_translation(Vec3::new(loc.x-3200.0, loc.y, 100.0)),
                            Transition,
                        ));
                    },
                    _ => {}
                }
            },
            Interaction::Hovered => {
                println!("Hovered!");
                image.image = source.button_hover.clone();
            },
            Interaction::None => {
                image.image = source.button.clone();
            },
        }
    }
}

fn cleanup_gameovermenu (
    mut commands: Commands,
    query: Query<Entity, With<GameOverMenu>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct test;

fn print_node_loc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Node, With<test>>,
) {
    if query.is_empty() {
        return;
    }
    let left = keyboard_input.pressed(KeyCode::ArrowLeft);
    let right = keyboard_input.pressed(KeyCode::ArrowRight);
    let up = keyboard_input.pressed(KeyCode::ArrowUp);
    let down = keyboard_input.pressed(KeyCode::ArrowDown);
    let zoom_up_w = keyboard_input.pressed(KeyCode::Equal);
    let zoom_down_w = keyboard_input.pressed(KeyCode::Minus);
    let zoom_up_h = keyboard_input.pressed(KeyCode::KeyQ);
    let zoom_down_h = keyboard_input.pressed(KeyCode::KeyE);

    let mut loc = Vec2::ZERO;
    let mut zoom = Vec2::ZERO;

    for mut node in query.iter_mut() {
        match node.top {
            Val::Percent(v) => {
                if up {
                    node.top = Val::Percent(v - 0.1);
                }
                if down {
                    node.top = Val::Percent(v + 0.1);
                }

                loc.y = v;
            },
            _ => {}
        }
        match node.left {
            Val::Percent(v) => {
                loc.x = v;
                if left {
                    node.left = Val::Percent(v - 0.1);
                }
                if right {
                    node.left = Val::Percent(v + 0.1);
                }
            },
            _ => {}
        }
        match node.width {
            Val::Percent(v) => {
                zoom.x = v;

                if zoom_up_w {
                    node.width = Val::Percent(v + 0.1);
                }
                if zoom_down_w {
                    node.width = Val::Percent(v - 0.1);
                }
            },
            _ => {}
        }
        match node.height {
            Val::Percent(v) => {
                zoom.y = v;

                if zoom_up_h {
                    node.height = Val::Percent(v + 0.1);
                }
                if zoom_down_h {
                    node.height = Val::Percent(v - 0.1);
                }
            },
            _ => {}
        }
        if keyboard_input.just_pressed(KeyCode::KeyP) {
            print!("width: {}, height: {}, top: {}, left: {},\n", zoom.x, zoom.y, loc.y, loc.x);
        }
    }
}