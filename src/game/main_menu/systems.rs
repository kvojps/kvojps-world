use crate::game::main_menu::components::{
    MenuItemActionButton, MenuItemDescription, MenuItemHint, MenuItemLabel,
};

use super::components::MenuItem;
use super::states::MainMenuState;
use super::styles::*;
use crate::game::states::AppScreen;
use bevy::prelude::*;

pub(super) fn sync_main_menu_ui(
    state: Res<MainMenuState>,
    mut text_queries: ParamSet<(
        Query<
            (&MenuItemLabel, &mut Text, &Parent),
            (With<MenuItemLabel>, Without<MenuItemDescription>),
        >,
        Query<&mut Text, With<MenuItemDescription>>,
        Query<&mut Text, With<MenuItemHint>>,
    )>,
    mut button_bg_query: Query<&mut BackgroundColor, With<Button>>,
) {
    let selected = state.selected_item();

    for (label, mut text, parent) in &mut text_queries.p0() {
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

    if let Ok(mut description) = text_queries.p1().get_single_mut() {
        description.sections[0].value = selected.description().to_string();
    }

    if let Ok(mut hint) = text_queries.p2().get_single_mut() {
        hint.sections[0].value = state.hint.clone().unwrap_or_default();
    }
}

pub(super) fn handle_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut interactions: Query<
        (&Interaction, &MenuItemActionButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<MainMenuState>,
    mut next_screen: ResMut<NextState<AppScreen>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
        state.select_previous();
        state.hint = None;
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        state.select_next();
        state.hint = None;
    }
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        let selected_item = state.selected_item();
        _activate_item(
            selected_item,
            &mut state,
            &mut next_screen,
            &mut app_exit_events,
        );
    }

    for (interaction, action) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        state.select_item(action.0);
        _activate_item(action.0, &mut state, &mut next_screen, &mut app_exit_events);
    }
}

fn _activate_item(
    item: MenuItem,
    state: &mut MainMenuState,
    _next_screen: &mut NextState<AppScreen>,
    app_exit_events: &mut EventWriter<AppExit>,
) {
    match item {
        MenuItem::NewGame => {
            state.hint = Some("Desenvolvimento da gameplay base em andamento.".to_string())
            // state.hint = None;
            // next_screen.set(AppScreen::CharacterCreation);
        }
        MenuItem::LoadGame => {
            state.hint = Some("Sistema de save/load será adicionado na próxima etapa.".to_string());
        }
        MenuItem::Options => {
            state.hint = Some("Menu de opções será implementado após o gameplay base.".to_string());
        }
        MenuItem::Quit => {
            app_exit_events.send(AppExit::Success);
        }
    }
}
