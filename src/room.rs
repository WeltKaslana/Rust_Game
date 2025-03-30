use bevy::{dev_tools::states::*, prelude::*,math::Vec3, };
// use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tiled::{prelude::*};
use bevy_ecs_tilemap::{map::TilemapSize, TilemapBundle};

use crate::{
    gamestate::GameState,
};
pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TiledMapPlugin::default())
            .add_systems(Update, log_transitions::<GameState>)
            // .add_systems(Startup, load_room)
            .add_systems(OnEnter(GameState::InGame), load_room);
    }
}
fn load_room(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) { 
    let map_handle: Handle<TiledMap> = asset_server.load("Boss房1.tmx");
    commands.spawn((
        TiledMapHandle(map_handle),
        TiledMapAnchor::Center,
        Transform::from_scale(Vec3::splat(2.0)),
    ));
}