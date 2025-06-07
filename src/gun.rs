use std::f32::consts::PI;
use bevy::ecs::event::EventCursor;
use bevy::input::keyboard;
use bevy::utils::Instant;

use bevy::math::{vec2, vec3};
use bevy::{dev_tools::states::*, prelude::*};
use bevy::time::{self, Stopwatch};
use bevy_rapier2d::prelude::*;
use rand::Rng;


use crate::character::{MK2Loc, MK2LockOn, Skill4Timer};
use crate::SKILL4_CD;
use crate::{
    character::{Character, AnimationConfig, Player, Buff, PlayerSkill4Event},
    gamestate::*,
    CursorPosition,
    GlobalCharacterTextureAtlas,
    enemy::{EnemyBullet, Enemy},
    configs::{BULLET_SPAWN_INTERVAL, 
              BULLET_SPEED, 
              BULLET_TIME_SECS,
              BULLET_DAMAGE, 
              NUM_BULLETS_PER_SHOT},
};

pub struct GunPlugin;

#[derive(Component)]
pub struct Gun;

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct GunTimer(pub Stopwatch);

#[derive(Component)]
pub struct GunFire;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletDamage(pub f32);

#[derive(Component)]
pub struct BulletHit;

#[derive(Component)]
pub struct SpawnInstant(pub Instant);

#[derive(Component)]
pub struct BulletDirection(pub Vec3);

#[derive(Component)]
pub struct ArisuSPDamage(pub i8);

#[derive(Component)]
pub enum GunState {
    Normal,
    Fire,
    SP,
}

#[derive(Event)]
pub struct PlayerFireEvent(pub usize);
#[derive(Event)]
pub struct PlayerSkill4FireEvent;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<PlayerFireEvent>()
        .add_event::<PlayerSkill4FireEvent>()
        .add_systems(OnEnter(GameState::Home), (setup_gun,setup_cursor))
        .add_systems(
        Update,(
            handle_gun_transform,
            handle_cursor_transform,
            handle_gun_fire,
            handle_mk2_fire,
            handle_utaha_attack,
            handle_bullet_move,
            despawn_old_bullets,
        ).run_if(in_state(HomeState::Running)))
        .add_systems(
            Update,(
                handle_gun_transform,
                handle_cursor_transform,
                handle_gun_fire,
                handle_mk2_fire,
                handle_utaha_attack,
                handle_bullet_move,
                despawn_old_bullets,
        ).run_if(in_state(InGameState::Running)))
        ;
    }
}

fn setup_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cursor_pos: Res<CursorPosition>,
) {
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => vec2(0.0, 0.0),
    };
    commands.spawn((Sprite {
        image: asset_server.load("FrontSight.png"),
        ..Default::default()
        },
        Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::new(cursor_pos.x,cursor_pos.y,120.0)),
        Cursor,
        Player,
        ));
}

fn handle_cursor_transform(
    mut events: EventReader<PlayerFireEvent>,
    time: Res<Time>,
    cursor_pos: Res<CursorPosition>,
    mut cursor_query: Query<&mut Transform, (With<Cursor>, Without<Character>)>,
    camera_query: Query<&mut Transform, (With<Camera2d>, Without<Character>, Without<Cursor>)>,
) {
    if cursor_query.is_empty() {
        println!("Cursor is empty!!!!!");
        return;
    }
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => return,
    };

    let camera_pos = camera_query.single().translation.truncate();
    let mut cursor_transform = cursor_query.single_mut();
    cursor_transform.translation = vec3(cursor_pos.x + camera_pos.x, 
                                        cursor_pos.y + camera_pos.y, 
                                        cursor_transform.translation.z);
    //鼠标旋转
    let rotation_speed = 1.0;
    let delta_rotation = Quat::from_rotation_z(rotation_speed * time.delta_secs());
    cursor_transform.rotation *= delta_rotation;
        
    //准心随开火抖动
    cursor_transform.scale = Vec3::splat(2.0);
    for _ in events.read() {
        cursor_transform.scale *= 1.3;
    }
    
}

fn setup_gun(
    mut commands: Commands,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    commands.spawn((Sprite {
        image: source.image_gun.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: source.lay_out_gun.clone(),
            index: 0,
        }),
        ..Default::default()
        },
        Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(15.0,-215.0,31.0)),
        AnimationConfig::new(30),
        GunState::Normal,
        Gun,
        GunTimer(Stopwatch::default()),
        ArisuSPDamage(1),
        Player,
    ));
}

fn handle_gun_transform(
    mut events: EventReader<PlayerFireEvent>,
    cursor_query: Query<&mut Transform, (With<Cursor>, Without<Gun>,Without<Character>)>,
    player_query: Query<&mut Transform, (With<Character>, Without<Gun>, Without<Cursor>)>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Character>, Without<Cursor>)>,
) {
    if cursor_query.is_empty() {
        println!("Cursor is empty!");
        return;
    }
    if player_query.is_empty() {
        println!("Player is empty!");
        return;
    }
    if gun_query.is_empty() {
        println!("Gun is empty!");
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    let cursor_pos = cursor_query.single().translation.truncate();
    let mut gun_transform = gun_query.single_mut();
    let angle = (player_pos.y - 15.0 - cursor_pos.y).atan2(player_pos.x + 15.0 - cursor_pos.x) + PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 15.0;
    let mut new_gun_pos = vec2(
        player_pos.x + offset * angle.cos() - 5.0,
        player_pos.y + offset * angle.sin() - 10.0,
    );

    // 枪随开火抖动,必须和帧率挂钩，不然太快了看不清
    for _ in events.read() {
        // println!("Gun recoil!");
        new_gun_pos -= vec2(offset * angle.cos(),
                            offset * angle.sin());
    }
    gun_transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
}

fn handle_gun_fire(
    time: Res<Time>,
    mut commands: Commands,
    mut gun_query: Query<(&mut Sprite, &mut ArisuSPDamage, &mut GunState, &Transform, &mut GunTimer), (With<Gun>, Without<Character>)>,
    mut player_query: Query<(&Buff, &mut Skill4Timer), (With<Character>, Without<Gun>)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ew: EventWriter<PlayerFireEvent>,
    mut events: EventWriter<PlayerSkill4Event>,
    mut events2: EventWriter<PlayerSkill4FireEvent>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    //枪的开火没法通用，ARISU的枪和子弹甚至有开火动画，而UTAHA居然用的不是枪
    //没法写通用的逻辑
    //后续考虑根据内存中的角色id判断角色，然后写不同的开火逻辑
    if gun_query.is_empty() || player_query.is_empty() || source.id == 3 {
        return;
    }

    let mut gun_speed = 1.0;
    let mut bullet_num = 1;
    let mut bullet_size =  1.0;
    let mut bullet_spread = 1.0;

    let (buff, mut skill_timer) = player_query.single_mut();
    bullet_num = buff.0;
    gun_speed += (buff.1 as f32 - 1.0) * 0.12;
    bullet_spread = buff.3 as f32;
    bullet_size += (buff.4 - 1) as f32 * 0.3;



    let (mut gun, mut arisu_damage, mut state, gun_transform, mut gun_timer) = gun_query.single_mut();
    let gun_pos = gun_transform.translation.truncate();
    let bullet_direction = gun_transform.local_x();

    if source.id == 2 {
        // 处理爱丽丝4技能蓄力炮
        //arisu
        skill_timer.0.tick(time.delta());
        gun_speed /= 3.0;
        if keyboard_input.pressed(KeyCode::ControlLeft) {
            if skill_timer.0.elapsed_secs() < SKILL4_CD {
                // println!("arisu skill4 cd!");
                return;
            }
            skill_timer.0.reset();
            // println!("大炮");
            match *state {
                
                GunState::SP => {
                    if let Some(atlas) = &gun.texture_atlas {
                        if atlas.index<13 && atlas.index>8 {
                            arisu_damage.0 += 1;
                            if arisu_damage.0 > 120 {
                                arisu_damage.0 = 120;
                            }
                        }
                    }
                },
                _ => {
                    *state = GunState::SP;
                    // 蓄力计伤初始化
                    arisu_damage.0 = 1;
                    gun.image = source.image_gun_fire_special.clone();
                    gun.texture_atlas = Some(TextureAtlas {
                        layout: source.layout_gun_fire_special.clone(),
                        index: 0,
                    });
                    events.send(PlayerSkill4Event);
                }
            }
            return;
            //  光之剑优先级比开火高
        }
        if keyboard_input.just_released(KeyCode::ControlLeft) {
            match *state {
                GunState::SP => {
                    // 光之剑进入发射阶段
                    if let Some(atlas) = &mut gun.texture_atlas {
                        atlas.index = 18;
                    }
                    println!("arisu damage: {}", arisu_damage.0);
                    events2.send(PlayerSkill4FireEvent);
                },
                _ =>{
                    return;
                }
            }
            //枪口焰动画
            commands.spawn((Sprite {
                image: source.image_gun_fire_effect.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: source.lay_out_gun_fire_effect.clone(),
                    index: 0,
                }),
                ..Default::default()
                },
                Transform{
                    translation: vec3(gun_pos.x + bullet_direction.x * 30.0, 
                                    gun_pos.y + bullet_direction.y * 30.0, 
                                    32.0),//深度要盖过枪
                    rotation: Quat::from_rotation_z(bullet_direction.y.atan2(bullet_direction.x)),
                    scale: Vec3::splat(2.5),
                },
                AnimationConfig::new(15),
                GunFire,
                Player,
            ));

            //光之剑生成
            commands.spawn((
                Sprite {
                    image: source.image_bullet_special.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source.layout_bullet_special.clone(),
                        index: 0,
                    }),
                    ..default()
                },
                Transform {
                    translation: vec3(
                        gun_pos.x + bullet_direction.x * 80.0, 
                        gun_pos.y + bullet_direction.y * 80.0, 
                        1.0),
                    rotation: Quat::from_rotation_z(bullet_direction.y.atan2(bullet_direction.x)),
                    scale: Vec3::splat(2.5) * bullet_size,
                },
                AnimationConfig::new(8),
                Player,
                Bullet,
                BulletDirection(bullet_direction.clone().into()),
                BulletDamage(40.0),
                SpawnInstant(Instant::now()),
                //碰撞体
                Collider::ball(bullet_size * 15.0),

                RigidBody::Dynamic,
                GravityScale(0.0),
                ColliderMassProperties::Mass(1000.0),
                LockedAxes::ROTATION_LOCKED,
                // Sensor,
                // CollisionGroups::new(Group::GROUP_3, Group::GROUP_2),
                ActiveEvents::COLLISION_EVENTS,
            ));
            return;
            //  光之剑优先级比开火高
        }
    }

    gun_timer.0.tick(time.delta());

    //如果tick在检测鼠标按键后就会出现单点不射击的情况
    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }
    // println!("mouse pressed!");

    let mut rng = rand::rng();

    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL / gun_speed {
        gun_timer.0.reset();
        if source.id == 2 {//arisu枪有开火动画
            match *state {
                GunState::SP => {},
                _ => {
                    gun.image = source.image_gun_fire.clone();
                    gun.texture_atlas = Some(TextureAtlas {
                        layout: source.layout_gun_fire.clone(),
                        index: 0,
                    });
                    *state = GunState::Fire;
                    // println!("arisu fire!");
                }
            }

        }
        ew.send(PlayerFireEvent(0));

        //枪口焰动画
        commands.spawn((Sprite {
            image: source.image_gun_fire_effect.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: source.lay_out_gun_fire_effect.clone(),
                index: 0,
            }),
            ..Default::default()
            },
            Transform{
                translation: vec3(gun_pos.x + bullet_direction.x * 30.0, 
                                  gun_pos.y + bullet_direction.y * 30.0, 
                                  32.0),//深度要盖过枪
                rotation: Quat::from_rotation_z(bullet_direction.y.atan2(bullet_direction.x)),
                scale: Vec3::splat(2.5),
            },
            AnimationConfig::new(15),
            GunFire,
            Player,
            ));

        for i in 0..bullet_num {
            // println!("{}",i);
            //子弹散布
            let mut dir = vec3(
                bullet_direction.x + rng.random_range(-0.1..0.1) / bullet_spread,
                bullet_direction.y + rng.random_range(-0.1..0.1) / bullet_spread,
                bullet_direction.z,
            );
            if i > 0 {
                // 给子弹分裂转角度
                let ang = PI / 12.0;
                let ud = if i % 2 == 0 {1.0} else {-1.0};
                let cosa = (ang * ud * ((i + 1) / 2) as f32).cos();
                let sina = (ang * ud * ((i + 1) / 2) as f32).sin();
                let rot_direction = Vec2::new(
                    bullet_direction.x * cosa + bullet_direction.y * sina,
                    bullet_direction.y * cosa - bullet_direction.x * sina,
                );

                dir = vec3(
                    rot_direction.x + rng.random_range(-0.1..0.1) / bullet_spread,
                    rot_direction.y + rng.random_range(-0.1..0.1) / bullet_spread,
                    bullet_direction.z,
                );
            }

            //子弹生成
            commands.spawn((
                Sprite {
                    image: source.image_bullet.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source.lay_out_bullet.clone(),
                        index: 0,
                    }),
                    ..default()
                },
                Transform {
                    translation: vec3(
                        gun_pos.x + bullet_direction.x * 80.0, 
                        gun_pos.y + bullet_direction.y * 80.0, 
                        1.0),
                    rotation: Quat::from_rotation_z(dir.y.atan2(dir.x)),
                    scale: Vec3::splat(2.5) * bullet_size,
                },
                AnimationConfig::new(8),
                Player,
                Bullet,
                BulletDirection(dir),
                BulletDamage(5.0),
                SpawnInstant(Instant::now()),
                //碰撞体
                Collider::cuboid(2.0 * bullet_size, 1.0 * bullet_size),

                RigidBody::Dynamic,
                GravityScale(0.0),
                ColliderMassProperties::Mass(1000.0),
                LockedAxes::ROTATION_LOCKED,
                // Sensor,
                // CollisionGroups::new(Group::GROUP_3, Group::GROUP_2),
                ActiveEvents::COLLISION_EVENTS,
            ));
        }
    }
}
fn handle_mk2_fire(
    time: Res<Time>,
    mut commands: Commands,
    mut mk2_query: Query<(&mut GunTimer, &MK2Loc, &MK2LockOn), (With<MK2Loc>, Without<Character>)>,
    player_query: Query<(&Buff), (With<Character>, Without<MK2Loc>)>,
    mut ew: EventWriter<PlayerFireEvent>,
    source: Res<GlobalCharacterTextureAtlas>,

    keyboard_input: Res<ButtonInput<KeyCode>>, // test
) {
    if mk2_query.is_empty() { 
        return;
    }
    let mut gun_speed = 1.0;
    let mut bullet_num = 1;
    let mut bullet_size =  1.0;
    let mut bullet_spread = 1.0;
    if !player_query.is_empty() { 
        let (buff) = player_query.single();
        bullet_num = buff.0;
        gun_speed += (buff.1 as f32 - 1.0) * 0.12;
        bullet_spread = buff.3 as f32;
        bullet_size += (buff.4 - 1) as f32 * 0.3;
    }
    for (mut timer, loc, lockon) in mk2_query.iter_mut() { 
        let sou = loc.0.truncate().clone();
        let tar = lockon.0.clone();
        let bullet_direction = (tar - sou).normalize();
        timer.0.tick(time.delta());
        if tar.x == 0.0 && tar.y == 0.0 || timer.0.elapsed_secs() < BULLET_SPAWN_INTERVAL / gun_speed *2.0 { 
            // 场上没有敌人
            continue;
        }
        timer.0.reset();

        let mut rng = rand::rng();

        ew.send(PlayerFireEvent(0));

        //枪口焰动画
        // 位置弄不对
        commands.spawn((Sprite {
            image: source.image_gun_fire_effect.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: source.lay_out_gun_fire_effect.clone(),
                index: 0,
            }),
            ..Default::default()
            },
            Transform{
                translation: vec3(sou.x + bullet_direction.x * 50.0, 
                                  sou.y + 25.0 + bullet_direction.y * 50.0, 
                                  32.0),//深度要盖过枪
                rotation: Quat::from_rotation_z(bullet_direction.y.atan2(bullet_direction.x)),
                scale: Vec3::splat(3.0),
            },
            AnimationConfig::new(15),
            GunFire,
            Player,
        ));
        for i in 0..bullet_num {
            // println!("{}",i);
            //子弹散布
            let mut dir = vec3(
                bullet_direction.x + rng.random_range(-0.1..0.1) / bullet_spread,
                bullet_direction.y + rng.random_range(-0.1..0.1) / bullet_spread,
                0.0,
            );
            if i > 0 {
                // 给子弹分裂转角度
                let ang = PI / 12.0;
                let ud = if i % 2 == 0 {1.0} else {-1.0};
                let cosa = (ang * ud * ((i + 1) / 2) as f32).cos();
                let sina = (ang * ud * ((i + 1) / 2) as f32).sin();
                let rot_direction = Vec2::new(
                    bullet_direction.x * cosa + bullet_direction.y * sina,
                    bullet_direction.y * cosa - bullet_direction.x * sina,
                );

                dir = vec3(
                    rot_direction.x + rng.random_range(-0.1..0.1) / bullet_spread,
                    rot_direction.y + rng.random_range(-0.1..0.1) / bullet_spread,
                    0.0,
                );
            }

            //子弹生成
            commands.spawn((
                Sprite {
                    image: source.image_bullet.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source.lay_out_bullet.clone(),
                        index: 0,
                    }),
                    ..default()
                },
                Transform {
                    translation: vec3(
                        sou.x + bullet_direction.x * 30.0, 
                        sou.y + 20.0 + bullet_direction.y * 30.0, 
                        1.0),
                    rotation: Quat::from_rotation_z(dir.y.atan2(dir.x)),
                    scale: Vec3::splat(2.5) * bullet_size,
                },
                AnimationConfig::new(8),
                Player,
                Bullet,
                BulletDirection(dir),
                BulletDamage(5.0),
                SpawnInstant(Instant::now()),
                //碰撞体
                Collider::cuboid(2.0 * bullet_size, 1.0 * bullet_size),

                RigidBody::Dynamic,
                GravityScale(0.0),
                ColliderMassProperties::Mass(1000.0),
                LockedAxes::ROTATION_LOCKED,
                // Sensor,
                // CollisionGroups::new(Group::GROUP_3, Group::GROUP_2),
                ActiveEvents::COLLISION_EVENTS,
            ));
        }
    }
}
fn handle_utaha_attack (
    time: Res<Time>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut ew: EventWriter<PlayerFireEvent>,
    mut gun_query: Query<(&mut GunState, &mut Sprite, &mut GunTimer), (With<Gun>, Without<Character>)>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    if source.id != 3 || gun_query.is_empty() {
        return;
    }
    let (mut state, mut gun, mut gun_timer) = gun_query.single_mut();
    gun_timer.0.tick(time.delta());
    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }
    let gun_speed = 0.5;
    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL / gun_speed {
        gun_timer.0.reset();
        match *state {
            GunState::Normal => {
                *state = GunState::Fire;
                gun.image = source.image_attack.clone();
                gun.texture_atlas = Some(TextureAtlas {
                    layout: source.layout_attack.clone(),
                    index: 0,
                });
                ew.send(PlayerFireEvent(1));
            },
            _ => {}
        }
    }
}

fn handle_bullet_move(
    mut bullet_query: Query<(&mut Transform, &BulletDirection), (With<Bullet>,Without<Buff>)>,
    buff_query: Query<&Buff, (With<Character>, Without<Bullet>)>,
) {
    if bullet_query.is_empty() {
        return;
    }
    let mut speed= 1.0;
    if !buff_query.is_empty() {
        speed += 0.7 * (buff_query.single().2 - 1) as f32;
    }
    for (mut t, dir) in bullet_query.iter_mut() {
        t.translation += dir.0.normalize() * Vec3::splat(BULLET_SPEED) * speed;
        t.translation.z = 30.0;
        // println!("here!,{:?}", dir.0);
    }
}


fn despawn_old_bullets(
    mut commands: Commands,
    bullet_query: Query<(&SpawnInstant, Entity, &Transform, &BulletDamage), (With<Bullet>, Without<Character>)>,
    enemy_bullet_query: Query<Entity, (Without<Bullet>, With<EnemyBullet>)>,
    enemy_query: Query<Entity, (Without<Bullet>, With<Enemy>)>,
    player_query: Query<Entity, (With<Character>, Without<Bullet>)>,
    mut collision_events: EventReader<CollisionEvent>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    for (instant, e, _, _) in bullet_query.iter() {
        if instant.0.elapsed().as_secs_f32() > BULLET_TIME_SECS {
            // println!("Despawning bullet!");
            commands.entity(e).despawn();
        }
    }
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let mut flag = false;
                let mut trans = Vec3::splat(-100.0);
                if let Ok((_, _, transf, damage)) = bullet_query.get(*entity1) {
                    if !bullet_query.get(*entity2).is_ok() && !player_query.get(*entity2).is_ok() {
                        // 子弹之间和子弹与玩家不碰撞
                        if source.id !=2 || (!enemy_bullet_query.get(*entity2).is_ok() && (damage.0 < 30.0 || !enemy_query.get(*entity2).is_ok())) {
                            // 爱丽丝的光之剑不会和敌人子弹碰撞
                            commands.entity(*entity1).despawn();
                            trans = transf.translation;
                            flag = true;
                        }
                    }
                }
                if let Ok((_, e, transf, damage)) = bullet_query.get(*entity2) {
                    if !bullet_query.get(*entity1).is_ok() && !player_query.get(*entity1).is_ok()  {
                        if source.id !=2 || (!enemy_bullet_query.get(*entity1).is_ok() && (damage.0 < 30.0 || !enemy_query.get(*entity1).is_ok())) {
                            commands.entity(*entity2).despawn();
                            trans = transf.translation;
                            flag = true;
                        }
                    }
                }
                if flag {
                    //产生子弹消失的特效
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
                            translation: trans.clone(),
                            scale: Vec3::splat(2.5),
                            ..default()
                        },
                        AnimationConfig::new(15),
                        BulletHit,
                    ));
                }
            },
            _ => {},
        }
    }
}

fn handle_bullet_collision(
    
) {

}
