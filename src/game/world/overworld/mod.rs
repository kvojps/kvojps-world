use bevy::prelude::*;

use crate::game::states::AppScreen;

mod builders;
mod components;
mod map;
mod state;
mod systems;

use builders::{cleanup_overworld, setup_overworld};
use systems::{
    draw_overworld_overlay, handle_overworld_player_movement, return_to_menu,
    sync_overworld_layout,
};

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppScreen::Overworld), setup_overworld)
            .add_systems(
                Update,
                (
                    return_to_menu,
                    handle_overworld_player_movement,
                    sync_overworld_layout,
                    draw_overworld_overlay,
                )
                    .run_if(in_state(AppScreen::Overworld)),
            )
            .add_systems(OnExit(AppScreen::Overworld), cleanup_overworld);
    }
}
