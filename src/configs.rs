// Window
pub const WW: f32 = 1600.0;
pub const WH: f32 = 700.0;

// Sprites
pub const SPRITE_SHEET_PATH: &str = "assets.png";
pub const SPRITE_SCALE_FACTOR: f32 = 2.5;
pub const TILE_W: usize = 16;
pub const TILE_H: usize = 16;
pub const SPRITE_SHEET_W: usize = 8;
pub const SPRITE_SHEET_H: usize = 8;

// World
pub const NUM_WORLD_DECORATIONS: usize = 500;
pub const WORLD_W: f32 = 3000.0;
pub const WORLD_H: f32 = 2500.0;
pub const fps: u8 = 60;

// Player
pub const PLAYER_SPEED: f32 = 2.0 * 240.0 / fps as f32;
pub const PLAYER_HEALTH: f32 = 100.0;
// pub const PLAYER_JUMP_SPEED: f32 = 20.0;
// pub const PLAYER_GRAVITY: f32 = 1.0;
pub const PLAYER_JUMP_SPEED: f32 = 4.0;
pub const PLAYER_GRAVITY: f32 = 0.2;
pub const PLAYER_JUMP_COUNTS: usize = 3;
pub const GRENADE_BOOM_RANGE:f32 = 150.0;

// Enemy
pub const MAX_NUM_ENEMIES: usize = 20000;
pub const ENEMY_DAMAGE: f32 = 1.5;
pub const SPAWN_RATE_PER_SECOND: usize = 500;
pub const ENEMY_HEALTH: f32 = 100.0;
pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPEED: f32 = 1.0 * 240.0 / fps as f32;
pub const ENEMY_BULLET_SPEED: f32 = 2.0 * 240.0 / fps as f32;
pub const ENEMY_ALARM: f32 = 500.0;//该数值为测试数据，实际游玩时应适当调大
pub const ENEMY_FIRE: f32 = 400.0;//该数值为测试数据，实际游玩时应适当调大
pub const ENEMY_ATTACK: f32 = 75.0;

// Boss
pub const BOOS_HEALTH: f32 = 2000.0;//暂定
pub const BOSS_CHARGE_SPEED: f32 = 2.0 * 240.0 / fps as f32;

// Kd-tree
pub const KD_TREE_REFRESH_RATE: f32 = 0.1;

// Gun
pub const BULLET_SPAWN_INTERVAL: f32 = 0.2 * 0.5;
pub const BULLET_TIME_SECS: f32 = 3.0;
pub const BULLET_SPEED: f32 = 8.0 * 100.0 / fps as f32;
pub const BULLET_DAMAGE: f32 = 15.0 * 2.0;

pub const NUM_BULLETS_PER_SHOT: usize = 10;

// Colors
pub const BG_COLOR: (u8, u8, u8) = (197, 204, 184);
