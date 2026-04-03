pub mod components;
pub mod systems;

use self::systems::{animate_player, movement_player, setup_player, setup_player_scene};
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player_scene)
            .add_systems(Update, setup_player)
            .add_systems(Update, (movement_player, animate_player).chain());
    }
}
