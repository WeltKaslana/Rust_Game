use bevy::{math::vec3, prelude::*, dev_tools::states::*};
use crate::gamestate::GameState;
use crate::character::Character;

pub  struct FollowCameraPlugin;


impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::Home)),
            )
            .add_systems(Update, log_transitions::<GameState>);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Character>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Character>)>,
) {
    if camera_query.is_empty() || player_query.is_empty() {
        return;
    }
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single().translation;
    let (x, y) = (player_transform.x, player_transform.y);
    camera_transform.translation = camera_transform.translation.lerp(vec3(x, y, 0.0), 0.1);
}