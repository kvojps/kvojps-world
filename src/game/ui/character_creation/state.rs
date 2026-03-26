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

impl CharacterPortraitCatalog {
    pub fn handle_for_class(&self, class_index: usize) -> &Handle<Image> {
        match class_index {
            0 => &self.warrior,
            1 => &self.ranger,
            2 => &self.mage,
            _ => &self.cleric,
        }
    }
}
