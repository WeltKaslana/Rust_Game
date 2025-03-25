use bevy::{dev_tools::states::*, prelude::*};

use crate::{
    gun::Gun,
    character::{Character, PlayerState, AnimationConfig}, 
    gamestate::GameState, CursorPosition,
};
// enemy::{Enemy, EnemyType},

pub struct AnimationPlugin;


impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_systems(
        //     Update,
        //     (
        //         animate_player,
        //         animate_enemy,
        //         flip_gun_sprite_y,
        //         flip_player_sprite_x,
        //         flip_enemy_sprite_x,
        //         flip_boss_sprite_x,
        //     ).run_if(in_state(GameState::InGame)),)
            .add_systems(Update, log_transitions::<GameState>)
            .add_systems(Update, 
                (
                    animate_player,
                    flip_player_sprite_x,
                    flip_gun_sprite_y,
            ).run_if(in_state(GameState::Home))
            );
    }
}

fn move_template(
    time: Res<Time>, 
    mut query: Query<(&mut AnimationConfig, &mut Sprite)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut config, mut sprite) in &mut query {
        if  keyboard_input.pressed(KeyCode::KeyD){
            // We track how long the current sprite has been displayed for
            config.frame_timer.tick(time.delta());
            // If it has been displayed for the user-defined amount of time (fps)...
            if config.frame_timer.just_finished(){
                if let Some(atlas) = &mut sprite.texture_atlas {
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
                    atlas.index = (atlas.index + 1) % 10;
                }
            }
        }
        else {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = 0;
            }
        }
    }
}

fn animate_player(
    time: Res<Time>,
    mut player_query: Query<(&mut AnimationConfig, &mut Sprite, &PlayerState), With<Character>>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut config, mut player, state) = player_query.single_mut();
    let all = match state {
        PlayerState::Move => 10,
        PlayerState::Idle => 6,
        _ => 0,
    };
    // We track how long the current sprite has been displayed for
    config.frame_timer.tick(time.delta());
    // If it has been displayed for the user-defined amount of time (fps)...
    if config.frame_timer.just_finished(){
        if let Some(atlas) = &mut player.texture_atlas {
            config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
            atlas.index = (atlas.index + 1) % all;
        }
    }

}

fn animate_enemy(

) {

}

fn flip_player_sprite_x(
    cursor_position: Res<CursorPosition>,
    mut player_query: Query<(&mut Sprite, &Transform), With<Character>>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut sprite, transform) = player_query.single_mut();
    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x > transform.translation.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}


fn flip_enemy_sprite_x(
    
) {

}

fn flip_boss_sprite_x(
    
) {

}

fn flip_gun_sprite_y(
    cursor_position: Res<CursorPosition>,
    mut gun_query: Query<(&mut Sprite, &Transform), With<Gun>>,
) {
    if gun_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = gun_query.single_mut();
    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x > transform.translation.x {
            sprite.flip_y = false;
        } else {
            sprite.flip_y = true;
        }
    }
}
