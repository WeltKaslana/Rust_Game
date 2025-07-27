use bevy::audio::Source;
use bevy::render::texture;
use bevy::state::commands;
use bevy::transform;
use bevy::{dev_tools::states::*, prelude::*, time::Stopwatch};
use crate::gun::BulletDamage;
use crate::{gamestate::*,
    configs::*,character::{*, Health}, gun::Bullet, enemy::*, room::{Map, EnemyBorn},};
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
pub struct BossDeathEffect;

#[derive(Component)]
pub struct BossBullet;

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

// #[derive(Component)]
// pub struct Health(pub f32);

#[derive(Component)]
pub struct Direction {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Skillflag(pub u8);

#[derive(Component)]
pub struct Timer {
    pub timer1: Stopwatch,
    pub timer2: Stopwatch,
}

#[derive(Event)]
pub struct BossSetupEvent;

#[derive(Event)]
pub struct BossDeathEvent;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BossSetupEvent>()
            .add_event::<BossDeathEvent>()
            .add_systems(
                Update,
                    (
                        handle_boss_animation,
                        handle_boss_skill,
                        handle_bossbullet_setup,
                        handle_bossgun_rotation,
                        handle_boss_hurt,
                        handle_boss_death,
                        handle_boss_charge_hurt,
                ).run_if(in_state(InGameState::Running))
            )
            ;
    }
}

// fn setup_boss (
//     source: Res<GlobalBossTextureAtlas>,
//     mut commands: Commands,
// ) {
//     set_boss(Vec2::new(-130.0, 140.0), &mut commands, &source);
// }

pub fn set_boss(
    loc: Vec2,
    commands: &mut Commands,
    source: &Res<GlobalBossTextureAtlas>,
    score: &ResMut<ScoreResource>,
) { 

    let mut xishu = score.boss_score as f32;
    xishu = 1.0 + xishu / 100.0 * 50.0;

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
        Health(BOSS_HEALTH * xishu),
        Skillflag(0),
        Direction{
            x: 0.0,
            y: 0.0,
        },
        Timer{
            timer1: Stopwatch::new(),
            timer2: Stopwatch::new(),
        },
        AnimationConfig::new(15),
    ));
    boss.insert((
        RigidBody::Dynamic,
        GravityScale(0.0),
        Collider::ball(33.0),
        LockedAxes::ROTATION_LOCKED,//防止旋转
        ActiveEvents::COLLISION_EVENTS,
        KinematicCharacterController {
            filter_groups: Some(CollisionGroups::new(Group::GROUP_3, Group::GROUP_4)),
            ..Default::default()
        },
        ColliderMassProperties::Mass(15000.0),
        CollisionGroups::new(Group::GROUP_3, Group::GROUP_4),
        Map,
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
            AnimationConfig::new(10),
            Timer{
                timer1: Stopwatch::new(),
                timer2: Stopwatch::new(),
            },
            Skillflag(0),
            Map,
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
            AnimationConfig::new(30),
            Timer{
                timer1: Stopwatch::new(),
                timer2: Stopwatch::new(),
            },
            Skillflag(0),
            Map,
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
            AnimationConfig::new(30),
            Timer{
                timer1: Stopwatch::new(),
                timer2: Stopwatch::new(),
            },
            Skillflag(0),
            Map,
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
    score: Res<ScoreResource>,
) {
    if boss_query.is_empty() || play_query.is_empty() || bosscomponent_query.is_empty() {
        return;
    }

    let mut xishu = score.boss_score as f32;
    xishu = 1.0 + xishu / 100.0 * 50.0;

    let (mut boss, bossloc,mut bossstate, health, mut timer, mut direction, mut flag, mut controller) = boss_query.single_mut();
    let playerloc = play_query.single_mut();
    let dx = playerloc.translation.x - bossloc.translation.x;
    let dy = playerloc.translation.y - bossloc.translation.y;
    let mut rng = rand::rng();
    if health.0 > BOSS_HEALTH * xishu /2.0 {
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
                            timer.timer1.reset();
                        }
                    }
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
        match *bossstate {
            BossState::Idea => {
                timer.timer1.tick(time.delta());
                if timer.timer1.elapsed() >= Duration::from_millis(300) {
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
                                    *bossstate = BossState::CollideStart;
                                    direction.x = dx;
                                    direction.y = dy;
                                },
                                45..75 => {//机枪开火
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
                timer.timer1.tick(time.delta());
                if timer.timer1.elapsed() >= Duration::from_millis(300) {
                    timer.timer1.reset();
                    let random_index = rng.random_range(0..100);
                    match random_index {
                        0..5 => {//继续停留
                        },
                        5..15 => {//冲撞技能
                        },
                        15..65 => {//机枪开火
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
                        65..100 => {//导弹开火
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
            },
            BossState::CollideLoop => {
                let dir =Vec2::new(direction.x, direction.y).normalize();
                controller.translation =  Some(dir.normalize_or_zero().clone() * BOSS_CHARGE_SPEED);
                timer.timer2.tick(time.delta());
                if timer.timer2.elapsed() >= Duration::from_millis(1000) {
                    if let Some(atlas) = &mut boss.texture_atlas {
                        if atlas.index == 7 {
                            atlas.index = 0;
                            *bossstate = BossState::CollideEnd;
                            timer.timer2.reset();
                        }
                    }
                }
                timer.timer1.tick(time.delta());
                if timer.timer1.elapsed() >= Duration::from_millis(300) {
                    timer.timer1.reset();
                    let random_index = rng.random_range(0..100);
                    match random_index {
                        0..5 => {//继续停留
                        },
                        5..15 => {//冲撞技能
                        },
                        15..65 => {//机枪开火
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
                        65..100 => {//导弹开火
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
            },
            BossState::CollideEnd => {
                let dir =Vec2::new(direction.x, direction.y).normalize();
                controller.translation =  Some(dir.normalize_or_zero().clone() * BOSS_CHARGE_SPEED);
                if let Some(atlas) = &mut boss.texture_atlas {
                    if atlas.index == 1 {
                        atlas.index = 0;
                        *bossstate = BossState::Idea;
                    }
                }
                timer.timer1.tick(time.delta());
                if timer.timer1.elapsed() >= Duration::from_millis(300) {
                    timer.timer1.reset();
                    let random_index = rng.random_range(0..100);
                    match random_index {
                        0..5 => {//继续停留
                        },
                        5..15 => {//冲撞技能
                        },
                        15..65 => {//机枪开火
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
                        65..100 => {//导弹开火
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
            },
            BossState::Gunfire => { },
            BossState::Missilefire => { },
        }
    }
}

fn handle_bossbullet_setup(
    mut commands: Commands,
    //source: &Res<GlobalBossbulletTextureAtlas>,
    mut bosscomponent_query: Query<(
        &mut Sprite,
        &mut BossState,
        & Boss,
        &mut Timer,
        &mut Skillflag,
    ), (With<Boss>, With<BossComponent>, Without<Character>)>,
    mut play_query: Query<& Transform, (With<Character>, Without<Boss>, Without<BossComponent>)>,
    mut boss_query: Query<(
        & Transform,
        &mut Skillflag,
        & Direction
    ), (With<Boss>, Without<BossComponent>, Without<Character>)>,
    time: Res<Time>,
    source: Res<GlobalEnemyTextureAtlas>,
) {
    if bosscomponent_query.is_empty() || play_query.is_empty() || boss_query.is_empty() {
        return;
    }

    let (boss_transform, mut flag, boss_direction) = boss_query.single_mut();
    let player_transform = play_query.single_mut();

    for (mut boss_component, mut component_state, component, mut timer, mut fireflag) in bosscomponent_query.iter_mut() {
        match component {
            Boss::Gun => {
                match *component_state {
                    BossState::Gunfire => {
                        if let Some(atlas) = &mut boss_component.texture_atlas {
                            if atlas.index == 6 && fireflag.0 == 0 {
                                fireflag.0 = 1;
                                if boss_direction.x >= 0.0 {
                                    commands.spawn( (
                                        Sprite {
                                            image: source.image_unknown_bullet.clone(),
                                            texture_atlas: Some(TextureAtlas {
                                                layout: source.layout_unknown_bullet.clone(),
                                                index: 0,
                                            }),
                                            ..Default::default()
                                        },
                                        Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(boss_transform.translation.x - 25.0, boss_transform.translation.y, 31.0)),
                                        EnemyBullet::UnknownGuardian,
                                        BulletDirection {
                                            x : player_transform.translation.x - boss_transform.translation.x + 25.0,
                                            y : player_transform.translation.y - boss_transform.translation.y,},
                                        AnimationConfig::new(5),
                                        Sensor,
                                        RigidBody::Dynamic,
                                        GravityScale(0.0),
                                        Collider::cuboid(6.0, 6.0),
                                        ActiveEvents::COLLISION_EVENTS,
                                        Map,
                                        )
                                    );
                                } else {
                                    commands.spawn( (
                                        Sprite {
                                            image: source.image_unknown_bullet.clone(),
                                            texture_atlas: Some(TextureAtlas {
                                                layout: source.layout_unknown_bullet.clone(),
                                                index: 0,
                                            }),
                                            ..Default::default()
                                        },
                                        Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(boss_transform.translation.x + 25.0 , boss_transform.translation.y, 31.0)),
                                        EnemyBullet::UnknownGuardian,
                                        BulletDirection {
                                            x : player_transform.translation.x - boss_transform.translation.x - 25.0,
                                            y : player_transform.translation.y - boss_transform.translation.y,},
                                        AnimationConfig::new(5),
                                        Sensor,
                                        RigidBody::Dynamic,
                                        GravityScale(0.0),
                                        Collider::cuboid(6.0, 6.0),
                                        ActiveEvents::COLLISION_EVENTS,
                                        Map,
                                        )
                                    );
                                }
                            }
                        }
                        timer.timer1.tick(time.delta());
                        if timer.timer1.elapsed() >= Duration::from_millis(1000) {
                            if let Some(atlas) = &mut boss_component.texture_atlas {
                                if atlas.index == 6 {
                                    atlas.index = 0;
                                    timer.timer1.reset();
                                    *component_state = BossState::Idea;
                                    flag.0 = 0;
                                }
                            }
                        }
                    },
                    _=> { },
                }
            },
            Boss::Shield => {
                match *component_state {
                    BossState::Gunfire => {
                        timer.timer1.tick(time.delta());
                        if timer.timer1.elapsed() >= Duration::from_millis(1000) {
                            if let Some(atlas) = &mut boss_component.texture_atlas {
                                if atlas.index == 6 {
                                    atlas.index = 0;
                                    timer.timer1.reset();
                                    *component_state = BossState::Idea;
                                }
                            }
                        }
                    },
                    _=> { },
                }
            },
            Boss::Missile => {
                match *component_state {
                    BossState::Missilefire => {
                        if let Some(atlas) = &mut boss_component.texture_atlas {
                            if (atlas.index == 19 || atlas.index ==20 || atlas.index ==21) && fireflag.0 == 0 {
                                fireflag.0 = 1;
                                if boss_direction.x >= 0.0 {
                                    commands.spawn( (
                                        Sprite {
                                            image: source.image_missile_bullet.clone(),
                                            texture_atlas: Some(TextureAtlas {
                                                layout: source.layout_missile_bullet.clone(),
                                                index: 0,
                                            }),
                                            ..Default::default()
                                        },
                                        Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(boss_transform.translation.x, boss_transform.translation.y + 40.0, 31.0)),
                                        EnemyBullet::DroneMissile,
                                        BulletDirection {
                                            x : 10.0,
                                            y : 10.0,},
                                        AnimationConfig::new(5),
                                        Sensor,
                                        RigidBody::Dynamic,
                                        GravityScale(0.0),
                                        Collider::cuboid(11.0, 5.0),
                                        ActiveEvents::COLLISION_EVENTS,
                                        Map,
                                        )
                                    );
                                } else {
                                    commands.spawn( (
                                        Sprite {
                                            image: source.image_missile_bullet.clone(),
                                            texture_atlas: Some(TextureAtlas {
                                                layout: source.layout_missile_bullet.clone(),
                                                index: 0,
                                            }),
                                            ..Default::default()
                                        },
                                        Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(boss_transform.translation.x, boss_transform.translation.y + 40.0, 31.0)),
                                        EnemyBullet::DroneMissile,
                                        BulletDirection {
                                            x : -10.0,
                                            y : 10.0,},
                                        AnimationConfig::new(5),
                                        Sensor,
                                        RigidBody::Dynamic,
                                        GravityScale(0.0),
                                        Collider::cuboid(11.0, 5.0),
                                        ActiveEvents::COLLISION_EVENTS,
                                        Map,
                                        )
                                    );
                                }
                            }else if atlas.index == 29 {
                                atlas.index = 0;
                                *component_state = BossState::Idea;
                                flag.0 = 0;
                            }
                        }
                    },
                    _=> { },
                }
            },
            _=> { },
        }
    }
}

fn handle_bossgun_rotation(
    mut gun_query: Query<(
        &mut Transform,
        & Boss
    ), (With<Boss>, With<BossComponent>, Without<Character>)>,
    player_query : Query<& Transform, (With<Character>, Without<Boss>, Without<BossComponent>)>,
    boss_query :Query<(& Transform, & Direction), (With<Boss>, Without<BossComponent>, Without<Character>)>,
) {
    if gun_query.is_empty() || player_query.is_empty() || boss_query.is_empty() {
        return;
    }
    let playtransfrom = player_query.single();
    let (bosstransform, direction) = boss_query.single();
    let mut dx =playtransfrom.translation.x - bosstransform.translation.x;
    let dy =playtransfrom.translation.y - bosstransform.translation.y;
    if direction.x >= 0.0 {
        dx = dx + 25.0;
    }else {
        dx = dx - 25.0;
    }
    let angle = (dy).atan2(dx);
    // if direction.x <= 0.0 {
    //     angle = angle + PI;
    // }
    for (mut componenttransform, component) in gun_query.iter_mut() {
        match component {
            Boss::Gun => {
                componenttransform.rotation = Quat::from_rotation_z(angle);
            },
            _=> { },
        }
    }
}

fn handle_boss_death(
    mut commands: Commands,
    mut boss_query: Query<(Entity, & Transform, & Health), (With<Boss>, Without<BossComponent>)>,
    mut bosscomponent_query: Query<Entity, (With<Boss>, With<BossComponent>)>,
    source: Res<GlobalBossTextureAtlas>,
    source1: Res<GlobalEnemyTextureAtlas>,
    mut events: EventWriter<BossDeathEvent>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    born_query: Query<Entity, With<EnemyBorn>>,
    mut enemybornpoint_query: Query<&mut Enemyterm, With<Enemybornflag>>,
) {
    if boss_query.is_empty() {
        return;
    }
    let (entity, loc, health) = boss_query.single_mut();
    if health.0 <= 0.0 {
        events.send(BossDeathEvent);
        commands.entity(entity).despawn();
        for bosscomponent in bosscomponent_query.iter_mut(){ 
            commands.entity(bosscomponent).despawn();
        }
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
            BossDeathEffect,
            Map,
        )
        );

        for e in born_query.iter() {
            commands.entity(e).despawn();
        }
        
        for (entity, loc) in enemy_query.iter() {
            commands.entity(entity).despawn();
            commands.spawn( (
                Sprite {
                    image: source1.image_death.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source1.layout_death.clone(),
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
        }
        for mut enemyterm in enemybornpoint_query.iter_mut() {
            enemyterm.0 = 0;
        }
    }
}

fn handle_boss_hurt(
    buff_query: Query<&Buff>,
    player_query: Query<(Entity, &BulletDamage), With<Bullet>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut boss_query: Query<(Entity, &mut Health), (With<Boss>, Without<BossComponent>)>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    if player_query.is_empty() || boss_query.is_empty() {
        return;
    }

    let buff = buff_query.single();
    let vulnerable = match source.id {
        // 不同子弹伤害不同，后续可以叠加dot之类的伤害
        2 => 3.0,
        _ => 1.0,
    };
    let bulletdamage = ((buff.4 - 1) as f32 * 0.25 + 1.0) * vulnerable;
    
    for collision_event in collision_events.read() {
        let (boss, mut health) =  boss_query.single_mut();
            match collision_event {
                CollisionEvent::Started(entity1,entity2, _) => {
                    if entity2.eq(&boss) {
                        if let Ok((_, arisu)) = player_query.get(*entity1) {
                            // 还有arisu的光之剑伤害
                            health.0 -= BULLET_DAMAGE * bulletdamage * match arisu.0 {
                                x if x > 30.0 => (x * 0.25) * (1.0 + (buff.5 - 1) as f32 * 0.05),
                                x if x == 30.0 => 3.0 * (1.0 + (buff.5 - 1) as f32 * 0.05),
                                _ => 1.0
                            };
                            // println!("boss health: {}", health.0);
                        }
                    }
                    if entity1.eq(&boss) {
                        if let Ok((_, arisu)) = player_query.get(*entity1) {
                            // 还有arisu的光之剑伤害
                            health.0 -= BULLET_DAMAGE * bulletdamage * match arisu.0 {
                                x if x > 30.0 => (x * 0.25) * (1.0 + (buff.5 - 1) as f32 * 0.05),
                                x if x == 30.0 => 3.0 * (1.0 + (buff.5 - 1) as f32 * 0.05),
                                _ => 1.0
                            };
                            // println!("boss health: {}", health.0);
                        }
                    }
                },
                CollisionEvent::Stopped(entity1, entity2, _) => { },
            }
        }
}

fn handle_boss_charge_hurt(
    mut play_query: Query<(& Transform, &mut character::Health, & PlayerState), (With<Character>, Without<Boss>, Without<BossComponent>)>,
    boss_query :Query<(& Transform, & BossState), (With<Boss>, Without<BossComponent>, Without<Character>)>,
    mut events: EventWriter<PlayerHurtEvent>,
) {
    if boss_query.is_empty() || play_query.is_empty() {
        return;
    }
    let (bossloc, bossstate) = boss_query.single();
    let (playerloc, mut health, playerstate) = play_query.single_mut();

    let dx =playerloc.translation.x - bossloc.translation.x;
    let dy =playerloc.translation.y - bossloc.translation.y;
    
    match bossstate {
        BossState::CollideStart | BossState::CollideLoop | BossState::CollideEnd => {
            match playerstate {
                PlayerState::Dodge => { },
                _=> {
                    if (dx * dx + dy * dy).sqrt() <= 100.0 {
                        let health_update = ENEMY_DAMAGE / -2.0;
                        // health.0 -= ENEMY_DAMAGE / 2.0;
                        events.send(PlayerHurtEvent(health_update, 0));
                    }
                }
            }
        },
        _=> { },
    }
}

