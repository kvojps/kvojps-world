use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CharacterCreationState {
    pub character_name: String,
    pub selected_gender: usize,
    pub selected_class: usize,
    pub error_text: Option<String>,
    pub name_input_active: bool,
}

#[derive(Resource, Default)]
pub struct CharacterPortraitCatalog {
    pub warrior: Handle<Image>,
    pub ranger: Handle<Image>,
    pub mage: Handle<Image>,
    pub cleric: Handle<Image>,
}

impl CharacterPortraitCatalog {}
