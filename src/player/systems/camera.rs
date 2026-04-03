use crate::player::components::Player;
use bevy::prelude::*;

pub fn follow_player_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
