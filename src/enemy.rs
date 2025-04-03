use bevy::math::vec3;
use bevy::reflect::Enum;
use bevy::transform;
use bevy::{dev_tools::states::*, prelude::*, time::Stopwatch};
use std::clone;
use std::{time::Duration};
use crate::{gamestate::GameState,
    configs::*,character::*};
use crate::*;
use rand::Rng;
use character::AnimationConfig;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Enemy_Bullet;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Velocity(pub f32);

#[derive(Component)]
pub enum EnemyType {
    Sweeper,
    DroneMissile,
    DroneVulcan,
}

#[derive(Component, Default)]
pub enum EnemyState {
    #[default]
    Idea,
    FireStart,
    FireLoop,
    FireEnd,
    Move,
}

#[derive(Component)]
pub struct PatrolState {
    pub direction: f32,
    pub timer: Stopwatch,
    pub patrol_duration: Duration,
}


impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), set_enemy)
            .add_systems(
                Update,
                    (
                        handle_enemy_move,
                        handle_enemy_animation,
                        // handle_enemy_fire,
                        handle_enemy_hurt,
                        handle_enemy_death,
                ).run_if(in_state(GameState::InGame))
            )
            .add_systems(Update, log_transitions::<GameState>)
            ;
    }
}

fn set_enemy(
    mut commands: Commands,
    source1: Res<GlobalSweeperTextureAtlas>,
    source2: Res<GlobalDroneVulcanTextureAtlas>,
    source3: Res<GlobalDroneMissileTextureAtlas>,
    //asset_server: Res<AssetServer>,
    //mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut rng = rand::rng();
    let random_index = rng.random_range(0..3);
    //let x=2;
    let random_enemy = match random_index {
        0 => EnemyType::Sweeper,
        1 => EnemyType::DroneMissile,
        2 => EnemyType::DroneVulcan,
        _ => unreachable!(),
    };
    
    let patrol_duration = Duration::from_secs(2); // 巡逻持续时间，可根据需要调整

    match random_enemy{
        EnemyType::Sweeper=>{
            commands.spawn( (
                Sprite {
                    image: source1.image_idle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source1.lay_out_idle.clone(),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(0.0, -200.0, -50.0)),
                EnemyState::default(),
                Enemy,
                EnemyType::Sweeper,
                Health(ENEMY_HEALTH),
                Velocity(0.0),
                AnimationConfig::new(13),
                PatrolState {
                    direction: 1.0,
                    timer: Stopwatch::new(),
                    patrol_duration,
                },
                )
            );
        },
        EnemyType::DroneMissile=>{
            commands.spawn( (
                Sprite {
                    image: source3.image_idle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source3.lay_out_idle.clone(),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(0.0, -200.0, -50.0)),
                EnemyState::default(),
                Enemy,
                EnemyType::DroneMissile,
                Health(ENEMY_HEALTH),
                Velocity(ENEMY_SPEED),
                AnimationConfig::new(13),
                PatrolState {
                    direction: 1.0,
                    timer: Stopwatch::new(),
                    patrol_duration,
                },
                )
            );
        },
        EnemyType::DroneVulcan=>{
            commands.spawn( (
                Sprite {
                    image: source2.image_idle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source2.lay_out_idle.clone(),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(0.0, -200.0, -50.0)),
                EnemyState::default(),
                Enemy,
                EnemyType::DroneVulcan,
                Health(ENEMY_HEALTH),
                Velocity(ENEMY_SPEED),
                AnimationConfig::new(13),
                PatrolState {
                    direction: 1.0,
                    timer: Stopwatch::new(),
                    patrol_duration,
                },
                )
            );
        },
    }
}

fn handle_enemy_move(
    mut player_query: Query< & Transform,(With<Character>,Without<Enemy>)>,
    mut enemy_query: Query<(&mut Sprite, &mut Transform, &mut EnemyState, & Velocity, & EnemyType, &mut PatrolState), (With<Enemy>,Without<Character>)>,
    //asset_server: Res<AssetServer>,
    //mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    source1: Res<GlobalSweeperTextureAtlas>,
    source2: Res<GlobalDroneVulcanTextureAtlas>,
    source3: Res<GlobalDroneMissileTextureAtlas>,
    time: Res<Time>, 
) {
    if player_query.is_empty() {
        return;
    }
    if enemy_query.is_empty() {
        return;
    }

    let player = player_query.single_mut();
    
    for (mut enemy, mut transform, mut enemystate, v, enemytype, mut patrol_state) in enemy_query.iter_mut(){
        let dx = player.translation.x - transform.translation.x;
        let dy = player.translation.y - transform.translation.y + 50.0;
        let distance = (dx * dx + dy * dy).sqrt();

        if let Some(atlas) = &mut enemy.texture_atlas {
            if distance <= ENEMY_ALARM {
                match enemytype {
                    EnemyType::DroneMissile => {

                        if dx * patrol_state.direction < 0.0 {
                            patrol_state.direction = -1.0 * patrol_state.direction;
                        }

                        if distance <= ENEMY_FIRE {
                            match *enemystate{
                                EnemyState::Idea | EnemyState::Move => { 
                                    if atlas.index == 4{
                                        atlas.index = 0;
                                        atlas.layout = source3.lay_out_fire_start.clone();
                                        *enemystate = EnemyState::FireStart;
                                    }
                                },
                                EnemyState::FireStart => {
                                    if atlas.index == 2{
                                        atlas.index = 0;
                                        atlas.layout = source3.lay_out_fire_loop.clone();
                                        *enemystate = EnemyState::FireLoop;
                                    }
                                },
                                EnemyState::FireLoop => {},
                                EnemyState::FireEnd => { 
                                    if atlas.index == 1{
                                        atlas.index = 0;
                                        atlas.layout = source3.lay_out_fire_start.clone();
                                        *enemystate = EnemyState::FireStart;
                                        }
                                },
                            }
                            //fire
                        } else {
                            match *enemystate{
                                EnemyState::Move => {},
                                EnemyState::Idea => { 
                                    *enemystate = EnemyState::Move;
                                },
                                EnemyState::FireLoop => { 
                                    if atlas.index == 4 {
                                        atlas.index=0;
                                        atlas.layout = source3.lay_out_fire_end.clone();
                                        *enemystate = EnemyState::FireEnd;
                                    }
                                },
                                EnemyState::FireStart => { 
                                    if atlas.index == 2 {
                                        atlas.index=0;
                                        atlas.layout = source3.lay_out_fire_end.clone();
                                        *enemystate = EnemyState::FireEnd;
                                    }
                                },
                                EnemyState::FireEnd => {
                                    if atlas.index == 1 {
                                        atlas.index=0;
                                        atlas.layout = source3.lay_out_idle.clone();
                                        *enemystate = EnemyState::Move;
                                    }
                                },
                            }
                            let direction = Vec3::new(dx, dy, 0.0).normalize();
                            transform.translation += direction * v.0;
                        }
                    },

                    EnemyType::DroneVulcan => {

                        if dx * patrol_state.direction < 0.0 {
                            patrol_state.direction = -1.0 * patrol_state.direction;
                        }

                        if distance <= ENEMY_FIRE {
                            match *enemystate{
                                EnemyState::Idea | EnemyState::Move => { 
                                    if atlas.index == 4{
                                        atlas.index = 0;
                                        atlas.layout = source2.lay_out_fire_start.clone();
                                        *enemystate = EnemyState::FireStart;
                                    }
                                },
                                EnemyState::FireStart => {
                                    if atlas.index == 2{
                                        atlas.index = 0;
                                        atlas.layout = source2.lay_out_fire_loop.clone();
                                        *enemystate = EnemyState::FireLoop;
                                    }
                                },
                                EnemyState::FireLoop => {},
                                EnemyState::FireEnd => { 
                                    if atlas.index == 1{
                                        atlas.index = 0;
                                        atlas.layout = source2.lay_out_fire_start.clone();
                                        *enemystate = EnemyState::FireStart;
                                    }
                                },
                            }
                            //fire
                        } else {
                            match *enemystate{
                                EnemyState::Move => {},
                                EnemyState::Idea => { 
                                    *enemystate = EnemyState::Move;
                                },
                                EnemyState::FireLoop => { 
                                    if atlas.index == 2 {
                                        atlas.index=0;
                                        atlas.layout = source2.lay_out_fire_end.clone();
                                        *enemystate = EnemyState::FireEnd;
                                    }
                                },
                                EnemyState::FireStart => { 
                                    if atlas.index == 2 {
                                        atlas.index=0;
                                        atlas.layout = source2.lay_out_fire_end.clone();
                                        *enemystate = EnemyState::FireEnd;
                                    }
                                },
                                EnemyState::FireEnd => {
                                    if atlas.index == 1 {
                                        atlas.index=0;
                                        atlas.layout = source2.lay_out_idle.clone();
                                        *enemystate = EnemyState::Move;
                                    }
                                },
                            }
                            let direction = Vec3::new(dx, dy, 0.0).normalize();
                            //if {
                            // transform.translation += direction * v.0;
                            // } else if {
                            //     transform.translation.x += direction.x * v.0;
                            // } else if {
                            //     transform.translation.y += direction.y * v.0;
                            // } else {}
                        }
                    },

                    EnemyType::Sweeper => {

                        if dx * patrol_state.direction < 0.0 {
                            patrol_state.direction = -1.0 * patrol_state.direction;
                        }

                        if dx.abs() <= ENEMY_ATTACK {
                            match *enemystate{
                                EnemyState::Move => {
                                    if atlas.index == 13 {
                                        atlas.index = 0;
                                        atlas.layout = source1.lay_out_attack.clone();
                                        *enemystate = EnemyState::FireLoop;
                                    }
                                },
                                EnemyState::Idea => {
                                    atlas.index=0;
                                    atlas.layout = source1.lay_out_attack.clone();
                                    *enemystate = EnemyState::FireLoop;
                                },
                                EnemyState::FireLoop => {},
                                EnemyState::FireStart => {},
                                EnemyState::FireEnd => {},
                            }
                            //fire
                        } else {
                            match *enemystate{
                                EnemyState::Move => {},
                                EnemyState::Idea => {
                                    atlas.index=0;
                                    atlas.layout = source1.lay_out_move.clone();
                                    *enemystate = EnemyState::Move;
                                },
                                EnemyState::FireLoop => {
                                    if atlas.index == 12 {
                                        atlas.index=0;
                                        atlas.layout = source1.lay_out_move.clone();
                                        *enemystate = EnemyState::Move;
                                    }
                                },
                                EnemyState::FireStart => {},
                                EnemyState::FireEnd => {},
                            }
                            let direction = Vec3::new(dx, 0.0, 0.0).normalize();
                            //if {//侧面碰撞检测
                            transform.translation += direction * ENEMY_SPEED;
                            //} else {}
                        }
                        //下方碰撞检测
                        // if {
                        //     v.0 -= PLAYER_GRAVITY;
                        //     transform.translation.y -= v.0;
                        // }else{
                        //     v.0=0.0;
                        // }
                    }
                }
            } else {//巡逻
                match *enemystate{
                    EnemyState::Move => {},
                    EnemyState::Idea => { 
                        atlas.index = 0;
                        match *enemytype {
                            EnemyType::Sweeper => {
                                atlas.layout = source1.lay_out_move.clone();
                            },
                            EnemyType::DroneMissile => {    
                                atlas.layout = source3.lay_out_idle.clone();
                            },
                            EnemyType::DroneVulcan => {
                                atlas.layout = source2.lay_out_idle.clone();
                            },
                        }
                        *enemystate = EnemyState::Move;
                    },
                    EnemyState::FireLoop => { 
                        match *enemytype {
                            EnemyType::DroneMissile => {
                                if atlas.index == 4 {
                                    atlas.index = 0;
                                    atlas.layout = source3.lay_out_fire_end.clone();
                                    *enemystate = EnemyState::FireEnd;
                                }
                            },
                            EnemyType::DroneVulcan => {
                                if atlas.index == 2 {
                                    atlas.index = 0;
                                    atlas.layout = source2.lay_out_fire_end.clone();
                                    *enemystate = EnemyState::FireEnd;
                                }
                            },
                            EnemyType::Sweeper =>{
                                if atlas.index == 12 {
                                    atlas.index = 0;
                                    atlas.layout = source1.lay_out_move.clone();
                                    *enemystate = EnemyState::Move;
                                }
                            },
                        }
                    },
                    EnemyState::FireStart => { 
                        if atlas.index == 2 {
                            atlas.index = 0;
                            match enemytype {
                                EnemyType::DroneMissile =>atlas.layout = source3.lay_out_fire_end.clone(),
                                EnemyType::DroneVulcan =>atlas.layout = source2.lay_out_fire_end.clone(),
                                _=>{},                             
                            }
                            *enemystate = EnemyState::FireEnd;
                        }
                    },
                    EnemyState::FireEnd => { 
                        if atlas.index == 1 {
                            atlas.index = 0;
                            match enemytype {
                                EnemyType::DroneMissile =>atlas.layout = source3.lay_out_idle.clone(),
                                EnemyType::DroneVulcan =>atlas.layout = source2.lay_out_idle.clone(),
                                _=>{},                             
                            }
                            *enemystate = EnemyState::Move;
                        }
                    },
                }
                
                patrol_state.timer.tick(time.delta());
                if patrol_state.timer.elapsed() >= patrol_state.patrol_duration {
                    patrol_state.direction = -1.0 * patrol_state.direction;
                    patrol_state.timer.reset();
                }
                
                transform.translation.x += patrol_state.direction * ENEMY_SPEED;

                //侧面碰撞检测
                // if {
                //     transform.translation.x += patrol_state.direction * ENEMY_SPEED;
                // }
                match enemytype{
                    EnemyType::Sweeper=>{
                        //下方碰撞检测
                        // if {
                        //     v.0 -= PLAYER_GRAVITY;
                        //     transform.translation.y -= v.0;
                        // }else{
                        //     v.0=0.0;
                        // }
                    },
                    _=>{},
                };
            }

        }
    }
}

fn handle_enemy_animation(
    mut enemy_query: Query<(&mut Sprite, & EnemyState, & EnemyType), With<Enemy>>,
    source1: Res<GlobalSweeperTextureAtlas>,
    source2: Res<GlobalDroneVulcanTextureAtlas>,
    source3: Res<GlobalDroneMissileTextureAtlas>,
) {
    for (mut enemy, enemystate, enemytype) in enemy_query.iter_mut() {
        match enemytype {
            EnemyType::Sweeper =>{
                match enemystate {
                    EnemyState::Idea => { enemy.image = source1.image_move.clone();},
                    EnemyState::Move => { enemy.image = source1.image_move.clone();},
                    EnemyState::FireLoop => { enemy.image = source1.image_attack.clone();},
                    EnemyState::FireEnd | EnemyState::FireStart => { },
                }
            },
            EnemyType::DroneVulcan => {
                match enemystate {
                    EnemyState::Move | EnemyState::Idea => {enemy.image = source2.image_idle.clone();},
                    EnemyState::FireStart => {enemy.image = source2.image_fire_start.clone();},
                    EnemyState::FireLoop => {enemy.image = source2.image_fire_loop.clone();},
                    EnemyState::FireEnd => {enemy.image = source2.image_fire_end.clone();},
                }
            },
            EnemyType::DroneMissile => {
                match enemystate {
                    EnemyState::Move | EnemyState::Idea => {enemy.image = source3.image_idle.clone();},
                    EnemyState::FireStart => {enemy.image = source3.image_fire_start.clone();},
                    EnemyState::FireLoop => {enemy.image = source3.image_fire_loop.clone();},
                    EnemyState::FireEnd => {enemy.image = source3.image_fire_end.clone();},
                }
            },
        }
    }
}

// fn handle_enemy_fire(
//     mut enemy_query : Query<(& Sprite, & Transform, & EnemyState, & EnemyType),(With<Enemy>,Without<Character>)>,
//     mut player_query : Query<& Transform, (With<Character>, Without<Enemy>)>,
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
//     source: Res<GlobalEnemyBulletTextureAtlas>,
// ) {
//     if enemy_query.is_empty() {
//         return;
//     }
//     if player_query.is_empty() {
//         return;
//     }
//     let player_transform =player_query.single_mut();
//     for (enemy, enemy_transform, enemystate, enemytype) in enemy_query.iter_mut() {
//         match enemystate {
//             EnemyState::FireLoop => {
//                 match enemytype {
//                     EnemyType::Sweeper => {return;},
//                     EnemyType::DroneMissile => {
//                         if let Some(atlas) = &enemy.texture_atlas {
//                             if atlas.index == 0 {
//                                 commands.spawn( (
//                                     Sprite {
//                                         image: source.image1.clone(),
//                                         texture_atlas: Some(TextureAtlas {
//                                             layout: source.layout1.clone(),
//                                             index: 0,
//                                         }),
//                                         ..Default::default()
//                                     },
//                                     Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y, 31.0)),
//                                     Enemy_Bullet,
//                                     )
//                                 );
//                             }
//                         }
//                     },
//                     EnemyType::DroneVulcan => {
//                         if let Some(atlas) = &enemy.texture_atlas {
//                             if atlas.index == 1 {
//                                 commands.spawn( (
//                                     Sprite {
//                                         image: source.image2.clone(),
//                                         texture_atlas: Some(TextureAtlas {
//                                             layout: source.layout2.clone(),
//                                             index: 0,
//                                         }),
//                                         ..Default::default()
//                                     },
//                                     Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y, 31.0)),
//                                     Enemy_Bullet,
//                                     )
//                                 );
//                             }
//                         }
//                     },
//                 }
//             },
//             _=> {return;},
//         }
//     }
// }

fn handle_enemy_death(
     
) {

}

fn handle_enemy_hurt(
     
) {

}