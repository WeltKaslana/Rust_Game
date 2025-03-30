use bevy::math::vec3;
use bevy::{dev_tools::states::*, prelude::*, time::Stopwatch};
use std::{time::Duration};
use crate::gamestate::GameState;
use crate::*;
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Velocity(pub f32);

#[derive(Component)]
pub struct PlayerTimer(pub Stopwatch);

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Jump,
    Move,
    Jumpover,
}

#[derive(Event)]
pub struct PlayerEnemyCollisionEvent;

#[derive(Event)]
pub struct PlayerRunEvent;

#[derive(Event)]
pub struct PlayerJumpEvent;

//定义角色动画帧率
#[derive(Component)]

pub struct AnimationConfig {
    pub fps2p: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(fps2p: u8) -> Self {
        Self {
            fps2p,
            frame_timer: Self::timer_from_fps(fps2p),
        }
    }

    pub fn timer_from_fps(fps2p: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps2p as f32)), TimerMode::Once)
    }
}


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_event::<PlayerEnemyCollisionEvent>()
            .add_systems(OnEnter(GameState::Home), setup_player)
            .add_systems(
            Update,
                (
                    handle_player_move,
                    // handle_player_skills,
                    // handle_player_enemy_collision_events,
                    // handle_play_bullet_collision_events,
            ).run_if(in_state(GameState::Home))
            )
            .add_systems(
                Update,
                    (
                        // handle_player_death,
                        handle_player_move,
                        // handle_player_skills,
                        // handle_player_enemy_collision_events,
                        // handle_play_bullet_collision_events,
                ).run_if(in_state(GameState::InGame))
                )
            .add_systems(Update, log_transitions::<GameState>)
            ;
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout_idle = TextureAtlasLayout::from_grid(UVec2::splat(64),6,1,None,None);
    commands.spawn( (Sprite {
        image: asset_server.load("Shiroko_Idle.png"),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layouts.add(layout_idle),
            index: 1,
        }),
        ..Default::default()
        },
        Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(0.0, -200.0, 30.0)),
        AnimationConfig::new(13),
        PlayerState::default(),
        Character,
        Health(PLAYER_HEALTH),
        Velocity(PLAYER_JUMP_SPEED),
        PlayerTimer(Stopwatch::default()),
        ))
        .with_child((Sprite {
            image: asset_server.load("Shiroko_Aura.png"),
            ..Default::default()
            },
            Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(0.0, 15.0, -1.0)),
            ));
}

fn handle_player_move(
    mut events: EventWriter<PlayerRunEvent>,
    mut events2: EventWriter<PlayerJumpEvent>,
    mut player_query: Query<(&mut Sprite, &mut Transform, &mut PlayerState, &mut Velocity), With<Character>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if player_query.is_empty() {
        return;
    }
    //之后可以改为自定义的键位，数据存到configs中
    let (mut player, mut transform, mut player_state, mut V) = player_query.single_mut();
    let jump = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::Space);
    let left = keyboard_input.pressed(KeyCode::KeyA);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let right = keyboard_input.pressed(KeyCode::KeyD);
    //到边界的检测缺
    let mut delta = Vec2::ZERO;
    if left {
        // println!("left!");
        delta.x -= 0.5;
    }
    if right {
        // println!("right!");
        delta.x += 0.5;
    }
    //
    //test
    if down {
        println!("down");
        // delta.y -= 0.5;
    }
    if jump {
        // println!("jump!");

        match *player_state {
            PlayerState::Jump => {},
            _=> {
                player.image = asset_server.load("Shiroko_Jump.png");
                let layout_jump = TextureAtlasLayout::from_grid(UVec2::splat(64),4,2,None,None);
                player.texture_atlas = Some(TextureAtlas {
                    layout: texture_atlas_layouts.add(layout_jump),
                    index: 0,
                });
                *player_state = PlayerState::Jump;
                events2.send(PlayerJumpEvent);
                transform.translation.y += V.0;
                V.0 -= PLAYER_GRAVITY;
            },
        };
        // if V.0 == PLAYER_JUMP_SPEED {
        //     transform.translation.y += V.0;
        //     V.0 -= PLAYER_GRAVITY;
        // }
    }
    delta = delta.normalize();
    if delta.is_finite() && (jump || down || left || right) {
        transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED;
        //
        transform.translation.z = 30.0;
        //
        match *player_state {
            PlayerState::Move =>{},
            PlayerState::Jump =>{},
            _ => {
                player.image = asset_server.load("Shiroko_Move.png");
                let layout_move = TextureAtlasLayout::from_grid(UVec2::splat(64),5,2,None,None);
                player.texture_atlas = Some(TextureAtlas {
                    layout: texture_atlas_layouts.add(layout_move),
                    index: 1,
                });
                *player_state = PlayerState::Move;
            },
        };
        events.send(PlayerRunEvent);
        
    } else {
        match *player_state {
            PlayerState::Idle =>{},
            PlayerState::Jump =>{},
            _ => {
                player.image = asset_server.load("Shiroko_Idle.png");
                let layout_idle = TextureAtlasLayout::from_grid(UVec2::splat(64),6,1,None,None);
                player.texture_atlas = Some(TextureAtlas {
                    layout: texture_atlas_layouts.add(layout_idle),
                    index: 1,
                });
                *player_state = PlayerState::Idle;
            },
        };
    }
    if transform.translation.y <= -200.0 {
        if transform.translation.y != -200.0 {
            transform.translation.y = -200.0;
        }
        if V.0 < 0.0 {
            V.0 = PLAYER_JUMP_SPEED;
        }
        match *player_state {
            PlayerState::Jump => {*player_state = PlayerState::Jumpover;},
            _ => {},
        }
        
    }
    else {
        transform.translation.y += V.0;
        V.0 -= PLAYER_GRAVITY;
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



fn handle_player_skills(

) {
}


fn handle_play_bullet_collision_events(
    
) {

}
