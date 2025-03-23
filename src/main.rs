use bevy::{dev_tools::states::*, diagnostic::LogDiagnosticsPlugin, prelude::*};
use std::{sync::Arc, time::Duration};
use bevy::math::vec3;


// #[derive(Component)]
// struct AnimationConfig {
//     fps: u8,
//     frame_timer: Timer,
// }

// impl AnimationConfig {
//     fn new(fps: u8) -> Self {
//         Self {
//             fps,
//             frame_timer: Self::timer_from_fps(fps),
//         }
//     }

//     fn timer_from_fps(fps: u8) -> Timer {
//         Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
//     }
// }

// fn execute_animations(
//     time: Res<Time>, 
//     mut query: Query<(&mut Transform, &mut AnimationConfig, &mut Sprite)>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
// ) {
//     for (mut transform,mut config, mut sprite) in &mut query {
//         if  keyboard_input.pressed(KeyCode::KeyD){
//             // We track how long the current sprite has been displayed for
//             config.frame_timer.tick(time.delta());
//             // If it has been displayed for the user-defined amount of time (fps)...
//             if config.frame_timer.just_finished(){
//                 if let Some(atlas) = &mut sprite.texture_atlas {
//                     config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
//                     atlas.index = (atlas.index + 1) % 10;
//                 }
                
//                 transform.translation += vec3(10.0, 0.0, 0.0);//test

//             }
//         }
//         else {
//             if let Some(atlas) = &mut sprite.texture_atlas {
//                 atlas.index = 1;
//             }

//             transform.translation.x = 50.0;

//         }
//     }
// }

// #[derive(Component)]
// struct LeftSprite;


// fn setup (
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
//     // mut state: ResMut<NextState<GameState>>,
// ) {

//     // commands.spawn(Camera2d);

//     // Load the sprite sheet using the `AssetServer`
//     let texture: Handle<Image> = asset_server.load("Shiroko_Move.png");

//     // The sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
//     let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 5, 2, None, None);
//     let texture_atlas_layout = texture_atlas_layouts.add(layout);
//     commands.spawn((
//         Sprite {
//             image: texture.clone(),
//             texture_atlas: Some(TextureAtlas {
//                 layout: texture_atlas_layout.clone(),
//                 index: 1,
//             }),
//             ..Default::default()
//         },
//         Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::new(70.0, 0.0, 30.0)),
//         LeftSprite,
//         AnimationConfig::new(10),
//     ));
//     // state.set(GameState::Home);
// }


use demo::gamestate::GameState;
use demo::gui::GuiPlugin;
use demo::camera::FollowCameraPlugin;
use demo::character::PlayerPlugin;
use demo::animation::AnimationPlugin;
use demo::resources::ResourcesPlugin;
use demo::audio::GameAudioPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fuck you!".to_string(),
                resolution: (1600.0,700.0).into(),
                resizable: false,
                decorations: true,
                ..default()
            }),
            ..default()
            })
            .set(ImagePlugin::default_nearest())) // prevents blurry sprites
        // .add_plugins((
        //     LogDiagnosticsPlugin::default(),))
        .add_plugins(GuiPlugin)
        .add_plugins(FollowCameraPlugin)
        // .add_plugins(PlayerPlugin)
        // .add_plugins(AnimationPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(GameAudioPlugin)
        .insert_state(GameState::MainMenu)
        // .add_systems(OnEnter(GameState::Home), setup)
        // .add_systems(Update, execute_animations.run_if(in_state(GameState::Home)))
        // .add_systems(Startup, audio_play)
        .add_systems(Update, log_transitions::<GameState>)
        .run();
}


 





