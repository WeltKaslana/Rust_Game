use bevy::{
    animation::transition, color::palettes::css::{BLUE, GREEN, RED}, dev_tools::states::*, ecs::{component::ComponentId, system::EntityCommands, world::DeferredWorld}, log::tracing_subscriber::fmt::time, math::{Vec3, VectorSpace}, prelude::*, time::Stopwatch, utils::info 
    };
use bevy_ecs_tiled::{prelude::*,};
use std::time::Duration;
use bevy_rapier2d::{prelude::*};
use rand::Rng;

use crate::{
    boss::{
        self, 
        Boss, 
        BossComponent, 
        BossDeathEvent, 
        BossSetupEvent
    }, 
    character::{
        AnimationConfig, 
        Character, 
        Health, 
        Player
    }, 
    configs::*, 
    enemy::{
        BaseSetupEvent, 
        Enemy, 
        EnemyDeathEffect, 
        EnemyDeathEvent, 
        EnemybornPoint, 
        Enemybornduration, 
        Enemybornflag, 
        Enemyterm
    }, 
    gamestate::{GameState, InGameState}, 
    gui::{test, Transition}, 
    gun::Bullet, 
    resources::*,
    components::*,
};
pub struct RoomPlugin;

#[derive(Component)]
pub struct EnemyBorn;

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct Progress(pub f32);

#[derive(Component)]
pub struct Chest(pub i32);
// 0:effect 1:big can't open 2: small can't open 3: big ready to open 4: small ready to open 5: big opening 6: small opening 

#[derive(Component)]
pub struct ChestType(pub u8);

#[derive(Component)]
pub struct Door(pub i32);
// 0:can't open 1:ready to open 2:opening 3:opened 4:closing

#[derive(Event)]
pub struct RoomCleanEvent;

////test
pub type MapInfosCallback = fn(&mut EntityCommands);

// 打完一个循环就重新刷新地图
static mut reload_map: bool = false;
// 播放房间清空音效
pub static mut clear_sound: bool = false;

pub struct MapInfos {
    pub asset: Handle<TiledMap>,
    path: String,
    description: String,
    callback: MapInfosCallback,
}

impl MapInfos {
    pub fn new(
        asset_server: &Res<AssetServer>,
        path: &str,
        description: &str,
        callback: MapInfosCallback,
    ) -> Self {
        Self {
            asset: asset_server.load(path.to_owned()),
            path: path.to_owned(),
            description: description.to_owned(),
            callback,
        }
    }
}

#[derive(Resource)]
pub struct AssetsManager {
    pub map_assets: Vec<MapInfos>,
    map_entity: Option<Entity>,
    // text_entity: Entity,
    pub map_index: usize,
}

impl AssetsManager {
    // const BASE_TEXT: &'static str = "<P> = Cycle through different maps";

    pub fn new(commands: &mut Commands) -> Self {
        Self {
            map_assets: Vec::new(),
            map_entity: None,
            // text_entity: commands.spawn(Text::from(AssetsManager::BASE_TEXT)).id(),
            map_index: 0,
        }
    }

    pub fn clear(&mut self, commands: &mut Commands) {
        self.map_assets = Vec::new();
        self.map_entity = None;
        // self.text_entity = commands.spawn(Text::from(AssetsManager::BASE_TEXT)).id();
        self.map_index = 0;
    }

    pub fn add_map(&mut self, map_infos: MapInfos) {
        self.map_assets.push(map_infos);
    }

    pub fn cycle_map(&mut self, commands: &mut Commands) {
        info!(
            " => Switching to map '{}'",
            self.map_assets[self.map_index].path
        );

        // Handle map update: despawn the map if it already exists
        if let Some(entity) = self.map_entity {
            commands.entity(entity).despawn_recursive();
        }

        // Then spawn the new map
        info!("map_index:{}",self.map_index);
        let mut entity_commands = commands.spawn((TiledMapHandle(
            self.map_assets[self.map_index].asset.to_owned(),
            ),
            TiledMapAnchor::Center,
            Transform::from_scale(Vec3::splat(3.0)),
        ));
        (self.map_assets[self.map_index].callback)(&mut entity_commands);
        self.map_entity = Some(entity_commands.id());

        // Update the map index
        self.map_index += 1;
        if self.map_index >= self.map_assets.len() {
            self.map_index = 0;

            // Reload the map
            unsafe {
                reload_map = true;
                // println!("Reloading map");//test
            }
        }
    }
    // 用于在从游戏回到大厅和封面时删除地图
    pub fn del_map(&mut self, commands: &mut Commands) {
        if let Some(entity) = self.map_entity {
            commands.entity(entity).despawn_recursive();
        }
        self.map_entity = None;
        self.map_index = 0;
        info!("map deleted!");
    }
}

////

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CollisionEvent>()
            .add_plugins(TiledMapPlugin::default())
            // Here we use the provided Rapier backend to automatically spawn colliders
            .add_plugins(TiledPhysicsPlugin::<TiledPhysicsRapierBackend>::default())
            // Rapier physics plugins to test and see the collider
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            // 显示碰撞体边框
            .add_plugins(RapierDebugRenderPlugin::default())

            .add_systems(Startup, load_room)
            .add_systems(OnEnter(GameState::Home), load_room1)
            .add_systems(OnEnter(GameState::Loading),(
                evt_object_created.before(load_room2),
                evt_map_created.before(load_room2),
                load_room2,
                ))
            .add_systems(OnEnter(GameState::Loading),(
                del_door_chest_base,
            ))
            .add_systems(Update, (
                check_ifcomplete,
            ).run_if(in_state(GameState::InGame)))
            .add_systems(Update, (
                handle_base_timer,
                handle_timer,
                handle_score,
            ).run_if(in_state(InGameState::Running)))
            ;
    }
}   
fn load_room(
    mut commands: Commands, 
) {
    let mgr = AssetsManager::new(&mut commands);
    commands.insert_resource(mgr);
}
fn load_room1(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut mgr: ResMut<AssetsManager>,
) {
    mgr.clear(&mut commands);
    mgr.add_map(MapInfos::new(
                &asset_server, 
                "跑酷房1.tmx", 
                "A finite orthogonal map with only object colliders", 
                |c| {
                    c.insert((
                        TiledMapAnchor::Center,
                        TiledPhysicsSettings::<TiledPhysicsRapierBackend> {
                            objects_filter: TiledName::All,
                            // objects_layer_filter: TiledName::Names(vec![String::from("1")]),
                            // tiles_objects_filter: TiledName::Names(vec![String::from("platform1")]),
                            //用来过滤图块层
                            // tiles_layer_filter: TiledName::Names(vec![String::from("decoration")]),
                            //用来过滤指定图块层中的图块，对象层同理
                            tiles_objects_filter: TiledName::All,
                            ..default()
                        },
                    ));
    }));
    // // 普通房数量
    // let room_size = 8;
    // // 普通房长度
    // // let len = 1;
    // let len = ROOMS - 1;
    // for _ in 1..=len {
    //     let path = format!("普通房{}.tmx", rand::rng().random_range(1..room_size + 1));
    //     mgr.add_map(MapInfos::new(
    //         &asset_server, 
    //         &path, 
    //         "A finite orthogonal map with only object colliders", 
    //         |c| {
    //             c.insert((
    //                 TiledMapAnchor::Center,
    //                 TiledPhysicsSettings::<TiledPhysicsRapierBackend> {
    //                     objects_filter: TiledName::All,
    //                     // objects_layer_filter: TiledName::Names(vec![String::from("1")]),
    //                     // tiles_objects_filter: TiledName::Names(vec![String::from("platform1")]),
    //                     //用来过滤图块层
    //                     // tiles_layer_filter: TiledName::Names(vec![String::from("decoration")]),
    //                     //用来过滤指定图块层中的图块，对象层同理
    //                     tiles_objects_filter: TiledName::All,
    //                     ..default()
    //                 },
    //             ));
    //         },
    //     ));
    // }
    let boss_room_size = 2;
    let boss_path = format!("boss房{}.tmx", rand::rng().random_range(1..boss_room_size + 1));
    // let boss_path = format!("boss房{}.tmx", rand::rng().random_range(2..3));
    // info!("boss_path:{}", &boss_path);
    mgr.add_map(MapInfos::new(
        &asset_server, 
        &boss_path, 
        "A finite orthogonal map with only object colliders", 
        |c| {
            c.insert((
                TiledMapAnchor::Center,
                TiledPhysicsSettings::<TiledPhysicsRapierBackend> {
                    objects_filter: TiledName::All,
                    // objects_layer_filter: TiledName::Names(vec![String::from("1")]),
                    // tiles_objects_filter: TiledName::Names(vec![String::from("platform1")]),
                    //用来过滤图块层
                    // tiles_layer_filter: TiledName::Names(vec![String::from("decoration")]),
                    //用来过滤指定图块层中的图块，对象层同理
                    tiles_objects_filter: TiledName::All,
                    ..default()
                },
            ));
        },
    ));
}

// 在加载boss房2的时候重载boss房1的话坐标会错位，反之也会错位
fn load_room2(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut mgr: ResMut<AssetsManager>,
) {
    unsafe {
        if !reload_map {
            return;
        }
    }
    let map_now = mgr.map_entity.clone();
    mgr.clear(&mut commands);
    mgr.map_entity = map_now.clone();
    // 普通房数量
    let room_size = 8;
    // 普通房长度
    let len = ROOMS - 1;
    for _ in 1..=len {
        let path = format!("普通房{}.tmx", rand::rng().random_range(1..room_size + 1));
        mgr.add_map(MapInfos::new(
            &asset_server, 
            &path, 
            "A finite orthogonal map with only object colliders", 
            |c| {
                c.insert((
                    TiledMapAnchor::Center,
                    TiledPhysicsSettings::<TiledPhysicsRapierBackend> {
                        objects_filter: TiledName::All,
                        tiles_objects_filter: TiledName::All,
                        ..default()
                    },
                ));
            },
        ));
    }
    let boss_room_size = 2;
    let boss_path = format!("boss房{}.tmx", rand::rng().random_range(2..boss_room_size + 1));

    // let boss_path = format!("boss房{}.tmx", rand::rng().random_range(1..2));
    // println!("boss_path: {}", boss_path);
    mgr.add_map(MapInfos::new(
        &asset_server, 
        &boss_path, 
        "A finite orthogonal map with only object colliders", 
        |c| {
            c.insert((
                TiledMapAnchor::Center,
                TiledPhysicsSettings::<TiledPhysicsRapierBackend> {
                    objects_filter: TiledName::All,
                    tiles_objects_filter: TiledName::All,
                    ..default()
                },
            ));
        },
    ));
    unsafe {
        reload_map = false;
    }
}


fn check_collision(
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.read() {
        println!("collision happen!");
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                println!("Collision started between {:?} and {:?}", entity1, entity2);
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                println!("Collision stopped between {:?} and {:?}", entity1, entity2);
            }
        }
    }
}

fn evt_object_created(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,

    mut object_events: EventReader<TiledObjectCreated>,
    mut object_query: Query<(&Name, &mut Transform), (With<TiledMapObject>, Without<Character>)>,
    mut player_query: Query<&mut Transform, (With<Character>, Without<TiledMapObject>)>,
    source: Res<AssetsManager>,
    maps: Res<Assets<TiledMap>>,
    source2: Res<GlobalEnemyTextureAtlas>,
    source3: Res<GlobalRoomTextureAtlas>,

    mut events: EventWriter<BossSetupEvent>,
    mut events2: EventWriter<BaseSetupEvent>,
    
    mut score: ResMut<ScoreResource>,
) {
    let mut size = Vec2::ZERO;
    let index = if source.map_index > 0 {source.map_index - 1} else {source.map_assets.len() - 1};
    let map = source.map_assets[index].asset.clone();
    if let Some(temp) = maps.get(&map) {
        size.x = temp.tilemap_size.x as f32;
        size.y = temp.tilemap_size.y as f32;
        // println!("size:{}, {}", temp.tilemap_size.x,temp.tilemap_size.y);
    };
    size *= 8.0;
    let mut termrng = rand::rng();
    let term = termrng.random_range(1..=3);

    let mut controller_m = false;

    for e in object_events.read() {
        let Ok((name, mut transform)) = object_query.get_mut(e.entity) else {
            return;
        };
        info!("=> Received TiledObjectCreated event for object '{}'", name);
        // println!("loc:{:?}",transform.translation.clone());
        //坐标信息不对，可能是地图迁移导致的
        // 对象的名字是Object(...)
        // 3.0是缩放比例，700和500是相对于程序坐标系的变异量
        if name.as_str() == "Object(Player)" {
            for mut trans in player_query.iter_mut() {
                trans.translation.x = (transform .translation.x - size.x) * 3.0;
                trans.translation.y = (transform .translation.y - size.y) * 3.0;
            }
        }
        if name.as_str() == "Object(Enemy)" {
            // let layout_born = TextureAtlasLayout::from_grid(UVec2::splat(48),12,1,None,None);
            commands.spawn((
                Sprite {
                    image: source2.image_bron.clone(),//后续改用source2
                    texture_atlas: Some(TextureAtlas {
                        layout: source2.layout_born.clone(),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(
                    (transform .translation.x - size.x) * 3.0, 
                    (transform .translation.y - size.y) * 3.0, 
                    0.0)).with_scale(Vec3::splat(2.5)),
                AnimationConfig::new(15),
                Map,
                Enemy,
                EnemyBorn,
            ));
            let duration = Duration::from_millis(1000);
            commands.spawn((
                Transform::from_translation(Vec3::new(
                    (transform .translation.x - size.x) * 3.0, 
                    (transform .translation.y - size.y) * 3.0, 
                    0.0)).with_scale(Vec3::splat(2.5)),
                EnemybornPoint,
                Enemybornduration{
                    timer: Stopwatch::new(),
                    duration,
                },
                Enemyterm(term),
                Enemybornflag(false),
                Map,
            ));
            info!("enemy created! ");
            // next_state.set(GameState::InGame);
        }
        if name.as_str() == "Object(Enemy1)" {//重复刷怪点
            // let layout_born = TextureAtlasLayout::from_grid(UVec2::splat(48),12,1,None,None);
            commands.spawn((
                Sprite {
                    image: source2.image_bron.clone(),//后续改用source2
                    texture_atlas: Some(TextureAtlas {
                        layout: source2.layout_born.clone(),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(
                    (transform .translation.x - size.x) * 3.0, 
                    (transform .translation.y - size.y) * 3.0, 
                    0.0)).with_scale(Vec3::splat(2.5)),
                AnimationConfig::new(15),
                Map,
                Enemy,
                EnemyBorn,
            ));
            let duration = Duration::from_millis(1000);
            commands.spawn((
                Transform::from_translation(Vec3::new(
                    (transform .translation.x - size.x) * 3.0, 
                    (transform .translation.y - size.y) * 3.0, 
                    0.0)).with_scale(Vec3::splat(2.5)),
                EnemybornPoint,
                Enemybornduration{
                    timer: Stopwatch::new(),
                    duration,
                },
                Enemyterm(10),
                Enemybornflag(true),
                Map,
            ));
            info!("enemy created! ");
        }
        if name.as_str() == "Object(Boss)" {
            events.send(BossSetupEvent);
            // boss诞生动画
            commands.spawn((
                Sprite {
                    image: source2.image_bron.clone(),//后续改用source2
                    texture_atlas: Some(TextureAtlas {
                        layout: source2.layout_born.clone(),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(
                    (transform .translation.x - size.x) * 3.0, 
                    (transform .translation.y - size.y) * 3.0, 
                    0.0)).with_scale(Vec3::splat(2.5)),
                AnimationConfig::new(10),
                Map,
                BossComponent,
                EnemyBorn,
            ));
            // boss诞生警告
            // commands.spawn((
            //     Sprite {
            //         image: source2.image_boss_alert.clone(),
            //         texture_atlas: Some(TextureAtlas {
            //             layout: source2.layout_boss_alert.clone(),
            //             index: 0,
            //         }),
            //         ..Default::default()
            //     },
            //     Transform::from_translation(Vec3::new(
            //         (transform .translation.x - size.x) * 3.0, 
            //         (transform .translation.y - size.y) * 3.0, 
            //         0.0)).with_scale(Vec3::splat(2.5)),
            //     AnimationConfig::new(10),
            //     Map,
            //     BossAlert,
            // ));
            info!("boss created! ");
            // next_state.set(GameState::InGame);
        }
        if name.as_str() == "Object(base)" { 
            events2.send(BaseSetupEvent);
            commands.spawn((
                Sprite {
                    image: asset_server.load("Prenapaters _Statue.png"),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(
                    (transform .translation.x - size.x) * 3.0, 
                    (transform .translation.y - size.y) * 3.0, 
                    0.0)).with_scale(Vec3::splat(2.5)),
                Map,
                Progress(1.0),
                Enemybornduration{
                    timer: Stopwatch::new(),
                    duration: Duration::from_secs(1),
                }
            ));
        }
        if name.as_str() == "Object(door)" {
            commands.spawn((
                Sprite {
                    image: source3.image_door_open.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: source3.layout_door_open.clone(),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(
                    (transform .translation.x - size.x) * 3.0, 
                    (transform .translation.y - size.y) * 3.0, 
                    -52.0)).with_scale(Vec3::splat(2.5)),
                AnimationConfig::new(15),
                Map,
                Door(0),
            ));
            info!("door created!");
        }
        if name.as_str() == "Object(chest)" {
            let mut rng = rand::rng();
            let mut i = rng.random_range(0..99);

            let map_index = score.map_index as i32;
            
            let map = map_index - map_index / ROOMS * ROOMS;
            if map == ROOMS - 1 {i = 0;}
            match i {
                0..=5 => {
                    commands.spawn((
                        Sprite {
                            image: source3.image_chest_big2.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: source3.layout_chest_big2.clone(),
                                index: 0,
                            }),
                            ..Default::default()
                        },
                       Transform::from_translation(Vec3::new(
                            (transform .translation.x - size.x) * 3.0, 
                            (transform .translation.y - size.y) * 3.0, 
                            -52.0)).with_scale(Vec3::splat(2.5)),
                        AnimationConfig::new(15),
                        Map,
                        Chest(1),
                        ChestType(0),
                    )).with_child((
                        Sprite {
                            image: source3.image_chest_big2_effect1.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: source3.layout_chest_big2_effect1.clone(),
                                index: 0,
                            }),
                            ..Default::default()
                        },
                        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)).with_scale(Vec3::splat(2.5)),
                        AnimationConfig::new(15),
                        Chest(0),
                    )).with_child((
                        Sprite {
                            image: source3.image_chest_big2_effect2.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: source3.layout_chest_big2_effect2.clone(),
                                index: 0,
                            }),
                            ..Default::default()
                        },
                        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)).with_scale(Vec3::splat(2.5)),
                        AnimationConfig::new(15),
                        Chest(0),
                    ));
                },
                6..=30 => {
                    commands.spawn((
                        Sprite {
                            image: source3.image_chest_big1.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: source3.layout_chest_big1.clone(),
                                index: 0,
                            }),
                            ..Default::default()
                        },
                        Transform::from_translation(Vec3::new(
                            (transform .translation.x - size.x) * 3.0, 
                            (transform .translation.y - size.y) * 3.0, 
                            -52.0)).with_scale(Vec3::splat(2.5)),
                        AnimationConfig::new(15),
                        Map,
                        Chest(1),
                        ChestType(1),
                    ));
                },
                31..=99 => {
                    commands.spawn((
                        Sprite {
                            image: source3.image_chest_small.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: source3.layout_chest_small.clone(),
                                index: 0,
                            }),
                            ..Default::default()
                        },
                        Transform::from_translation(Vec3::new(
                            (transform .translation.x - size.x) * 3.0, 
                            (transform .translation.y - size.y) * 3.0, 
                            -52.0)).with_scale(Vec3::splat(2.5)),
                        AnimationConfig::new(15),
                        Map,
                        Chest(2),
                        ChestType(2),
                    ));
                },
                _ => {},
            }
            info!("chest created!");
        }
        if name.as_str() == "Object(parkour)"  {
            controller_m = true;
        }
    }
    // 检测是否为跑酷房
    score.controller_mode = controller_m;
    println!("controller mode: {}", controller_m);
}

fn evt_map_created (
    mut commands: Commands,
    mut map_events: EventReader<TiledMapCreated>,
    mut q: Query<Entity,(With<Collider>, Without<RigidBody>, Without<Bullet>, Without<Character>)>,

) {
    //为瓦片添加属性
    for _ in map_events.read() {
        if q.is_empty(){
            return;
        }
        for e in q.iter_mut() {
            commands.entity(e)
                .insert(RigidBody::Fixed)
                .insert(LockedAxes::TRANSLATION_LOCKED);
                // .insert(ActiveEvents::COLLISION_EVENTS)
                // .insert(GravityScale(0.0));
                // .insert(Sensor);
        }
    }

}

fn check_ifcomplete(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut enemyclear_query1: Query<(&mut Health), (With<Enemy>, Without<EnemyBorn>, Without<BossComponent>)>,
    enemyclear_query2: Query<Entity, (With<EnemyBorn>)>,
    bossclear_query: Query<Entity, (With<BossComponent>, Without<EnemyBorn>, Without<Enemy>)>,
    transition_query: Query<Entity, (With<Transition>, Without<Enemy>)>,
    mut camera_query: Query<(&Transform, &mut GameState), With<Camera2d>>,
    mut door_query: Query<(&mut Door, & Transform), With<Door>>,
    mut chest_query: Query<&mut Chest, With<Chest>>,
    player_query: Query<&Transform, With<Character>>,
    mut bornplace_query: Query<(&Transform, &mut Enemybornflag, &mut Enemybornduration, &mut Enemyterm), With<EnemybornPoint>>,
    source: Res<GlobalEnemyTextureAtlas>,
    clear: Res<GlobalMenuTextureAtlas>,
    time: Res<Time>,
    mut room_clean_events: EventWriter<RoomCleanEvent>,
    mut score: ResMut<ScoreResource>,
) {

    let mut flag = true;
    let mut timerng = rand::rng();

    for (transform, mut bornflag, mut duration, mut term) in bornplace_query.iter_mut() {
        if bornflag.0 == true {
            flag = false;
            match term.0 {
                0 => {flag = true;},
                1..=3 => {
                    bornflag.0 = false;
                    term.0 -= 1;
                    commands.spawn((
                        Sprite {
                            image: source.image_bron.clone(),//后续改用source2
                            texture_atlas: Some(TextureAtlas {
                                layout: source.layout_born.clone(),
                                index: 0,
                            }),
                            ..Default::default()
                        },
                        Transform::from_translation(Vec3::new(
                            transform.translation.x, 
                            transform.translation.y,
                            0.0)).with_scale(Vec3::splat(2.5)),
                        AnimationConfig::new(15),
                        Map,
                        Enemy,
                        EnemyBorn,
                    ));
                },
                _ => {
                    duration.timer.tick(time.delta());
                    if duration.timer.elapsed() >= duration.duration {
                        duration.timer.reset();
                        let random_duration = Duration::from_millis(timerng.random_range(500..=2000));
                        duration.duration = random_duration;
                        commands.spawn((
                            Sprite {
                                image: source.image_bron.clone(),//后续改用source2
                                texture_atlas: Some(TextureAtlas {
                                    layout: source.layout_born.clone(),
                                    index: 0,
                                }),
                                ..Default::default()
                            },
                            Transform::from_translation(Vec3::new(
                                transform.translation.x, 
                                transform.translation.y,
                                0.0)).with_scale(Vec3::splat(2.5)),
                            AnimationConfig::new(15),
                            Map,
                            Enemy,
                            EnemyBorn,
                        ));
                    }
                    
                }
            }
        } else if bornflag.0 == false && term.0 != 0 {
            if enemyclear_query1.is_empty() && enemyclear_query2.is_empty() {
                bornflag.0 = true;
            }
            flag = false;
        }
    }

    if enemyclear_query1.is_empty() && enemyclear_query2.is_empty() && bossclear_query.is_empty() && flag == true{
        unsafe {
            if !clear_sound &&  !score.controller_mode {
                // 房间清空
                clear_sound = true;
                room_clean_events.send(RoomCleanEvent);
            }   
        }
        let player_transform = player_query.single();
        let (mut door, door_transform) =door_query.single_mut();
        if door.0 == 0 { door.0 = 1; } 

        for mut chest in chest_query.iter_mut() {
            if chest.0 == 1 { chest.0 = 3; }
            if chest.0 == 2 { chest.0 = 4; }
        }

        //通关提示

        let distance = player_transform.translation.distance(door_transform.translation);

        if distance <= 125.0 &&  door.0 == 3 {

            if keyboard_input.just_pressed(KeyCode::KeyE) && transition_query.is_empty() {
                println! ("你过关!");

                score.map_index += 1;//difficulty

                for (trans, mut nextstate) in camera_query.iter_mut() {
                    commands.spawn((
                        Sprite {
                            image: clear.transition.clone(),
                            ..Default::default()
                        },
                        Transform::from_scale(Vec3::new(0.7,0.7,0.5))
                            .with_translation(Vec3::new(trans.translation.x-3200.0, trans.translation.y, 100.0)),
                        Transition,
                        ZIndex(1),
                    )); 
                    *nextstate = GameState::Loading;
                }
            }
        }
    }

}

fn del_door_chest_base(
    mut commands: Commands,
    door_query: Query<Entity, With<Door>>,
    chest_query: Query<Entity, With<Chest>>,
    base_query: Query<Entity, With<Progress>>
) {
    if !door_query.is_empty() {
        let entity = door_query.single();
        commands.entity(entity).despawn();
    }
    if !chest_query.is_empty() {
        for entity in chest_query.iter() {
            commands.entity(entity).despawn();
        }
    }
    if !base_query.is_empty() {
        let entity = base_query.single();
        commands.entity(entity).despawn();
    }
}

fn handle_base_timer(
    mut base_query: Query<(&mut Progress, &mut Enemybornduration), With<Progress>>,
    mut enemybornpoint_query: Query<&mut Enemyterm, With<Enemybornflag>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    born_query: Query<Entity, With<EnemyBorn>>,
    time: Res<Time>,
    mut commands: Commands,
    source: Res<GlobalEnemyTextureAtlas>
) {
    if base_query.is_empty() {
        return ;
    }
    let (mut progress, mut dtimer) = base_query.single_mut();
    dtimer.timer.tick(time.delta());
    if dtimer.timer.elapsed() >= dtimer.duration  && progress.0 < 10.0 {
        progress.0 += 1.0;
        dtimer.timer.reset();
        println!("进度:{}", progress.0);
        if progress.0 >= Survial_Time {
            for e in born_query.iter() {
                commands.entity(e).despawn();
            }
            for (entity, loc) in enemy_query.iter() {
                commands.entity(entity).despawn();
                commands.spawn( (
                    Sprite {
                        image: source.image_death.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: source.layout_death.clone(),
                            index: 0,
                        }),
                        ..Default::default()
                    },
                    Transform::from_scale(Vec3::splat(2.5)).with_translation(Vec3::new(loc.translation.x, loc.translation.y, -50.0)),
                    AnimationConfig::new(10),
                    EnemyDeathEffect,
                    Map,
                )
                );
            }
            for mut enemyterm in enemybornpoint_query.iter_mut() {
                enemyterm.0 = 0;
            }
        }
    }
}   

fn handle_timer(
    time: Res<Time>,
    mut score:  ResMut<ScoreResource>,
) {
    score.timer.tick(time.delta());
    if score.timer.elapsed() >= Duration::from_secs(1) {
        score.time_sec += 1;
        score.timer.reset();
        if score.time_sec >= 60 {
            score.time_min += 1;
            score.time_sec = 0;
        }
        // println!("{}:{},map:{}",score.time_min,score.time_sec,score.map_index);
    }
}

fn handle_score(
    mut score:  ResMut<ScoreResource>,
    mut enemy_death_event: EventReader<EnemyDeathEvent>,
    mut boss_death_event: EventReader<BossDeathEvent>,
) {
    for _ in enemy_death_event.read() {
        score.enemy_score += 1;
    }
    for _ in boss_death_event.read() {
        score.boss_score += 1;
    }
}
