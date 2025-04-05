use bevy::math::vec3;
use bevy::reflect::Enum;
use bevy::transform;
use bevy::{dev_tools::states::*, prelude::*, time::Stopwatch};
use bevy_ecs_tiled::physics::collider;
use std::clone;
use std::{time::Duration};
use crate::{gamestate::GameState,
    configs::*,character::*};
use crate::*;
use rand::Rng;
use character::AnimationConfig;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub enum EnemyBullet {
    DroneMissile,
    DroneVulcan,
    UnknownGuardian,
} 

#[derive(Component)]
pub struct Health(pub f32);

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


impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), set_enemy)
            .add_systems(
                Update,
                    (
                        handle_enemy_move,
                        handle_enemy_animation,
                        handle_enemy_fire,
                        handle_bullet_move,
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
    let x=1;
    let random_enemy = match x {
        0 => EnemyType::Sweeper,
        1 => EnemyType::DroneMissile,
        2 => EnemyType::DroneVulcan,
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
                Fireflag::Done,
                Health(ENEMY_HEALTH),
                //Velocity(ENEMY_SPEED),
                AnimationConfig::new(20),
                PatrolState {
                    directionx: 0.0,
                    directiony: 0.0,
                    timer1: Stopwatch::new(),
                    timer2: Stopwatch::new(),
                    patrol_duration,
                },
                Idleflag::Patrol,
                Sensor,
                RigidBody::Dynamic,
                GravityScale(0.0),
                Collider::convex_hull(&collider_box).unwrap(),
                ActiveEvents::COLLISION_EVENTS,
                )
            );
            enemy_entity.insert(KinematicCharacterController {
                ..Default::default()
            });
        },
        EnemyType::DroneMissile=>{
            let mut enemy_entity =
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
                Fireflag::Fire,
                Health(ENEMY_HEALTH),
                //Velocity(ENEMY_SPEED),
                AnimationConfig::new(10),
                PatrolState {
                    directionx: 0.0,
                    directiony: 0.0,
                    timer1: Stopwatch::new(),
                    timer2: Stopwatch::new(),
                    patrol_duration,
                },
                Idleflag::Patrol,
                Sensor,
                RigidBody::Dynamic,
                GravityScale(0.0),
                Collider::cuboid(10.0, 10.0),
                )
            );
            enemy_entity.insert(KinematicCharacterController {
                ..Default::default()
            });
        },
        EnemyType::DroneVulcan=>{
            let mut enemy_entity =
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
                Fireflag::Fire,
                Health(ENEMY_HEALTH),
                //Velocity(ENEMY_SPEED),
                AnimationConfig::new(10),
                PatrolState {
                    directionx: 0.0,
                    directiony: 0.0,
                    timer1: Stopwatch::new(),
                    timer2: Stopwatch::new(),
                    patrol_duration,
                },
                Idleflag::Patrol,
                Sensor,
                RigidBody::Dynamic,
                GravityScale(0.0),
                Collider::cuboid(10.0, 10.0),
                )
            );
            enemy_entity.insert(KinematicCharacterController {
                ..Default::default()
            });
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
    //asset_server: Res<AssetServer>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    // source1: Res<GlobalSweeperTextureAtlas>,
    // source2: Res<GlobalDroneVulcanTextureAtlas>,
    // source3: Res<GlobalDroneMissileTextureAtlas>,
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
                    }
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
    source1: Res<GlobalSweeperTextureAtlas>,
    source2: Res<GlobalDroneVulcanTextureAtlas>,
    source3: Res<GlobalDroneMissileTextureAtlas>,
) {
    for (mut enemy, enemystate, enemytype) in enemy_query.iter_mut() {
        match enemytype {
            EnemyType::Sweeper =>{
                match enemystate {
                    EnemyState::Idea => { 
                        enemy.image = source1.image_move.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source1.lay_out_idle.clone();
                        }
                    },
                    EnemyState::Move => { 
                        enemy.image = source1.image_move.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source1.lay_out_move.clone();
                        }
                    },
                    EnemyState::FireLoop => { 
                        enemy.image = source1.image_attack.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source1.lay_out_attack.clone();
                        }
                    },
                    EnemyState::FireEnd | EnemyState::FireStart => { },
                }
            },
            EnemyType::DroneVulcan => {
                match enemystate {
                    EnemyState::Move | EnemyState::Idea => {
                        enemy.image = source2.image_idle.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source2.lay_out_idle.clone();
                        }
                    },
                    EnemyState::FireStart => {
                        enemy.image = source2.image_fire_start.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source2.lay_out_fire_start.clone();
                        }
                    },
                    EnemyState::FireLoop => {
                        enemy.image = source2.image_fire_loop.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source2.lay_out_fire_loop.clone();
                        }
                    },
                    EnemyState::FireEnd => {
                        enemy.image = source2.image_fire_end.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source2.lay_out_fire_end.clone();
                        }
                    },
                }
            },
            EnemyType::DroneMissile => {
                match enemystate {
                    EnemyState::Move | EnemyState::Idea => {
                        enemy.image = source3.image_idle.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source3.lay_out_idle.clone();
                        }
                    },
                    EnemyState::FireStart => {
                        enemy.image = source3.image_fire_start.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source3.lay_out_fire_start.clone();
                        }
                    },
                    EnemyState::FireLoop => {
                        enemy.image = source3.image_fire_loop.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source3.lay_out_fire_loop.clone();
                        }
                    },
                    EnemyState::FireEnd => {
                        enemy.image = source3.image_fire_end.clone();
                        if let Some(atlas) = &mut enemy.texture_atlas {
                            atlas.layout = source3.lay_out_fire_end.clone();
                        }
                    },
                }
            },
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
    source: Res<GlobalEnemyBulletTextureAtlas>,
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
                match enemytype {
                    EnemyType::Sweeper => {return;},
                    EnemyType::DroneMissile => {
                        if let Some(atlas) = &enemy.texture_atlas {
                            if atlas.index == 0 {
                                match *flag {
                                    Fireflag::Fire => {
                                        *flag = Fireflag::Done;
                                        commands.spawn( (
                                            Sprite {
                                                image: source.image_bullet_dronemissile.clone(),
                                                texture_atlas: Some(TextureAtlas {
                                                    layout: source.lay_out_bullet_dronemissile.clone(),
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
                                            KinematicCharacterController {
                                                ..Default::default()
                                            },
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
                                                image: source.image_bullet_dronevulcan.clone(),
                                                texture_atlas: Some(TextureAtlas {
                                                    layout: source.lay_out_bullet_dronevulcan.clone(),
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
                                            KinematicCharacterController {
                                                ..Default::default()
                                            },
                                            )
                                        );
                                    },
                                    Fireflag::Done => { },
                                }
                            }
                        }
                    },
                }
            },
            _=> {return;},
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
     
) {

}

fn handle_enemy_hurt(
     
) {

}