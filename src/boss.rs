use bevy::state::commands;
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
pub struct Timer {
    pub timer1: Stopwatch,
    pub timer2: Stopwatch,
    pub duration_idle: Duration,
    pub duration_attack: Duration,
}

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(OnEnter(GameState::InGame), setup_boss)
            .add_systems(
                Update,
                    (
                        handle_boss_animation,
                        handle_boss_skill,
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
    set_boss(Vec2::new(0.0, 20.0), &mut commands, &source);
}

pub fn set_boss(
    loc: Vec2,
    commands: &mut Commands,
    source: &Res<GlobalBossTextureAtlas>,
) {
    let collider_box = vec![
                Vec2::new(-9.0,4.0),
                Vec2::new(-9.0,-18.0),
                Vec2::new(9.0,4.0),
                Vec2::new(9.0,-18.0)];//碰撞箱
    
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
        Direction{
            x: 0.0,
            y: 0.0,
        },
        Timer{
            timer1: Stopwatch::new(),
            timer2: Stopwatch::new(),
            duration_idle: Duration::from_millis(500),
            duration_attack: Duration::from_millis(3000),
        },
        AnimationConfig::new(15),

        RigidBody::Dynamic,
        GravityScale(0.0),
        Collider::convex_hull(&collider_box).unwrap(),
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
            Boss::Gun,
            BossState::Idea,
            AnimationConfig::new(15),
        )
    );//机枪
}

fn handle_boss_animation(

) {

}

fn handle_boss_skill(

) {

}

fn handle_bossbullet_move(

) {

}