use bevy::math::vec3;
use bevy::{
    dev_tools::states::*, 
    prelude::*, 
    time::Stopwatch,
    ecs::world::DeferredWorld,};
use bevy_rapier2d::prelude::*;

use std::{time::Duration};
use crate::{
    gamestate::GameState,
    enemy::{
        EnemyBullet,
        Enemy,},
    gun::Gun,
};
use crate::*;
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Velocity(pub f32);
//用于判断当前player是否不在空中
#[derive(Component)]
pub struct Lastlocy(pub f32);
#[derive(Component)]
pub struct Lastvy(pub f32);

#[derive(Component)]
pub struct PlayerTimer(pub Stopwatch);

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Jump,
    Move,
    Jumpover,
    Dodge,
}

#[derive(Event)]
pub struct PlayerEnemyCollisionEvent;

#[derive(Event)]
pub struct PlayerRunEvent;

#[derive(Event)]
pub struct PlayerJumpEvent;

#[derive(Event)]
pub struct PlayerHurtEvent;

//定义角色动画帧率
#[derive(Component)]

pub struct AnimationConfig {
    pub fps2p: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(fps2p: u8) -> Self {
        Self {
            fps2p,
            frame_timer: Self::timer_from_fps(fps2p),
        }
    }

    pub fn timer_from_fps(fps2p: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps2p as f32)), TimerMode::Once)
    }
}


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_event::<PlayerEnemyCollisionEvent>()
            .add_systems(OnEnter(GameState::Home), setup_player)
            .add_systems(
            Update,
                (
                    handle_player_move2,
                    handle_player_skills,
                    // handle_play_bullet_collision_events,
            ).run_if(in_state(GameState::Home))
            )
            .add_systems(
                Update,
                    (
                        handle_player_death,
                        handle_player_move3,
                        handle_player_skills,
                        // handle_player_enemy_collision_events,
                        handle_player_bullet_collision_events
                ).run_if(in_state(GameState::InGame))
                )
            .add_systems(Update, log_transitions::<GameState>)
            ;
    }
}

fn setup_player(
    mut commands: Commands,
    source: Res<GlobalCharacterTextureAtlas>,
    asset_server: Res<AssetServer>,
) {
    let mut player = 
    commands.spawn( (Sprite {
        image: source.image_idle.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: source.lay_out_idle.clone(),
            index: 1,
        }),
        ..Default::default()
        },
        Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(-200.0, -200.0 + 5.0, 30.0)),
        AnimationConfig::new(13),
        PlayerState::default(),
        Character,
        // 血条
        Health(PLAYER_HEALTH),
        // //跳跃起始速度
        Velocity(PLAYER_JUMP_SPEED),
        //音效播放间隔计时器
        PlayerTimer(Stopwatch::default()),
        Collider::cuboid(9.0, 16.5),

        RigidBody::Fixed,

        ActiveEvents::COLLISION_EVENTS,
        Sensor,//不加这个部件的话碰撞就会产生实际碰撞效果，否则只会发送碰撞事件而无效果
        //后续可以为碰撞体分组
        // CollisionGroups::new(Group::GROUP_1, Group::GROUP_2),
        ));

        // //尝试插入运动部件
        player
            .insert(KinematicCharacterController {
                ..Default::default()
            });

    //插入碰撞组件和移动控制组件
    // player
    //     .insert((
    //         //碰撞
    //         Collider::cuboid(9.0, 18.0),
    //         RigidBody::Dynamic,
    //         LockedAxes::ROTATION_LOCKED,//防止旋转
    //     ));
    //白子的光环
    if source.id == 1 {
        player.with_child((Sprite {
            image: asset_server.load("Shiroko_Aura.png"),
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(0.0, 15.0, -1.0)),
            ));
    }
}

fn handle_player_move(
    mut commands: Commands,
    mut events: EventWriter<PlayerRunEvent>,
    mut events2: EventWriter<PlayerJumpEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(
        &mut Sprite, 
        &mut Transform, 
        &mut PlayerState, 
        &mut Velocity,
        Entity,
    ), With<Character>>,
    transform_query: Query<&Transform, Without<Character>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    if player_query.is_empty() {
        return;
    }

    //之后可以改为自定义的键位，数据存到configs中
    let (
        mut player, 
        mut transform, 
        mut player_state, 
        mut V, 
        entity,
    ) = player_query.single_mut();



    match *player_state {
        PlayerState::Dodge => {
            if player.flip_x{
                if  transform.translation.x - 10.0 > -520.0 {
                    transform.translation.x -= 10.0;
                }
                else {
                    transform.translation.x = -520.0;
                }
            }
            else {
                if  transform.translation.x + 10.0 < 520.0 {
                    transform.translation.x += 10.0;
                }
                else {
                    transform.translation.x = 520.0;
                }
            }
            return;
        },
        _ => {},
    }


    let jump = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::Space);
    let left = keyboard_input.pressed(KeyCode::KeyA);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let right = keyboard_input.pressed(KeyCode::KeyD);
    //到边界的检测缺
    let mut delta = Vec2::ZERO;
    let mut effect = 1.0;
    if left {
        // println!("left!");
        delta.x -= 0.5;
        if transform.translation.x + delta.x < -520.0 {
            effect = 0.0;
        }
    }
    if right {
        // println!("right!");
        delta.x += 0.5;
        if transform.translation.x + delta.x > 520.0 {
            effect = 0.0;
        }
    }
    if down {
        // println!("down");
        match *player_state {
            PlayerState::Jump => {},
            _ => {
                if transform.translation.y >-200.0 {
                    player.image = source.image_jump.clone();
                    player.texture_atlas = Some(TextureAtlas {
                        layout: source.lay_out_jump.clone(),
                        index: 0,
                    });
                    *player_state = PlayerState::Jump;
            
                    transform.translation.y += V.0;
                    V.0 -= PLAYER_GRAVITY;
                }
            },
        };    
    }
    if jump {
        // println!("jump!");
        match *player_state {
            PlayerState::Jump => {},
            _=> {
                player.image = source.image_jump.clone();
                player.texture_atlas = Some(TextureAtlas {
                    layout: source.lay_out_jump.clone(),
                    index: 0,
                });
                *player_state = PlayerState::Jump;
                events2.send(PlayerJumpEvent);
                V.0 = PLAYER_JUMP_SPEED;
                transform.translation.y += V.0;
                V.0 -= PLAYER_GRAVITY;
            },
        };
    }
    delta = delta.normalize();
    if delta.is_finite() && (jump || down || left || right) {
        transform.translation += vec3(delta.x * effect, delta.y, 0.0) * PLAYER_SPEED;
        //
        transform.translation.z = 30.0;
        //
        match *player_state {
            PlayerState::Move =>{},
            PlayerState::Jump =>{},
            _ => {
                player.image = source.image_move.clone();
                player.texture_atlas = Some(TextureAtlas {
                    layout: source.lay_out_move.clone(),
                    index: 1,
                });
                *player_state = PlayerState::Move;
            },
        };
        events.send(PlayerRunEvent);
        
    } else {
        match *player_state {
            PlayerState::Idle =>{},
            PlayerState::Jump =>{},
            _ => {
                player.image = source.image_idle.clone();
                player.texture_atlas = Some(TextureAtlas {
                    layout: source.lay_out_idle.clone(),
                    index: 1,
                });
                *player_state = PlayerState::Idle;
            },
        };
    }
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                println!("Collision started between {:?} and {:?}", entity1, entity2);
                if entity1.eq(&entity) || entity2.eq(&entity) {
                    if V.0.abs()  >= 20.0 {
                        transform.translation.y -= V.0;
                        transform.translation.y -= 20.0;
                    }

                    V.0 = 0.0;
                    transform.translation.y += V.0;
                    match *player_state {
                        PlayerState::Jump => {*player_state = PlayerState::Jumpover;},
                        _ => {},
                    }                    
                }
                return;
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                println!("Collision stopped between {:?} and {:?}", entity1, entity2);
                
                let mut y1 = 0.0;
                let mut y2 = 0.0;
                let mut tip = false;//用于判断碰撞事件是否跟自己有关
                if entity1.eq(&entity) {
                    y1 = transform.translation.y.clone();
                    tip = true;
                } else {
                    if let Ok(trans) = transform_query.get(*entity1) {
                        y1 = trans.translation.y.clone();
                    } else {
                        return;
                    }
                    // println!("y1={}", y1);
                }
                if entity2.eq(&entity) {
                    y2 = transform.translation.y.clone();
                    tip = true;
                } else {
                    if let Ok(trans) = transform_query.get(*entity2) {
                        y2 = trans.translation.y.clone();
                    } else {
                        return;
                    }
                    // println!("y1 - y2={}", y1 - y2);
                }
                //说明不是横向产生的碰撞，需要下降
                if tip && (y1 - y2).abs() > 50.0 {
                    player.image = source.image_jump.clone();
                    player.texture_atlas = Some(TextureAtlas {
                        layout: source.lay_out_jump.clone(),
                        index: 0,
                    });
                    *player_state = PlayerState::Jump;
    
                    transform.translation.y += V.0;
                    V.0 -= PLAYER_GRAVITY;
                    return;
                }

            }
        }
    }
    match *player_state {
        PlayerState::Jump => {
            transform.translation.y += V.0;
            V.0 -= PLAYER_GRAVITY;
        },
        _ => {},
    }
    if transform.translation.y <= -500.0 {
        transform.translation.y = 600.0;
        V.0 = 0.0;
        //防止掉出界外
    }
} 

fn handle_player_enemy_collision_events(
    mut player_query: Query<&mut Health, With<Character>>,
    mut events: EventReader<PlayerEnemyCollisionEvent>,
) {
    // if player_query.is_empty() {
    //     return;
    // }
    // let mut health = player_query.single_mut();
    // for _ in events.read() {
    //     health.0 -= ENEMY_DAMAGE;
    // }
}


fn handle_player_skills(
    // mut commands: Commands,
    mut player_query: Query<(
        &mut Sprite, 
        &mut Transform, 
        &mut PlayerState, 
        &mut KinematicCharacterController,
        Entity,
    ), With<Character>>,
    // transform_query: Query<&Transform, Without<Character>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    if player_query.is_empty() {
        return;
    }
    if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
        for (mut player, mut transform, mut player_state, mut kinematic_character_controller, entity) in player_query.iter_mut() {
            match *player_state {
                PlayerState::Jump => {},
                PlayerState::Dodge => {},
                _ => {
                    *player_state = PlayerState::Dodge;
                    if let Some(image) = source.image_skill.clone() {
                        player.image = image;
                    } else {
                        //Utaha skill

                    }
                    if let Some(layout) = source.lay_out_skill.clone() {
                        player.texture_atlas = Some(TextureAtlas {
                            layout: layout,
                            index: 0,
                        });
                    } else {
                        //Utaha skill

                    }
                },
            }
        }
    }
}


fn handle_player_bullet_collision_events(
    mut commands: Commands,
    mut events: EventWriter<PlayerHurtEvent>,
    mut player_query: Query<(Entity, &mut Health, &PlayerState), With<Character>>,
    mut collision_events: EventReader<CollisionEvent>,
    enemy_query: Query<Entity, With<EnemyBullet>>,
) {
    for collision_event in collision_events.read() {
        if player_query.is_empty() || enemy_query.is_empty() {
            return;
        }

        let (player, mut health, state) = player_query.single_mut();
        
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                if entity1.eq(&player) {
                    if let Ok(e) = enemy_query.get(*entity2) {
                        
                        match state {
                            PlayerState::Dodge => {},
                            _ => {
                                commands.entity(*entity2).despawn();
                                health.0 -= ENEMY_DAMAGE * 5.0;
                            },
                        }
                        if health.0 <= 0.0 {
                            health.0 = 0.0;
                        }
                        events.send(PlayerHurtEvent); 
                    }
                    // commands.entity(*entity2).despawn();
                    // health.0 -= ENEMY_DAMAGE * 5.0;
                    // events.send(PlayerHurtEvent);                  
                }  
                if entity2.eq(&player) {
                    if let Ok(e) = enemy_query.get(*entity1) {
                        
                        match state {
                            PlayerState::Dodge => {},
                            _ => {
                                commands.entity(*entity1).despawn();
                                health.0 -= ENEMY_DAMAGE * 5.0;
                            },
                        }
                        events.send(PlayerHurtEvent); 
                    }
                    // commands.entity(*entity1).despawn();
                    // health.0 -= ENEMY_DAMAGE * 5.0; 
                    // events.send(PlayerHurtEvent);                 
                }            
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {

            }
        }
    }
}

fn handle_player_death(
    mut player_query: Query<&mut Health, With<Character>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_query.is_empty() {
        return;
    }
    let mut health = player_query.single_mut();
    if health.0 <= 0.0 {
        //可以的话加个死亡动画慢动作
        health.0 = 0.0;
        println!("Game Over!");
        // next_state.set(GameState::OverMenu);//进结算界面
    }
}

fn handle_player_move2(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<PlayerRunEvent>,
    mut events2: EventWriter<PlayerJumpEvent>,
    // mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(
        &mut Sprite, 
        &mut Transform,
        &mut PlayerState, 
        &mut Velocity,
        &mut KinematicCharacterController,
        ), With<Character>>,
    flag_query: Query<&KinematicCharacterControllerOutput, With<Character>>,
    source: Res<GlobalCharacterTextureAtlas>,
    
) {
    if player_query.is_empty() {
        return;
    }
    
    //之后可以改为自定义的键位，数据存到configs中
    let (
        mut player, 
        mut transform,
        mut player_state,
        mut V,
        mut controller,
        ) = player_query.single_mut();
    let jump = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::Space);
    let left = keyboard_input.pressed(KeyCode::KeyA);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let right = keyboard_input.pressed(KeyCode::KeyD);

    let mut ground = false;
    let mut d = Vec2::ZERO;
    let mut e = Vec2::ZERO;
    for flag in flag_query.iter() {
        //就角色一个插件
        ground = flag.grounded.clone();
        d = flag.desired_translation.clone();
        e = flag.effective_translation.clone();
    }
    //test if grounded
    // if ground {
    //     info!("grounded!");
    // }

    let mut delta = Vec2::ZERO;
    if left {
        // println!("left!");
        delta.x -= 1.0;
    }
    if right {
        // println!("right!");
        delta.x += 1.0;
    }
    //
    //test
    if down {
        println!("down");
        if ground && transform.translation.y >= -190.0 {
            transform.translation.y -= 15.0;
            player.image = source.image_jump.clone();
            player.texture_atlas = Some(TextureAtlas {
                layout: source.lay_out_jump.clone(),
                index: 0,
            });
            *player_state = PlayerState::Jump;
            // delta.y -= 0.5;
        } 
    }
    if jump {
        // println!("jump!");
        match *player_state {
            PlayerState::Jump => {},
            _=> {
                player.image = source.image_jump.clone();
                player.texture_atlas = Some(TextureAtlas {
                    layout: source.lay_out_jump.clone(),
                    index: 0,
                });
                *player_state = PlayerState::Jump;
                events2.send(PlayerJumpEvent);
                V.0 = PLAYER_JUMP_SPEED;
                delta.y = V.0;
                // transform.translation.y += V.0;

                V.0 -= PLAYER_GRAVITY;
            },
        };
    }
    //不主动在外面赋值的话当没有按键时translation会变为none导致错误

    if delta.is_finite() && (jump || down || left || right) {
        events.send(PlayerRunEvent);
    }
    match *player_state {
        PlayerState::Idle => {
            if delta.is_finite() && (jump || down || left || right) && ground {
                player.image = source.image_move.clone();
                player.texture_atlas = Some(TextureAtlas {
                    layout: source.lay_out_move.clone(),
                    index: 1,
                });
                *player_state = PlayerState::Move; 
            }
            if !ground {
                player.image = source.image_jump.clone();
                player.texture_atlas = Some(TextureAtlas {
                    layout: source.lay_out_jump.clone(),
                    index: 0,
                });
                *player_state = PlayerState::Jump;
                V.0 -= PLAYER_GRAVITY;
                delta.y = V.0;
                // transform.translation.y += V.0;
                // println!("fall!!!,v={}",V.0);
            }
        },
        PlayerState::Move =>{
            if !(delta.is_finite() && (jump || down || left || right)) && ground {
                    player.image = source.image_idle.clone();
                    player.texture_atlas = Some(TextureAtlas {
                        layout: source.lay_out_idle.clone(),
                        index: 1,
                    });
                    *player_state = PlayerState::Idle;
            }
            if !ground {
                player.image = source.image_jump.clone();
                player.texture_atlas = Some(TextureAtlas {
                    layout: source.lay_out_jump.clone(),
                    index: 0,
                });
                *player_state = PlayerState::Jump;
                V.0 -= PLAYER_GRAVITY;
                delta.y = V.0;
                // transform.translation.y += V.0;
                // println!("fall!!!,v={}",V.0);
            }
        },
        PlayerState::Jump =>{
            if ground {
                V.0 = 0.0;
                *player_state = PlayerState::Jumpover;
            } else {
                delta.y = V.0;
                // transform.translation.y += V.0;
                // println!("fall!!!,v={}",V.0);
                V.0 -= PLAYER_GRAVITY;
            }
        },
        PlayerState::Jumpover => {
            if delta.is_finite() && (jump || down || left || right) {
                player.image = source.image_move.clone();
                player.texture_atlas = Some(TextureAtlas {
                    layout: source.lay_out_move.clone(),
                    index: 1,
                });
                *player_state = PlayerState::Move; 
            } else {
                player.image = source.image_idle.clone();
                player.texture_atlas = Some(TextureAtlas {
                    layout: source.lay_out_idle.clone(),
                    index: 1,
                });
                *player_state = PlayerState::Idle; 
            }
        },
        PlayerState::Dodge => {
            if player.flip_x {
                delta.x -= 5.0;
            }
            else {
                delta.x += 5.0;
            }
            delta.y = 0.0;
        },
    }
    controller.translation = Some(delta.clone() * PLAYER_SPEED);
}
    


fn handle_player_move3(
    mut events: EventWriter<PlayerRunEvent>,
    mut player_query: Query<(
        &mut Sprite, 
        &mut PlayerState, 
        &mut KinematicCharacterController,
        ), (With<Character>, Without<Gun>)>,
    gun_query: Query<&Transform, (With<Gun>, Without<Character>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    source: Res<GlobalCharacterTextureAtlas>,

    // test: Query<&KinematicCharacterControllerOutput>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    // 
    // for m in test.iter() {
    //     if m.grounded {
    //         info!("grounded!");
    //     }
    //     if keyboard_input.pressed(KeyCode::KeyO) {
    //         let d = m.desired_translation.clone();
    //         let e = m.effective_translation.clone();
    //         let text = format!("d={:?}, e={:?}", d, e);
    //         info!(text);
    //     }
    // }

    //之后可以改为自定义的键位，数据存到configs中
    let (
        mut player, 
        mut player_state,
        mut controller,
        ) = player_query.single_mut();

        // controller.

        match *player_state {
            PlayerState::Dodge => {
                let gun_transform = gun_query.single();
                let direction = gun_transform.local_x();
                controller.translation = Some(
                    Vec2::new(direction.x, direction.y).normalize() * 10.0);
                return;
            },
            _ => {},
        }


    let jump = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::Space);
    let left = keyboard_input.pressed(KeyCode::KeyA);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let right = keyboard_input.pressed(KeyCode::KeyD);
    //到边界的检测缺
    let mut delta = Vec2::ZERO;
    if left {
        // println!("left!");
        delta.x -= 2.0;
    }
    if right {
        // println!("right!");
        delta.x += 2.0;
    }
    if down {
        // println!("down");
        delta.y -= 2.0;
    }
    if jump {
        // println!("up");
        delta.y += 2.0;
    }
    //不主动在外面赋值的话当没有按键时translation会变为none导致错误
    controller.translation = Some(delta.normalize_or_zero().clone() * PLAYER_SPEED);
    if delta.is_finite() && (jump || down || left || right) {
        match *player_state {
            PlayerState::Move =>{},
            _ => {
                player.image = source.image_move.clone();
                player.texture_atlas = Some(TextureAtlas {
                    layout: source.lay_out_move.clone(),
                    index: 0,
                });
                *player_state = PlayerState::Move;
            },
        };
        events.send(PlayerRunEvent);
        
    } else {
        match *player_state {
            PlayerState::Idle =>{},
            _ => {
                player.image = source.image_idle.clone();
                player.texture_atlas = Some(TextureAtlas {
                    layout: source.lay_out_idle.clone(),
                    index: 0,
                });
                *player_state = PlayerState::Idle;
            },
        };
    }
}
