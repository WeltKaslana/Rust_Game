use bevy::{math::vec3, prelude::*, dev_tools::states::*};
use crate::{gamestate::GameState,
            character::Character,
            gun::PlayerFireEvent, };

pub  struct FollowCameraPlugin;


impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerFireEvent>()
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::Home)),
            )
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::InGame)),
            )
            .add_systems(Update, log_transitions::<GameState>);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn camera_follow_player(
    mut events: EventReader<PlayerFireEvent>,
    player_query: Query<&Transform, With<Character>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Character>)>,
) {
    if camera_query.is_empty() || player_query.is_empty() {
        return;
    }
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single().translation;
    let (x, mut y) = (player_transform.x, player_transform.y);
    //摄像机限位
    let mut x = match x {
        x if x < -50.0 => -50.0,
        x if x > 50.0 => 50.0,
        _ => x,
    };
    //镜头随开火抖动
    for _ in events.read() {
        x -= 30.0;
    }
    camera_transform.translation = camera_transform.translation.lerp(vec3(x, y + 100.0, 0.0), 0.1);
}