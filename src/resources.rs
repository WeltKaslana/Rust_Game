use bevy::render::camera;
use bevy::{dev_tools::states::*, prelude::*};
use bevy::window::PrimaryWindow;

use crate::gamestate::GameState;

pub struct ResourcesPlugin;



// #[derive(Resource,Default)]
// pub struct GlobalCharacterTextureAtlas {
//     pub lay_out_idle: Handle<TextureAtlasLayout>,
//     pub image_idle: Handle<Image>,
//     pub lay_out_move: Option<Handle<TextureAtlasLayout>>,
//     pub image_move: Option<Handle<Image>>,
// }



#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CursorPosition(None))
            // .add_systems(OnEnter(GameState::MainMenu), load_assets)
            // .add_systems(
            //     Update,
            //     update_cursor_position.run_if(in_state(GameState::MainMenu)))
            // .add_systems(
            //     Update,
            //     update_cursor_position.run_if(in_state(GameState::Home)))
            .add_systems(Update,update_cursor_position)
            .add_systems(Update, log_transitions::<GameState>);
    }
}



fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if window_query.is_empty() {
        cursor_pos.0 = None;
    }
    let window = window_query.single();
    if let Some(pos) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let mut pos = pos - size / 2.0;
        pos.y *= -1.0;
        cursor_pos.0 = Some(pos);
    } else {
        cursor_pos.0 = None;
    }
}

// //暂时给白子移动的图集
// fn load_assets (
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,) {
//     let layout_move = TextureAtlasLayout::from_grid(UVec2::splat(64),5,2,None,None);
//     let layout_idle = TextureAtlasLayout::from_grid(UVec2::splat(64),6,1,None,None);
//     let player = Box::new(GlobalCharacterTextureAtlas {
//         lay_out_idle: texture_atlas_layouts.add(layout_idle), 
//         image_idle: asset_server.load("Shiroko_Idle.png"),
//         lay_out_move: Some(texture_atlas_layouts.add(layout_move)),
//         image_move: Some(asset_server.load("Shiroko_Move.png")),
//     });
//     unsafe {
//         Shiroko = Some(Box::leak(player));
//         println!("ok!");
//     }
// }