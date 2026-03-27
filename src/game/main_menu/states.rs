use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MainMenuState {
    pub selected: usize,
    pub hint: Option<String>,
}
