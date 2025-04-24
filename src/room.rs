use bevy::{
    animation::transition, color::palettes::css::{BLUE, GREEN, RED}, dev_tools::states::*, ecs::{component::ComponentId, system::EntityCommands, world::DeferredWorld}, math::{Vec3, VectorSpace}, prelude::* 
    };
use bevy_ecs_tiled::{prelude::*,};
// use bevy_ecs_tilemap::{map::TilemapSize, TilemapBundle};

use bevy_rapier2d::{prelude::*};

use crate::{
    boss::{self, BossComponent}, character::{AnimationConfig, Character}, enemy::Enemy, gamestate::GameState, gui::Transition, gun::Bullet, resources::*
};
pub struct RoomPlugin;

#[derive(Component)]
pub struct EnemyBorn;

////test
pub type MapInfosCallback = fn(&mut EntityCommands);

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
    text_entity: Entity,
    pub map_index: usize,
}

impl AssetsManager {
    const BASE_TEXT: &'static str = "<P> = Cycle through different maps";

    pub fn new(commands: &mut Commands) -> Self {
        Self {
            map_assets: Vec::new(),
            map_entity: None,
            text_entity: commands.spawn(Text::from(AssetsManager::BASE_TEXT)).id(),
            map_index: 0,
        }
    }

    pub fn add_map(&mut self, map_infos: MapInfos) {
        self.map_assets.push(map_infos);
    }

    pub fn cycle_map(&mut self, commands: &mut Commands) {
        info!(
            " => Switching to map '{}'",
            self.map_assets[self.map_index].path
        );

        // Update displayed text
        commands.entity(self.text_entity).insert(Text::from(format!(
            "{}\nmap name = {}\n{}",
            AssetsManager::BASE_TEXT,
            self.map_assets[self.map_index].path,
            self.map_assets[self.map_index].description
        )));

        // Handle map update: despawn the map if it already exists
        if let Some(entity) = self.map_entity {
            commands.entity(entity).despawn_recursive();
        }

        // Then spawn the new map
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
        }
    }
}

////

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CollisionEvent>()
            .add_plugins(TiledMapPlugin::default())
            // .add_plugins(TiledPhysicsPlugin::<MyCustomPhysicsBackend>::default())
            // Here we use the provided Rapier backend to automatically spawn colliders
            .add_plugins(TiledPhysicsPlugin::<TiledPhysicsRapierBackend>::default())
            // Rapier physics plugins to test and see the collider
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())

            // .add_systems(OnEnter(GameState::InGame), load_room)
            .add_systems(Update, switch_map.run_if(in_state(GameState::InGame)))

            // .add_systems(OnEnter(GameState::Loading), load_room)
            .add_systems(Startup, load_room)
            .add_systems(Update, (
                // check_collision,
                // check_contact,
                evt_object_created,
                evt_map_created,
                ).run_if(in_state(GameState::Loading)))
            .add_systems(Update, (
                check_ifcomplete,
            ).run_if(in_state(GameState::InGame)))
            .add_systems(Update, log_transitions::<GameState>);
    }
}   
fn load_room(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let mut mgr = AssetsManager::new(&mut commands);
    
    mgr.add_map(MapInfos::new(
        &asset_server, 
        "普通房1.tmx", 
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
    mgr.add_map(MapInfos::new(
        &asset_server, 
        "普通房2.tmx", 
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
    // mgr.add_map(MapInfos::new(
    //     &asset_server, 
    //     "boss房2.tmx", 
    //     "A finite orthogonal map with only object colliders", 
    //     |c| {
    //         c.insert((
    //             TiledMapAnchor::Center,
    //             TiledPhysicsSettings::<TiledPhysicsRapierBackend> {
    //                 objects_filter: TiledName::All,
    //                 // objects_layer_filter: TiledName::Names(vec![String::from("1")]),
    //                 // tiles_objects_filter: TiledName::Names(vec![String::from("platform1")]),
    //                 //用来过滤图块层
    //                 // tiles_layer_filter: TiledName::Names(vec![String::from("decoration")]),
    //                 //用来过滤指定图块层中的图块，对象层同理
    //                 tiles_objects_filter: TiledName::All,
    //                 ..default()
    //             },
    //         ));
    //     },
    // ));
    // mgr.cycle_map(&mut commands);
    commands.insert_resource(mgr);
}

fn switch_map(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mgr: ResMut<AssetsManager>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        mgr.cycle_map(&mut commands);
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
fn check_contact(
    q: Query<(&RapierContextSimulation)>,
    b: Query<Entity, With<Bullet>>,
) {
    for r in q.iter() {
        for bu in b.iter() {
            
        }
        // r.contact_pairs_with(context_colliders, rigidbody_set, collider)
    }
}
fn evt_object_created(
    mut commands: Commands,
    //敌人诞生动画还没加入到resources中，后续完善了就改用source2调用图片
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,

    mut object_events: EventReader<TiledObjectCreated>,
    mut object_query: Query<(&Name, &mut Transform), (With<TiledMapObject>, Without<Character>)>,
    mut player_query: Query<&mut Transform, (With<Character>, Without<TiledMapObject>)>,
    source: Res<AssetsManager>,
    maps: Res<Assets<TiledMap>>,
    // source: Res<GlobalEnemyTextureAtlas>,
    // source2: Res<GlobalEnemyTextureAtlas>,
    // mut next_state: ResMut<NextState<GameState>>,
    
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
            let layout_born = TextureAtlasLayout::from_grid(UVec2::splat(48),12,1,None,None);
            commands.spawn((
                Sprite {
                    image: asset_server.load("Entity_Spawn.png"),//后续改用source2
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layouts.add(layout_born),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(
                    (transform .translation.x - size.x) * 3.0, 
                    (transform .translation.y - size.y) * 3.0, 
                    0.0)).with_scale(Vec3::splat(2.5)),
                AnimationConfig::new(15),
                Enemy,
                EnemyBorn,
            ));
            info!("enemy created! ");
            // next_state.set(GameState::InGame);
        }
        if name.as_str() == "Object(Boss)" {
            let layout_born = TextureAtlasLayout::from_grid(UVec2::splat(48),12,1,None,None);
            commands.spawn((
                Sprite {
                    image: asset_server.load("Entity_Spawn.png"),//后续改用source2
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layouts.add(layout_born),
                        index: 0,
                    }),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(
                    (transform .translation.x - size.x) * 3.0, 
                    (transform .translation.y - size.y) * 3.0, 
                    0.0)).with_scale(Vec3::splat(2.5)),
                AnimationConfig::new(15),
                BossComponent,
                EnemyBorn,
            ));
            info!("boss created! ");
            // next_state.set(GameState::InGame);
        }
    }

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
    asset_server: Res<AssetServer>,//test
    keyboard_input: Res<ButtonInput<KeyCode>>,
    enemyclear_query1: Query<Entity, (With<Enemy>, Without<EnemyBorn>, Without<BossComponent>)>,
    enemyclear_query2: Query<Entity, (With<EnemyBorn>, Without<Enemy>, Without<BossComponent>)>,
    bossclear_query: Query<Entity, (With<BossComponent>, Without<EnemyBorn>, Without<Enemy>)>,
    transition_query: Query<Entity, (With<Transition>, Without<Enemy>)>,
    camera_query: Query<&Transform, With<Camera2d>>,
) {
    if enemyclear_query1.is_empty() && enemyclear_query2.is_empty() && bossclear_query.is_empty() {
        println!("你过关!");
        if keyboard_input.just_pressed(KeyCode::KeyE) && transition_query.is_empty() {
            for trans in camera_query.iter() {
                commands.spawn((
                    Sprite {
                        image: asset_server.load("Menu_Transition1.png"),
                        ..Default::default()
                    },
                    Transform::from_scale(Vec3::new(0.7,0.7,0.5))
                        .with_translation(Vec3::new(trans.translation.x-3200.0, trans.translation.y, 100.0)),
                    Transition,
                ));                     
            }
        }

    }

}




