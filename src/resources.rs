use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::gamestate::GameState;
use crate::*;

pub struct ResourcesPlugin;

#[derive(Resource,Default)]
pub struct GlobalTextureAtlas {
    pub lay_out: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource,Default)]
pub struct GlobalCharacterTextureAtlas {
    pub lay_out_idle: Option<Handle<TextureAtlasLayout>>,
    pub image_idle: Option<Handle<Image>>,
    pub lay_out_move: Option<Handle<TextureAtlasLayout>>,
    pub image_move: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextureAtlas::default())
            .insert_resource(CursorPosition(None))
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                update_cursor_position.run_if(in_state(GameState::InGame)));
    }
}

//存疑，暂时给白子移动的图集
fn load_assets(
    mut handle: ResMut<GlobalCharacterTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    //?精灵图片途径可能不固定
    handle.image_move = Some(asset_server.load("assets/Shiroko_Move.png"));
    
    handle.image_idle = Some(asset_server.load("assets/Shiroko_Idle.png"));
    
    //
    let layout_move = TextureAtlasLayout::from_grid(UVec2::splat(64),5,2,None,None);
    handle.lay_out_move = Some(texture_atlas_layouts.add(layout_move));
    let layout_idle = TextureAtlasLayout::from_grid(UVec2::splat(64),6,1,None,None);
    handle.lay_out_idle = Some(texture_atlas_layouts.add(layout_idle));
    next_state.set(GameState::Home);
}

//存疑
fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // camera_query: Query<&Camera, With<Camera>>,
) {
    // if window_query.is_empty() || camera_query.is_empty() {
    //     cursor_pos.0 = None;
    // }
    if window_query.is_empty() {
        cursor_pos.0 = None;
    }
    // let camera = camera_query.single();
    let window = window_query.single();
    if let Some(pos) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let pos = pos - size / 2.0;
        cursor_pos.0 = Some(pos);
    } else {
        cursor_pos.0 = None;
    }
}