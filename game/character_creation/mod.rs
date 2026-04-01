mod components;
mod states;
mod styles;
mod systems;
mod views;
use crate::game::states::AppScreen;
use bevy::prelude::*;
use states::{CharacterCreationState, CharacterPortraitCatalog};
use systems::*;
use views::{cleanup_character_creation, setup_character_creation};

pub struct CharacterCreationPlugin;

impl Plugin for CharacterCreationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterCreationState>()
            .init_resource::<CharacterPortraitCatalog>()
            .add_systems(
                OnEnter(AppScreen::CharacterCreation),
                setup_character_creation,
            )
            .add_systems(
                OnExit(AppScreen::CharacterCreation),
                cleanup_character_creation,
            )
            .add_systems(
                Update,
                (
                    sync_character_creation_ui,
                    handle_character_creation_interactions,
                    handle_character_name_input,
                )
                    .run_if(in_state(AppScreen::CharacterCreation)),
            );
    }
}
