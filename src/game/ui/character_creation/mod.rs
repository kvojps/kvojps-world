use bevy::prelude::*;

use crate::game::states::AppScreen;

mod builders;
mod components;
mod state;
mod systems;

use builders::{cleanup_character_creation_ui, setup_character_creation};
use state::{CharacterCreationState, CharacterPortraitCatalog};
use systems::{
    animate_creation_background, handle_creation_button_interactions, handle_creation_input,
    handle_creation_text_input, sync_creation_ui, update_creation_layout,
};

pub struct CharacterCreationPlugin;

impl Plugin for CharacterCreationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterCreationState>()
            .init_resource::<CharacterPortraitCatalog>()
            .add_systems(OnEnter(AppScreen::CharacterCreation), setup_character_creation)
            .add_systems(
                OnExit(AppScreen::CharacterCreation),
                cleanup_character_creation_ui,
            )
            .add_systems(
                Update,
                (
                    animate_creation_background,
                    handle_creation_input,
                    handle_creation_button_interactions,
                    handle_creation_text_input,
                    sync_creation_ui,
                    update_creation_layout,
                )
                    .run_if(in_state(AppScreen::CharacterCreation)),
            );
    }
}
