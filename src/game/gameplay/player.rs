use bevy::prelude::*;

use crate::game::ui::character_creation::CharacterCreationState;

#[derive(Component, Clone)]
pub struct Player {
    pub name: String,
    pub class: CharacterClass,
    pub gender: CharacterGender,
    pub level: u32,
    pub hp_current: i32,
    pub hp_max: i32,
}

#[derive(Clone, Copy)]
pub enum CharacterClass {
    Warrior,
    Ranger,
    Mage,
    Cleric,
}

#[derive(Clone, Copy)]
pub enum CharacterGender {
    Masculine,
    Feminine,
}

impl Player {
    pub fn from_creation_state(state: &CharacterCreationState) -> Self {
        let name = {
            let trimmed = state.character_name.trim();
            if trimmed.is_empty() {
                "Heroi".to_string()
            } else {
                trimmed.to_string()
            }
        };

        Self {
            name,
            class: CharacterClass::from_index(state.selected_class),
            gender: CharacterGender::from_index(state.selected_gender),
            level: 1,
            hp_current: 100,
            hp_max: 100,
        }
    }

    pub fn fallback() -> Self {
        Self {
            name: "Heroi".to_string(),
            class: CharacterClass::Warrior,
            gender: CharacterGender::Masculine,
            level: 1,
            hp_current: 100,
            hp_max: 100,
        }
    }
}

impl CharacterClass {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Warrior,
            1 => Self::Ranger,
            2 => Self::Mage,
            _ => Self::Cleric,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Warrior => "Guerreiro",
            Self::Ranger => "Ranger",
            Self::Mage => "Mago",
            Self::Cleric => "Clerigo",
        }
    }
}

impl CharacterGender {
    pub fn from_index(index: usize) -> Self {
        match index {
            1 => Self::Feminine,
            _ => Self::Masculine,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Masculine => "Masculino",
            Self::Feminine => "Feminino",
        }
    }
}
