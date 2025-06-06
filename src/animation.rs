use bevy::{
    dev_tools::states::*, 
    prelude::*,
};
use bevy::utils::Instant;
use bevy_rapier2d::prelude::*;
use crate::GlobalRoomTextureAtlas;
use crate::{
    boss::{
        set_boss, Boss, BossComponent, BossDeathEffect, BossState, Direction, Skillflag
    }, 
    character::{
        AnimationConfig, Character, Drone, DroneBullet, PlayerState, State, GrenadeHit, handle_player_skill4,
    }, 
    enemy::{
        set_enemy, BulletDirection, Enemy, EnemyBullet, EnemyDeathEffect, EnemyState, EnemyType, Fireflag, PatrolState
    }, 
    gamestate::*, 
    gun::{self, Bullet, BulletHit, Cursor, Gun, GunFire, SpawnInstant, GunState, BulletDamage, }, 
    home::{Fridge, FridgeState, Sora, SoraState}, 
    resources::{
        GlobalBossTextureAtlas, GlobalCharacterTextureAtlas, GlobalEnemyTextureAtlas, GlobalHomeTextureAtlas
    }, 
    room::{EnemyBorn, Door, Chest},
};
use bevy::math::{vec2, vec3};

pub struct AnimationPlugin;


impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
            (
                animate_player,
                animate_player_gun_and_bullet,
                animate_enemy_born,
                animate_enemy,
                flip_gun_sprite_y,
                flip_player_sprite_x,
                animate_gunfire,
                animate_enemy_bullet,
                animate_boss,
                boss_filpx,
                enemyboss_death_effect,
                animate_droneskill,
                animate_door_and_chest,
            ).run_if(in_state(InGameState::Running)),)
            .add_systems(Update, 
                (
                    animate_player,
                    animate_player_gun_and_bullet,
                    animate_droneskill,
                    flip_player_sprite_x,
                    flip_gun_sprite_y,
                    animate_gunfire,
                    animate_sora,
                    animate_fridge,
            ).run_if(in_state(HomeState::Running))
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
    mut player_query: Query<(
        &mut AnimationConfig, 
        &mut Sprite, 
        &mut PlayerState, 
        &mut KinematicCharacterController,
    ), (With<Character>, Without<Gun>)>,
    mut gun_query: Query<&mut Visibility, (With<Gun>, Without<Character>)>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut config, mut player, mut state, mut controller) = player_query.single_mut();
    let all = match *state {
        //得分角色
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
            match *state {
                PlayerState::Jump => {
                    if atlas.index == 2 {
                        atlas.index = 4;//纹理集莫名其妙少一块
                    }
                    if atlas.index < 7 {
                        atlas.index += 1;
                    }
                },
                PlayerState::Jumpover => {},

                PlayerState::Dodge => {
                    match source.id {
                        1 => {
                            //shiroko
                            atlas.index = atlas.index + 1;
                            if atlas.index == 12 {
                                atlas.index = 0;
                                *state = PlayerState::Jumpover;
                                controller.filter_groups = None;
                            }
                        }
                        2 => {
                            //arisu
                            atlas.index = atlas.index + 1;
                            if atlas.index == 11 {
                                atlas.index = 0;
                                *state = PlayerState::Jumpover;
                                controller.filter_groups = None;
                                // arisu的光之剑需要隐藏
                                for mut gun in gun_query.iter_mut() {
                                    *gun = Visibility::Visible;
                                }
                            }
                        }
                        3 => {
                            //Utaha
                            atlas.index = match atlas.index {
                                0..2 => atlas.index + 1,
                                2 => 5,
                                5..8 => atlas.index + 1,
                                8 => 10,
                                10..=14 => atlas.index + 1,
                                _ => 0,
                            };
                            if atlas.index == 15 {
                                atlas.index = 0;
                                *state = PlayerState::Jumpover;
                                controller.filter_groups = None;
                            }
                        },
                        _ => {
                            
                        }
                    }
                },

                _ => {
                    atlas.index = (atlas.index + 1) % all;
                },
            }
        }
    }

}

fn animate_player_gun_and_bullet (
    time: Res<Time>,
    mut gun_query: Query<(
        &mut AnimationConfig, 
        &mut Sprite, 
        &mut GunState,
    ), (With<Gun>, Without<Bullet>)>,
    mut bullet_query: Query<(
        &mut AnimationConfig, 
        & BulletDamage,
        &mut Sprite, 
    ), (With<Bullet>, Without<Gun>)>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    if source.id == 2 {
        for (mut config,damage , mut bullet) in bullet_query.iter_mut() {
                // arisu有个大的炮要单独设置
                config.frame_timer.tick(time.delta());
                if config.frame_timer.just_finished(){
                    let mut flame = 0;
                    if damage.0 > 20.0 {
                        // 光之剑
                        flame = 8;
                    } else {
                        flame = 4;
                    }
                    if let Some(atlas) = &mut bullet.texture_atlas {
                        config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
                        atlas.index = (atlas.index+1) % flame;
                    }
                }
        }
        for (mut config, mut gun, mut state) in gun_query.iter_mut() {
            match *state {
                GunState::Fire => {
                    config.frame_timer.tick(time.delta());
                    if config.frame_timer.just_finished(){
                        // info!("ok!");
                        if let Some(atlas) = &mut gun.texture_atlas {
                            config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
                            atlas.index += 1;
                            if atlas.index == 8 {
                                gun.image = source.image_gun.clone();
                                gun.texture_atlas = Some(TextureAtlas {
                                    layout: source.lay_out_gun.clone(),
                                    index: 0,
                                });
                                *state = GunState::Normal;
                                // println!("fire over");
                            }
                        }
                    }
                },
                GunState::SP => {
                    // arisu大招
                    config.frame_timer.tick(time.delta());
                    if config.frame_timer.just_finished(){
                        // info!("ok!");
                        if let Some(atlas) = &mut gun.texture_atlas {
                            config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
                            atlas.index = match atlas.index {
                                0..=11 => atlas.index + 1,
                                12 => 9,
                                18..=24 =>  atlas.index + 1,
                                _ => 0
                            };
                            if atlas.index ==25 {
                                // 退出大招模式
                                gun.image = source.image_gun.clone();
                                gun.texture_atlas = Some(TextureAtlas {
                                    layout: source.lay_out_gun.clone(),
                                    index: 0,
                                });
                                *state = GunState::Normal;
                            }
                        }
                    }
                }
                _ => {},
            }

        }
    }
}
fn animate_enemy_born(
    mut commands: Commands,
    time: Res<Time>,
    mut e_query: Query<(&mut Transform, &mut AnimationConfig, &mut Sprite, Entity), (With<EnemyBorn>, With<Enemy>, Without<BossComponent>)>,
    mut b_query: Query<(&mut Transform, &mut AnimationConfig, &mut Sprite, Entity), (With<EnemyBorn>, With<BossComponent>, Without<Enemy>)>,
    source1: Res<GlobalEnemyTextureAtlas>,
    source2: Res<GlobalBossTextureAtlas>,
) {
    //小怪
    for (mut trans, mut config, mut enemy, e) in e_query.iter_mut() {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished(){
            if let Some(atlas) = &mut enemy.texture_atlas {
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
                atlas.index += 1;
                if atlas.index == 12 {
                    //产生敌人
                    atlas.index = 0;
                    commands.entity(e).despawn();
                    set_enemy(
                        0, 
                        Vec2::new(
                            trans.translation.x, 
                            trans.translation.y), 
                        &mut commands, 
                        &source1);
                }
            }
        }
    }
    for (mut trans, mut config, mut boss, e) in b_query.iter_mut() {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished(){
            if let Some(atlas) = &mut boss.texture_atlas {
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
                atlas.index += 1;
                if atlas.index == 12 {
                    //产生敌人
                    atlas.index = 0;
                    commands.entity(e).despawn();
                    set_boss(
                        Vec2::new(
                            trans.translation.x, 
                            trans.translation.y), 
                        &mut commands, 
                        &source2);
                    println!("boss born!");
                }
            }
        }
    }
}

fn animate_enemy(
    time: Res<Time>,
    mut enemy_query: Query<(
        &mut AnimationConfig, 
        &mut Sprite, 
        &mut EnemyState, 
        &EnemyType, 
        &mut PatrolState,
        &mut Fireflag), With<Enemy>>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (   mut aconfig, 
            mut enemy, 
            enemy_state, 
            enemy_type, 
            patrolstate,
            mut flag) in enemy_query.iter_mut() {
        
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
                        *flag = Fireflag::Fire;
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
                        if atlas.index == all - 1 {
                            *flag = Fireflag::Fire;
                        }
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
                        if atlas.index == all - 1 {
                            *flag = Fireflag::Fire;
                        }
                    }
                }
            },
            EnemyType::UnknownGuardianTypeF => {
                let all = match *enemy_state {
                    EnemyState::Idea => 1,
                    EnemyState::Move => 9,
                    EnemyState::FireLoop => 8,
                    EnemyState::FireEnd | EnemyState::FireStart => 1,
                };
                aconfig.frame_timer.tick(time.delta());
                if aconfig.frame_timer.just_finished(){
                    if let Some(atlas) = &mut enemy.texture_atlas {
                        aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                        atlas.index = (atlas.index + 1) % all;
                        if atlas.index == all - 1 {
                            *flag = Fireflag::Fire;
                        }
                    }
                }
            },
        }
    }
}

fn animate_enemy_bullet(
    time: Res<Time>,
    mut bullet_query : Query<(
        &mut Sprite,
        &mut AnimationConfig,
        & EnemyBullet),With<EnemyBullet>>,
) {
    if bullet_query.is_empty() {
        return;
    }

    for (   mut bullet,
            mut aconfig,
            bullettype) in bullet_query.iter_mut(){
        
        match bullettype {
            EnemyBullet::DroneMissile => {
                let all = 5;
                aconfig.frame_timer.tick(time.delta());
                if aconfig.frame_timer.just_finished(){
                    if let Some(atlas) = &mut bullet.texture_atlas {
                        aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                        atlas.index = (atlas.index + 1) % all;
                    }
                }
            },
            EnemyBullet::DroneVulcan=> {
                let all = 4;
                aconfig.frame_timer.tick(time.delta());
                if aconfig.frame_timer.just_finished(){
                    if let Some(atlas) = &mut bullet.texture_atlas {
                        aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                        atlas.index = (atlas.index + 1) % all;
                    }
                }
            },
            EnemyBullet::UnknownGuardian => {
                let all = 4;
                aconfig.frame_timer.tick(time.delta());
                if aconfig.frame_timer.just_finished(){
                    if let Some(atlas) = &mut bullet.texture_atlas {
                        aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                        atlas.index = (atlas.index + 1) % all;
                    }
                }
            }
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



fn animate_boss(
    time: Res<Time>,
    mut boss_query: Query<(
        &mut AnimationConfig,
        &mut Sprite,
        & BossState,
        & Boss,
        &mut Skillflag,
    ), With<Boss>>,
) {
    if boss_query.is_empty() {
        return;
    }
    for (mut aconfig, mut boss, bossstate, bosscomponent, mut fireflag) in boss_query.iter_mut() {
        match bosscomponent {
            Boss::Body => {
                let all = match bossstate {
                    BossState::Idea => 4,
                    BossState::Move => 8,
                    BossState::CollideStart => 10,
                    BossState::CollideLoop => 8,
                    BossState::CollideEnd => 2,
                    _ => 1,
                };
                aconfig.frame_timer.tick(time.delta());
                if aconfig.frame_timer.just_finished(){
                    if let Some(atlas) = &mut boss.texture_atlas {
                        aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                        atlas.index = (atlas.index + 1) % all;
                    }
                }
            },
            Boss::Gun => {
                let all = match bossstate {
                    BossState::Gunfire => 7,
                    _ => 1,
                };
                aconfig.frame_timer.tick(time.delta());
                if aconfig.frame_timer.just_finished(){
                    if let Some(atlas) = &mut boss.texture_atlas {
                        aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                        atlas.index = (atlas.index + 1) % all;
                        fireflag.0 = 0;
                    }
                }

            },
            Boss::Missile => {
                let all = match bossstate {
                    BossState::Missilefire => 30,
                    _ => 1,
                };
                aconfig.frame_timer.tick(time.delta());
                if aconfig.frame_timer.just_finished(){
                    if let Some(atlas) = &mut boss.texture_atlas {
                        aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                        atlas.index = (atlas.index + 1) % all;
                        fireflag.0 = 0;
                    }
                }

            },
            Boss::Shield => {
                let all = match bossstate {
                    BossState::Gunfire => 7,
                    _ => 1,
                };
                aconfig.frame_timer.tick(time.delta());
                if aconfig.frame_timer.just_finished(){
                    if let Some(atlas) = &mut boss.texture_atlas {
                        aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                        atlas.index = (atlas.index + 1) % all;
                    }
                }
            },
        }
    }
}

fn boss_filpx(
    mut boss_query: Query<(
        &mut Sprite,
        & Direction
    ), (With<Boss>, Without<BossComponent>)>,
    mut bosscomponent_query: Query<(
        &mut Sprite,
        &mut Transform,
        & Boss
    ),(With<Boss>, With<BossComponent>)>,
) {
    if boss_query.is_empty() || bosscomponent_query.is_empty() {
        return;
    }
    let (mut boss, direction) = boss_query.single_mut();
    if direction.x >= 0.0 {
        boss.flip_x = false;
    }else {
        boss.flip_x = true;
    }
    for (mut bosscomponent, mut transform, component) in bosscomponent_query.iter_mut() {
        if direction.x >= 0.0 {
            bosscomponent.flip_x = false;
            match component {
                Boss::Gun => {
                    bosscomponent.flip_y = false;
                    transform.translation.x = -25.0;
                },
                _=> { },
            }
        }else {
            bosscomponent.flip_x = true;
            match component {
                Boss::Gun => {
                    bosscomponent.flip_x = false;
                    bosscomponent.flip_y = true;
                    transform.translation.x = 25.0;
                },
                _=> { },
            }
        }
    }
    
}

fn enemyboss_death_effect(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_effect_quey: Query<(Entity, &mut Sprite, &mut AnimationConfig), (With<EnemyDeathEffect>, Without<BossDeathEffect>)>,
    mut boss_effect_query: Query<(Entity, &mut Sprite, &mut AnimationConfig), (With<BossDeathEffect>, Without<EnemyDeathEffect>)>,
) {
    if !enemy_effect_quey.is_empty() {
        for (entity, mut sprite, mut aconfig) in enemy_effect_quey.iter_mut() {
            let all = 7;
            aconfig.frame_timer.tick(time.delta());
            if aconfig.frame_timer.just_finished(){
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.index == all - 1 {
                        commands.entity(entity).despawn();
                        continue;
                    }
                    aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                    atlas.index = (atlas.index + 1) % all;
                }
            }
        }
    }

    if !boss_effect_query.is_empty() {
        let (entity, mut sprite, mut aconfig) = boss_effect_query.single_mut();
        let all = 8;
            aconfig.frame_timer.tick(time.delta());
            if aconfig.frame_timer.just_finished(){
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.index == all - 1 {
                        commands.entity(entity).despawn();
                        return;
                    }
                    aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                    atlas.index = (atlas.index + 1) % all;
                }
            }
    }
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
    mut Gun_query: Query<(&mut AnimationConfig, &mut Sprite, Entity), (With<GunFire>, Without<BulletHit>, Without<GrenadeHit>)>,
    mut Hit_query: Query<(&mut AnimationConfig, &mut Sprite, Entity), (With<BulletHit>, Without<GunFire>, Without<GrenadeHit>)>,
    mut grenade_query: Query<(&mut AnimationConfig, &mut Sprite, Entity), (With<GrenadeHit>, Without<GunFire>, Without<BulletHit>)>,
) {
    // if Gun_query.is_empty() {
    //     return;
    // }
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
    for (mut config, mut sprite, entity) in &mut Hit_query.iter_mut() {
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
    for (mut config, mut sprite, entity) in &mut grenade_query.iter_mut() {
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

// fn animate_grenadehit(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut grenade_query: Query<(&mut AnimationConfig, &mut Sprite, Entity), (With<GrenadeHit>)>,
// ) {
//     for (mut config, mut sprite, entity) in &mut grenade_query.iter_mut() {
//         config.frame_timer.tick(time.delta());
//         if config.frame_timer.just_finished(){
//             if let Some(atlas) = &mut sprite.texture_atlas {
//                 config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
//                 atlas.index = atlas.index + 1;
//                 if atlas.index == 5 {
//                     commands.entity(entity).despawn();
//                 }
//             }
//         }
//     }
// }

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

fn animate_droneskill (
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut drone_query: Query<(
        Entity, 
        &mut AnimationConfig, 
        &mut Sprite,
        & Transform,
        &mut State), (With<Drone>, Without<DroneBullet>)>,
    mut drone_bullet_query: Query<(
        &mut Sprite,
        &mut Transform,
        &mut AnimationConfig,
        & gun::BulletDirection), (Without<Drone>, With<DroneBullet>)>,
    source: Res<GlobalCharacterTextureAtlas>,
) {
    if !drone_query.is_empty() {
        let (drone, mut config, mut sprite,dronetransform, mut state) = drone_query.single_mut();
        let all = 7;
        let flag = sprite.flip_x;
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished(){
            if let Some(atlas) = &mut sprite.texture_atlas {
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
                atlas.index = (atlas.index + 1) % all;
                if atlas.index == all - 1 {
                    if state.0 == 8 {
                        commands.entity(drone).despawn();
                    } else {
                        state.0 += 1;
                    }
                } else if (state.0 == 1 || state.0 == 3 || state.0 == 5 || state.0 == 7) && atlas.index == 3 {
                    let mut dir =vec3(1.0, 0.0, 0.0);
                    if flag {
                        dir = vec3(-1.0, 0.0, 0.0);
                    }
                    commands.spawn((
                        Sprite {
                            // image: asset_server.load("Player_Bullet_Missile.png"),
                            // texture_atlas: Some(TextureAtlas {
                            //     layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(32),5,1,None,None)),
                            //     index: 0,
                            // }),
                            image: source.image_drone_missle.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: source.layout_drone_missle.clone(),
                                index: 0,
                            }),
                            ..Default::default()
                        },
                        Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(dronetransform.translation.x-20.0, dronetransform.translation.y, 32.0)),
                        DroneBullet,
                        Bullet,
                        gun::BulletDirection(dir),
                        BulletDamage(5.0),
                        AnimationConfig::new(10),
                        SpawnInstant(Instant::now()),
                        
                        Sensor,
                        RigidBody::Dynamic,
                        GravityScale(0.0),
                        Collider::cuboid(11.0, 5.0),
                        ActiveEvents::COLLISION_EVENTS,
                        CollisionGroups::new(Group::GROUP_4, Group::GROUP_3),

                    ));
                    commands.spawn((
                        Sprite {
                            // image: asset_server.load("Player_Bullet_Missile.png"),
                            // texture_atlas: Some(TextureAtlas {
                            //     layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(32),5,1,None,None)),
                            //     index: 0,
                            // }),
                            image: source.image_drone_missle.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: source.layout_drone_missle.clone(),
                                index: 0,
                            }),
                            ..Default::default()
                        },
                        Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(dronetransform.translation.x+20.0, dronetransform.translation.y, 32.0)),
                        DroneBullet,
                        Bullet,
                        gun::BulletDirection(dir),
                        BulletDamage(5.0),
                        AnimationConfig::new(10),
                        SpawnInstant(Instant::now()),
                        
                        Sensor,
                        RigidBody::Dynamic,
                        GravityScale(0.0),
                        Collider::cuboid(11.0, 5.0),
                        ActiveEvents::COLLISION_EVENTS,
                        CollisionGroups::new(Group::GROUP_4, Group::GROUP_3),
                        
                    ));
                    state.0 += 1;
                }
            }
        }
    }
    if !drone_bullet_query.is_empty() {
        for (mut bullet, mut transform, mut config, dir) in drone_bullet_query.iter_mut() {
            let all = 5;
            config.frame_timer.tick(time.delta());
            if config.frame_timer.just_finished(){
                if let Some(atlas) = &mut bullet.texture_atlas {
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps2p);
                    atlas.index = (atlas.index + 1) % all;
                }
            }
            let dx = dir.0.x;
            let dy = dir.0.y;
            let angle = dy.atan2(dx);
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

fn animate_door_and_chest (
    mut door_query: Query<(
        &mut Sprite,
        & Transform,
        &mut AnimationConfig,
        &mut Door,
    ), (With<Door>, Without<Chest>, Without<Character>)>,
    mut chest_query: Query<(
        &mut Sprite,
        & Transform,
        &mut AnimationConfig,
        &mut Chest,
    ), (With<Chest>, Without<Door>, Without<Character>)>,
    time: Res<Time>,
    player_query: Query<& Transform, (With<Character>, Without<Door>, Without<Chest>)>,
    mut windows: Query<&mut Window>,
    source: Res<GlobalRoomTextureAtlas>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if door_query.is_empty() && chest_query.is_empty() { return; }
    let player_transform = player_query.single();

    if !door_query.is_empty() { 
        let (mut door_sprite, door_transfrom, mut aconfig, mut door) = door_query.single_mut();
        match door.0 {
            0 => { },
            1 => {//ready to open
                let distance = player_transform.translation.distance(door_transfrom.translation);
                if distance <= 150.0 { 
                    door.0 = 2;
                }
            },
            2 => {
                let all = 13;
                aconfig.frame_timer.tick(time.delta());
                if aconfig.frame_timer.just_finished(){
                    if let Some(atlas) = &mut door_sprite.texture_atlas {
                        aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                        atlas.index = atlas.index + 1;
                        if atlas.index == all - 1 {
                            door.0 = 3;
                        }
                    }
                }
            },
            3 => {
                let distance = player_transform.translation.distance(door_transfrom.translation);
                if distance > 150.0 { 
                    door.0 = 4;
                    door_sprite.image = source.image_door_close.clone();
                    if let Some(atlas) =&mut door_sprite.texture_atlas {
                        atlas.index = 0;
                    }
                }
            },
            4 => {
                let all = 15;
                aconfig.frame_timer.tick(time.delta());
                if aconfig.frame_timer.just_finished(){
                    if let Some(atlas) = &mut door_sprite.texture_atlas {
                        aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                        atlas.index = atlas.index + 1;
                        if atlas.index == all - 1 {
                            atlas.index = 0;
                            door.0 = 1;
                        }
                    }
                    if door.0 == 1 {
                        door_sprite.image = source.image_door_open.clone();
                    }
                }
            },
            _ => { }
        }
    }

    if !chest_query.is_empty() {
        for (mut sprite, transform, mut aconfig, mut chest) in chest_query.iter_mut() {
            match chest.0 {
                0 => {
                    let all = 3;
                    aconfig.frame_timer.tick(time.delta());
                    if aconfig.frame_timer.just_finished(){
                        if let Some(atlas) = &mut sprite.texture_atlas {
                            aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                            atlas.index = (atlas.index + 1) % all;
                        }
                    }
                },
                1 => { },
                2 => { },
                3  => { 
                    let distance = player_transform.translation.distance(transform.translation);
                    if distance <= 150.0 {
                        if keyboard_input.just_pressed(KeyCode::KeyF) {
                            chest.0 = 5;
                        }
                    }
                },
                4 => {
                    let distance = player_transform.translation.distance(transform.translation);
                    if distance <= 150.0 {
                        if keyboard_input.just_pressed(KeyCode::KeyF) {
                            chest.0 = 6;
                        }
                    }
                },
                5 => {
                    let all = 34;
                    aconfig.frame_timer.tick(time.delta());
                    if aconfig.frame_timer.just_finished(){
                        if let Some(atlas) = &mut sprite.texture_atlas {
                            aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                            if atlas.index < all - 1 {
                                atlas.index = atlas.index + 1;
                            } else {
                                // 进入选buff界面
                                if let Ok(mut window) = windows.get_single_mut() {
                                    window.cursor_options.visible = true;
                                    next_state.set(InGameState::ChoosingBuff);
                                    chest.0 = 7;
                                }
                            }
                        }
                    }
                },
                6 => {
                    let all = 24;
                    aconfig.frame_timer.tick(time.delta());
                    if aconfig.frame_timer.just_finished(){
                        if let Some(atlas) = &mut sprite.texture_atlas {
                            aconfig.frame_timer = AnimationConfig::timer_from_fps(aconfig.fps2p);
                            if atlas.index < all - 1 {
                                atlas.index = atlas.index + 1;
                            } else {
                                // 进入选buff界面
                                if let Ok(mut window) = windows.get_single_mut() {
                                    window.cursor_options.visible = true;
                                    next_state.set(InGameState::ChoosingBuff);
                                    chest.0 = 7;
                                }
                            }
                        }
                    }
                },
                _ => {},
            }
        }
    }

}