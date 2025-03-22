use bevy::math::vec3;
use bevy::prelude::*;

use crate::gamestate::GameState;
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

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerEnemyCollisionEvent>().add_systems(
            Update,
            (
                handle_player_death,
                handle_player_move,
                handle_player_skills,
                handle_player_shoot,
                handle_player_enemy_collision_events,
                handle_play_bullet_collision_events,
            ).run_if(in_state(GameState::InGame)),

        );
    }
}

fn handle_player_enemy_collision_events(
    mut player_query: Query<&mut Health, With<Character>>,
    mut events: EventReader<PlayerEnemyCollisionEvent>,
) {
    if player_query.is_empty() {
        return;
    }
    let mut health = player_query.single_mut();
    for _ in events.read() {
        health.0 -= ENEMY_DAMAGE;
    }
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

fn handle_player_move(
    mut player_query: Query<(&mut Transform, &mut PlayerState), With<Character>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }
    //之后可以改为自定义的键位，数据存到configs中
    let (mut transform, mut player_state) = player_query.single_mut();
    let jump = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
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
        transform.translation.z = 10.0;
        //
        *player_state = PlayerState::Move;
    } else {
        *player_state = PlayerState::Idle;
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
