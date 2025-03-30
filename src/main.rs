use bevy::{dev_tools::states::*, diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy::window::CursorOptions;
use bevy_framepace::{FramepacePlugin, Limiter};


use demo::gamestate::GameState;
use demo::gui::GuiPlugin;
use demo::camera::FollowCameraPlugin;
use demo::character::PlayerPlugin;
use demo::animation::AnimationPlugin;
use demo::gun::GunPlugin;
use demo::resources::ResourcesPlugin;
use demo::audio::GameAudioPlugin;
use demo::home::HomePlugin;

use demo::room::RoomPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Blue Archieve!".to_string(),
                resolution: (1600.0,700.0).into(),
                cursor_options: CursorOptions{visible: false, ..Default::default()},
                resizable: false,
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize:false,
                    minimize:true,
                    close:true,
                },
                decorations: true,
                ..default()
            }),
            ..default()
            })
            .set(ImagePlugin::default_nearest())) // prevents blurry sprites
        // .add_plugins((
        //     LogDiagnosticsPlugin::default(),))
        .add_plugins(FramepacePlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(FollowCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GunPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(GameAudioPlugin)
        .add_plugins(HomePlugin)

        .add_plugins(RoomPlugin)

        .insert_state(GameState::MainMenu)
        .add_systems(Startup, set_rate)
        .add_systems(Update, log_transitions::<GameState>)
        .run();
}

fn set_rate(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    settings.limiter = Limiter::from_framerate(70.0); 
}

 





