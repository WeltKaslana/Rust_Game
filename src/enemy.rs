use bevy::transform;
use bevy::{dev_tools::states::*, prelude::*, time::Stopwatch};

use crate::gun::BulletDamage;
use crate::{
    gamestate::*,
    configs::*, 
    character::*, 
    gun::{BulletHit, Bullet, GunState, Gun},
    boss::{Boss, BossComponent},
    room::{Map, EnemyBorn},
};
use crate::*;
use rand::Rng;
use character::AnimationConfig;
use bevy_rapier2d::prelude::*;
use std::time::Duration;


pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyDeathEffect;

#[derive(Component)]
pub enum EnemyBullet {
    DroneMissile,
    DroneVulcan,
    UnknownGuardian,
} 

#[derive(Event)]
pub struct BaseSetupEvent;

#[derive(Component)]
pub enum Idleflag {
    Idle,
    Patrol,
}

#[derive(Component)]
pub enum Fireflag {
    Fire,
    Done,
}

#[derive(Component)]
pub struct BulletDirection {
    pub x : f32,
    pub y : f32,
}

#[derive(Component)]
pub enum EnemyType {
    Sweeper,
    DroneMissile,
    DroneVulcan,
    UnknownGuardianTypeF,
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
    pub directionx: f32,
    pub directiony: f32,
    pub timer1: Stopwatch,
    pub timer2: Stopwatch,
    pub patrol_duration: Duration,
}

#[derive(Component)]
pub struct EnemybornPoint;

#[derive(Component)]
pub struct Enemyterm(pub u8);

#[derive(Component)]
pub struct Enemybornduration{
    pub timer: Stopwatch,
    pub duration: Duration,
}

#[derive(Component)]
pub struct Enemybornflag(pub bool);

#[derive(Event)]
pub struct EnemyDeathEvent(pub Vec2);


impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EnemyDeathEvent>()
            .add_systems(
                Update,
                    (
                        handle_enemy_move,
                        handle_enemy_animation,
                        handle_enemy_fire,
                        handle_sweeper_hit,
                        handle_bullet_move,
                        handle_enemy_death,
                        handle_enemy_bullet_collision_events,
                        handle_enemy_hurt_collision_events,
                        handle_enemy_hurt_collision_events_special,
                        // handle_enemy_bron,
                ).run_if(in_state(InGameState::Running))
            )
            .add_event::<BaseSetupEvent>()
            ;
    }
}

// fn setup_enemy (
//     source: Res<GlobalEnemyTextureAtlas>,
//     mut commands: Commands,
// ) {
//     set_enemy(2,Vec2::new(0.0, 20.0), &mut commands, &source);
// }

pub fn set_enemy(
    id : u8,
    loc : Vec2,
    commands: &mut Commands,
    source: &Res<GlobalEnemyTextureAtlas>,
    mut score: &ResMut<ScoreResource>,
) {
    let mut rng = rand::rng();
    let random_index = rng.random_range(0..4);
    let mut x =random_index;

    let mut xishu:f32 =score.map_index as f32;
    xishu = 1.0 + xishu/100.0 * 5.0;

    match id {
        0 => { },//随机产生敌人
        1 => {x = 0;},//产生Sweeper
        2 => {x = 1;},//产生DroneMissile
        3 => {x = 2;},//产生DroneVulcan
        4 => {x = 3;},//产生UnknownGuardian_TypeF
        _ => unreachable!(),
    }
    let random_enemy = match x {
        0 => EnemyType::Sweeper,
        1 => EnemyType::DroneMissile,
        2 => EnemyType::DroneVulcan,
        3 => EnemyType::UnknownGuardianTypeF,
        _ => unreachable!(),
    };
    
    let patrol_duration = Duration::from_millis(500); // 巡逻持续时间，可根据需要调整

    match random_enemy{
        EnemyType::Sweeper=>{
            let collider_box = vec![
                Vec2::new(-9.0,4.0),
                Vec2::new(-9.0,-18.0),
                Vec2::new(9.0,4.0),
                Vec2::new(9.0,-18.0)];
            let mut enemy_entity =
            commands.spawn( (
                Sprite {
                    image: source.image_sweeper_idle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source.layout_sweeper_idle.clone(),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(loc.x, loc.y, -50.0)),
                EnemyState::default(),
                Enemy,
                EnemyType::Sweeper,
                Fireflag::Fire,
                Health(ENEMY_HEALTH * xishu),
                //Velocity(ENEMY_SPEED),
                AnimationConfig::new(13),
                PatrolState {
                    directionx: 0.0,
                    directiony: 0.0,
                    timer1: Stopwatch::new(),
                    timer2: Stopwatch::new(),
                    patrol_duration,
                },
                Idleflag::Patrol,
                // Sensor,
                RigidBody::Dynamic,
                GravityScale(0.0),
                Collider::convex_hull(&collider_box).unwrap(),
                LockedAxes::ROTATION_LOCKED,//防止旋转
                ActiveEvents::COLLISION_EVENTS,
                )
            );
            enemy_entity.insert(KinematicCharacterController {
                filter_groups: Some(CollisionGroups::new(Group::GROUP_3, Group::GROUP_4)),
                ..Default::default()
            });
            enemy_entity.insert((
                ColliderMassProperties::Mass(150.0),
                CollisionGroups::new(Group::GROUP_3, Group::GROUP_4),
            ));
            enemy_entity.insert(Map);
        },
        EnemyType::DroneMissile=>{
            let mut enemy_entity =
            commands.spawn( (
                Sprite {
                    image: source.image_missile_idle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source.layout_missile_idle.clone(),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(loc.x, loc.y, -50.0)),
                EnemyState::default(),
                Enemy,
                EnemyType::DroneMissile,
                Fireflag::Fire,
                Health(ENEMY_HEALTH * xishu),
                //Velocity(ENEMY_SPEED),
                AnimationConfig::new(9),
                PatrolState {
                    directionx: 0.0,
                    directiony: 0.0,
                    timer1: Stopwatch::new(),
                    timer2: Stopwatch::new(),
                    patrol_duration,
                },
                Idleflag::Patrol,
                // Sensor,
                RigidBody::Dynamic,
                GravityScale(0.0),
                Collider::cuboid(10.0, 10.0),
                LockedAxes::ROTATION_LOCKED,//防止旋转
                ActiveEvents::COLLISION_EVENTS,
                )
            );
            enemy_entity.insert(KinematicCharacterController {
                filter_groups: Some(CollisionGroups::new(Group::GROUP_3, Group::GROUP_4)),
                ..Default::default()
            });
            enemy_entity.insert((
                ColliderMassProperties::Mass(150.0),
                CollisionGroups::new(Group::GROUP_3, Group::GROUP_4),
            ));
            enemy_entity.insert(Map);
        },
        EnemyType::DroneVulcan=>{
            let mut enemy_entity =
            commands.spawn( (
                Sprite {
                    image: source.image_vulcan_idle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source.layout_vulcan_idle.clone(),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(loc.x, loc.y, -50.0)),
                EnemyState::default(),
                Enemy,
                EnemyType::DroneVulcan,
                Fireflag::Fire,
                Health(ENEMY_HEALTH * xishu),
                //Velocity(ENEMY_SPEED),
                AnimationConfig::new(9),
                PatrolState {
                    directionx: 0.0,
                    directiony: 0.0,
                    timer1: Stopwatch::new(),
                    timer2: Stopwatch::new(),
                    patrol_duration,
                },
                Idleflag::Patrol,
                // Sensor,
                RigidBody::Dynamic,
                GravityScale(0.0),
                Collider::cuboid(10.0, 10.0),
                LockedAxes::ROTATION_LOCKED,//防止旋转
                ActiveEvents::COLLISION_EVENTS,
                )
            );
            enemy_entity.insert(KinematicCharacterController {
                filter_groups: Some(CollisionGroups::new(Group::GROUP_3, Group::GROUP_4)),
                ..Default::default()
            });
            enemy_entity.insert((
                ColliderMassProperties::Mass(150.0),
                CollisionGroups::new(Group::GROUP_3, Group::GROUP_4),
            ));
            enemy_entity.insert(Map);
        },
        EnemyType::UnknownGuardianTypeF=>{
            let mut enemy_entity =
            commands.spawn( (
                Sprite {
                    image: source.image_unknown_idle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source.layout_unknown_idle.clone(),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(loc.x, loc.y, -50.0)),
                EnemyState::default(),
                Enemy,
                EnemyType::UnknownGuardianTypeF,
                Fireflag::Fire,
                Health(ENEMY_HEALTH * xishu),
                //Velocity(ENEMY_SPEED),
                AnimationConfig::new(9),
                PatrolState {
                    directionx: 0.0,
                    directiony: 0.0,
                    timer1: Stopwatch::new(),
                    timer2: Stopwatch::new(),
                    patrol_duration,
                },
                Idleflag::Patrol,
                // Sensor,
                RigidBody::Dynamic,
                GravityScale(0.0),
                Collider::cuboid(10.0, 10.0),
                LockedAxes::ROTATION_LOCKED,//防止旋转
                ActiveEvents::COLLISION_EVENTS,
                )
            );
            enemy_entity.insert(KinematicCharacterController {
                filter_groups: Some(CollisionGroups::new(Group::GROUP_3, Group::GROUP_4)),
                ..Default::default()
            });
            enemy_entity.insert((
                ColliderMassProperties::Mass(150.0),
                CollisionGroups::new(Group::GROUP_3, Group::GROUP_4),
            ));
            enemy_entity.insert(Map);
        },
    }
}

fn handle_enemy_move(
    mut player_query: Query< & Transform,(With<Character>,Without<Enemy>)>,
    mut enemy_query: Query<(
        &mut Sprite, 
        &mut Transform, 
        &mut EnemyState, 
        //& Velocity, 
        & EnemyType, 
        &mut PatrolState,
        &mut Idleflag,
        &mut KinematicCharacterController
        ), (With<Enemy>,Without<Character>)>,
    time: Res<Time>, 
) {
    if player_query.is_empty() {
        return;
    }
    if enemy_query.is_empty() {
        return;
    }

    let player = player_query.single_mut();
    
    for (   mut enemy, 
            transform, 
            mut enemystate,
            //v,
            enemytype, 
            mut patrol_state,
            mut flag,
            mut controller) in enemy_query.iter_mut(){
        let dx = player.translation.x - transform.translation.x;
        let dy = player.translation.y - transform.translation.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if let Some(atlas) = &mut enemy.texture_atlas {
            if distance <= ENEMY_ALARM {

                patrol_state.directionx = dx;
                patrol_state.directiony = dy;

                match enemytype {
                    EnemyType::DroneMissile => {
                        
                        if dx >= 0.0 {
                            patrol_state.directionx -= 50.0;
                        }else {
                            patrol_state.directionx += 50.0;
                        }
                        patrol_state.directiony += 50.0;
                        

                        if distance <= ENEMY_FIRE && transform.translation.y >= player.translation.y + 20.0 && dx.abs() >= 30.0 {
                            
                            if dx * patrol_state.directionx < 0.0 {
                                patrol_state.directionx = -1.0 * patrol_state.directionx;
                            }

                            match *enemystate{
                                EnemyState::Idea | EnemyState::Move => { 
                                    if atlas.index == 4{
                                        atlas.index = 0;
                                        *enemystate = EnemyState::FireStart;
                                    }
                                },
                                EnemyState::FireStart => {
                                    if atlas.index == 2{
                                        atlas.index = 0;
                                        *enemystate = EnemyState::FireLoop;
                                    }
                                },
                                EnemyState::FireLoop => {},
                                EnemyState::FireEnd => { 
                                    if atlas.index == 1{
                                        atlas.index = 0;
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
                                        *enemystate = EnemyState::FireEnd;
                                    }
                                },
                                EnemyState::FireStart => { 
                                    if atlas.index == 2 {
                                        atlas.index=0;
                                        *enemystate = EnemyState::FireEnd;
                                    }
                                },
                                EnemyState::FireEnd => {
                                    if atlas.index == 1 {
                                        atlas.index=0;
                                        *enemystate = EnemyState::Move;
                                    }
                                },
                            }
                            
                            // let direction = Vec3::new(dx, dy, 0.0).normalize();
                            // transform.translation += direction * v.0;
                            let direction = Vec2::new(patrol_state.directionx,patrol_state.directiony).normalize();
                            controller.translation = Some(direction.normalize_or_zero().clone() * ENEMY_SPEED);

                        }
                    },

                    EnemyType::DroneVulcan => {
                        
                        if dx >= 0.0 {
                            patrol_state.directionx -= 50.0;
                        }else {
                            patrol_state.directionx += 50.0;
                        }
                        patrol_state.directiony += 50.0;
                        
                        if distance <= ENEMY_FIRE && transform.translation.y >= player.translation.y + 20.0 && dx.abs() >= 30.0 {

                            if dx * patrol_state.directionx < 0.0 {
                                patrol_state.directionx = -1.0 * patrol_state.directionx;
                            }

                            match *enemystate{
                                EnemyState::Idea | EnemyState::Move => { 
                                    if atlas.index == 4{
                                        atlas.index = 0;
                                        *enemystate = EnemyState::FireStart;
                                    }
                                },
                                EnemyState::FireStart => {
                                    if atlas.index == 2{
                                        atlas.index = 0;
                                        *enemystate = EnemyState::FireLoop;
                                    }
                                },
                                EnemyState::FireLoop => {},
                                EnemyState::FireEnd => { 
                                    if atlas.index == 1{
                                        atlas.index = 0;
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
                                        *enemystate = EnemyState::FireEnd;
                                    }
                                },
                                EnemyState::FireStart => { 
                                    if atlas.index == 2 {
                                        atlas.index=0;
                                        *enemystate = EnemyState::FireEnd;
                                    }
                                },
                                EnemyState::FireEnd => {
                                    if atlas.index == 1 {
                                        atlas.index=0;
                                        *enemystate = EnemyState::Move;
                                    }
                                },
                            }
                            
                            // let direction = Vec3::new(dx, dy, 0.0).normalize();
                            // transform.translation += direction * v.0;
                            let direction = Vec2::new(patrol_state.directionx,patrol_state.directiony).normalize();
                            controller.translation = Some(direction.normalize_or_zero().clone() * ENEMY_SPEED);
                        }
                    },

                    EnemyType::Sweeper => {

                        if dx >= 0.0 {
                            patrol_state.directionx -= 20.0;
                        }else {
                            patrol_state.directionx += 20.0;
                        }
                        
                        if dx * patrol_state.directionx < 0.0 {
                            patrol_state.directionx = -1.0 * patrol_state.directionx;
                        }

                        if dx.abs() <= ENEMY_ATTACK && dy <= ENEMY_ATTACK-50.0 && dy >= 25.0-ENEMY_ATTACK {
                            match *enemystate{
                                EnemyState::Move => {
                                    if atlas.index == 13 {
                                        atlas.index = 0;
                                        *enemystate = EnemyState::FireLoop;
                                    }
                                },
                                EnemyState::Idea => {
                                    atlas.index=0;
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
                                    *enemystate = EnemyState::Move;
                                },
                                EnemyState::FireLoop => {
                                    if atlas.index == 12 {
                                        atlas.index=0;
                                        *enemystate = EnemyState::Move;
                                    }
                                },
                                EnemyState::FireStart => {},
                                EnemyState::FireEnd => {},
                            }
                            
                            // let direction = Vec3::new(dx, dy, 0.0).normalize();
                            // transform.translation += direction * ENEMY_SPEED;
                            let direction = Vec2::new(dx,dy).normalize();
                            controller.translation = Some(direction.normalize_or_zero().clone() * ENEMY_SPEED);
                        }
                    },

                    EnemyType::UnknownGuardianTypeF => {

                        if dx >= 0.0 {
                            patrol_state.directionx -= 50.0;
                        }else {
                            patrol_state.directionx += 50.0;
                        }

                        if distance <= ENEMY_FIRE && dx.abs() >= 30.0 {
                            
                            if dx * patrol_state.directionx < 0.0 {
                                patrol_state.directionx = -1.0 * patrol_state.directionx;
                            }

                            match *enemystate{
                                EnemyState::Move => { 
                                    if atlas.index == 8{
                                        atlas.index = 0;
                                        *enemystate = EnemyState::FireLoop;
                                    }
                                },
                                EnemyState::Idea => {
                                    atlas.index=0;
                                    *enemystate = EnemyState::FireLoop;
                                },
                                _=> { },
                            }
                            //fire
                        } else {
                            match *enemystate{
                                EnemyState::Idea => { 
                                    *enemystate = EnemyState::Move;
                                },
                                EnemyState::FireLoop => { 
                                    if atlas.index == 7 {
                                        atlas.index=0;
                                        *enemystate = EnemyState::Move;
                                    }
                                },
                                _=> { },
                            }
                            
                            let direction = Vec2::new(patrol_state.directionx,patrol_state.directiony).normalize();
                            controller.translation = Some(direction.normalize_or_zero().clone() * ENEMY_SPEED);

                        }
                    },
                }
                
                *flag = Idleflag::Patrol;
                patrol_state.timer1.reset();
                patrol_state.timer2.reset();

            } else {//巡逻
                let mut rng = rand::rng();
                match *flag {
                    Idleflag::Patrol => {
                        match *enemystate{
                            EnemyState::Move => {},
                            EnemyState::Idea => { 
                                atlas.index = 0;
                                *enemystate = EnemyState::Move;
                            },
                            EnemyState::FireLoop => { 
                                match *enemytype {
                                    EnemyType::DroneMissile => {
                                        if atlas.index == 4 {
                                            atlas.index = 0;
                                            *enemystate = EnemyState::FireEnd;
                                        }
                                    },
                                    EnemyType::DroneVulcan => {
                                        if atlas.index == 2 {
                                            atlas.index = 0;
                                            *enemystate = EnemyState::FireEnd;
                                        }
                                    },
                                    EnemyType::Sweeper =>{
                                        if atlas.index == 12 {
                                            atlas.index = 0;
                                            *enemystate = EnemyState::Move;
                                        }
                                    },
                                    EnemyType::UnknownGuardianTypeF => {
                                        if atlas.index == 7 {
                                            atlas.index = 0;
                                            *enemystate = EnemyState::Move;
                                        }
                                    },
                                }
                            },
                            EnemyState::FireStart => { 
                                if atlas.index == 2 {
                                    atlas.index = 0;
                                    *enemystate = EnemyState::FireEnd;
                                }
                            },
                            EnemyState::FireEnd => { 
                                if atlas.index == 1 {
                                    atlas.index = 0;
                                    *enemystate = EnemyState::Move;
                                }
                            },
                        }
                        
                        patrol_state.timer1.tick(time.delta());

                        if patrol_state.timer1.elapsed() >= patrol_state.patrol_duration {
                            patrol_state.timer1.reset();

                            let random_x = rng.random_range(-1.0..=1.0);
                            let random_y = rng.random_range(-1.0..=1.0);
                            patrol_state.directionx = random_x as f32;
                            patrol_state.directiony = random_y as f32;

                            atlas.index = 0;
                            *enemystate = EnemyState::Idea;
                            *flag = Idleflag::Idle;
                        }
                        //println!("patrol");
                        
                        // transform.translation.x += patrol_state.direction * ENEMY_SPEED;
                        let direction = Vec2::new(patrol_state.directionx, patrol_state.directiony);
                        controller.translation = Some(direction.normalize_or_zero().clone() * ENEMY_SPEED);
                    },
                    Idleflag::Idle=> {
                        patrol_state.timer2.tick(time.delta());
                        if patrol_state.timer2.elapsed() >= patrol_state.patrol_duration {
                            patrol_state.timer2.reset();

                            let random_x = rng.random_range(-1.0..=1.0);
                            let random_y = rng.random_range(-1.0..=1.0);
                            patrol_state.directionx = random_x as f32;
                            patrol_state.directiony = random_y as f32;

                            atlas.index = 0;
                            *flag = Idleflag::Patrol;
                        }
                        //println!("idle");
                    },
                };
            }
        }
    }
}

fn handle_enemy_animation(
    mut enemy_query: Query<(&mut Sprite, & EnemyState, & EnemyType), With<Enemy>>,
    source: Res<GlobalEnemyTextureAtlas>,
) {
    for (mut enemy, enemystate, enemytype) in enemy_query.iter_mut() {
        match enemytype {
            EnemyType::Sweeper =>{
                match enemystate {
                    EnemyState::Idea => { 
                        enemy.image = source.image_sweeper_idle.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_sweeper_idle.clone();
                        }
                    },
                    EnemyState::Move => { 
                        enemy.image = source.image_sweeper_move.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_sweeper_move.clone();
                        }
                    },
                    EnemyState::FireLoop => { 
                        enemy.image = source.image_sweeper_attack.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_sweeper_attack.clone();
                        }
                    },
                    EnemyState::FireEnd | EnemyState::FireStart => { },
                }
            },
            EnemyType::DroneVulcan => {
                match enemystate {
                    EnemyState::Move | EnemyState::Idea => {
                        enemy.image = source.image_vulcan_idle.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_vulcan_idle.clone();
                        }
                    },
                    EnemyState::FireStart => {
                        enemy.image = source.image_vulcan_fire_start.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_vulcan_fire_start.clone();
                        }
                    },
                    EnemyState::FireLoop => {
                        enemy.image = source.image_vulcan_fire_loop.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_vulcan_fire_loop.clone();
                        }
                    },
                    EnemyState::FireEnd => {
                        enemy.image = source.image_vulcan_fire_end.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_vulcan_fire_end.clone();
                        }
                    },
                }
            },
            EnemyType::DroneMissile => {
                match enemystate {
                    EnemyState::Move | EnemyState::Idea => {
                        enemy.image = source.image_missile_idle.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_missile_idle.clone();
                        }
                    },
                    EnemyState::FireStart => {
                        enemy.image = source.image_missile_fire_start.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_missile_fire_start.clone();
                        }
                    },
                    EnemyState::FireLoop => {
                        enemy.image = source.image_missile_fire_loop.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_missile_fire_loop.clone();
                        }
                    },
                    EnemyState::FireEnd => {
                        enemy.image = source.image_missile_fire_end.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_missile_fire_end.clone();
                        }
                    },
                }
            },
            EnemyType::UnknownGuardianTypeF => {
                match enemystate {
                    EnemyState::Idea => { 
                        enemy.image = source.image_unknown_idle.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_unknown_idle.clone();
                        }
                    },
                    EnemyState::Move => { 
                        enemy.image = source.image_unknown_move.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_unknown_move.clone();
                        }
                    },
                    EnemyState::FireLoop => { 
                        enemy.image = source.image_unknown_attack.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source.layout_unknown_attack.clone();
                        }
                    },
                    EnemyState::FireEnd | EnemyState::FireStart => { },
                }
            }
        }
    }
}

fn handle_enemy_fire(
    mut enemy_query : Query<(& Sprite, 
        & Transform, 
        & EnemyState, 
        & EnemyType,
        &mut Fireflag),(With<Enemy>,Without<Character>)>,
    mut player_query : Query<& Transform, (With<Character>, Without<Enemy>)>,
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    source: Res<GlobalEnemyTextureAtlas>,
) {
    if enemy_query.is_empty() {
        return;
    }
    if player_query.is_empty() {
        return;
    }
    let player_transform =player_query.single_mut();
    for (enemy, 
        enemy_transform, 
        enemystate, 
        enemytype,
        mut flag) in enemy_query.iter_mut() {
        
        let dx = player_transform.translation.x - enemy_transform.translation.x;
        let dy = player_transform.translation.y - enemy_transform.translation.y;
        match enemystate {
            EnemyState::FireLoop => {
                // println!("1");
                match enemytype {
                    EnemyType::Sweeper => { },
                    EnemyType::DroneMissile => {
                        if let Some(atlas) = &enemy.texture_atlas {
                            if atlas.index == 0 {
                                match *flag {
                                    Fireflag::Fire => {
                                        *flag = Fireflag::Done;
                                        commands.spawn( (
                                            Sprite {
                                                image: source.image_missile_bullet.clone(),
                                                texture_atlas: Some(TextureAtlas {
                                                    layout: source.layout_missile_bullet.clone(),
                                                    index: 0,
                                                }),
                                                ..Default::default()
                                            },
                                            Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y, 31.0)),
                                            EnemyBullet::DroneMissile,
                                            BulletDirection {
                                                x : dx,
                                                y : dy,},
                                            AnimationConfig::new(5),
                                            Sensor,
                                            RigidBody::Dynamic,
                                            GravityScale(0.0),
                                            Collider::cuboid(11.0, 5.0),
                                            ActiveEvents::COLLISION_EVENTS,
                                            Map,
                                            )
                                        );
                                    },
                                    Fireflag::Done => { }
                                }
                            }
                        }
                    },
                    EnemyType::DroneVulcan => {
                        if let Some(atlas) = &enemy.texture_atlas {
                            if atlas.index == 1 {
                                match *flag {
                                    Fireflag::Fire => {
                                        *flag = Fireflag::Done;
                                        commands.spawn( (
                                            Sprite {
                                                image: source.image_vulcan_bullet.clone(),
                                                texture_atlas: Some(TextureAtlas {
                                                    layout: source.layout_vulcan_bullet.clone(),
                                                    index: 0,
                                                }),
                                                ..Default::default()
                                            },
                                            Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y, 31.0)),
                                            EnemyBullet::DroneVulcan,
                                            BulletDirection {
                                                x : dx,
                                                y : dy,},
                                            AnimationConfig::new(5),
                                            Sensor,
                                            RigidBody::Dynamic,
                                            GravityScale(0.0),
                                            Collider::cuboid(6.0, 6.0),
                                            ActiveEvents::COLLISION_EVENTS,
                                            Map,
                                            )
                                        );
                                    },
                                    Fireflag::Done => { },
                                }
                            }
                        }
                    },
                    EnemyType::UnknownGuardianTypeF => {
                        if let Some(atlas) = &enemy.texture_atlas {
                            if atlas.index == 4 {
                                match *flag {
                                    Fireflag::Fire => {
                                        *flag = Fireflag::Done;
                                        commands.spawn( (
                                            Sprite {
                                                image: source.image_unknown_bullet.clone(),
                                                texture_atlas: Some(TextureAtlas {
                                                    layout: source.layout_unknown_bullet.clone(),
                                                    index: 0,
                                                }),
                                                ..Default::default()
                                            },
                                            Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y, 31.0)),
                                            EnemyBullet::UnknownGuardian,
                                            BulletDirection {
                                                x : dx,
                                                y : dy,},
                                            AnimationConfig::new(5),
                                            Sensor,
                                            RigidBody::Dynamic,
                                            GravityScale(0.0),
                                            Collider::cuboid(6.0, 6.0),
                                            ActiveEvents::COLLISION_EVENTS,
                                            Map
                                            )
                                        );
                                    },
                                    Fireflag::Done => { }
                                }
                            }
                        }
                    }
                }
            },
            _=> {},
        }
    }
}

fn handle_bullet_move(
    mut player_query : Query<& Transform, (With<Character>, Without<EnemyBullet>)>,
    mut bullet_query : Query<(
        &mut Transform,
        &EnemyBullet,
        &mut BulletDirection), (With<EnemyBullet>, Without<Character>)>,
) {
    if player_query.is_empty() {
        return;
    }
    if bullet_query.is_empty() {
        return;
    }
    let playertransform = player_query.single_mut();
    for (mut bullettransform,
        bullertype,
        mut BulletDirection) in &mut bullet_query.iter_mut() {
        
        match bullertype {
            EnemyBullet::DroneMissile => {
                let dx = playertransform.translation.x - bullettransform.translation.x;
                let dy = playertransform.translation.y - bullettransform.translation.y;
                let add_speed = Vec2::new(dx, dy).normalize();
                BulletDirection.x += add_speed.x * ENEMY_BULLET_SPEED / 2.0;
                BulletDirection.y += add_speed.y * ENEMY_BULLET_SPEED / 2.0;

                let angle = (BulletDirection.y).atan2(BulletDirection.x);
                bullettransform.rotation = Quat::from_rotation_z(angle);

            },
            _=> { }
        }
        let direction = Vec3::new(BulletDirection.x, BulletDirection.y,0.0).normalize();
        bullettransform.translation += direction * ENEMY_BULLET_SPEED;
    }
}

fn handle_enemy_death(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, & Health, & Transform), With<Enemy>>,
    source: Res<GlobalEnemyTextureAtlas>,
    mut event: EventWriter<EnemyDeathEvent>
) {
    if enemy_query.is_empty() {
        return;
    }
    for (enemy,health, loc) in enemy_query.iter_mut() {
        if health.0 <= 0.0 {
            commands.entity(enemy).despawn();
            commands.spawn( (
                Sprite {
                    image: source.image_death.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source.layout_death.clone(),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(loc.translation.x, loc.translation.y, -50.0)),
                AnimationConfig::new(10),
                EnemyDeathEffect,
                Map,
            )
            );
            event.send(EnemyDeathEvent(Vec2::new(loc.translation.x, loc.translation.y)));
        }
    }
}

fn handle_enemy_hurt_collision_events(
    buff_query: Query<&Buff>,
    player_query: Query<(Entity, &BulletDamage), (With<Bullet>)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut enemy_query: Query<(Entity, &mut Health), (With<Enemy>, Without<Bullet>)>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    for collision_event in collision_events.read() {
        if player_query.is_empty() || enemy_query.is_empty() {
            return;
        }

        let buff = buff_query.single();
        let vulnerable = match source.id {
            // 不同子弹伤害不同，后续可以叠加dot之类的伤害
            2 => 3.0,
            _ => 1.0,
        };
        let bulletdamage = ((buff.4 - 1) as f32 * 0.25 + 1.0) * vulnerable;
        for (enemy, mut health) in &mut enemy_query.iter_mut() {
            match collision_event {
                CollisionEvent::Started(entity1,entity2, _) => {
                    if entity2.eq(&enemy) {
                        if let Ok((_, arisu)) = player_query.get(*entity1) {
                            // 还有arisu的光之剑伤害
                            health.0 -= BULLET_DAMAGE * bulletdamage * match arisu.0 {
                                x if x > 30.0 => (x * 0.25) * (1.0 + (buff.5 - 1) as f32 * 0.05),
                                x if x == 30.0 => 3.0 * (1.0 + (buff.5 - 1) as f32 * 0.05),
                                _ => 1.0
                            };
                        }
                    }
                    if entity1.eq(&enemy) {
                        if let Ok((_, arisu)) = player_query.get(*entity2) {
                            // 还有arisu的光之剑伤害
                            health.0 -= BULLET_DAMAGE * bulletdamage * match arisu.0 {
                                x if x > 30.0 => (x * 0.25) * (1.0 + (buff.5 - 1) as f32 * 0.05),
                                x if x == 30.0 => 3.0 * (1.0 + (buff.5 - 1) as f32 * 0.05),
                                _ => 1.0
                            };
                        }
                    }
                },
                CollisionEvent::Stopped(entity1, entity2, _) => { },
            }
        }
    }
}


fn handle_enemy_hurt_collision_events_special(
    buff_query: Query<&Buff>,
    grenade_query: Query<(Entity, &Transform), (With<Grenade>)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut enemy_query: Query<(Entity, &mut Health, &Transform), (With<Enemy>, Without<Bullet>, Without<Boss>)>,
    mut boss_query: Query<(Entity, &mut Health, &Transform), (With<Boss>, Without<BossComponent>, Without<Enemy>)>,
) {
    for collision_event in collision_events.read() {
        if grenade_query.is_empty() || enemy_query.is_empty() || buff_query.is_empty() {
            break;
        }
        let buff = buff_query.single();
        let damage = 1.0 + (buff.5 - 1) as f32 * 0.05;
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                // 手雷爆炸范围内的敌人都扣血
                if let Ok((_, transg)) = grenade_query.get(*entity1) {
                    for (_, mut health, trans) in &mut enemy_query.iter_mut()  {
                        if trans.translation.distance(transg.translation) < GRENADE_BOOM_RANGE {
                            health.0 -= BULLET_DAMAGE * 5.0 * damage;
                            println!("BOOM!");
                        }
                    }
                    for (_, mut health, trans) in &mut boss_query.iter_mut()  {
                        if trans.translation.distance(transg.translation) < GRENADE_BOOM_RANGE {
                            health.0 -= BULLET_DAMAGE * 5.0 * damage;
                            println!("BOOM!");
                        }
                    }
                }
                if let Ok((_, transg)) = grenade_query.get(*entity2) {
                    for (_, mut health, trans) in &mut enemy_query.iter_mut()  {
                        if trans.translation.distance(transg.translation) < GRENADE_BOOM_RANGE {
                            health.0 -= BULLET_DAMAGE * 5.0 * damage;
                            println!("BOOM!");
                        }
                    }
                    for (_, mut health, trans) in &mut boss_query.iter_mut()  {
                        if trans.translation.distance(transg.translation) < GRENADE_BOOM_RANGE {
                            health.0 -= BULLET_DAMAGE * 5.0 * damage;
                            println!("BOOM!");
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

fn handle_enemy_bullet_collision_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    enemy_query: Query<Entity, (With<Enemy>, Without<EnemyBullet>, Without<Boss>)>,
    enemy_bullet_query: Query<(Entity, & Transform), (With<EnemyBullet>, Without<Enemy>, Without<Boss>)>,
    boss_query: Query<Entity, (With<Boss>, Without<Enemy>, Without<EnemyBullet>)>,
    source: Res<GlobalEnemyTextureAtlas>,
) {
    if enemy_bullet_query.is_empty() {
        return;
    }
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1,entity2, _) => {
                for (bulletentity, transform) in enemy_bullet_query.iter() {
                    if entity1.eq(&bulletentity) {
                        if let Ok(b) = enemy_query.get(*entity2) {
                            continue;
                        }
                        if let Ok(b) = boss_query.get(*entity2) {
                            continue;
                        }
                        if let Ok(b) = enemy_bullet_query.get(*entity2) {
                            continue;
                        }
                        commands.entity(bulletentity).despawn();
                        commands.spawn((
                            Sprite {
                                image: source.image_gun_hit.clone(),
                                texture_atlas: Some(TextureAtlas {
                                    layout: source.layout_gun_hit.clone(),
                                    index: 0,
                                }),
                                ..default()
                            },
                            Transform {
                                translation: transform.translation.clone(),
                                scale: Vec3::splat(2.5),
                                ..default()
                            },
                            AnimationConfig::new(15),
                            BulletHit,
                            Map,
                        ));

                    } else if entity2.eq(&bulletentity) {
                        if let Ok(b) = enemy_query.get(*entity1) {
                            continue;
                        }
                        if let Ok(b) = boss_query.get(*entity1) {
                            continue;
                        }
                        if let Ok(b) = enemy_bullet_query.get(*entity1) {
                            continue;
                        }
                        commands.entity(bulletentity).despawn();
                        commands.spawn((
                            Sprite {
                                image: source.image_gun_hit.clone(),
                                texture_atlas: Some(TextureAtlas {
                                    layout: source.layout_gun_hit.clone(),
                                    index: 0,
                                }),
                                ..default()
                            },
                            Transform {
                                translation: transform.translation.clone(),
                                scale: Vec3::splat(2.5),
                                ..default()
                            },
                            AnimationConfig::new(15),
                            BulletHit,
                            Map,
                        ));
                    }
                }
            },
            CollisionEvent::Stopped(entity1, entity2, _) => { },
        }
    }
}

fn handle_sweeper_hit(
    mut player_query: Query<(
        &mut crate::character::Health,
        & Transform,
        & PlayerState
    ), (With<Character>, Without<Enemy>)>,
    mut enemy_query: Query<(
        & Sprite,
        & Transform,
        & EnemyType,
        & EnemyState,
        &mut Fireflag,
        & PatrolState,
    ), (With<Enemy>, Without<Character>)>,
    mut events: EventWriter<PlayerHurtEvent>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }
    let (mut health, playertransform, playerstate) = player_query.single_mut();
    for (enemy, enemytransform, enemytype, enemystate, mut flag, direction) in enemy_query.iter_mut() {
        match enemytype {
            EnemyType::Sweeper => {
                match enemystate {
                    EnemyState::FireLoop => {
                        if let Some(atlas) = &enemy.texture_atlas {
                            let dx = playertransform.translation.x - enemytransform.translation.x;
                            let dy = playertransform.translation.y - enemytransform.translation.y;
                            if (atlas.index >= 4 && atlas.index <= 5) || atlas.index == 8 || atlas.index == 10 || atlas.index == 12 {
                                if direction.directionx >= 0.0 {
                                    if dx >= 0.0 && dx.abs() <= ENEMY_ATTACK && dy <= ENEMY_ATTACK-50.0 && dy >= 25.0-ENEMY_ATTACK {
                                        match *flag {
                                            Fireflag::Fire => {
                                                match playerstate {
                                                    PlayerState::Dodge => { },
                                                    _=> { 
                                                        *flag = Fireflag::Done;
                                                        health.0 -=ENEMY_DAMAGE;
                                                        events.send(PlayerHurtEvent);
                                                    },
                                                }
                                            },
                                            Fireflag::Done => {continue;}
                                        }
                                    }
                                }else {
                                    if dx < 0.0 && dx.abs() <= ENEMY_ATTACK && dy <= ENEMY_ATTACK-50.0 && dy >= 25.0-ENEMY_ATTACK {
                                        match *flag {
                                            Fireflag::Fire => {
                                                match playerstate {
                                                    PlayerState::Dodge => { },
                                                    _=> { 
                                                        *flag = Fireflag::Done;
                                                        health.0 -= ENEMY_DAMAGE;
                                                        events.send(PlayerHurtEvent);
                                                    },
                                                }
                                            },
                                            Fireflag::Done => {continue;}
                                        }
                                    }
                                }
                            }
                            else {continue;}
                        }
                    },
                    _=> {continue;}
                }
            },
            _=> {continue;}
        }
    }
}