use bevy::render::texture;
use bevy::{dev_tools::states::*, prelude::*};
use bevy::math::vec3;
use crate::{gamestate::GameState,
            character::{AnimationConfig, Character},
            };

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
impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, log_transitions::<GameState>)
            .add_systems(OnEnter(GameState::Home), setup)
            .add_systems(Update, check_state.run_if(in_state(GameState::Home)))
            .add_systems(OnExit(GameState::Home), cleanup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    //背景板
    commands.spawn( (Sprite {
        image: asset_server.load("ForeGround.png"),
        ..Default::default()
        },
        Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::new(0.0, 0.0, 5.0)),
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
            Transform::from_scale(Vec3::splat(0.8)).with_translation(Vec3::new(-60.0, -58.0, -1.0)),
            ))
        .with_child(
            (Sprite {
                image: asset_server.load("StorageRacks2.png"),
                ..Default::default()
                },
            Transform::from_scale(Vec3::splat(0.8)).with_translation(Vec3::new(-10.0, -58.0, -2.0)),
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
    let layout_start = TextureAtlasLayout::from_grid(UVec2::splat(96),10,3,None,None);
    commands.spawn( 
        (Sprite {
            image: asset_server.load("Teleporter_2_Start.png"),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layouts.add(layout_start),
                index: 0,
            }),
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(-470.0, -118.0, -1.0)),
            AnimationConfig::new(10),
            Fridge,
            FridgeState::default(),
            ));
    //小空
    let layout_sora = TextureAtlasLayout::from_grid(UVec2::splat(80),8,1,None,None);
    commands.spawn( 
        (Sprite {
            image: asset_server.load("Sora_RestLoop.png"),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layouts.add(layout_sora),
                index: 0,
            }),
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(430.0, -158.0, -1.0)),
            AnimationConfig::new(7),
            Sora,
            SoraState::default(),
            ));
    //看板
    commands.spawn( 
        (Sprite {
            image: asset_server.load("Billboard.png"),
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::new(70.0, -200.0, -0.5)),
            ));
}
fn check_state(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, (With<Character>, Without<Sora>, Without<Fridge>)>,
    mut sora_query: Query<(&Transform, &mut Sprite, &mut SoraState), (With<Sora>, Without<Fridge>, Without<Character>)>,
    mut fridge_query: Query<(&Transform, &mut Sprite, &mut FridgeState), (With<Fridge>, Without<Character>, Without<Sora>)>,
 ) {
    if player_query.is_empty() || sora_query.is_empty() || fridge_query.is_empty() {
        // println!("empty1!");
        return;
    }
    // if fridge_query.is_empty() {
    //     println!("empty2!");
    //     return;
    // }
    let player_pos = player_query.single().translation;
    let (sora_transform, mut sora_sprite, mut sora_state) = sora_query.single_mut();
    let (fridge_transform, mut fridge_sprite, mut fridge_state) = fridge_query.single_mut();
    //小空
    if (sora_transform.translation.x - player_pos.x).abs() < 100.0 {
        // println!("activate Sora!");
        match *sora_state {
            SoraState::Loop => {
                sora_sprite.image = asset_server.load("Sora_RestEnd.png");
                let layout_sora = TextureAtlasLayout::from_grid(UVec2::splat(80),14,1,None,None);
                sora_sprite.texture_atlas = Some(TextureAtlas {
                    layout: texture_atlas_layouts.add(layout_sora),
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
                sora_sprite.image = asset_server.load("Sora_Rest.png");
                let layout_sora = TextureAtlasLayout::from_grid(UVec2::splat(80),18,1,None,None);
                sora_sprite.texture_atlas = Some(TextureAtlas {
                    layout: texture_atlas_layouts.add(layout_sora),
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
                fridge_sprite.image = asset_server.load("Teleporter_2_Open.png");
                let layout_fridge = TextureAtlasLayout::from_grid(UVec2::splat(96),10,2,None,None);
                fridge_sprite.texture_atlas = Some(TextureAtlas {
                    layout: texture_atlas_layouts.add(layout_fridge),
                    index: 0,
                });
                *fridge_state = FridgeState::Open;
            },
            FridgeState::Open => {
                if keyboard_input.just_pressed(KeyCode::KeyE) {
                    println!("Game Start!");
                    // to do: change game state to GameState::InGame
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
fn cleanup(
    mut commands: Commands, 
    mut menu_items_query: Query<Entity, With<Sprite>>) {
    for parent in &mut menu_items_query {
        commands.entity(parent).despawn_recursive();
    }
    println!("Home cleaned up!");
}