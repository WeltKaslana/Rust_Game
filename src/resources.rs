use std::path;

use bevy::render::texture::TRANSPARENT_IMAGE_HANDLE;
use bevy::text;
use bevy::time::Stopwatch;
use bevy::{dev_tools::states::*, prelude::*};
use bevy::window::PrimaryWindow;

use crate::boss;
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

    pub lay_out_skill: Option<Handle<TextureAtlasLayout>>,
    pub image_skill: Option<Handle<Image>>,

    pub lay_out_gun: Handle<TextureAtlasLayout>,
    pub image_gun: Handle<Image>,

    pub lay_out_gun_fire_effect: Handle<TextureAtlasLayout>,
    pub image_gun_fire_effect: Handle<Image>,

    pub lay_out_gun_hit: Handle<TextureAtlasLayout>,
    pub image_gun_hit: Handle<Image>,

    pub lay_out_bullet: Handle<TextureAtlasLayout>,
    pub image_bullet: Handle<Image>,
    // pub image_bullet_fly: Handle<Image>,

    pub id: u8,

    //以下为角色独有的素材，如技能动画等
    // shiroko
    pub image_grenade: Handle<Image>,
    pub layout_grenade_hit: Handle<TextureAtlasLayout>,
    pub image_grenade_hit: Handle<Image>,

    pub layout_drone_idle: Handle<TextureAtlasLayout>,
    pub image_drone_idle: Handle<Image>,

    pub layout_drone_fire: Handle<TextureAtlasLayout>,
    pub image_drone_fire: Handle<Image>,

    pub layout_drone_fire_effect: Handle<TextureAtlasLayout>,
    pub image_drone_fire_effect: Handle<Image>,

    pub layout_drone_missle: Handle<TextureAtlasLayout>,
    pub image_drone_missle: Handle<Image>,

    // arisu
    pub layout_gun_fire: Handle<TextureAtlasLayout>,
    pub image_gun_fire: Handle<Image>,

    pub layout_shield_back: Handle<TextureAtlasLayout>,
    pub image_shield_back: Handle<Image>,

    pub layout_gun_fire_special: Handle<TextureAtlasLayout>,
    pub image_gun_fire_special: Handle<Image>,

    pub layout_bullet_special: Handle<TextureAtlasLayout>,
    pub image_bullet_special: Handle<Image>,

    // utaha
    // 炮台和小无人机
    pub layout_attack: Handle<TextureAtlasLayout>,
    pub image_attack: Handle<Image>,

    pub image_shield: Handle<Image>,

    pub layout_MK1: Handle<TextureAtlasLayout>,
    pub image_MK1: Handle<Image>,

    pub layout_MK2_born: Handle<TextureAtlasLayout>,
    pub image_MK2_born: Handle<Image>,
    pub image_MK2_head: Handle<Image>,
    pub image_MK2_body: Handle<Image>,

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
        let mut layout_skill = TextureAtlasLayout::from_grid(UVec2::splat(96),12,1,None,None);

        let mut layout_gun = TextureAtlasLayout::from_grid(UVec2::splat(64),1,1,None,None);
        let mut layout_gun_hit = TextureAtlasLayout::from_grid(UVec2::splat(32),6,1,None,None);
        let mut layout_gun_fire_effect = TextureAtlasLayout::from_grid(UVec2::splat(32),5,1,None,None);
        let mut layout_bullet = TextureAtlasLayout::from_grid(UVec2::splat(32),1,1,None,None);

        let mut path_move = String::from("Shiroko_Move.png");
        let mut path_idle = String::from("Shiroko_Idle.png");
        let mut path_jump = String::from("Shiroko_Jump.png");
        let mut path_skill = String::from("Shiroko_Dash.png");
        let mut path_gun = String::from("Shiroko_Gun.png");
        let mut path_gun_hit = String::from("Shiroko_Hit_Effect.png");
        
        let mut path_gun_fire_effect = String::from("Shiroko_Gun_Fire_Effect.png");
        let mut path_bullet = String::from("Shiroko_Projectile.png");
        
        // shiroko
        let mut path_grenade = String::from("Shiroko_Grenade.png");
        let mut layout_grenade_hit = TextureAtlasLayout::from_grid(UVec2::splat(96),6,1,None,None);
        let mut path_grenade_hit = String::from("Shiroko_Grenade_Effect.png");

        let mut layout_drone_idle = TextureAtlasLayout::from_grid(UVec2::splat(64),7,1,None,None);
        let mut path_drone_idle = String::from("Shiroko_Drone_Idle.png");

        let mut layout_drone_fire = TextureAtlasLayout::from_grid(UVec2::splat(96),7,1,None,None);
        let mut path_drone_fire = String::from("Shiroko_Drone_Fire.png");

        let mut layout_drone_fire_effect = TextureAtlasLayout::from_grid(UVec2::splat(64),4,1,None,None);
        let mut path_drone_fire_effect = String::from("Shiroko_Drone_Fire_Effect.png");

        let mut layout_drone_missle = TextureAtlasLayout::from_grid(UVec2::splat(32),5,1,None,None);
        let mut path_drone_missle = String::from("Player_Bullet_Missile.png");

        // arisu
        let mut layout_gun_fire = TextureAtlasLayout::from_grid(UVec2::splat(64),8,1,None,None);
        let mut path_gun_fire = String::from("Arisu_Gun_Fire.png");

        let mut layout_shield_back = TextureAtlasLayout::from_grid(UVec2::splat(96),11,1,None,None);
        let mut path_shield_back = String::from("Arisu_Shield_Effect.png");

        let mut layout_gun_fire_special = TextureAtlasLayout::from_grid(UVec2::splat(96),9,3,None,None);
        let mut path_gun_fire_special = String::from("Arisu_Gun_Fire_Special.png");

        let mut layout_bullet_special = TextureAtlasLayout::from_grid(UVec2::splat(96),8,1,None,None);
        let mut path_bullet_special = String::from("Arisu_Projectile_Big.png");
        
        // utaha
        let mut layout_attack = TextureAtlasLayout::from_grid(UVec2::splat(96),7,1,None,None);
        let mut path_attack = String::from("Utaha_Weapon_Attack.png");
        
        let mut path_shield = String::from("Abnormal_Aura.png");

        let mut layout_MK1 = TextureAtlasLayout::from_grid(UVec2::splat(64),4,2,None,None);
        let mut path_MK1 = String::from("Utaha_MK1.png");

        let mut layout_MK2_born = TextureAtlasLayout::from_grid(UVec2::new(96, 256),8,1,None,None);
        let mut path_MK2_born = String::from("Utaha_MK2_Effect.png");
        let mut path_MK2_head = String::from("Utaha_MK2_Weapon.png");
        let mut path_MK2_body = String::from("Utaha_MK2.png");


        match id {
            1 => {//Shiroko
                println!("Shiroko!");        
            }
            2 => {//Arisu
                println!("Arisu!");
                layout_move = TextureAtlasLayout::from_grid(UVec2::splat(64),9,2,None,None);
                layout_idle = TextureAtlasLayout::from_grid(UVec2::splat(64),11,1,None,None);
                layout_jump = TextureAtlasLayout::from_grid(UVec2::splat(64),4,2,None,None);
                layout_skill = TextureAtlasLayout::from_grid(UVec2::splat(96),8,2,None,None);
                layout_gun_hit = TextureAtlasLayout::from_grid(UVec2::splat(128),7,1,None,None);
                
                layout_gun = TextureAtlasLayout::from_grid(UVec2::splat(64),1,1,None,None);
                layout_gun_hit = TextureAtlasLayout::from_grid(UVec2::splat(128),7,1,None,None);
                layout_gun_fire_effect = TextureAtlasLayout::from_grid(UVec2::splat(64),6,1,None,None);
                layout_bullet = TextureAtlasLayout::from_grid(UVec2::splat(64),4,1,None,None);
                
                path_move = String::from("Arisu_Move.png");
                path_idle = String::from("Arisu_Idle.png");
                path_jump = String::from("Arisu_Jump.png"); 
                path_skill = String::from("Arisu_Shield.png");
                path_gun =  String::from("Arisu_Gun.png"); 
                path_gun_hit = String::from("Arisu_Hit_Effect.png");

                path_gun_fire_effect = String::from("Arisu_Gun_Fire_Effect.png");
                path_bullet = String::from("Arisu_Projectile.png");
            }
            3 => {//Utaha
                println!("Utaha!");
                layout_move = TextureAtlasLayout::from_grid(UVec2::splat(64),9,2,None,None);
                layout_idle = TextureAtlasLayout::from_grid(UVec2::splat(64),11,1,None,None);
                layout_jump = TextureAtlasLayout::from_grid(UVec2::splat(64),4,2,None,None);
                layout_skill = TextureAtlasLayout::from_grid(UVec2::splat(64),5,3,None,None);
                path_move = String::from("Utaha_Move.png");
                path_idle = String::from("Utaha_Idle.png");
                path_jump = String::from("Utaha_Jump.png");
                path_skill = String::from("Utaha_Secondary.png");
                path_gun =  String::from("Utaha_Weapon.png"); 
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

            lay_out_gun_hit: texture_atlas_layouts.add(layout_gun_hit),
            image_gun_hit: asset_server.load(path_gun_hit),

            lay_out_gun: texture_atlas_layouts.add(layout_gun),
            image_gun: asset_server.load(path_gun),

            lay_out_gun_fire_effect: texture_atlas_layouts.add(layout_gun_fire_effect),
            image_gun_fire_effect: asset_server.load(path_gun_fire_effect),

            lay_out_bullet: texture_atlas_layouts.add(layout_bullet),
            image_bullet: asset_server.load(path_bullet),

            lay_out_skill: Some(texture_atlas_layouts.add(layout_skill)),
            // image_skill: if id != 3 { Some(asset_server.load(path_skill)) } else { None },
            image_skill: Some(asset_server.load(path_skill)),
            id: id,

            // shiroko
            image_grenade: asset_server.load(path_grenade),
            layout_grenade_hit: texture_atlas_layouts.add(layout_grenade_hit),
            image_grenade_hit: asset_server.load(path_grenade_hit),

            layout_drone_idle: texture_atlas_layouts.add(layout_drone_idle),
            image_drone_idle: asset_server.load(path_drone_idle),

            layout_drone_fire: texture_atlas_layouts.add(layout_drone_fire),
            image_drone_fire: asset_server.load(path_drone_fire),

            layout_drone_fire_effect: texture_atlas_layouts.add(layout_drone_fire_effect),
            image_drone_fire_effect: asset_server.load(path_drone_fire_effect),

            layout_drone_missle: texture_atlas_layouts.add(layout_drone_missle),
            image_drone_missle: asset_server.load(path_drone_missle),

            // arisu
            layout_gun_fire: texture_atlas_layouts.add(layout_gun_fire),
            image_gun_fire: asset_server.load(path_gun_fire),

            layout_shield_back: texture_atlas_layouts.add(layout_shield_back),
            image_shield_back: asset_server.load(path_shield_back),

            layout_gun_fire_special: texture_atlas_layouts.add(layout_gun_fire_special),
            image_gun_fire_special: asset_server.load(path_gun_fire_special),

            layout_bullet_special: texture_atlas_layouts.add(layout_bullet_special),
            image_bullet_special: asset_server.load(path_bullet_special),

            // utaha
            layout_attack: texture_atlas_layouts.add(layout_attack),
            image_attack: asset_server.load(path_attack),

            image_shield: asset_server.load(path_shield),

            layout_MK1: texture_atlas_layouts.add(layout_MK1),
            image_MK1: asset_server.load(path_MK1),

            layout_MK2_born: texture_atlas_layouts.add(layout_MK2_born),
            image_MK2_born: asset_server.load(path_MK2_born),
            image_MK2_head: asset_server.load(path_MK2_head),
            image_MK2_body: asset_server.load(path_MK2_body),
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

#[derive(Resource,Default)]
pub struct GlobalMenuTextureAtlas {
    pub close: Handle<Image>,
    pub menu: Handle<Image>,
    pub top: Handle<Image>,
    pub list: Handle<Image>,
    pub button: Handle<Image>,
    pub button_hover: Handle<Image>,
    pub button_click: Handle<Image>,
    pub bookmark: Handle<Image>,
    pub tips: Handle<Image>,
    pub title: Handle<Image>,
    pub sub_title: Handle<Image>,
    pub border: Handle<Image>,

    //图片素材
    pub shiroko: Handle<Image>,
    pub shiroko_hover: Handle<Image>,
    pub shiroko_unselect: Handle<Image>,
    pub shiroko_skill1: Handle<Image>,
    pub shiroko_skill2: Handle<Image>,
    pub shiroko_skill3: Handle<Image>,
    pub shiroko_skill4: Handle<Image>,
    pub shiroko_skill2_cool: Handle<Image>,
    pub shiroko_skill3_cool: Handle<Image>,
    pub shiroko_skill4_cool: Handle<Image>,

    pub arisu: Handle<Image>,
    pub arisu_hover: Handle<Image>,
    pub arisu_unselect: Handle<Image>,
    pub arisu_skill1: Handle<Image>,
    pub arisu_skill2: Handle<Image>,
    pub arisu_skill3: Handle<Image>,
    pub arisu_skill4: Handle<Image>,
    pub arisu_skill2_cool: Handle<Image>,
    pub arisu_skill3_cool: Handle<Image>,
    pub arisu_skill4_cool: Handle<Image>,

    pub utaha: Handle<Image>,
    pub utaha_hover: Handle<Image>,
    pub utaha_unselect: Handle<Image>,
    pub utaha_skill1: Handle<Image>,
    pub utaha_skill2: Handle<Image>,
    pub utaha_skill3: Handle<Image>,
    pub utaha_skill4: Handle<Image>,
    pub utaha_skill2_cool: Handle<Image>,
    pub utaha_skill3_cool: Handle<Image>,
    pub utaha_skill4_cool: Handle<Image>,
}

impl GlobalMenuTextureAtlas {
    pub fn init(
        asset_server: &Res<AssetServer>,
    ) -> Self {
        let path_close = String::from("BookMenu_Close.png");
        let path_menu = String::from("BookMenu.png");
        let path_top = String::from("BookMenu_Top.png");
        let path_list = String::from("BookMenu_List.png");
        let path_button = String::from("BookMenu_ButtonBig.png");
        let path_button_hover = String::from("BookMenu_ButtonBig_Hover.png");
        let path_button_click = String::from("BookMenu_ButtonBig_Click.png");
        let path_bookmark = String::from("BookMenu_Options_Small.png");
        let path_tips = String::from("BookMenu_Tips.png");
        let path_title = String::from("BookMenu_Title.png");
        let path_sub_title = String::from("BookMenu_Gray_ButtonSmall.png");
        let path_border = String::from("BookMenu_PauseBorderSmall.png");

        // 图片资源
        let path_shiroko = String::from("UI_Hub_Portrait_Shiroko.png");
        let path_shiroko_hover = String::from("UI_Hub_Portrait_Shiroko_Hover.png");
        let path_shiroko_unselect = String::from("UI_Hub_Portrait_Shiroko_Select.png");
        let path_shiroko_skill1 = String::from("Skill_Shiroko_1.png");
        let path_shiroko_skill2 = String::from("Skill_Shiroko_2.png");
        let path_shiroko_skill3 = String::from("Skill_Shiroko_3.png");
        let path_shiroko_skill4 = String::from("Skill_Shiroko_4.png");
        let path_shiroko_skill2_cool = String::from("Skill_Shiroko_2_cool.png");
        let path_shiroko_skill3_cool = String::from("Skill_Shiroko_3_cool.png");
        let path_shiroko_skill4_cool = String::from("Skill_Shiroko_4_cool.png");

        let path_arisu = String::from("UI_Hub_Portrait_Arisu.png");
        let path_arisu_hover = String::from("UI_Hub_Portrait_Arisu_Hover.png");
        let path_arisu_unselect = String::from("UI_Hub_Portrait_Arisu_Select.png");
        let path_arisu_skill1 = String::from("Skill_Arisu_1.png");
        let path_arisu_skill2 = String::from("Skill_Arisu_2.png");
        let path_arisu_skill3 = String::from("Skill_Arisu_3.png");
        let path_arisu_skill4 = String::from("Skill_Arisu_4.png");
        let path_arisu_skill2_cool = String::from("Skill_Arisu_2_cool.png");
        let path_arisu_skill3_cool = String::from("Skill_Arisu_3_cool.png");
        let path_arisu_skill4_cool = String::from("Skill_Arisu_4_cool.png");

        let path_utaha = String::from("UI_Hub_Portrait_Utaha.png");
        let path_utaha_hover = String::from("UI_Hub_Portrait_Utaha_Hover.png");
        let path_utaha_unselect = String::from("UI_Hub_Portrait_Utaha_Select.png");
        let path_utaha_skill1 = String::from("Skill_Utaha_1.png");
        let path_utaha_skill2 = String::from("Skill_Utaha_2.png");
        let path_utaha_skill3 = String::from("Skill_Utaha_3.png");
        let path_utaha_skill4 = String::from("Skill_Utaha_4.png");
        let path_utaha_skill2_cool = String::from("Skill_Utaha_2_cool.png");
        let path_utaha_skill3_cool = String::from("Skill_Utaha_3_cool.png");
        let path_utaha_skill4_cool = String::from("Skill_Utaha_4_cool.png"); 

        Self {
            close: asset_server.load(path_close),
            menu: asset_server.load(path_menu),
            top: asset_server.load(path_top),
            list: asset_server.load(path_list),
            button: asset_server.load(path_button),
            button_hover: asset_server.load(path_button_hover),
            button_click: asset_server.load(path_button_click),
            bookmark: asset_server.load(path_bookmark),
            tips: asset_server.load(path_tips),
            title: asset_server.load(path_title),
            sub_title: asset_server.load(path_sub_title),
            border: asset_server.load(path_border),

            // 图片
            shiroko: asset_server.load(path_shiroko),
            shiroko_hover: asset_server.load(path_shiroko_hover),
            shiroko_unselect: asset_server.load(path_shiroko_unselect),
            shiroko_skill1: asset_server.load(path_shiroko_skill1),
            shiroko_skill2: asset_server.load(path_shiroko_skill2),
            shiroko_skill3: asset_server.load(path_shiroko_skill3),
            shiroko_skill4: asset_server.load(path_shiroko_skill4),
            shiroko_skill2_cool: asset_server.load(path_shiroko_skill2_cool),
            shiroko_skill3_cool: asset_server.load(path_shiroko_skill3_cool),
            shiroko_skill4_cool: asset_server.load(path_shiroko_skill4_cool),

            arisu: asset_server.load(path_arisu),
            arisu_hover: asset_server.load(path_arisu_hover),
            arisu_unselect: asset_server.load(path_arisu_unselect),
            arisu_skill1: asset_server.load(path_arisu_skill1),
            arisu_skill2: asset_server.load(path_arisu_skill2),
            arisu_skill3: asset_server.load(path_arisu_skill3),
            arisu_skill4: asset_server.load(path_arisu_skill4),
            arisu_skill2_cool: asset_server.load(path_arisu_skill2_cool),
            arisu_skill3_cool: asset_server.load(path_arisu_skill3_cool),
            arisu_skill4_cool: asset_server.load(path_arisu_skill4_cool),

            utaha: asset_server.load(path_utaha),
            utaha_hover: asset_server.load(path_utaha_hover),
            utaha_unselect: asset_server.load(path_utaha_unselect),
            utaha_skill1: asset_server.load(path_utaha_skill1),
            utaha_skill2: asset_server.load(path_utaha_skill2),
            utaha_skill3: asset_server.load(path_utaha_skill3),
            utaha_skill4: asset_server.load(path_utaha_skill4),
            utaha_skill2_cool: asset_server.load(path_utaha_skill2_cool),
            utaha_skill3_cool: asset_server.load(path_utaha_skill3_cool),
            utaha_skill4_cool: asset_server.load(path_utaha_skill4_cool),
        }
    }
}
#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CursorPosition(None))
            .add_systems(Startup, (
                load_assets,
                load_menu,
            ))
            .add_systems(Update,update_cursor_position)
            .add_systems(OnEnter(GameState::Home),(
                load_assets_enemy,
                load_assets_room,
                init_score,
            ))
            // .add_systems(Update, log_transitions::<GameState>)
            ;
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
    let id = 1;
    let gcta = GlobalCharacterTextureAtlas::init(id, &asset_server, &mut texture_atlas_layouts);
    commands.insert_resource(gcta);

    let ghta = GlobalHomeTextureAtlas::init(&asset_server, &mut texture_atlas_layouts);
    commands.insert_resource(ghta);
    println!("over!");
}
fn load_menu (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ghta = GlobalMenuTextureAtlas::init(&asset_server);
    commands.insert_resource(ghta);
    info!("Menu Resourse Loaded");
}
fn load_assets_enemy (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let enemy_resources = GlobalEnemyTextureAtlas::init(&asset_server, &mut texture_atlas_layouts);
    commands.insert_resource(enemy_resources);

    let boss_resources = GlobalBossTextureAtlas::init(&asset_server, &mut texture_atlas_layouts);
    commands.insert_resource(boss_resources);

    println!("Enemy Resourse Loaded");
}

fn load_assets_room (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let room_resources = GlobalRoomTextureAtlas::init(&asset_server, &mut texture_atlas_layouts);
    commands.insert_resource(room_resources);

    println!("Room Resourse Loaded");
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
    pub layout_unknown_bullet: Handle<TextureAtlasLayout>,
    pub image_unknown_bullet: Handle<Image>,

    pub layout_death: Handle<TextureAtlasLayout>,
    pub image_death: Handle<Image>,

    pub layout_born: Handle<TextureAtlasLayout>,
    pub image_bron: Handle<Image>,

    pub layout_gun_hit: Handle<TextureAtlasLayout>,
    pub image_gun_hit: Handle<Image>,
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
        let imageunknownattack = String::from("UnknownGuardian_TypeF_Attack1.png");
        let layoutunknownbullet = TextureAtlasLayout::from_grid(UVec2::splat(32),4,1,None,None);
        let imageunknownbullet = String::from("Entity_Bullet_UnKnownGuardian.png");

        let layoutdeath = TextureAtlasLayout::from_grid(UVec2::splat(96), 7, 1, None,None);
        let imagedeath = String::from("Entity_Defeated.png");

        let layoutborn = TextureAtlasLayout::from_grid(UVec2::splat(48), 12, 1, None,None);
        let imagebeon = String::from("Entity_Spawn.png");

        let layoutgunhit = TextureAtlasLayout::from_grid(UVec2::splat(32),6,1,None,None);
        let imagegunhit = String::from("Shiroko_Hit_Effect.png");


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
            layout_unknown_bullet: texture_atlas_layouts.add(layoutunknownbullet),
            image_unknown_bullet: asset_server.load(imageunknownbullet),

            layout_death: texture_atlas_layouts.add(layoutdeath),
            image_death: asset_server.load(imagedeath),

            layout_born: texture_atlas_layouts.add(layoutborn),
            image_bron: asset_server.load(imagebeon),

            layout_gun_hit: texture_atlas_layouts.add(layoutgunhit),
            image_gun_hit: asset_server.load(imagegunhit),
        }
    }
}

#[derive(Resource,Default)]
pub struct GlobalBossTextureAtlas {
    pub layout_boss_idle: Handle<TextureAtlasLayout>,
    pub image_boss_idle: Handle<Image>,//Boss1_Idea
    pub layout_boss_move: Handle<TextureAtlasLayout>,
    pub image_boss_move: Handle<Image>,//Boss1_Move
    pub layout_boss_collide_start: Handle<TextureAtlasLayout>,
    pub image_boss_collide_start: Handle<Image>,//Boss1_Collide_Start2
    pub layout_boss_collide_loop: Handle<TextureAtlasLayout>,
    pub image_boss_collide_loop: Handle<Image>,//Boss1_Collide_Loop
    pub layout_boss_collide_end: Handle<TextureAtlasLayout>,
    pub image_boss_collide_end: Handle<Image>,//Boss1_Collide_End
    pub layout_boss_collide_effect: Handle<TextureAtlasLayout>,
    pub image_boss_collide_effect: Handle<Image>,//Boss1_Collide_Loop_Effect

    pub layout_weaponmissile_idle: Handle<TextureAtlasLayout>,
    pub image_weaponmissile_idle: Handle<Image>,//Boss1_Lid_GuidedMissile_Idea
    pub layout_weaponmissile_fire: Handle<TextureAtlasLayout>,
    pub image_weaponmissile_fire: Handle<Image>,//Boss1_Lid_GuidedMissile_Fire

    pub layout_weaponlid_idle: Handle<TextureAtlasLayout>,
    pub image_weaponlid_idle: Handle<Image>,//Boss1_WeaponLid
    pub layout_weaponlid_fire: Handle<TextureAtlasLayout>,
    pub image_weaponlid_fire: Handle<Image>,//Boss1_WeaponLid_Fire

    pub layout_weapongun_idle: Handle<TextureAtlasLayout>,
    pub image_weapongun_idle: Handle<Image>,//Boss1_Weapon1_Idea
    pub layout_weapongun_fire: Handle<TextureAtlasLayout>,
    pub image_weapongun_fire: Handle<Image>,//Boss1_Weapon1_Fire

    pub layout_death: Handle<TextureAtlasLayout>,
    pub image_death: Handle<Image>,
}

impl GlobalBossTextureAtlas {
    pub fn init(
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let layoutbossidle = TextureAtlasLayout::from_grid(UVec2::splat(96),4,1,None,None);
        let imagebossidle = String::from("Boss1_Idea.png");
        let layoutbossmove = TextureAtlasLayout::from_grid(UVec2::splat(160), 8, 1, None, None);
        let imagebossmove = String::from("Boss1_Move.png");
        let layoutbosscollidestart = TextureAtlasLayout::from_grid(UVec2::splat(160), 6, 2, None, None);
        let imagebosscollidestart = String::from("Boss1_Collide_Start2.png");//10
        let layoutbosscollideloop = TextureAtlasLayout::from_grid(UVec2::splat(160), 6, 2, None, None);
        let imagebosscollideloop = String::from("Boss1_Collide_Loop.png");//8
        let layoutbosscollideend = TextureAtlasLayout::from_grid(UVec2::splat(160), 2, 1, None, None);
        let imagebosscollideend = String::from("Boss1_Collide_End.png");
        let layoutbosscollideeffect = TextureAtlasLayout::from_grid(UVec2::splat(160), 7, 1, None, None);
        let imagebosscollideeffect = String::from("Boss1_Collide_Loop_Effect.png");

        let layoutweaponmissileidle = TextureAtlasLayout::from_grid(UVec2::splat(128), 1, 1, None, None);
        let imageweaponmissileidle = String::from("Boss1_Lid_GuidedMissile_Idea.png");
        let layoutweaponmissilefire = TextureAtlasLayout::from_grid(UVec2::splat(128), 30, 1, None, None);
        let imageweaponmissilefire = String::from("Boss1_Lid_GuidedMissile_Fire.png");

        let layoutweaponlididle = TextureAtlasLayout::from_grid(UVec2::splat(96), 1, 1, None, None);
        let imageweaponlididle = String::from("Boss1_WeaponLid.png");
        let layoutweaponlidfire = TextureAtlasLayout::from_grid(UVec2::splat(128), 7, 1, None, None);
        let imageweaponlidfire = String::from("Boss1_WeaponLid_Fire.png");

        let layoutweapongunidle = TextureAtlasLayout::from_grid(UVec2::splat(64), 1, 1, None, None);
        let imageweapongunidle = String::from("Boss1_Weapon1_Idea.png");
        let layoutweapongunfire = TextureAtlasLayout::from_grid(UVec2::splat(96), 7, 1, None, None);
        let imageweapongunfire = String::from("Boss1_Weapon1_Fire.png");

        let layoutdeath = TextureAtlasLayout::from_grid(UVec2::splat(128), 8, 1, None, None);
        let imagedeath = String::from("Entity_Boss_Defeated.png");

        Self {
            layout_boss_idle: texture_atlas_layouts.add(layoutbossidle),
            image_boss_idle: asset_server.load(imagebossidle),
            layout_boss_move: texture_atlas_layouts.add(layoutbossmove),
            image_boss_move: asset_server.load(imagebossmove),
            layout_boss_collide_start: texture_atlas_layouts.add(layoutbosscollidestart),
            image_boss_collide_start: asset_server.load(imagebosscollidestart),
            layout_boss_collide_loop: texture_atlas_layouts.add(layoutbosscollideloop),
            image_boss_collide_loop: asset_server.load(imagebosscollideloop),
            layout_boss_collide_end: texture_atlas_layouts.add(layoutbosscollideend),
            image_boss_collide_end: asset_server.load(imagebosscollideend),
            layout_boss_collide_effect: texture_atlas_layouts.add(layoutbosscollideeffect),
            image_boss_collide_effect: asset_server.load(imagebosscollideeffect),

            layout_weaponmissile_idle: texture_atlas_layouts.add(layoutweaponmissileidle),
            image_weaponmissile_idle: asset_server.load(imageweaponmissileidle),
            layout_weaponmissile_fire: texture_atlas_layouts.add(layoutweaponmissilefire),
            image_weaponmissile_fire: asset_server.load(imageweaponmissilefire),

            layout_weaponlid_idle: texture_atlas_layouts.add(layoutweaponlididle),
            image_weaponlid_idle: asset_server.load(imageweaponlididle),
            layout_weaponlid_fire: texture_atlas_layouts.add(layoutweaponlidfire),
            image_weaponlid_fire: asset_server.load(imageweaponlidfire),

            layout_weapongun_idle: texture_atlas_layouts.add(layoutweapongunidle),
            image_weapongun_idle: asset_server.load(imageweapongunidle),
            layout_weapongun_fire: texture_atlas_layouts.add(layoutweapongunfire),
            image_weapongun_fire: asset_server.load(imageweapongunfire),

            layout_death: texture_atlas_layouts.add(layoutdeath),
            image_death: asset_server.load(imagedeath),
        }
    }


}

#[derive(Resource,Default)]
pub struct GlobalRoomTextureAtlas { 
    pub layout_chest_big1: Handle<TextureAtlasLayout>,
    pub image_chest_big1: Handle<Image>,

    pub layout_chest_big2: Handle<TextureAtlasLayout>,
    pub image_chest_big2: Handle<Image>,

    pub layout_chest_small: Handle<TextureAtlasLayout>,
    pub image_chest_small: Handle<Image>,

    pub layout_door_open: Handle<TextureAtlasLayout>,
    pub image_door_open: Handle<Image>,

    pub layout_door_close: Handle<TextureAtlasLayout>,
    pub image_door_close: Handle<Image>,

    pub layout_chest_big2_effect1: Handle<TextureAtlasLayout>,
    pub image_chest_big2_effect1: Handle<Image>,

    pub layout_chest_big2_effect2: Handle<TextureAtlasLayout>,
    pub image_chest_big2_effect2: Handle<Image>,
}

impl GlobalRoomTextureAtlas {
    pub fn init(
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self { 
        let layoutchestbig1 = TextureAtlasLayout::from_grid(UVec2::splat(96), 10, 4, None, None);
        let imagechestbig1 = String::from("Chest_Big1_Open.png");

        let layoutchestbig2 = TextureAtlasLayout::from_grid(UVec2::splat(96), 10, 4, None, None);
        let imagechestbig2 = String::from("Chest_Big2_Open.png");

        let layoutchestsmall = TextureAtlasLayout::from_grid(UVec2::splat(64), 24, 1, None, None);
        let imagechestsmall = String::from("Chest_Small_Open.png");

        let layoutdooropen = TextureAtlasLayout::from_grid(UVec2::splat(96), 8, 2, None, None);
        let imagedooropen = String::from("Boss_Door_Open.png");

        let layoutdoorclose = TextureAtlasLayout::from_grid(UVec2::splat(96), 8, 2, None, None);
        let imagedoorclose = String::from("Boss_Door_Close.png");

        let layoutchestbig2effect1 = TextureAtlasLayout::from_grid(UVec2::splat(32), 3, 1, None, None);
        let imagechestbig2effect1 = String::from("Chest_Big2_Effect1.png");

        let layoutchestbig2effect2 = TextureAtlasLayout::from_grid(UVec2::splat(32), 3, 1, None, None);
        let imagechestbig2effect2 = String::from("Chest_Big2_Effect2.png");

        Self {
            layout_chest_big1: texture_atlas_layouts.add(layoutchestbig1),
            image_chest_big1: asset_server.load(imagechestbig1),

            layout_chest_big2: texture_atlas_layouts.add(layoutchestbig2),
            image_chest_big2: asset_server.load(imagechestbig2),

            layout_chest_small: texture_atlas_layouts.add(layoutchestsmall),
            image_chest_small: asset_server.load(imagechestsmall),

            layout_door_open: texture_atlas_layouts.add(layoutdooropen),
            image_door_open: asset_server.load(imagedooropen),

            layout_door_close: texture_atlas_layouts.add(layoutdoorclose),
            image_door_close: asset_server.load(imagedoorclose),

            layout_chest_big2_effect1: texture_atlas_layouts.add(layoutchestbig2effect1),
            image_chest_big2_effect1: asset_server.load(imagechestbig2effect1),

            layout_chest_big2_effect2: texture_atlas_layouts.add(layoutchestbig2effect2),
            image_chest_big2_effect2: asset_server.load(imagechestbig2effect2),
        }
    }
}

#[derive(Resource,Default)]
pub struct ScoreResource { 
    pub enemy_score: u8,
    pub boss_score: u8,
    pub map_index: u8,
    pub time_min:  u8,
    pub time_sec:  u8,
    pub timer: Stopwatch,
}

impl ScoreResource {
    pub fn init() -> Self { 
        ScoreResource {
            enemy_score: 0,
            boss_score: 0,
            map_index: 0,
            time_min: 0,
            time_sec: 0,
            timer: Stopwatch::new(),
        }
    }
}

fn init_score(
    mut commands: Commands,
) {
    let score = ScoreResource::init();
    commands.insert_resource(score);
    println!("init score");
}