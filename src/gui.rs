use bevy::{ecs::query, prelude::*, render::camera};

use crate::{
    gamestate::*,
    character::{Character, Player},
    gun::{Gun, Cursor},
    home::Home,
    ui::UI,
    room::AssetsManager,
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
pub enum ButtonType {
    Close,
    ReturntoMainMenu,
    ReturntoHome,
    Settings,
    Quit,
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::MainMenu), (
            clear_all.before(setup_main_menu),
            setup_main_menu))
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
        .add_systems(Update, handle_stopmenu.run_if(in_state(HomeState::Pause)))
        .add_systems(OnExit(HomeState::Pause), cleanup_stopmenu)

        .add_systems(OnEnter(InGameState::Pause), setup_stopmenu)
        .add_systems(Update, handle_stopmenu.run_if(in_state(InGameState::Pause)))
        .add_systems(OnExit(InGameState::Pause), cleanup_stopmenu)
        .add_systems(Update, (animation1::<LeftSlide1>, animation1::<LeftSlide2>, animation2::<RightSlide1>, animation2::<RightSlide2>).run_if(in_state(GameState::MainMenu)))
        .add_systems(Update, statetransition)
        // .add_systems(Update, log_transitions::<GameState>)
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
    query1: Query<Entity, (With<Player>, (Without<UI>))>,
    query2: Query<Entity, (With<UI>, (Without<Player>))>,
) {
    for entity in query1.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in query2.iter() {
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
    camera_query: Query<&Transform, (With<Camera2d>, Without<Transition>)>,
) {

    if camera_query.is_empty() {
        return;
    }

    if  (keyboard_input.get_just_pressed().count() > 0 || mouse_button_input.pressed(MouseButton::Left)) && transition_query.is_empty() {
        println!("Working!");

        let trans = camera_query.single().translation;

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
    camera_query: Query<&Transform, (With<Camera2d>, Without<Transition>)>,
    state: Res<State<GameState>>,
    mut mgr: ResMut<AssetsManager>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if transition_query.is_empty() || camera_query.is_empty() {
        return;
    }
    let (mut transform, e) = transition_query.single_mut();
    let trans = camera_query.single().translation;
    let x = trans.x;
    transform.translation.x += 20.0;
    let delta = transform.translation.x - x;

    // println!("delta: {}", delta);
    match *state.get() {
        GameState::MainMenu if delta >= 400.0 && delta < 420.0 => {
            next_state.set(GameState::Home);
            // info!("transition to Home!");
        },
        GameState::Home if delta >= 400.0 && delta < 420.0 => {
            next_state.set(GameState::Loading);
            mgr.cycle_map(&mut commands);
            // info!("transition to loading!");
        },
        GameState::Loading if delta >= 800.0 => {
            next_state.set(GameState::InGame);
            // info!("transition to game!");
        },

        GameState::InGame if delta >= 400.0 && delta < 420.0 => {
            next_state.set(GameState::Loading);
            mgr.cycle_map(&mut commands);
            // info!("transition to loading!");
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
        });
        }


    }
}

fn handle_ingame_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<InGameState>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(match *state.get() {
            InGameState::Running => InGameState::Pause,
            InGameState::Pause => InGameState::Running,
            InGameState::Reloading => InGameState::Running,
        });
    }
}

fn setup_stopmenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, (With<Camera2d>)>
) {
    if camera_query.is_empty() {
        return;
    }
    let loc = camera_query.single().translation.truncate();
    commands.spawn((
        Sprite {
            image: asset_server.load("BookMenu_PauseBorderSmall.png"),
            flip_x: true,
            flip_y: true,
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(0.8)).with_translation(Vec3::new(loc.x - 530.0, loc.y + 510.0, 100.0)),
            PauseMenu::BorderUp,
    ));
    commands.spawn((
        Sprite {
            image: asset_server.load("BookMenu_PauseBorderSmall.png"),
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(0.8)).with_translation(Vec3::new(loc.x + 530.0, loc.y - 510.0, 100.0)),
            PauseMenu::BorderDown,
    )); 

    let image_button = asset_server.load("BookMenu_ButtonBig.png");
    let font: Handle<Font> = asset_server.load("Fonts/FIXEDSYS-EXCELSIOR-301.ttf");
    let image_menu = asset_server.load("BookMenu_List.png");
    commands.spawn((
        ImageNode::new(image_menu.clone()),
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
                Text::new("Pause"),
                TextFont {
                    font: font.clone(),
                    font_size: 45.0,
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
                ImageNode::new(image_button.clone()),
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
                Text::new("back to game"),
                TextFont {
                        font: font.clone(),
                        font_size: 35.0,
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
                ImageNode::new(image_button.clone()),
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
                Text::new("back to menu"),
                TextFont {
                        font: font.clone(),
                        font_size: 35.0,
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

fn handle_stopmenu (
    camera_query: Query<&Transform, (With<Camera2d>, Without<PauseMenu>)>,
    mut menu_query: Query<(&mut Transform, &PauseMenu,), (Without<Camera2d>, Without<Node>)>,
    mut interaction_query: Query<(
            &Interaction,
            &Name,),
        (Changed<Interaction>, With<Button>),
    >,
    mut windows: Query<&mut Window>,
    mut query: Query<&mut Node, (With<PauseMenu>, Without<Camera2d>)>,
    
    mut next_state: ResMut<NextState<HomeState>>,
    mut next_state2: ResMut<NextState<GameState>>,
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
                    for (interaction, name) in &mut interaction_query {
                        info!("interaction: ");
                        match *interaction {
                            Interaction::Pressed => {
                                println!("{}Clicked!", name);
                                match name.as_str() {
                                    "back to game" => {
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
                                        next_state2.set(GameState::MainMenu);
                                    },
                                    _ => {}
                                }
                            },
                            Interaction::Hovered => {
                                println!("Hovered!");
                            },
                            Interaction::None => {},
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