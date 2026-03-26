use bevy::prelude::*;

pub(super) const CHARACTER_GENDERS: [&str; 2] = ["Masculino", "Feminino"];
pub(super) const CHARACTER_CLASSES: [&str; 4] = ["Guerreiro", "Ranger", "Mago", "Clerigo"];
pub(super) const CREATION_STACK_BREAKPOINT: f32 = 900.0;
pub(super) const NAME_MAX_LEN: usize = 24;

#[derive(Component)]
pub(super) struct CharacterCreationUiRoot;

#[derive(Component)]
pub(super) struct CreationContentRow;

#[derive(Component)]
pub(super) struct CreationActionsRow;

#[derive(Component)]
pub(super) struct CreationPortraitCard;

#[derive(Component)]
pub(super) struct NameInputButton;

#[derive(Component)]
pub(super) struct NameValueText;

#[derive(Component)]
pub(super) struct GenderValueText;

#[derive(Component)]
pub(super) struct ClassValueText;

#[derive(Component)]
pub(super) struct ErrorTextLabel;

#[derive(Component)]
pub(super) struct PortraitImageNode;

#[derive(Component)]
pub(super) struct PortraitStatusText;

#[derive(Component)]
pub(super) struct PortraitClassText;

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
