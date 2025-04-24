use bevy::{dev_tools::states::*, ecs::query, prelude::*};
use crate::{gamestate::GameState,
            character::{AnimationConfig, Character},
            resources::GlobalHomeTextureAtlas,
            };
use bevy_rapier2d::{prelude::*, rapier::prelude::{RigidBodyMassProps, RigidBodyType}};

pub struct HomePlugin;
//小空叫Sora
#[derive(Component)]
pub struct Sora;
#[derive(Component, Default)]
pub enum SoraState {
    #[default]
    Loop,
    Awake,
    Asleep,
}

#[derive(Component)]
pub struct Fridge;
#[derive(Component, Default)]
pub enum FridgeState {
    #[default]
    Loop,
    Open,
    Close,
}

#[derive(Component)]
pub struct Home;
#[derive(Component)]
pub struct Wall;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, log_transitions::<GameState>)
            .add_systems(OnEnter(GameState::Home), setup)
            .add_systems(Update, (check_state, update_wall).run_if(in_state(GameState::Home)))
            .add_systems(OnExit(GameState::Home), cleanup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    source: Res<GlobalHomeTextureAtlas>,
) {
    //背景板
    commands.spawn( (Sprite {
        image: asset_server.load("ForeGround.png"),
        ..Default::default()
        },
        Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::new(0.0, 0.0, 5.0)),
        Home,
        ))
        .with_child(
            (Sprite {
                image: asset_server.load("BackGround.png"),
                ..Default::default()
                },
            Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(0.0, -20.0, -5.0)),
            ))
        .with_child(
            (Sprite {
                image: asset_server.load("StorageRacks1.png"),
                ..Default::default()
                },
            Transform::from_scale(Vec3::splat(0.8)).with_translation(Vec3::new(-65.0, -58.0, -1.0)),
            ))
        .with_child(
            (Sprite {
                image: asset_server.load("StorageRacks2.png"),
                ..Default::default()
                },
            Transform::from_scale(Vec3::splat(0.8)).with_translation(Vec3::new(-15.0, -58.0, -2.0)),
            ))
        .with_child(
            (Sprite {
                image: asset_server.load("StorageRacks3.png"),
                ..Default::default()
                },
            Transform::from_scale(Vec3::splat(0.8)).with_translation(Vec3::new(170.0, -60.0, -2.0)),
            ))
        .with_child(
            (Sprite {
                image: asset_server.load("Checkout.png"),
                ..Default::default()
                },
            Transform::from_scale(Vec3::splat(1.1)).with_translation(Vec3::new(140.0, -73.0, -1.0)),
            ))
        .with_child(
            (Sprite {
                image: asset_server.load("Checkout2.png"),
                ..Default::default()
                },
            Transform::from_scale(Vec3::splat(0.8)).with_translation(Vec3::new(114.0, -57.0, -2.0)),
            ))
        .with_child(
            (Sprite {
                image: asset_server.load("Dustbin.png"),
                ..Default::default()
                },
            Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(-110.0, -68.0, -2.0)),
            ));

    //以下三个需调位置和加动画还有功能。
    //冰箱
    commands.spawn( 
        (Sprite {
            image: source.Fridge_image_loop.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: source.Fridge_lay_out_loop.clone(),
                index: 0,
            }),
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(-470.0, -123.0, -1.0)),
            AnimationConfig::new(10),
            Fridge,
            FridgeState::default(),
            Home,
            ));
    //小空
    commands.spawn( 
        (Sprite {
            image: source.Sora_image_loop.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: source.Sora_lay_out_loop.clone(),
                index: 0,
            }),
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(430.0, -158.0, -1.0)),
            AnimationConfig::new(7),
            Sora,
            SoraState::default(),
            Home,
            ));
    //看板
    commands.spawn( 
        (Sprite {
            image: asset_server.load("Billboard.png"),
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::new(75.0, -170.0 + 300.0, -0.5)),
            Home,
            //test
            Collider::cuboid(15.0, 18.0),
            ColliderMassProperties::Mass(100.0),
            // CollisionGroups::new(Group::GROUP_2, Group::ALL),
            RigidBody::Dynamic,
            LockedAxes::TRANSLATION_LOCKED_X,
            GravityScale(5.0),
            // Sensor,
            ));
    //地板和墙
    // let joint = FixedJointBuilder::new().local_anchor1(Vec2::new(-750.0, 500.0));
    commands.spawn((
        Collider::cuboid(200.0, 500.0),
        RigidBody::Dynamic,
        ColliderMassProperties::Mass(10.0),
        LockedAxes::TRANSLATION_LOCKED,
        // GravityScale(0.0),
        // Sensor,
        // CollisionGroups::new(Group::GROUP_2, Group::ALL),
        // ImpulseJoint::new(parentid, joint),
        Transform::from_translation(Vec3::new(-750.0, 260.0, 0.0)),
        Wall,
        Home,
        ));
    commands.spawn((
        Collider::cuboid(200.0, 500.0),
        RigidBody::Dynamic,
        ColliderMassProperties::Mass(1000.0),
        LockedAxes::TRANSLATION_LOCKED,
        // GravityScale(0.0),
        // Sensor,
        // CollisionGroups::new(Group::GROUP_2, Group::ALL),
        Transform::from_translation(Vec3::new(750.0, 260.0, 0.0)),
        Wall,
        Home,
        ));
    commands.spawn((
        Collider::cuboid(2400.0, 25.0),
        RigidBody::Dynamic,
        ColliderMassProperties::Mass(1000.0),
        // LockedAxes::TRANSLATION_LOCKED,
        // GravityScale(0.0),
        // Sensor,
        // CollisionGroups::new(Group::GROUP_2, Group::ALL),
        Transform::from_translation(Vec3::new(0.0, -270.0, 0.0)),
        Home,
        ));
    commands.spawn((
        Collider::cuboid(2400.0, 1.0),
        RigidBody::Dynamic,
        // GravityScale(0.0),
        // Sensor,
        Transform::from_translation(Vec3::new(0.0, -295.0, 10.0)),
        Home,
        ));
    commands.spawn((
        Collider::cuboid(2400.0, 5.0),
        RigidBody::Fixed,
        Transform::from_translation(Vec3::new(0.0, -296.0, 10.0)),
        Home,
        ));

}
fn check_state(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, (With<Character>, Without<Sora>, Without<Fridge>)>,
    mut sora_query: Query<(&Transform, &mut Sprite, &mut SoraState), (With<Sora>, Without<Fridge>, Without<Character>)>,
    mut fridge_query: Query<(&Transform, &mut Sprite, &mut FridgeState), (With<Fridge>, Without<Character>, Without<Sora>)>,
    source: Res<GlobalHomeTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
 ) {
    if player_query.is_empty() || sora_query.is_empty() || fridge_query.is_empty() {
        // println!("empty1!");
        return;
    }
    let player_pos = player_query.single().translation;
    let (sora_transform, mut sora_sprite, mut sora_state) = sora_query.single_mut();
    let (fridge_transform, mut fridge_sprite, mut fridge_state) = fridge_query.single_mut();
    //小空
    if (sora_transform.translation.x - player_pos.x).abs() < 100.0 {
        // println!("activate Sora!");
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            println!("Menu!");
        }
        match *sora_state {
            SoraState::Loop => {
                sora_sprite.image = source.Sora_image_awake.clone();
                sora_sprite.texture_atlas = Some(TextureAtlas {
                    layout: source.Sora_lay_out_awake.clone(),
                    index: 0,
                });
                *sora_state = SoraState::Awake;
            },
            _ => {},
        }
    }
    else {
        // println!("deactivate Sora!");
        match *sora_state {
            SoraState::Awake => {
                sora_sprite.image = source.Sora_image_asleep.clone();
                sora_sprite.texture_atlas = Some(TextureAtlas {
                    layout: source.Sora_lay_out_asleep.clone(),
                    index: 0,
                });
                *sora_state = SoraState::Asleep;
            }
            _ => {},
        }
    }
    //冰箱
    if (fridge_transform.translation.x - player_pos.x).abs() < 70.0 {
        // println!("activate Fridge!");
        match *fridge_state {
            FridgeState::Loop => {
                fridge_sprite.image = source.Fridge_image_oc.clone();
                fridge_sprite.texture_atlas = Some(TextureAtlas {
                    layout: source.Fridge_lay_out_oc.clone(),
                    index: 0,
                });
                *fridge_state = FridgeState::Open;
            },
            FridgeState::Open => {
                if keyboard_input.just_pressed(KeyCode::KeyE) {
                    println!("Game Start!");
                    // commands.spawn((
                    //     Sprite {
                    //         image: asset_server.load("Collapse.png"),
                    //         ..Default::default()
                    //     },
                    //     Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::splat(4.0)),
                    // ));
                    //为了测试，先将状态转换到游戏中，游戏初始化状态之后再设置
                    // next_state.set(GameState::InGame);
                    next_state.set(GameState::Loading);
                }                                                                       
            }
            _ => {},
        }
    }
    else {
        // println!("deactivate Fridge!");
        match *fridge_state {
            FridgeState::Open => {
                *fridge_state = FridgeState::Close;
            }
            _ => {},
        }
    }
 }
 fn update_wall (
    mut query: Query<&mut Transform, With<Wall>>,
 ) {
    for mut transform in &mut query {
        if transform.translation.x < 0.0 {
            transform.translation.x = -749.0;
        }
        else {
            transform.translation.x = 749.0;
        }
    }
 }
 fn cleanup(
    mut commands: Commands, 
    mut menu_items_query: Query<Entity, With<Home>>) {
    for parent in &mut menu_items_query {
        commands.entity(parent).despawn_recursive();
    }
    println!("Home cleaned up!");
}