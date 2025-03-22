use std::f32::consts::PI;
use bevy::scene::ron::de::Position;
use bevy::utils::Instant;

use bevy::math::{vec2, vec3};
use bevy::{prelude::*, transform};
use bevy::time::Stopwatch;
use rand::Rng;

use crate::character::Character;
use crate::gamestate::GameState;
use crate::*;

pub struct GunPlugin;

#[derive(Component)]
pub struct Gun;

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
        app.add_systems(
            Update,
            (
                handle_gun_transform,
                handle_gun_fire,
                handle_bullet_spawn,
                handle_bullet_move,
                handle_bullet_collision,
            ).run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_gun_transform(
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&mut Transform, (With<Gun>,Without<Character>)>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Character>)>,
) {
    if player_query.is_empty() {
        return;
    }
    let player_pos = player_query.single().translation.truncate();
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };
    let mut gun_transform = gun_query.single_mut();
    let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
    gun_transform.rotation = Quat::from_rotation_z(angle);

    //?
    let offset = 20.0;
    let new_gun_pos = vec2(
        player_pos.x + offset * angle.cos() - 5.0,
        player_pos.y + offset * angle.sin() - 10.0,
    );
    //
    gun_transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
    gun_transform.translation.z = 15.0;
}

fn handle_gun_fire(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    handle: Res<GlobalTextureAtlas>,
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
    
    let mut rng = rand::rng();
    let bullet_direction = gun_transform.local_x();
    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        gun_timer.0.reset();
        for _ in 0..NUM_BULLETS_PER_SHOT {
            //子弹散布
            let dir = vec3(
                bullet_direction.x + rng.random_range(-0.1..0.1),
                bullet_direction.y + rng.random_range(-0.1..0.1),
                bullet_direction.z,
            );
            commands.spawn((
                Sprite {
                    // image:asset_server.load("assets/Shiroko_Projectile.png"),
                    ..default()
                },
                Transform {
                    translation: vec3(gun_pos.x, gun_pos.y, 1.0),
                    rotation: Quat::from_rotation_z(dir.y.atan2(dir.x)),
                    scale: Vec3::splat(SPRITE_SCALE_FACTOR),
                },
                Bullet,
                BulletDirection(dir),
                SpawnInstant(Instant::now()),
            ));
        }
    }
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
