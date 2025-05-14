use std::path;

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
    pub image_gun: Handle<Image>,
    pub lay_out_gun_hit: Handle<TextureAtlasLayout>,
    pub image_gun_hit: Handle<Image>,
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
        let mut layout_skill = TextureAtlasLayout::from_grid(UVec2::splat(96),12,1,None,None);
        let mut layout_gun_hit = TextureAtlasLayout::from_grid(UVec2::splat(32),6,1,None,None);
        let mut path_move = String::from("Shiroko_Move.png");
        let mut path_idle = String::from("Shiroko_Idle.png");
        let mut path_jump = String::from("Shiroko_Jump.png");
        let mut path_skill = String::from("Shiroko_Dash.png");
        let mut path_gun = String::from("Shiroko_Gun.png");
        let mut path_gun_hit = String::from("Shiroko_Hit_Effect.png");
        
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
                path_move = String::from("Arisu_Move.png");
                path_idle = String::from("Arisu_Idle.png");
                path_jump = String::from("Arisu_Jump.png"); 
                path_skill = String::from("Arisu_Shield.png");
                path_gun =  String::from("Arisu_Gun.png"); 
                path_gun_hit = String::from("Arisu_Hit_Effect.png");
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
            lay_out_gun_hit: texture_atlas_layouts.add(layout_gun_hit),
            image_gun_hit: asset_server.load(path_gun_hit),
            image_gun: asset_server.load(path_gun),
            lay_out_skill: if id != 3 { Some(texture_atlas_layouts.add(layout_skill)) } else { None },
            image_skill: if id != 3 { Some(asset_server.load(path_skill)) } else { None },
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
            .add_systems(OnEnter(GameState::Home),load_assets_enemy)
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