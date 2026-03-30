mod components;
mod styles;
mod views;
use crate::game::states::AppScreen;
use bevy::prelude::*;
use views::{cleanup_character_creation, setup_character_creation};

pub struct CharacterCreationPlugin;

impl Plugin for CharacterCreationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppScreen::CharacterCreation),
            setup_character_creation,
        )
        .add_systems(
            OnExit(AppScreen::CharacterCreation),
            cleanup_character_creation,
        );
    }
}
