pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::{animate_player, player_movement, setup_camera_and_player_sheet, setup_player};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera_and_player_sheet)
            .add_systems(Update, setup_player)
            .add_systems(Update, (player_movement, animate_player).chain());
    }
}
