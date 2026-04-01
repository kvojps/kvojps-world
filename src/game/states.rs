use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppScreen {
    #[default]
    MainMenu,
    CharacterCreation,
}
