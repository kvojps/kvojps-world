use bevy::prelude::*;

#[derive(Component)]
pub(super) struct CharacterCreationUiRoot;

pub(super) const CHARACTER_GENDERS: [&str; 2] = ["Masculino", "Feminino"];
pub(super) const CHARACTER_CLASSES: [&str; 4] = ["Guerreiro", "Ranger", "Mago", "Clérigo"];
#[derive(Component, Clone, Copy)]
pub(super) enum CreationButtonAction {
    // Back,
    // Begin,
    NameInput,
    GenderPrev,
    GenderNext,
    ClassPrev,
    ClassNext,
}

#[derive(Component)]
pub(super) struct NameInputButton;

#[derive(Component)]
pub(super) struct NameInputValue;

#[derive(Component)]
pub(super) struct GenderInputValue;

#[derive(Component)]
pub(super) struct ClassInputValue;
