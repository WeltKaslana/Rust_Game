use bevy::math::vec3;
use bevy::render::render_resource::encase::private::Length;
use bevy::transform;
use bevy::{
    prelude::*, 
    time::Stopwatch,
    ecs::world::DeferredWorld,};
use bevy_rapier2d::na::distance_squared;
use bevy_rapier2d::prelude::*;
use bevy::utils::Instant;

use std::default;
use std::{time::Duration};
use crate::boss::BossComponent;
use crate::gui::Transition;
use crate::{
    gamestate::*,
    enemy::{
        EnemyBullet,
        Enemy,
        EnemyDeathEvent,
    },
    gun::{
        Gun,
        GunState,
        BulletDirection,
        ArisuSPDamage,
        SpawnInstant,
        BulletHit},
    boss::{Boss, BossState},
};
use crate::*;
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Character;

//表示所有与玩家相关的实体
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Shield;

#[derive(Component)]
pub struct Drone;

#[derive(Component)]
pub struct DroneBullet;

#[derive(Component)]
pub struct MK1;

#[derive(Component)]
pub struct MK2;

#[derive(Component)]
pub struct MK2Loc(pub Vec3);

#[derive(Component)]
pub struct MK2Born;

#[derive(Component)]
pub struct MK2LockOn(pub Vec2);

#[derive(Component)]
pub struct Grenade;

#[derive(Component)]
pub struct GrenadeHit;



#[derive(Component)]
pub struct State(pub i32);

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Velocity(pub f32);

#[derive(Component)]
pub struct PlayerTimer(pub Stopwatch);

#[derive(Component)]
pub struct Skill2Timer(pub Stopwatch);

#[derive(Component)]
pub struct Skill3Timer(pub Stopwatch);

#[derive(Component)]
pub struct Skill4Timer(pub Stopwatch);

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Jump,
    Move,
    Jumpover,
    Dodge,
}

#[derive(Component)]
pub struct Buff(
    pub i8, // 0.bullet_num
    pub i8, // 1.fire_speed
    pub i8, // 2.bullet_speed
    pub i8, // 3.bullet_spread
    pub i8, // 4.bullet_damage
    pub i8, // 5.grenade_range
    pub i8, // 6.skill_cooldown
    pub i8, // 7.resistence_up
    // to design 
);

impl Default for Buff {
    fn default() -> Self {
        Buff(
            1, 
            1, 
            1, 
            1, 
            1, 
            1, 
            1, 
            1)
    }
}
impl Buff {
    pub fn sum(&self) -> i8 {
        self.0 + self.1 + self.2 + self.3 + self.4 + self.5 + self.6 + self.7
    }
}

#[derive(Event)]
pub struct PlayerEnemyCollisionEvent;

#[derive(Event)]
pub struct PlayerRunEvent;



#[derive(Event)]
pub struct PlayerJumpEvent;

#[derive(Event)]
pub struct PlayerHurtEvent;

#[derive(Event)]
pub struct PlayerParryEvent;

#[derive(Event)]
pub struct PlayerSkill2Event;

#[derive(Event)]
pub struct PlayerSkill3Event;

#[derive(Event)]
pub struct PlayerSkill4Event;

#[derive(Event)]
pub struct ReloadPlayerEvent(pub u8);

#[derive(Event)]
pub struct GameOverEvent;

#[derive(Component)]
pub struct Fire(pub i32, pub Vec2);

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
            .add_event::<ReloadPlayerEvent>()
            .add_event::<GameOverEvent>()
            .add_event::<PlayerParryEvent>()
            .add_event::<PlayerSkill2Event>()
            .add_event::<PlayerSkill3Event>()
            .add_event::<PlayerSkill4Event>()
            .add_systems(OnEnter(GameState::Home), setup_player)
            .add_systems(Update, reload_player)

            .add_systems(
            Update,
                (
                    handle_player_move2,
                    handle_player_skill2,
                    handle_player_skill3,
                    handle_player_skill4,
                    handle_grenade_despawn,
                    handle_shield_despawn,
                    handle_mk1,
                    handle_mk1_move,
                    // handle_play_bullet_collision_events,
            ).run_if(in_state(HomeState::Running))
            )
            .add_systems(
                Update,
                    (
                        handle_player_death,
                        handle_player_move3,
                        handle_player_skill2,
                        handle_utaha_attack_damage,
                        handle_player_skill3,
                        handle_grenade_despawn,
                        handle_shield_despawn,
                        handle_player_skill4,
                        handle_player_enemy_parry_events,
                        handle_player_bullet_collision_events,
                        handle_mk1,
                        handle_mk1_move,
                ).run_if(in_state(InGameState::Running))
            )
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
            index: 0,
        }),
        ..Default::default()
        },
        Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(-200.0, -200.0 + 5.0, 30.0)),
        AnimationConfig::new(13),
        PlayerState::default(),
        Character,
        Player,
        // 血条
        Health(PLAYER_HEALTH),
        // //跳跃起始速度
        Velocity(PLAYER_JUMP_SPEED),
        //音效播放间隔计时器
        PlayerTimer(Stopwatch::default()),
        // 状态栏
        Buff::default(),

        Collider::cuboid(9.0, 16.5),

        RigidBody::Fixed,
        ColliderMassProperties::Mass(150.0),

        ActiveEvents::COLLISION_EVENTS,
        Sensor,//不加这个部件的话碰撞就会产生实际碰撞效果，否则只会发送碰撞事件而无效果
        //后续可以为碰撞体分组
        ));

        // //尝试插入运动部件
        player
            .insert(KinematicCharacterController {
                ..Default::default()
            });
        player.insert((
            Skill2Timer(Stopwatch::default()),
            Skill3Timer(Stopwatch::default()),
            Skill4Timer(Stopwatch::default()),
        ));

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
fn reload_player(
    mut events: EventReader<ReloadPlayerEvent>,
    mut player_query: Query<(&mut Sprite, &PlayerState), (With<Character>, Without<Gun>)>,
    mut gun_query: Query<(&mut Sprite, &mut Visibility), (With<Gun>, Without<Character>)>,
    source: Res<GlobalCharacterTextureAtlas>,
) {//角色重载
    // 如果不同角色对应技能cd不同，技能cd可能要重新刷新
    for _ in events.read() {
        for (mut player, player_state) in player_query.iter_mut() {
            match *player_state {
                PlayerState::Jump => {
                    player.image = source.image_jump.clone();
                    player.texture_atlas = Some(TextureAtlas {
                        layout: source.lay_out_jump.clone(),
                        index: 0,
                    });
                },
                PlayerState::Move => {
                    player.image = source.image_move.clone();
                    player.texture_atlas = Some(TextureAtlas {
                        layout: source.lay_out_move.clone(),
                        index: 0,
                    });
                },
                PlayerState::Idle => {
                    player.image = source.image_idle.clone();
                    player.texture_atlas = Some(TextureAtlas {
                        layout: source.lay_out_idle.clone(),
                        index: 0,
                    });
                },
                _ => {
                    // 可能有bug
                    player.image = source.image_idle.clone();
                    player.texture_atlas = Some(TextureAtlas {
                        layout: source.lay_out_idle.clone(),
                        index: 0,
                    });
                },
            }
        }
        for (mut gun, mut vis) in gun_query.iter_mut() {
            info!("gun reload!");
            gun.image = source.image_gun.clone();
            gun.texture_atlas = Some(TextureAtlas {
                layout: source.lay_out_gun.clone(),
                index: 0,
            });
            *vis = Visibility::Visible;
        }
        info!("reload player!");

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

fn handle_player_enemy_parry_events(
    mut events: EventReader<PlayerParryEvent>,
    player_query: Query<&Transform, (With<Character>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Health, &Transform), (With<Enemy>, Without<Character>)>,

) {
    for _ in events.read() {
        for transp in player_query.iter() {
            for (mut health, trans) in &mut enemy_query.iter_mut()  {
                if trans.translation.distance(transp.translation) < GRENADE_BOOM_RANGE * 2.0 {
                    health.0 -= BULLET_DAMAGE * 5.0;
                    println!("BANG!");
                }
            }
        }
    }
}


fn handle_player_skills1(
    // mut commands: Commands,
    mut player_query: Query<(
        &mut Sprite, 
        &mut PlayerState, 
        &mut KinematicCharacterController,
    ), With<Character>>,
    // transform_query: Query<&Transform, Without<Character>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    if player_query.is_empty() {
        return;
    }
    if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
        for (mut player, mut player_state, mut controller) in player_query.iter_mut() {
            match *player_state {
                PlayerState::Jump => {},
                PlayerState::Dodge => {},
                _ => {
                    *player_state = PlayerState::Dodge;
                    //使得玩家不与敌人产生碰撞
                    controller.filter_groups = Some(CollisionGroups::new(Group::GROUP_1, Group::GROUP_2));

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
fn handle_player_skill2(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(
        &mut Sprite, 
        &mut PlayerState,
        &mut Velocity,
        &mut KinematicCharacterController,
        &mut Skill2Timer,
    ), (With<Character>, Without<Gun>)>,
    mut gun_query: Query<&mut Visibility, (With<Gun>, Without<Character>)>,
    mk1_query: Query<Entity, With<MK1>>,
    mut events: EventWriter<PlayerSkill2Event>,
    mut little_drone_events: EventReader<EnemyDeathEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    if player_query.is_empty() {
        return;
    }
    
    for (mut player, mut player_state, mut V, mut controller, mut timer) in player_query.iter_mut() {
        for EnemyDeathEvent(dloc) in little_drone_events.read() {
            match *player_state {
                PlayerState::Dodge if source.id ==3 => {
                    if mk1_query.iter().count() < 3 {
                        commands.spawn((
                            Sprite {
                                image: source.image_MK1.clone(),
                                texture_atlas: Some(TextureAtlas {
                                    layout: source.layout_MK1.clone(),
                                    index: 0,
                                }),
                                ..Default::default()
                            },
                            Transform::from_scale(Vec3::splat(2.5))
                                .with_translation(Vec3::new(dloc.x, dloc.y, 40.0)),
                            MK1,
                            Player, 
                            AnimationConfig::new(10),
                            SpawnInstant(Instant::now()),
                            Fire(0,Vec2::ZERO),
                        ));
                    }
                },
                _ => {}
            }
        }

        timer.0.tick(time.delta());
        // println!("timer={}", timer.0.elapsed_secs().ceil() as i8);
        if !keyboard_input.just_pressed(KeyCode::ShiftLeft) {
            return;
        }
        if timer.0.elapsed_secs() < SKILL2_CD {
            //技能冷却中
            info!("skill2 is cooling down!");
            return;
        }
        timer.0.reset();

        match *player_state {
            PlayerState::Jump => {
                V.0 = 0.0;
                *player_state = PlayerState::Dodge;

                if let Some(image) = source.image_skill.clone() {
                    player.image = image;
                }
                if let Some(layout) = source.lay_out_skill.clone() {
                    player.texture_atlas = Some(TextureAtlas {
                        layout: layout,
                        index: 0,
                    });
                }
                if source.id == 2 {
                    // arisu的光之剑需要隐藏
                    for mut gun in gun_query.iter_mut() {
                        *gun = Visibility::Hidden;
                    }
                } else {
                    //使得玩家不与敌人产生碰撞，但arisu要盾反
                    controller.filter_groups = Some(CollisionGroups::new(Group::GROUP_1, Group::GROUP_2));
                }
                events.send(PlayerSkill2Event);
            },
            PlayerState::Dodge => {},
            _ => {
                *player_state = PlayerState::Dodge;
                events.send(PlayerSkill2Event);
                if let Some(image) = source.image_skill.clone() {
                    player.image = image;
                }
                if let Some(layout) = source.lay_out_skill.clone() {
                    player.texture_atlas = Some(TextureAtlas {
                        layout: layout,
                        index: 0,
                    });
                }
                if source.id == 2 {
                    // arisu的光之剑需要隐藏
                    for mut gun in gun_query.iter_mut() {
                        *gun = Visibility::Hidden;
                    }
                } else {
                    //使得玩家不与敌人产生碰撞
                    controller.filter_groups = Some(CollisionGroups::new(Group::GROUP_1, Group::GROUP_2));
                }
            },
        }
    }
}

fn handle_utaha_attack_damage (
    mut commands: Commands,
    player_query: Query<(&Transform, &PlayerState), (With<Character>, Without<Enemy>)>,
    weapen_query: Query<(&Transform, &GunState), (With<Gun>, Without<Enemy>)>,
    enemy_bullet_query: Query<(Entity, &Transform), (With<EnemyBullet>, Without<Character>)>,
    mut enemy_query: Query<(&mut Health, &Transform), (With<Enemy>, Without<EnemyBullet>, Without<Boss>)>,
    mut boss_query: Query<(&mut Health, &Transform), (With<Boss>, Without<EnemyBullet>, Without<Enemy>)>,
    source: Res<GlobalCharacterTextureAtlas>,
) {

    let mut flag = false;
    let mut trans = Vec2::ZERO;
    let mut damage = 1.0;
    if source.id == 3 {
        if !weapen_query.is_empty() { 
            let (wtrans, state) = weapen_query.single();
            match *state {
                GunState::Fire => {
                    trans = wtrans.translation.truncate();
                    flag = true;
                },
                _ => {}
            }
        }

        if !player_query.is_empty() {
            let (ptrans, state) = player_query.single();
            match *state {
                PlayerState::Dodge => {
                    trans = ptrans.translation.truncate().clone();
                    flag = true;
                    damage = 3.0;
                },
                _ => {}
            }
        }
    }
    if flag {
        let mut xishu = 1.0;
        if damage > 2.0 {
            xishu = 2.0;
        }
        // 消除敌方子弹
        for (bullet, btrans) in enemy_bullet_query.iter() {
            if btrans.translation.truncate().distance(trans) < 70.0 * xishu {
                commands.entity(bullet).despawn();
            }
        }
        // 对敌方造成伤害
        for (mut health, etrans) in enemy_query.iter_mut() {
            if (etrans.translation.x - trans.x).abs() < 90.0 && (etrans.translation.y - trans.y).abs() < 130.0 {
                health.0 -= damage * BULLET_DAMAGE;
            }
        }
        // 对boss造成伤害
        for (mut health, btrans) in boss_query.iter_mut() {
            if (btrans.translation.x - trans.x).abs() < 90.0 && (btrans.translation.y - trans.y).abs() < 130.0 {
                health.0 -= damage * BULLET_DAMAGE * 0.2;
            }
        }
    }
}

fn handle_player_skill3 (
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(
        Entity,
        &Transform,
        &mut Skill3Timer,
    ), (With<Character>, Without<Gun>)>,
    gun_query: Query<&Transform, (With<Gun>, Without<Character>)>,
    mut grenade_query: Query<(&mut Transform, &BulletDirection, &mut Velocity), (With<Grenade>, Without<Character>, Without<Gun>)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    source: Res<GlobalCharacterTextureAtlas>,   
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }
    // 手雷的运动
    for (mut trans, dir, mut V) in grenade_query.iter_mut() {
        trans.translation += dir.0.normalize() * Vec3::splat(BULLET_SPEED);
        trans.translation.y -= V.0;
        V.0 += 0.2;
        trans.translation.z = 30.0;
    }

    let gun_transform = gun_query.single();
    let direction = gun_transform.local_x();
    for (player, trans, mut timer) in player_query.iter_mut() {
        timer.0.tick(time.delta());
        // println!("timer3={}", timer.0.elapsed_secs().ceil() as i8);
        if !mouse_button_input.just_pressed(MouseButton::Right) {
            return;
        }
        if timer.0.elapsed_secs() < SKILL3_CD {
            //技能冷却中
            info!("skill3 is cooling down!");
            return;
        }
        timer.0.reset();
        
        match source.id {
            3 => {
                commands.entity(player).with_child((
                    Sprite {
                        image: source.image_shield.clone(),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)).with_scale(Vec3::splat(0.6)),
                    Player,
                    Shield,
                    SpawnInstant(Instant::now()),
                ));
            },
            _ => {
                commands.spawn((
                    Sprite {
                        image: source.image_grenade.clone(),
                        ..Default::default()
                    },
                    Transform::from_translation(trans.translation.clone()).with_scale(Vec3::splat(2.5)),
                    BulletDirection(Vec3::new(direction.x,direction.y,direction.z,)),
                    Player,
                    Grenade,
                    Velocity(0.5),
                    Collider::ball(7.0),
                    RigidBody::Dynamic,
                    GravityScale(0.0),
                    ColliderMassProperties::Mass(1000.0),
                    ActiveEvents::COLLISION_EVENTS,
                ));
            }
        }
    }
}

fn handle_grenade_despawn (
    mut commands: Commands,
    grenade_query: Query<(Entity, &Transform), (With<Grenade>, Without<Character>)>,
    player_query: Query<Entity, (With<Character>, Without<Grenade>)>,
    mut collision_events: EventReader<CollisionEvent>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let mut flag = false;
                let mut trans = Vec3::splat(-100.0);
                if let Ok((e, transf)) = grenade_query.get(*entity1) {
                    if !player_query.get(*entity2).is_ok() {
                        // 手雷与玩家不碰撞
                        commands.entity(*entity1).despawn();
                        trans = transf.translation;
                        flag = true;
                    }

                }
                if let Ok((e, transf)) = grenade_query.get(*entity2) {
                    if !player_query.get(*entity1).is_ok() {
                        commands.entity(*entity2).despawn();
                        trans = transf.translation;
                        flag = true;
                    }
                }
                if flag {
                    //产生手雷消失的特效
                    commands.spawn((
                        Sprite {
                            image: source.image_grenade_hit.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: source.layout_grenade_hit.clone(),
                                index: 0,
                            }),
                            ..default()
                        },
                        Transform {
                            translation: trans.clone(),
                            scale: Vec3::splat(2.5),
                            ..default()
                        },
                        AnimationConfig::new(15),
                        GrenadeHit,
                    ));
                }
            },
            _ => {}
        }
    }
}

fn handle_shield_despawn (
    mut commands: Commands,
    loc_query: Query<&Transform, (With<Character>, Without<EnemyBullet>)>,
    shield_query: Query<(Entity, &SpawnInstant), (With<Shield>, Without<EnemyBullet>)>,
    enemy_bullet_query: Query<(Entity, &Transform), (With<EnemyBullet>, Without<Shield>)>,
) {
    if shield_query.is_empty() || loc_query.is_empty() {
        return;
    }

    let loc = loc_query.single().translation;

    for (shield, instant) in shield_query.iter() {
        if instant.0.elapsed().as_secs_f32() > 1.5 {// 后续更改持续时间
            // 1.5秒后消失
            commands.entity(shield).despawn();
            continue;
        }
        for (b, btrans) in enemy_bullet_query.iter() { 
            if btrans.translation.distance(loc) < 70.0 {
                commands.entity(b).despawn();
                println!("shield hit by bullet!");
                // 后续加个子弹消除的特效
            }
        }
    }
}

pub fn handle_player_skill4 (
    time: Res<Time>,
    mut commands: Commands,
    mut transform_query: Query<(&Sprite, &Transform, &mut Skill4Timer), (With<Character>, Without<Drone>, Without<DroneBullet>, Without<Enemy>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut drone_query: Query<(&mut Sprite, & State), (With<Drone>, Without<DroneBullet>, Without<Enemy>, Without<Character>)>,
    mut drone_bullet_query: Query<(&Transform, &mut BulletDirection), (With<DroneBullet>, Without<Drone>, Without<Enemy>, Without<Character>)>,
    mk2_query: Query<(Entity, &SpawnInstant, &Transform), With<MK2>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<DroneBullet>, Without<Drone>, Without<Character>)>,
    mut events: EventWriter<PlayerSkill4Event>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    if transform_query.is_empty() {
        return;
    }
    match source.id {
        1 => {
            // shiroko drone
            let (player, player_transform, mut timer) = transform_query.single_mut();
            timer.0.tick(time.delta());
            // println!("timer4={}", timer.0.elapsed_secs().ceil() as i8);

            if drone_bullet_query.is_empty() ||  enemy_query.is_empty() {
            } else {
                for (bullet_transform, mut bullet_direction) in drone_bullet_query.iter_mut() {
                    let mut dis = 99999.9;
                    for enemy_transform in enemy_query.iter() {
                        let d = (bullet_transform.translation - enemy_transform.translation).length();
                        if d < dis {
                            dis = d;
                            bullet_direction.0 = (enemy_transform.translation - bullet_transform.translation).normalize();
                        }
                    }
                }
            }

            if !drone_query.is_empty() {//存在小飞机
                let (mut drone, state) = drone_query.single_mut();
                if state.0 != 0 {
                    drone.image = source.image_drone_fire.clone();
                    if let Some(atlas) = &mut drone.texture_atlas {
                        atlas.layout = source.layout_drone_fire.clone();
                    }
                }
            } else if keyboard_input.just_pressed(KeyCode::KeyQ)  {//不存在小飞机，则按Q生成小飞机
                if timer.0.elapsed_secs() < SKILL4_CD {
                    //技能冷却中
                    info!("skill4 is cooling down!");
                    return;
                }
                timer.0.reset();
                commands.spawn((
                    Sprite {
                        image:  source.image_drone_idle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: source.layout_drone_idle.clone(),
                            index: 0,
                        }),
                        flip_x: player.flip_x,
                        ..Default::default()
                    },
                    Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(player_transform.translation.x, player_transform.translation.y, 31.0)),
                    Player,
                    Drone,
                    State(0),
                    AnimationConfig::new(10),
                ));
                events.send(PlayerSkill4Event);
            }

        },
        2 => {
            // arisu的技能为光之剑蓄力，放在了gun开火的函数中
        },
        3 => {
            let (player, player_transform, mut timer) = transform_query.single_mut();
            timer.0.tick(time.delta());
            if !keyboard_input.just_pressed(KeyCode::KeyQ) && !keyboard_input.just_pressed(KeyCode::KeyR)  { 
                return;
            }
            events.send(PlayerSkill4Event);
            let mut num = 0;
            let mut eldest_mk2: Option<Entity> = None;
            let mut life = 0.0;
            let mut mktrans = Vec3::new(0.0, 0.0, 0.0);

            if keyboard_input.just_pressed(KeyCode::KeyR) {
                for (mk2, instant, mk2trans) in mk2_query.iter() {
                    num += 1;
                    let temp = instant.0.elapsed().as_secs_f32();
                    if temp > life {
                        life = temp;
                        eldest_mk2 = Some(mk2); 
                        mktrans = mk2trans.translation.clone();
                    }
                }
                if let Some(mk2) = eldest_mk2 {
                    commands.entity(mk2).despawn_recursive();
                    //产生炮台消失的特效
                    commands.spawn((
                        Sprite {
                            image: source.image_grenade_hit.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: source.layout_grenade_hit.clone(),
                                index: 0,
                            }),
                            ..default()
                        },
                        Transform {
                            translation: mktrans.clone(),
                            scale: Vec3::splat(2.5),
                            ..default()
                        },
                        AnimationConfig::new(15),
                        GrenadeHit,
                    ));
                }
                return;
            }
            if timer.0.elapsed_secs() < SKILL4_CD {
                //技能冷却中
                info!("skill4 is cooling down!");
                return;
            }
            timer.0.reset();
            // 删除最早的一个炮台
            for (mk2, instant, mk2trans) in mk2_query.iter() {
                num += 1;
                let temp = instant.0.elapsed().as_secs_f32();
                if temp > life {
                    life = temp;
                    eldest_mk2 = Some(mk2); 
                    mktrans = mk2trans.translation.clone();
                }
            }
            if num >= MK2_NUM {
                if let Some(mk2) = eldest_mk2 {
                    commands.entity(mk2).despawn_recursive();
                    //产生炮台消失的特效
                    commands.spawn((
                        Sprite {
                            image: source.image_grenade_hit.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: source.layout_grenade_hit.clone(),
                                index: 0,
                            }),
                            ..default()
                        },
                        Transform {
                            translation: mktrans.clone(),
                            scale: Vec3::splat(2.5),
                            ..default()
                        },
                        AnimationConfig::new(15),
                        GrenadeHit,
                    ));
                }
            }
            commands.spawn((
                Sprite {
                    image: source.image_MK2_born.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source.layout_MK2_born.clone(),
                        index: 0,
                    }),
                    flip_x: player.flip_x,
                    ..Default::default()
                },
                Transform::from_scale(Vec3::splat(2.5))
                    .with_translation(Vec3::new(0.0,280.0,-5.0) + player_transform.translation.clone()),
                Player,
                AnimationConfig::new(10),
                MK2Born,
            ));
        },
        _ => {}
    }
}

fn handle_player_bullet_collision_events(
    // mut commands: Commands,
    mut events: EventWriter<PlayerHurtEvent>,
    mut events2: EventWriter<PlayerParryEvent>,
    mut player_query: Query<(Entity, &mut Health, &PlayerState), With<Character>>,
    mut collision_events: EventReader<CollisionEvent>,
    enemy_query: Query<Entity, With<EnemyBullet>>,
    shield_query: Query<Entity, (With<Shield>)>,
    source: Res<GlobalCharacterTextureAtlas>,
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
                            PlayerState::Dodge => {
                                if source.id == 2 {
                                    // arisu盾反
                                    events2.send(PlayerParryEvent);
                                }
                            },
                            _ => {
                                if shield_query.is_empty() {
                                    health.0 -= ENEMY_DAMAGE * 5.0;
                                }
                            },
                        }
                        if health.0 <= 0.0 {
                            health.0 = 0.0;
                        }
                        events.send(PlayerHurtEvent); 
                    }                
                }  
                if entity2.eq(&player) {
                    if let Ok(e) = enemy_query.get(*entity1) {
                        
                        match state {
                            PlayerState::Dodge => {
                                if source.id == 2 {
                                    // arisu盾反
                                    events2.send(PlayerParryEvent);
                                }
                            },
                            _ => {
                                if shield_query.is_empty() {
                                    health.0 -= ENEMY_DAMAGE * 5.0;
                                }
                            },
                        }
                        events.send(PlayerHurtEvent);
                    }               
                }            
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {

            }
        }
    }
}

fn handle_player_death(
    mut player_query: Query<&mut Health, With<Character>>,
    mut windows: Query<&mut Window>,
    query:  Query<Entity, With<Transition>>,
    mut next_state: ResMut<NextState<InGameState>>,
    mut events: EventWriter<GameOverEvent>,
) {
    if player_query.is_empty() {
        return;
    }
    let mut health = player_query.single_mut();
    if health.0 <= 0.0 {
        //可以的话加个死亡动画慢动作
        health.0 = 0.0;
        if query.is_empty() {
            events.send(GameOverEvent);
            println!("Game Over!");
            if let Ok(mut window) = windows.get_single_mut() {
                window.cursor_options.visible = true;
            }
            next_state.set(InGameState::GameOver);
        }

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
            PlayerState::Dodge => {},
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
            if source.id == 1 || source.id == 3 {
                if player.flip_x {
                    delta.x -= 2.0;
                }
                else {
                    delta.x += 2.0;
                }
                delta.y = 0.0;
            }
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

    //之后可以改为自定义的键位，数据存到configs中
    let (
        mut player, 
        mut player_state,
        mut controller,
        ) = player_query.single_mut();

        // controller.

        match *player_state {
            PlayerState::Dodge => {
                if source.id == 1 || source.id == 3 {
                    let gun_transform = gun_query.single();
                    let direction = gun_transform.local_x();
                    controller.translation = Some(
                        Vec2::new(direction.x, direction.y).normalize() * 10.0);
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

fn handle_mk1(
    mut commands: Commands,
    mut mk1_query: Query<(Entity, &Transform, &SpawnInstant, &mut Fire), (With<MK1>, Without<Character>)>,
    player_query: Query<& Transform, (With<Character>, Without<MK1>)>,
    enemy_query: Query<& Transform, (With<Enemy>, Without<Boss>)>,
    boss_query: Query<& Transform, (With<Boss>, Without<BossComponent>)>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    if mk1_query.is_empty() || player_query.is_empty() {
        return;
    }
    let playerloc =  player_query.single();

    for (mk1, mk1_transform, mk1_spawn_instant, mut fire) in mk1_query.iter_mut() {
        let elapsed = mk1_spawn_instant.0.elapsed();
        if elapsed >= Duration::from_secs(15) {
            commands.entity(mk1).despawn();
            commands.spawn((
                Sprite {
                    image: source.image_gun_hit.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source.lay_out_gun_hit.clone(),
                        index: 0,
                    }),
                    ..default()
                },
                Transform {
                    translation: mk1_transform.translation.clone(),
                    scale: Vec3::splat(5.0),
                    ..default()
                },
                AnimationConfig::new(15),
                BulletHit,
            ));
            continue;
        }
        let mut distance = 9999.9;
        let mut dx = 0.0;
        let mut dy = 0.0;
        if boss_query.is_empty() {
            
        } else {
            let boss_transform = boss_query.single();
            dx = boss_transform.translation.x - mk1_transform.translation.x;
            dy = boss_transform.translation.y - mk1_transform.translation.y;
            distance = (dx * dx + dy * dy).sqrt();
        }
        for enemy_transform in enemy_query.iter() {
            if enemy_transform.translation.distance(mk1_transform.translation) < distance { 
                distance = enemy_transform.translation.distance(mk1_transform.translation);
                dx = enemy_transform.translation.x - mk1_transform.translation.x;
                dy = enemy_transform.translation.y - mk1_transform.translation.y;
            }
        }
        fire.1 = Vec2::new(dx, dy);

        if distance <= 300.0 &&  fire.0 == 0 {
            fire.0 = 1;
        } else if distance > 300.0 {
            fire.0 = 0;
        }

    }
}

fn handle_mk1_move(
    mut mk1_query: Query<&mut Transform, (With<MK1>, Without<Character>)>,
    player_query: Query<&Transform, (With<Character>, Without<MK1>)>,
) {
    if mk1_query.is_empty() || player_query.is_empty() {
        return;
    }
    let playerloc = player_query.single();
    for mut mk1_transform in mk1_query.iter_mut() {
        if playerloc.translation.distance(mk1_transform.translation) >= 200.0 {
            let moveoff = Vec3::new(playerloc.translation.x - mk1_transform.translation.x, playerloc.translation.y - mk1_transform.translation.y, 0.0).normalize() * 5.0;
            mk1_transform.translation += moveoff;
        }
    }
}