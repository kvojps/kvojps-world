pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::{
    animate_player, player_movement, setup_camera_and_player_sheet, spawn_player_from_sheet_when_ready,
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera_and_player_sheet)
            .add_systems(Update, spawn_player_from_sheet_when_ready)
            .add_systems(Update, (player_movement, animate_player).chain());
    }
}
