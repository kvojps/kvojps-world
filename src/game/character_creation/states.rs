use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CharacterPortraitCatalog {
    pub warrior: Handle<Image>,
    pub ranger: Handle<Image>,
    pub mage: Handle<Image>,
    pub cleric: Handle<Image>,
}

impl CharacterPortraitCatalog {}
