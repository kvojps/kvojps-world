mod components;
mod states;
mod styles;
mod views;
use crate::game::states::AppScreen;
use bevy::prelude::*;
use states::MainMenuState;
use views::{cleanup_main_menu, setup_main_menu};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainMenuState>()
            .add_systems(OnEnter(AppScreen::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppScreen::MainMenu), cleanup_main_menu);
    }
}
