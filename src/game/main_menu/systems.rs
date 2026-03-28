use crate::game::main_menu::components::MenuItemLabel;

use super::states::MainMenuState;
use super::styles::*;
use bevy::prelude::*;

pub(super) fn sync_main_menu_ui(
    state: Res<MainMenuState>,
    mut text_query: Query<(&MenuItemLabel, &mut Text, &Parent)>,
    mut button_bg_query: Query<&mut BackgroundColor, With<Button>>,
) {
    let selected = state.selected_item();

    for (label, mut text, parent) in &mut text_query {
        let is_selected = label.0 == selected;
        let marker = if is_selected { ">" } else { " " };

        text.sections[0].value = format!("{marker} {}", label.0.label());
        text.sections[0].style.color = if is_selected {
            menu_item_selected_color()
        } else {
            menu_item_color()
        };

        if let Ok(mut background) = button_bg_query.get_mut(parent.get()) {
            background.0 = if is_selected {
                menu_item_selected_bg_color()
            } else {
                menu_item_bg_color()
            }
        }
    }
}
