use bevy::{color::palettes::css::{BLUE, GREEN, RED}, dev_tools::states::*, ecs::{component::ComponentId, query, system::EntityCommands, world::DeferredWorld
                }, math::Vec3, prelude::*, transform};
use bevy_ecs_tiled::{prelude::*,};
// use bevy_ecs_tilemap::{map::TilemapSize, TilemapBundle};

use bevy_rapier2d::prelude::*;

use crate::{
    gamestate::GameState,
    character::Character,
};
pub struct RoomPlugin;

////test
pub type MapInfosCallback = fn(&mut EntityCommands);

pub struct MapInfos {
    asset: Handle<TiledMap>,
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
    map_assets: Vec<MapInfos>,
    map_entity: Option<Entity>,
    text_entity: Entity,
    map_index: usize,
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

            .add_systems(OnEnter(GameState::InGame), load_room)
            .add_systems(Update, switch_map.run_if(in_state(GameState::InGame)))
            .add_systems(Update, (
                check_collision,
                evt_object_created,
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
        "Boss房1.tmx", 
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
        "boss房2.tmx", 
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
    mgr.cycle_map(&mut commands);
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
    // mut commands:Commands,
    mut collision_events: EventReader<CollisionEvent>,
    // mut q: Query<Entity,(With<Collider>,Without<RigidBody>)>,
) {
    
    // if q.is_empty(){
    //     return;
    // }
    // for e in q.iter_mut() {
    //     commands.entity(e).insert(RigidBody::Dynamic)
    //         .insert(GravityScale(0.0))
    //         .insert(Sensor);
        
    //     // println!("over!");
    // }
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
    mut object_events: EventReader<TiledObjectCreated>,
    mut object_query: Query<(&Name, &mut Transform), With<TiledMapObject>>,
) {
    for e in object_events.read() {
        let Ok((name, mut transform)) = object_query.get_mut(e.entity) else {
            return;
        };
        info!("=> Received TiledObjectCreated event for object '{}'", name);
        println!("loc:{:?}",transform.translation.clone());
        //坐标信息不对，可能是地图迁移导致的
        commands.spawn((
            Transform::from_translation(transform.translation.clone()),
            Collider::ball(5.0),
        ));
    }
}
// #[derive(Default, Debug, Clone, Reflect)]
// #[reflect(Default, Debug)]
// struct MyCustomPhysicsBackend;

// // This one will just spawn an entity with a `MyCustomPhysicsComponent` Component,
// // at the center of where the Tiled collider is.
// impl TiledPhysicsBackend for MyCustomPhysicsBackend {
//     fn spawn_colliders(
//         &self,
//         commands: &mut Commands,
//         _tiled_map: &TiledMap,
//         _filter: &TiledNameFilter,
//         collider: &TiledCollider,
//     ) -> Vec<TiledColliderSpawnInfos> {
//         match collider {
//             TiledCollider::Object {
//                 layer_id: _,
//                 object_id: _,
//             } => {
//                 vec![TiledColliderSpawnInfos {
//                     name: String::from("Custom[Object]"),
//                     entity: commands
//                         .spawn(MyCustomPhysicsComponent(Color::from(BLUE)))
//                         .id(),
//                     transform: Transform::default(),
//                 }]
//             }
//             TiledCollider::TilesLayer { layer_id: _ } => {
//                 vec![TiledColliderSpawnInfos {
//                     name: String::from("Custom[TilesLayer]"),
//                     entity: commands
//                         .spawn(MyCustomPhysicsComponent(Color::from(RED)))
//                         .id(),
//                     transform: Transform::default(),
//                 }]
//             }
//         }
//     }
// }

// // For debugging purpose, we will also add a 2D mesh where the collider is.
// #[derive(Component)]
// #[component(on_add = on_physics_component_added)]
// struct MyCustomPhysicsComponent(pub Color);

// fn on_physics_component_added(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
//     let color = world.get::<MyCustomPhysicsComponent>(entity).unwrap().0;
//     let mesh = world
//         .resource_mut::<Assets<Mesh>>()
//         .add(Rectangle::from_length(32.));
//     let material = world.resource_mut::<Assets<ColorMaterial>>().add(color);
//     world
//         .commands()
//         .entity(entity)
//         .insert((Mesh2d(mesh), MeshMaterial2d(material)));
// }

