use bevy::{dev_tools::states::*, prelude::*};

use crate::{
    gun::{Gun, Cursor, GunFire,},
    character::{Character, PlayerState, AnimationConfig, }, 
    gamestate::GameState,
    home::{Sora,
           SoraState,
           Fridge,
           FridgeState,},
    enemy::{
        Enemy, 
        EnemyState, 
        EnemyType, 
        PatrolState}, 
    resources::GlobalHomeTextureAtlas,
};

pub struct AnimationPlugin;


impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, log_transitions::<GameState>)
        .add_systems(
            Update,
            (
                animate_player,
                animate_enemy,
                flip_gun_sprite_y,
                flip_player_sprite_x,
                animate_gunfire,
                // flip_boss_sprite_x,
            ).run_if(in_state(GameState::InGame)),)
            .add_systems(Update, 
                (
                    animate_player,
                    flip_player_sprite_x,
                    flip_gun_sprite_y,
                    animate_gunfire,
                    animate_sora,
                    animate_fridge,
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
        PlayerState::Jump => 8,
        _ => 100,
    };
    // We track how long the current sprite has been displayed for
    config.frame_timer.tick(time.delta());
    // If it has been displayed for the user-defined amount of time (fps)...
    if config.frame_timer.just_finished(){
        if let Some(atlas) = &mut player.texture_atlas {
            config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
            match state {
                PlayerState::Jump => {
                    if atlas.index == 2 {
                        atlas.index = 4;//纹理集莫名其妙少一块
                    }
                    if atlas.index < 7 {
                        atlas.index += 1;
                    }
                },
                PlayerState::Jumpover => {},
                _ => {
                    atlas.index = (atlas.index + 1) % all;
                },
            }
        }
    }

}

fn animate_enemy(
    time: Res<Time>,
    mut enemy_query: Query<(&mut AnimationConfig, &mut Sprite, &mut EnemyState, &EnemyType, &mut PatrolState), With<Enemy>>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (mut aconfig, mut enemy, mut enemy_state, enemy_type, mut patrolstate) in enemy_query.iter_mut() {
        
        if patrolstate.directionx >= 0.0 {
            enemy.flip_x = false;
        }else{
            enemy.flip_x = true;
        }

        match enemy_type {
            EnemyType::Sweeper => {
                let all = match *enemy_state {
                    EnemyState::Idea => 1,
                    EnemyState::Move => 14,
                    EnemyState::FireLoop => 13,
                    EnemyState::FireEnd | EnemyState::FireStart => 1,
                };
                aconfig.frame_timer.tick(time.delta());
                if aconfig.frame_timer.just_finished(){
                    if let Some(atlas) = &mut enemy.texture_atlas {
                        aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                        atlas.index = (atlas.index + 1) % all;
                    }
                }
            },
            EnemyType::DroneVulcan => {
                let all = match *enemy_state {
                    EnemyState::Idea => 5,
                    EnemyState::Move => 5,
                    EnemyState::FireLoop => 3,
                    EnemyState::FireEnd => 2,
                    EnemyState::FireStart => 3,
                };
                aconfig.frame_timer.tick(time.delta());
                if aconfig.frame_timer.just_finished(){
                    if let Some(atlas) = &mut enemy.texture_atlas {
                        aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                        atlas.index = (atlas.index + 1) % all;
                    }
                }
            },
            EnemyType::DroneMissile => {
                let all = match *enemy_state {
                    EnemyState::Idea => 5,
                    EnemyState::Move => 5,
                    EnemyState::FireLoop => 5,
                    EnemyState::FireEnd => 2,
                    EnemyState::FireStart => 3,
                };
                aconfig.frame_timer.tick(time.delta());
                if aconfig.frame_timer.just_finished(){
                    if let Some(atlas) = &mut enemy.texture_atlas {
                        aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                        atlas.index = (atlas.index + 1) % all;
                    }
                }
            },
        }
    }
}

fn flip_player_sprite_x(
    cursor_query: Query<&Transform, (With<Cursor>, Without<Character>)>,
    mut player_query: Query<(&mut Sprite, &Transform), (With<Character>, Without<Cursor>)>,
) {
    if cursor_query.is_empty() {
        return;
    }
    if player_query.is_empty() {
        return;
    }
    let cursor_position = cursor_query.single().translation.truncate();
    let (mut sprite, transform) = player_query.single_mut();
    if cursor_position.x > transform.translation.x {
        sprite.flip_x = false;
    } else {
        sprite.flip_x = true;
    }
}



fn flip_boss_sprite_x(
    
) {

}

fn flip_gun_sprite_y(
    cursor_query: Query<&Transform, (With<Cursor>, Without<Gun>)>,
    mut gun_query: Query<(&mut Sprite, &Transform), (With<Gun>,Without<Cursor>)>,
) {
    if cursor_query.is_empty() {
        return;
    }
    if gun_query.is_empty() {
        return;
    }
    let cursor_position = cursor_query.single().translation.truncate();
    let (mut sprite, transform) = gun_query.single_mut();
    if cursor_position.x > transform.translation.x {
        sprite.flip_y = false;
    } else {
        sprite.flip_y = true;
    }
}

fn animate_gunfire(
    mut commands: Commands,
    time: Res<Time>,
    mut Gun_query: Query<(&mut AnimationConfig, &mut Sprite, Entity), With<GunFire>>,
) {
    if Gun_query.is_empty() {
        return;
    }
    // let (mut config, mut sprite, entity) = Gun_query.single_mut();
    for (mut config, mut sprite, entity) in &mut Gun_query.iter_mut() {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished(){
            if let Some(atlas) = &mut sprite.texture_atlas {
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
                atlas.index = atlas.index + 1;
                if atlas.index == 5 {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn animate_sora(
    // asset_server: Res<AssetServer>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
    mut sora_query: Query<(&mut AnimationConfig, &mut Sprite, &mut SoraState), With<Sora>>,
    source: Res<GlobalHomeTextureAtlas>,
) {
    if sora_query.is_empty() {
        return;
    }
    let (mut config, mut sora, mut state) = sora_query.single_mut();
    config.frame_timer.tick(time.delta());
    if config.frame_timer.just_finished(){
        if let Some(atlas) = &mut sora.texture_atlas {
            config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
            match *state {
                SoraState::Loop => {
                    atlas.index = (atlas.index + 1) % 8;
                },
                SoraState::Awake => {
                    if atlas.index != 13 {
                        atlas.index += 1;
                    }
                },
                SoraState::Asleep => {
                    atlas.index += 1;
                    if atlas.index == 18 {
                        *state = SoraState::Loop;
                        sora.image = source.Sora_image_loop.clone();
                        sora.texture_atlas = Some(TextureAtlas {
                            layout: source.Sora_lay_out_loop.clone(),
                            index: 0,
                        });
                    }
                },
            }
        }
    }
}

fn animate_fridge(
    // asset_server: Res<AssetServer>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
    mut player_query: Query<(&mut AnimationConfig, &mut Sprite,&mut FridgeState), With<Fridge>>,
    source: Res<GlobalHomeTextureAtlas>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut config, mut fridge, mut state) = player_query.single_mut();
    config.frame_timer.tick(time.delta());
    if config.frame_timer.just_finished(){
        if let Some(atlas) = &mut fridge.texture_atlas {
            config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
            match *state {
                FridgeState::Loop => {
                    atlas.index = (atlas.index + 1) % 24;
                },
                FridgeState::Open => {
                    if atlas.index != 6 {
                        atlas.index += 1;
                    }
                },
                FridgeState::Close => {
                    atlas.index += 1;
                    if atlas.index == 14 {
                        *state = FridgeState::Loop;
                        fridge.image = source.Fridge_image_loop.clone();
                        fridge.texture_atlas = Some(TextureAtlas {
                            layout: source.Fridge_lay_out_loop.clone(),
                            index: 0,
                        });
                    }
                },
            }
        }
    }
}
