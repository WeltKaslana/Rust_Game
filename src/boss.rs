use bevy::render::texture;
use bevy::state::commands;
use bevy::transform;
use bevy::{dev_tools::states::*, prelude::*, time::Stopwatch};
use crate::{gamestate::GameState,
    configs::*,character::*};
use crate::*;
use rand::Rng;
use character::AnimationConfig;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub struct BossPlugin;

#[derive(Component)]
pub enum Boss {
    Body,
    Gun,
    Missile,
    Shield,    
}

#[derive(Component)]
pub struct BossComponent;

#[derive(Component)]
pub enum BossState {
    Idea,
    CollideStart,
    CollideLoop,
    CollideEnd,
    Missilefire,
    Gunfire,
    Move,
}

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Direction {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Skillflag(pub u8);

#[derive(Component)]
pub struct Timer {
    pub timer1: Stopwatch
}

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), setup_boss)
            .add_systems(
                Update,
                    (
                        handle_boss_animation,
                        handle_boss_skill,
                        handle_bossbullet,
                        handle_bossbullet_move,
                ).run_if(in_state(GameState::InGame))
            )
            .add_systems(Update, log_transitions::<GameState>)
            ;
    }
}

fn setup_boss (
    source: Res<GlobalBossTextureAtlas>,
    mut commands: Commands,
) {
    set_boss(Vec2::new(-130.0, 140.0), &mut commands, &source);
}

pub fn set_boss(
    loc: Vec2,
    commands: &mut Commands,
    source: &Res<GlobalBossTextureAtlas>,
) { 
    let mut boss = 
    commands.spawn( (
        Sprite {
            image: source.image_boss_idle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: source.layout_boss_idle.clone(),
                index: 0,
            }),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(loc.x, loc.y, -50.0)),
        BossState::Idea,
        Boss::Body,
        Health(BOOS_HEALTH),
        Skillflag(0),
        Direction{
            x: 0.0,
            y: 0.0,
        },
        Timer{
            timer1: Stopwatch::new(),
        },
        AnimationConfig::new(15),

        RigidBody::Fixed,
        // GravityScale(0.0),
        Collider::ball(33.0),
        LockedAxes::ROTATION_LOCKED,//防止旋转
        ActiveEvents::COLLISION_EVENTS,
        KinematicCharacterController {
            ..Default::default()
        },


        )
    );

    boss.with_child(
        (
            Sprite {
                image: source.image_weaponmissile_idle.clone(),
                texture_atlas: Some(TextureAtlas { 
                    layout: source.layout_weaponmissile_idle.clone(), 
                    index: 0,
                }),
                ..Default::default()
            },
            Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(0.0, 0.0, 2.0)),
            BossComponent,
            Boss::Missile,
            BossState::Idea,
            AnimationConfig::new(15),
        )
    );//导弹仓

    boss.with_child(
        (
            Sprite {
                image: source.image_weaponlid_idle.clone(),
                texture_atlas: Some(TextureAtlas { 
                    layout: source.layout_weaponlid_idle.clone(), 
                    index: 0,
                }),
                ..Default::default()
            },
            Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(0.0, 0.0, 2.0)),
            BossComponent,
            Boss::Shield,
            BossState::Idea,
            AnimationConfig::new(15),
        )
    );//机枪盖

    boss.with_child(
        (
            Sprite {
                image: source.image_weapongun_idle.clone(),
                texture_atlas: Some(TextureAtlas { 
                    layout: source.layout_weapongun_idle.clone(), 
                    index: 0,
                }),
                ..Default::default()
            },
            Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(-25.0, 0.0, 1.0)),
            BossComponent,
            Boss::Gun,
            BossState::Idea,
            AnimationConfig::new(15),
        )
    );//机枪
}

fn handle_boss_animation(
    mut boss_query: Query<(
        &mut Sprite,
        & BossState,
        & Boss
    ), With<Boss>>,
    source: Res<GlobalBossTextureAtlas>,
) {
    if boss_query.is_empty() {
        return;
    }
    
    for (mut boss, bossstate,bosscomponent) in boss_query.iter_mut() {
        match bosscomponent {
            Boss::Body => {
                match bossstate {
                    BossState::Idea => {
                        boss.image = source.image_boss_idle.clone();
                        if let Some(atlas) = &mut boss.texture_atlas {
                            atlas.layout = source.layout_boss_idle.clone();
                        }
                    },
                    BossState::Move => {
                        boss.image = source.image_boss_move.clone();
                        if let Some(atlas) = &mut boss.texture_atlas {
                            atlas.layout = source.layout_boss_move.clone();
                        }
                    },
                    BossState::CollideStart => {
                        boss.image = source.image_boss_collide_start.clone();
                        if let Some(atlas) = &mut boss.texture_atlas {
                            atlas.layout = source.layout_boss_collide_start.clone();
                        }
                    },
                    BossState::CollideLoop => {
                        boss.image = source.image_boss_collide_loop.clone();
                        if let Some(atlas) = &mut boss.texture_atlas {
                            atlas.layout = source.layout_boss_collide_loop.clone();
                        }
                    },
                    BossState::CollideEnd => {
                        boss.image = source.image_boss_collide_end.clone();
                        if let Some(atlas) = &mut boss.texture_atlas {
                            atlas.layout = source.layout_boss_collide_end.clone();
                        }
                    },
                    _=> { },
                }
            },
            Boss::Gun => {
                match bossstate {
                    BossState::Gunfire => {
                        boss.image = source.image_weapongun_fire.clone();
                        if let Some(atlas) = &mut boss.texture_atlas {
                            atlas.layout = source.layout_weapongun_fire.clone();
                        }
                    },
                    _=> {
                        boss.image = source.image_weapongun_idle.clone();
                        if let Some(atlas) = &mut boss.texture_atlas {
                            atlas.layout = source.layout_weapongun_idle.clone();
                        }
                        //println!("1");
                    },
                }
            },
            Boss::Missile => {
                match bossstate {
                    BossState::Missilefire => {
                        boss.image = source.image_weaponmissile_fire.clone();
                        if let Some(atlas) = &mut boss.texture_atlas {
                            atlas.layout = source.layout_weaponmissile_fire.clone();
                        }
                    },
                    _=> {
                        boss.image = source.image_weaponmissile_idle.clone();
                        if let Some(atlas) = &mut boss.texture_atlas {
                            atlas.layout = source.layout_weaponmissile_idle.clone();
                        }
                    },
                }
            },
            Boss::Shield => {
                match bossstate {
                    BossState::Gunfire => {
                        boss.image = source.image_weaponlid_fire.clone();
                        if let Some(atlas) = &mut boss.texture_atlas {
                            atlas.layout = source.layout_weaponlid_fire.clone();
                        }
                    },
                    _=> {
                        boss.image = source.image_weaponlid_idle.clone();
                        if let Some(atlas) = &mut boss.texture_atlas {
                            atlas.layout = source.layout_weaponlid_idle.clone();
                        }
                    },
                }
            },
        }
    }
}

fn handle_boss_skill(
    mut boss_query: Query<(
        &mut Sprite,
        & Transform,
        &mut BossState, 
        & Health,
        &mut Timer,
        &mut Direction,
        &mut Skillflag,
        &mut KinematicCharacterController
    ), (With<Boss>, Without<BossComponent>, Without<Character>)>,
    time: Res<Time>,
    mut play_query: Query<& Transform, (With<Character>, Without<Boss>, Without<BossComponent>)>,
    mut bosscomponent_query: Query<(
        &mut Sprite,
        &mut BossState,
        & Boss
    ), (With<Boss>, With<BossComponent>, Without<Character>)>,
) {
    if boss_query.is_empty() || play_query.is_empty() || bosscomponent_query.is_empty() {
        return;
    }
    let (mut boss, bossloc,mut bossstate, health, mut timer, mut direction, mut flag, mut controller) = boss_query.single_mut();
    let (playerloc) = play_query.single_mut();
    let dx = playerloc.translation.x - bossloc.translation.x;
    let dy = playerloc.translation.y - bossloc.translation.y;
    let mut rng = rand::rng();
    if health.0 > BOOS_HEALTH/2.0 {
        match *bossstate {
            BossState::Idea => {
                timer.timer1.tick(time.delta());
                if timer.timer1.elapsed() >= Duration::from_millis(300) && flag.0 == 0 {
                    if let Some(atlas) = &mut boss.texture_atlas {
                        if atlas.index == 3 {
                            atlas.index = 0;
                            timer.timer1.reset();
                            let random_index = rng.random_range(0..100);
                            match random_index {
                                0..5 => {//继续停留
                                    *bossstate = BossState::Idea;
                                },
                                5..45 => {//冲撞技能
                                    flag.0 = 1;
                                    *bossstate = BossState::CollideStart;
                                    direction.x = dx;
                                    direction.y = dy;
                                },
                                45..75 => {//机枪开火
                                    flag.0 = 1;
                                    *bossstate = BossState::Idea;
                                    for (mut bosscomponent, mut componentstate, component) in bosscomponent_query.iter_mut() {
                                        match component {
                                            Boss::Gun => {
                                                match *componentstate {
                                                    BossState::Gunfire => { },
                                                    _=>{
                                                        *componentstate = BossState::Gunfire;
                                                        if let Some(componentatlas) = &mut bosscomponent.texture_atlas {
                                                            componentatlas.index = 0;
                                                        }
                                                    },
                                                }
                                            },
                                            Boss::Shield => {
                                                match *componentstate {
                                                    BossState::Gunfire => { },
                                                    _=>{
                                                        *componentstate = BossState::Gunfire;
                                                        if let Some(componentatlas) = &mut bosscomponent.texture_atlas {
                                                            componentatlas.index = 0;
                                                        }
                                                    },
                                                }
                                            },
                                            _=> { },
                                        }
                                    }
                                },
                                75..100 => {//导弹开火
                                    flag.0 = 1;
                                    *bossstate = BossState::Idea;
                                    for (mut bosscomponent, mut componentstate, component) in bosscomponent_query.iter_mut() {
                                        match component {
                                            Boss::Missile => {
                                                match *componentstate {
                                                    BossState::Missilefire => { },
                                                    _=>{
                                                        *componentstate = BossState::Missilefire;
                                                        if let Some(componentatlas) = &mut bosscomponent.texture_atlas {
                                                            componentatlas.index = 0;
                                                        }
                                                    },
                                                }
                                            },
                                            _=> { },
                                        }
                                    }
                                },
                                _=> {timer.timer1.reset();},
                            }
                        }
                    }
                }
            },
            BossState::Move => { },
            BossState::CollideStart => {
                let dir =Vec2::new(direction.x, direction.y).normalize();
                controller.translation =  Some(dir.normalize_or_zero().clone() * BOSS_CHARGE_SPEED);
                if let Some(atlas) = &mut boss.texture_atlas {
                    if atlas.index == 9 {
                        atlas.index = 0;
                        *bossstate = BossState::CollideLoop;
                    }
                }
            },
            BossState::CollideLoop => {
                let dir =Vec2::new(direction.x, direction.y).normalize();
                controller.translation =  Some(dir.normalize_or_zero().clone() * BOSS_CHARGE_SPEED);
                timer.timer1.tick(time.delta());
                if timer.timer1.elapsed() >= Duration::from_millis(1000) {
                    if let Some(atlas) = &mut boss.texture_atlas {
                        if atlas.index == 7 {
                            atlas.index = 0;
                            *bossstate = BossState::CollideEnd;
                        }
                    }
                    timer.timer1.reset();
                }
            },
            BossState::CollideEnd => {
                let dir =Vec2::new(direction.x, direction.y).normalize();
                controller.translation =  Some(dir.normalize_or_zero().clone() * BOSS_CHARGE_SPEED);
                if let Some(atlas) = &mut boss.texture_atlas {
                    if atlas.index == 1 {
                        atlas.index = 0;
                        *bossstate = BossState::Idea;
                    }
                    flag.0 = 0;
                }
            },
            BossState::Gunfire => { },
            BossState::Missilefire => { },
        }
    } else {

    }
}

fn handle_bossbullet(

) {
    
}

fn handle_bossbullet_move(

) {

}