use std::f32::consts::PI;
use bevy::scene::ron::de::Position;
use bevy::utils::Instant;

use bevy::math::{vec2, vec3};
use bevy::{dev_tools::states::*, prelude::*, transform};
use bevy::time::Stopwatch;
use rand::Rng;

use crate::{
    character::Character,
    gamestate::GameState,
    CursorPosition,
    configs::{BULLET_SPAWN_INTERVAL, BULLET_SPEED, BULLET_DAMAGE,},
};

pub struct GunPlugin;

#[derive(Component)]
pub struct Gun;

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct GunTimer(pub Stopwatch);

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct SpawnInstant(Instant);

#[derive(Component)]
struct BulletDirection(Vec3);

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Home), (setup_gun,setup_cursor))
        .add_systems(
            Update,(
                handle_gun_transform,
                handle_cursor_transform,
                handle_gun_fire,
            ).run_if(in_state(GameState::Home)))
        .add_systems(Update, log_transitions::<GameState>);
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
        Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::new(cursor_pos.x,cursor_pos.y,90.0)),
        Cursor,
        ));
}

fn handle_cursor_transform(
    time: Res<Time>,
    cursor_pos: Res<CursorPosition>,
    mut cursor_query: Query<&mut Transform, (With<Cursor>, Without<Character>)>,
    player_query: Query<&mut Transform, (With<Character>, Without<Cursor>)>,
) {
    if cursor_query.is_empty() {
        return;
    }
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => return,
    };
    let player_pos = player_query.single().translation.truncate();
    let mut cursor_transform = cursor_query.single_mut();
    cursor_transform.translation = vec3(cursor_pos.x + player_pos.x, 
                                        cursor_pos.y + player_pos.y, 
                                        cursor_transform.translation.z);
    //鼠标旋转
    let rotation_speed = 1.0;
    let delta_rotation = Quat::from_rotation_z(rotation_speed * time.delta_secs());
    cursor_transform.rotation *= delta_rotation;
}

fn setup_gun(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Sprite {
        image: asset_server.load("Shiroko_Gun.png"),
        ..Default::default()
        },
        Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(15.0,-215.0,31.0)),
        Gun,
        GunTimer(Stopwatch::default()),
        ));
}

fn handle_gun_transform(
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&mut Transform, (With<Character>,Without<Gun>)>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Character>)>,
) {
    if player_query.is_empty() {
        return;
    }
    if gun_query.is_empty() {
        return;
    }
    let player_pos = player_query.single().translation.truncate();
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };
    let mut gun_transform = gun_query.single_mut();
    let angle = (player_pos.y - 15.0 - cursor_pos.y).atan2(player_pos.x + 15.0 - cursor_pos.x) + PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 15.0;
    let new_gun_pos = vec2(
        player_pos.x + offset * angle.cos() - 5.0,
        player_pos.y + offset * angle.sin() - 10.0,
    );
    gun_transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
}

fn handle_gun_fire(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    // handle: Res<GlobalTextureAtlas>,
) {
    if gun_query.is_empty() {
        return;
    }

    let (gun_transform, mut gun_timer) = gun_query.single_mut();
    let gun_pos = gun_transform.translation.truncate();
    gun_timer.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }
    //test
    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        gun_timer.0.reset();
        println!("fire!");
    }
    //

    // let mut rng = rand::rng();
    // let bullet_direction = gun_transform.local_x();
    // if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
    //     gun_timer.0.reset();
    //     for _ in 0..NUM_BULLETS_PER_SHOT {
    //         //子弹散布
    //         let dir = vec3(
    //             bullet_direction.x + rng.random_range(-0.1..0.1),
    //             bullet_direction.y + rng.random_range(-0.1..0.1),
    //             bullet_direction.z,
    //         );
    //         commands.spawn((
    //             Sprite {
    //                 // image:asset_server.load("assets/Shiroko_Projectile.png"),
    //                 ..default()
    //             },
    //             Transform {
    //                 translation: vec3(gun_pos.x, gun_pos.y, 1.0),
    //                 rotation: Quat::from_rotation_z(dir.y.atan2(dir.x)),
    //                 scale: Vec3::splat(SPRITE_SCALE_FACTOR),
    //             },
    //             Bullet,
    //             BulletDirection(dir),
    //             SpawnInstant(Instant::now()),
    //         ));
    //     }
    // }
}

fn handle_bullet_spawn(
    
) {

}

fn handle_bullet_move(
    
) {

}

fn handle_bullet_collision(
    
) {

}
