use bevy::math::vec3;
use bevy::prelude::*;
use std::{sync::Arc, time::Duration};

use crate::gamestate::GameState;
use crate::resources::Shiroko;
use crate::*;
pub struct PlayerPlugin;
#[derive(Component)]
pub struct Character;
#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Jump,
    Move,
}

#[derive(Event)]
pub struct PlayerEnemyCollisionEvent;

//定义角色动画帧率
#[derive(Component)]

struct AnimationConfig {
    fps2p: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(fps2p: u8) -> Self {
        Self {
            fps2p,
            frame_timer: Self::timer_from_fps(fps2p),
        }
    }

    fn timer_from_fps(fps2p: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps2p as f32)), TimerMode::Once)
    }
}


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerEnemyCollisionEvent>()
            .add_systems(
            Update,
                (
                    handle_player_death,
                    handle_player_move,
                    handle_player_skills,
                    handle_player_shoot,
                    handle_player_enemy_collision_events,
                    handle_play_bullet_collision_events,
            ).run_if(in_state(GameState::InGame))
             .run_if(in_state(GameState::Home)),
            )
            .add_systems(OnEnter(GameState::Home), setup_player);
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

fn handle_player_death(
    player_query: Query<&Health, With<Character>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_query.is_empty() {
        return;
    }
    let health = player_query.single();
    if health.0 <= 0.0 {
        //可以的话加个死亡动画慢动作
        next_state.set(GameState::OverMenu);//进结算界面
    }
}

fn setup_player(
    mut commands: Commands,
    handle: ResMut<GlobalCharacterTextureAtlas>,
) {
    commands.spawn( (Sprite {
        image: handle.image_idle.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: handle.lay_out_idle.clone(),
            index: 1,
        }),
        ..Default::default()
        },
        Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::new(0.0, 0.0, 30.0)),
        AnimationConfig::new(10),
        ));
}

fn handle_player_move(
    handle: ResMut<GlobalCharacterTextureAtlas>,
    mut player_query: Query<(&mut Sprite, &mut Transform, &mut PlayerState), With<Character>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }
    //之后可以改为自定义的键位，数据存到configs中
    let (mut player, mut transform, mut player_state) = player_query.single_mut();
    let jump = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::Space);
    let left = keyboard_input.pressed(KeyCode::KeyA);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let right = keyboard_input.pressed(KeyCode::KeyD);
    //到边界的检测缺
    let mut delta = Vec2::ZERO;
    if left {
        delta.x -= 1.0;
    }
    if right {
        delta.x += 1.0;
    }
    //
    //test
    if down {
        delta.y -= 1.0;
    }
    if jump {
        delta.y += 1.0;
    }
    delta = delta.normalize();
    if delta.is_finite() && (jump || down || left || right) {
        transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED;
        //
        transform.translation.z = 30.0;
        //
        match *player_state {
            PlayerState::Move=> {},
            _=> {
                if let Some(image) = handle.image_move.clone() {
                    player.image = image;
                }
                if let Some(lay_out) = handle.lay_out_move.clone() {
                    player.texture_atlas = Some(TextureAtlas {
                        layout: lay_out,
                        index: 1,
                    });
                }
                *player_state = PlayerState::Move;
            },
        };
        
    } else {
        match *player_state {
            PlayerState::Idle=> {},
            _=> {
                player.image = handle.image_idle.clone();
                player.texture_atlas = Some(TextureAtlas {
                    layout: handle.lay_out_idle.clone(),
                    index: 1,
                });
                *player_state = PlayerState::Idle;
            },
        };
    }
}

fn handle_player_skills(

) {
}

fn handle_player_shoot(

) {
}

fn handle_play_bullet_collision_events(
    
) {

}
