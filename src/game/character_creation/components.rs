use bevy::prelude::*;

#[derive(Component)]
pub(super) struct CharacterCreationUiRoot;

pub(super) const CHARACTER_GENDERS: [&str; 2] = ["Masculino", "Feminino"];
#[derive(Component, Clone, Copy)]
pub(super) enum CreationButtonAction {
    // Back,
    // Begin,
    NameInput,
    GenderPrev,
    GenderNext,
    // ClassPrev,
    // ClassNext,
}

#[derive(Component)]
pub(super) struct NameInputButton;

#[derive(Component)]
pub(super) struct NameInputValue;

#[derive(Component)]
pub(super) struct GenderInputValue;
