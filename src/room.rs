use bevy::{dev_tools::states::*, 
           prelude::*, 
           math::Vec3, 
           ecs::system::EntityCommands, };
use bevy_ecs_tiled::{prelude::*, };
use bevy_ecs_tilemap::{map::TilemapSize, TilemapBundle};

use crate::{
    gamestate::GameState,
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
            Transform::from_scale(Vec3::splat(2.0)),
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
            .add_plugins(TiledMapPlugin::default())
            .add_plugins(TiledPhysicsPlugin::<TiledPhysicsRapierBackend>::default())

            .add_systems(OnEnter(GameState::InGame), load_room)
            .add_systems(Update, switch_map.run_if(in_state(GameState::InGame)))
            .add_systems(Update, log_transitions::<GameState>);
    }
}
fn load_room(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    // let map_handle: Handle<TiledMap> = asset_server.load("Boss房1.tmx");
    // commands.spawn((
    //     TiledMapHandle(map_handle),
    //     TiledMapAnchor::Center,
    //     Transform::from_scale(Vec3::splat(2.0)),
    // ));
    let mut mgr = AssetsManager::new(&mut commands);
    mgr.add_map(MapInfos::new(
        &asset_server, 
        "Boss房1.tmx", 
        "A finite orthogonal map with all colliders", 
        |c| {
            c.insert((
                TiledMapAnchor::Center,
                TiledPhysicsSettings::<TiledPhysicsRapierBackend>::default(),
            ));
        },
    ));
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