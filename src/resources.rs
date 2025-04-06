use bevy::{dev_tools::states::*, prelude::*};
use bevy::window::PrimaryWindow;

use crate::gamestate::GameState;

pub struct ResourcesPlugin;



#[derive(Resource,Default)]
pub struct GlobalCharacterTextureAtlas {
    pub lay_out_idle: Handle<TextureAtlasLayout>,
    pub image_idle: Handle<Image>,
    pub lay_out_move: Handle<TextureAtlasLayout>,
    pub image_move: Handle<Image>,
    pub lay_out_jump: Handle<TextureAtlasLayout>,
    pub image_jump: Handle<Image>,
    pub image_gun: Handle<Image>,
    pub id: u8,
}

impl GlobalCharacterTextureAtlas {
    pub fn init(
        id: u8,
        asset_server: &Res<AssetServer>,
        mut texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let mut layout_move = TextureAtlasLayout::from_grid(UVec2::splat(64),5,2,None,None);
        let mut layout_idle = TextureAtlasLayout::from_grid(UVec2::splat(64),6,1,None,None);
        let mut layout_jump = TextureAtlasLayout::from_grid(UVec2::splat(64),4,2,None,None);
        let mut path_move = String::from("Shiroko_Move.png");
        let mut path_idle = String::from("Shiroko_Idle.png");
        let mut path_jump = String::from("Shiroko_Jump.png");
        let mut path_gun = String::from("Shiroko_Gun.png");
        match id {
            1 => {//Shiroko
                println!("Shiroko!");
                // layout_move = TextureAtlasLayout::from_grid(UVec2::splat(64),5,2,None,None);
                // layout_idle = TextureAtlasLayout::from_grid(UVec2::splat(64),6,1,None,None);
                // layout_jump = TextureAtlasLayout::from_grid(UVec2::splat(64),4,2,None,None);
                // path_move = String::from("Shiroko_move.png");
                // path_idle = String::from("Shiroko_idle.png");
                // path_jump = String::from("Shiroko_jump.png");           
            }
            2 => {//Arisu
                println!("Arisu!");
                layout_move = TextureAtlasLayout::from_grid(UVec2::splat(64),9,2,None,None);
                layout_idle = TextureAtlasLayout::from_grid(UVec2::splat(64),11,1,None,None);
                layout_jump = TextureAtlasLayout::from_grid(UVec2::splat(64),4,2,None,None);
                path_move = String::from("Arisu_Move.png");
                path_idle = String::from("Arisu_Idle.png");
                path_jump = String::from("Arisu_Jump.png"); 
                path_gun =  String::from("Arisu_Gun.png"); 
            }
            3 => {//Utaha
                println!("Utaha!");
                layout_move = TextureAtlasLayout::from_grid(UVec2::splat(64),9,2,None,None);
                layout_idle = TextureAtlasLayout::from_grid(UVec2::splat(64),11,1,None,None);
                layout_jump = TextureAtlasLayout::from_grid(UVec2::splat(64),4,2,None,None);
                path_move = String::from("Utaha_Move.png");
                path_idle = String::from("Utaha_Idle.png");
                path_jump = String::from("Utaha_Jump.png");
            }
            _ => {},
        }
        Self {
            lay_out_idle: texture_atlas_layouts.add(layout_idle),
            image_idle: asset_server.load(path_idle),
            lay_out_move: texture_atlas_layouts.add(layout_move),
            image_move: asset_server.load(path_move),
            lay_out_jump: texture_atlas_layouts.add(layout_jump),
            image_jump: asset_server.load(path_jump),
            image_gun: asset_server.load(path_gun),
            id: id,
        }
    }
}

#[derive(Resource,Default)]
pub struct GlobalHomeTextureAtlas {
    //小空
    pub Sora_lay_out_loop: Handle<TextureAtlasLayout>,
    pub Sora_image_loop: Handle<Image>,
    pub Sora_lay_out_awake: Handle<TextureAtlasLayout>,
    pub Sora_image_awake: Handle<Image>,
    pub Sora_lay_out_asleep: Handle<TextureAtlasLayout>,
    pub Sora_image_asleep: Handle<Image>,
    //冰箱
    pub Fridge_lay_out_loop: Handle<TextureAtlasLayout>,
    pub Fridge_image_loop: Handle<Image>,
    pub Fridge_lay_out_oc: Handle<TextureAtlasLayout>,
    pub Fridge_image_oc: Handle<Image>,
}
impl GlobalHomeTextureAtlas {
    pub fn init(
        asset_server: &Res<AssetServer>,
        mut texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let mut Sora_lay_out_loop = TextureAtlasLayout::from_grid(UVec2::splat(80),8,1,None,None);
        let mut Sora_lay_out_awake = TextureAtlasLayout::from_grid(UVec2::splat(80),14,1,None,None);
        let mut Sora_lay_out_asleep = TextureAtlasLayout::from_grid(UVec2::splat(80),18,1,None,None);
        let mut Sora_path_loop = String::from("Sora_RestLoop.png");
        let mut Sora_path_awake = String::from("Sora_RestEnd.png");
        let mut Sora_path_asleep = String::from("Sora_Rest.png");

        let mut Fridge_lay_out_loop = TextureAtlasLayout::from_grid(UVec2::splat(96),10,3,None,None);
        let mut Fridge_lay_out_oc = TextureAtlasLayout::from_grid(UVec2::splat(96),10,2,None,None);
        let mut Fridge_path_loop = String::from("Teleporter_2_Start.png");
        let mut Fridge_path_oc = String::from("Teleporter_2_Open.png");
        
        Self {
            Sora_lay_out_loop: texture_atlas_layouts.add(Sora_lay_out_loop),
            Sora_image_loop: asset_server.load(Sora_path_loop),
            Sora_lay_out_awake: texture_atlas_layouts.add(Sora_lay_out_awake),
            Sora_image_awake: asset_server.load(Sora_path_awake),
            Sora_lay_out_asleep: texture_atlas_layouts.add(Sora_lay_out_asleep),
            Sora_image_asleep: asset_server.load(Sora_path_asleep),
            Fridge_lay_out_loop: texture_atlas_layouts.add(Fridge_lay_out_loop),
            Fridge_image_loop: asset_server.load(Fridge_path_loop),
            Fridge_lay_out_oc: texture_atlas_layouts.add(Fridge_lay_out_oc),
            Fridge_image_oc: asset_server.load(Fridge_path_oc),
        }
    }
}


#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CursorPosition(None))

            .add_systems(Startup, load_assets)
            .add_systems(Update,update_cursor_position)
            .add_systems(OnEnter(GameState::Home),load_assets_enemy)
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

//暂时给白子移动的图集
fn load_assets (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    //根据id选择角色
    let id = 2;
    let gcta = GlobalCharacterTextureAtlas::init(id, &asset_server, &mut texture_atlas_layouts);
    commands.insert_resource(gcta);

    let ghta = GlobalHomeTextureAtlas::init(&asset_server, &mut texture_atlas_layouts);
    commands.insert_resource(ghta);
    println!("over!");
}

fn load_assets_enemy (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let enemy_resources = GlobalEnemyTextureAtlas::init(&asset_server, &mut texture_atlas_layouts);
    commands.insert_resource(enemy_resources);

    println!("Enemy Resourse Loaded");
}


#[derive(Resource,Default)]
pub struct GlobalEnemyTextureAtlas {
    pub layout_sweeper_idle: Handle<TextureAtlasLayout>,
    pub image_sweeper_idle: Handle<Image>,
    pub layout_sweeper_move: Handle<TextureAtlasLayout>,
    pub image_sweeper_move: Handle<Image>,
    pub layout_sweeper_attack: Handle<TextureAtlasLayout>,
    pub image_sweeper_attack: Handle<Image>,

    pub layout_vulcan_idle: Handle<TextureAtlasLayout>,
    pub image_vulcan_idle: Handle<Image>,
    pub layout_vulcan_fire_start: Handle<TextureAtlasLayout>,
    pub image_vulcan_fire_start: Handle<Image>,
    pub layout_vulcan_fire_loop: Handle<TextureAtlasLayout>,
    pub image_vulcan_fire_loop: Handle<Image>,
    pub layout_vulcan_fire_end: Handle<TextureAtlasLayout>,
    pub image_vulcan_fire_end: Handle<Image>,
    pub layout_vulcan_bullet: Handle<TextureAtlasLayout>,
    pub image_vulcan_bullet: Handle<Image>,

    pub layout_missile_idle: Handle<TextureAtlasLayout>,
    pub image_missile_idle: Handle<Image>,
    pub layout_missile_fire_start: Handle<TextureAtlasLayout>,
    pub image_missile_fire_start: Handle<Image>,
    pub layout_missile_fire_loop: Handle<TextureAtlasLayout>,
    pub image_missile_fire_loop: Handle<Image>,
    pub layout_missile_fire_end: Handle<TextureAtlasLayout>,
    pub image_missile_fire_end: Handle<Image>,
    pub layout_missile_bullet: Handle<TextureAtlasLayout>,
    pub image_missile_bullet: Handle<Image>,

    pub layout_unknown_idle: Handle<TextureAtlasLayout>,
    pub image_unknown_idle: Handle<Image>,
    pub layout_unknown_move: Handle<TextureAtlasLayout>,
    pub image_unknown_move: Handle<Image>,
    pub layout_unknown_attack: Handle<TextureAtlasLayout>,
    pub image_unknown_attack: Handle<Image>,
    pub layout_unknown_fireeffect: Handle<TextureAtlasLayout>,
    pub image_unknown_fireeffect: Handle<Image>,
    pub layout_unknown_bullet: Handle<TextureAtlasLayout>,
    pub image_unknown_bullet: Handle<Image>,
}

impl GlobalEnemyTextureAtlas {
    pub fn init(
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let layoutsweeperidle = TextureAtlasLayout::from_grid(UVec2::splat(64),1,1,None,None);
        let imagesweeperidle = String::from("Sweeper_Idea.png");
        let layoutsweepermove =  TextureAtlasLayout::from_grid(UVec2::splat(64),14,1,None,None);
        let imagesweepermove = String::from("Sweeper_Move.png");
        let layoutsweeperattack = TextureAtlasLayout::from_grid(UVec2::splat(128),13,1,None,None);
        let imagesweeperattack = String::from("Sweeper_Attack.png");

        let layoutvulcanidle = TextureAtlasLayout::from_grid(UVec2::splat(64),5,1,None,None);
        let imagevulcanidle = String::from("DroneVulcan_Idea.png");
        let layoutvulcanstartfire = TextureAtlasLayout::from_grid(UVec2::splat(64),3,1,None,None);
        let imagevulcanstartfire = String::from("DroneVulcan_Fire_Start.png");
        let layoutvulcanfireloop =  TextureAtlasLayout::from_grid(UVec2::splat(64),3,1,None,None);
        let imagevulcanfireloop = String::from("DroneVulcan_Fire.png");
        let layoutvulcanfireend =  TextureAtlasLayout::from_grid(UVec2::splat(64),2,1,None,None);
        let imagevulcanfireend = String::from("DroneVulcan_Fire_End.png");
        let layoutvulcanbullet = TextureAtlasLayout::from_grid(UVec2::splat(32),4,1,None,None);
        let imagevulcanbullet = String::from("Entity_Bullet_Normal.png");

        let layoutmissileidle = TextureAtlasLayout::from_grid(UVec2::splat(64),5,1,None,None);
        let imagemissileidle = String::from("DroneMissile_Idea.png");
        let layoutmissilestartfire = TextureAtlasLayout::from_grid(UVec2::splat(64),3,1,None,None);
        let imagemissilestartfire = String::from("DroneMissile_Fire_Start.png");
        let layoutmissilefireloop =  TextureAtlasLayout::from_grid(UVec2::splat(64),5,1,None,None);
        let imagemissilefireloop = String::from("DroneMissile_Fire.png");
        let layoutmissilefireend =  TextureAtlasLayout::from_grid(UVec2::splat(64),2,1,None,None);
        let imagemissilefireend = String::from("DroneMissile_Fire_End.png");
        let layoutmissilebullet = TextureAtlasLayout::from_grid(UVec2::splat(32),5,1,None,None);
        let imagemissilebullet = String::from("Entity_Bullet_Missile.png");

        let layoutunknownidle = TextureAtlasLayout::from_grid(UVec2::splat(64),1,1,None,None);
        let imageunknownidle = String::from("UnknownGuardian_TypeF_Idle.png");
        let layoutunknownmove =  TextureAtlasLayout::from_grid(UVec2::splat(64),9,1,None,None);
        let imageunknownmove = String::from("UnknownGuardian_TypeF_Move.png");
        let layoutunknownattack = TextureAtlasLayout::from_grid(UVec2::splat(64),8,1,None,None);
        let imageunknownattack = String::from("UnknownGuardian_TypeF_Attack.png");
        let layoutunknownfireeffect = TextureAtlasLayout::from_grid(UVec2::splat(96),7,1,None,None);
        let imageunknownfireeffect = String::from("UnknownGuardian_TypeF_Fire_Effect.png");
        let layoutunknownbullet = TextureAtlasLayout::from_grid(UVec2::splat(32),4,1,None,None);
        let imageunknownbullet = String::from("Entity_Bullet_UnKnownGuardian.png");

        Self {
            layout_sweeper_idle: texture_atlas_layouts.add(layoutsweeperidle),
            image_sweeper_idle: asset_server.load(imagesweeperidle),
            layout_sweeper_move: texture_atlas_layouts.add(layoutsweepermove),
            image_sweeper_move: asset_server.load(imagesweepermove),
            layout_sweeper_attack: texture_atlas_layouts.add(layoutsweeperattack),
            image_sweeper_attack: asset_server.load(imagesweeperattack),

            layout_vulcan_idle: texture_atlas_layouts.add(layoutvulcanidle),
            image_vulcan_idle: asset_server.load(imagevulcanidle),
            layout_vulcan_fire_start: texture_atlas_layouts.add(layoutvulcanstartfire),
            image_vulcan_fire_start: asset_server.load(imagevulcanstartfire),
            layout_vulcan_fire_loop: texture_atlas_layouts.add(layoutvulcanfireloop),
            image_vulcan_fire_loop: asset_server.load(imagevulcanfireloop),
            layout_vulcan_fire_end: texture_atlas_layouts.add(layoutvulcanfireend),
            image_vulcan_fire_end: asset_server.load(imagevulcanfireend),
            layout_vulcan_bullet: texture_atlas_layouts.add(layoutvulcanbullet),
            image_vulcan_bullet: asset_server.load(imagevulcanbullet),

            layout_missile_idle: texture_atlas_layouts.add(layoutmissileidle),
            image_missile_idle: asset_server.load(imagemissileidle),
            layout_missile_fire_start: texture_atlas_layouts.add(layoutmissilestartfire),
            image_missile_fire_start: asset_server.load(imagemissilestartfire),
            layout_missile_fire_loop: texture_atlas_layouts.add(layoutmissilefireloop),
            image_missile_fire_loop: asset_server.load(imagemissilefireloop),
            layout_missile_fire_end: texture_atlas_layouts.add(layoutmissilefireend),
            image_missile_fire_end: asset_server.load(imagemissilefireend),
            layout_missile_bullet: texture_atlas_layouts.add(layoutmissilebullet),
            image_missile_bullet: asset_server.load(imagemissilebullet),

            layout_unknown_idle: texture_atlas_layouts.add(layoutunknownidle),
            image_unknown_idle: asset_server.load(imageunknownidle),
            layout_unknown_move: texture_atlas_layouts.add(layoutunknownmove),
            image_unknown_move: asset_server.load(imageunknownmove),
            layout_unknown_attack: texture_atlas_layouts.add(layoutunknownattack),
            image_unknown_attack: asset_server.load(imageunknownattack),
            layout_unknown_fireeffect: texture_atlas_layouts.add(layoutunknownfireeffect),
            image_unknown_fireeffect: asset_server.load(imageunknownfireeffect),
            layout_unknown_bullet: texture_atlas_layouts.add(layoutunknownbullet),
            image_unknown_bullet:asset_server.load(imageunknownbullet),
        }
    }
}