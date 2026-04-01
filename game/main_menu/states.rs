use super::components::MenuItem;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MainMenuState {
    pub selected: usize,
    pub hint: Option<String>,
}

impl MainMenuState {
    pub fn selected_item(&self) -> MenuItem {
        MenuItem::ALL[self.selected]
    }

    pub fn select_previous(&mut self) {
        self.selected = if self.selected == 0 {
            MenuItem::ALL.len() - 1
        } else {
            self.selected - 1
        };
    }

    pub fn select_next(&mut self) {
        self.selected = (self.selected + 1) % MenuItem::ALL.len();
    }

    pub fn select_item(&mut self, item: MenuItem) {
        self.selected = item.index();
    }
}
