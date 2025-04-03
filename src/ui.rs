use bevy::{dev_tools::states::*, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, state::{self, commands}, transform};

use crate::{gamestate::GameState,
            character::Character,};

pub struct UIPlugin;

#[derive(Component)]
pub struct Bar;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Home), setup_ui_all)
        .add_systems(Update, log_transitions::<GameState>);
    }
}

fn setup_ui_all (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load("UI_Hub_PlayerHealth_Bar.png"),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(2.0))
            .with_translation(Vec3::new(-600.0, -250.0, 90.0)),
    ))
    .with_child((
        Sprite {
            image:asset_server.load("UI_Hub_TeamHealth.png"),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(2.0))
            .with_translation(Vec3::new(-600.0, -250.0, 90.0)),
        // Color::
    ));
}