use bevy::{dev_tools::states::*, prelude::*};
use bevy::math::vec3;
use crate::gamestate::GameState;

pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, log_transitions::<GameState>)
            .add_systems(OnEnter(GameState::Home), setup)
            .add_systems(OnExit(GameState::Home), cleanup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn( (Sprite {
        image: asset_server.load("BackGround.png"),
        ..Default::default()
        },
        Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::new(0.0, -90.0, 0.0)),
        ));
    commands.spawn( (Sprite {
        image: asset_server.load("ForeGround.png"),
        ..Default::default()
        },
        Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::new(0.0, 0.0, 5.0)),
        ));
}

fn cleanup(
    mut commands: Commands, 
    mut menu_items_query: Query<Entity, With<Sprite>>) {
    for parent in &mut menu_items_query {
        commands.entity(parent).despawn_recursive();
    }
}