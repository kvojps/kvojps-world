use bevy::prelude::*;

#[derive(Component)]
pub(super) struct CharacterCreationUiRoot;

pub(super) const CHARACTER_GENDERS: [&str; 2] = ["Masculino", "Feminino"];
pub(super) const CHARACTER_CLASSES: [&str; 4] = ["Guerreiro", "Ranger", "Mago", "Clérigo"];
#[derive(Component, Clone, Copy)]
pub(super) enum CreationButtonAction {
    Back,
    Begin,
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

#[derive(Component)]
pub(super) struct ErrorLabel;

#[derive(Component)]
pub(super) struct PortraitCard;

#[derive(Component)]
pub(super) struct PortraitImageNode;

#[derive(Component)]
pub(super) struct PortraitClassText;

#[derive(Component)]
pub(super) struct PortraitStatusText;
