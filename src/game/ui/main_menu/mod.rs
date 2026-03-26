use bevy::prelude::*;

use crate::game::states::AppScreen;

mod builders;
mod components;
mod state;
mod systems;

use builders::{cleanup_main_menu_ui, setup_main_menu};
use state::MainMenuState;
use systems::{
    animate_menu_background, handle_menu_button_interactions, handle_menu_input, sync_main_menu_ui,
    update_menu_layout,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainMenuState>()
            .add_systems(OnEnter(AppScreen::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppScreen::MainMenu), cleanup_main_menu_ui)
            .add_systems(
                Update,
                (
                    animate_menu_background,
                    handle_menu_input,
                    handle_menu_button_interactions,
                    sync_main_menu_ui,
                    update_menu_layout,
                )
                    .run_if(in_state(AppScreen::MainMenu)),
            );
    }
}
