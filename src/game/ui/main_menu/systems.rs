use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::states::AppScreen;
use crate::game::ui::theme;

use super::components::{
    MENU_STACK_BREAKPOINT, MainMenuActionButton, MainMenuDescriptionText, MainMenuHintText,
    MainMenuOptionLabel, MainMenuPanel, MenuItem,
};
use super::state::MainMenuState;

pub(super) fn animate_menu_background(time: Res<Time>, mut clear_color: ResMut<ClearColor>) {
    let t = time.elapsed_seconds();
    let pulse = ((t * theme::MENU_BACKGROUND_SPEED).sin() + 1.0) * 0.5;
    clear_color.0 = theme::ember_background(pulse);
}

pub(super) fn handle_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<MainMenuState>,
    mut next_screen: ResMut<NextState<AppScreen>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
        state.select_previous();
    }

    if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        state.select_next();
    }

    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        let selected_item = state.selected_item();
        _activate_item(selected_item, &mut state, &mut next_screen, &mut app_exit_events);
    }
}

pub(super) fn handle_menu_button_interactions(
    mut interactions: Query<
        (&Interaction, &MainMenuActionButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<MainMenuState>,
    mut next_screen: ResMut<NextState<AppScreen>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        state.select_item(action.0);
        _activate_item(action.0, &mut state, &mut next_screen, &mut app_exit_events);
    }
}

pub(super) fn sync_main_menu_ui(
    state: Res<MainMenuState>,
    mut text_queries: ParamSet<(
        Query<
            (&MainMenuOptionLabel, &mut Text, &Parent),
            (With<MainMenuOptionLabel>, Without<MainMenuDescriptionText>),
        >,
        Query<&mut Text, With<MainMenuDescriptionText>>,
        Query<&mut Text, With<MainMenuHintText>>,
    )>,
    mut button_bg_query: Query<&mut BackgroundColor, With<Button>>,
) {
    let selected = state.selected_item();

    for (label, mut text, parent) in &mut text_queries.p0() {
        let is_selected = label.0 == selected;
        let marker = if is_selected { ">" } else { " " };

        text.sections[0].value = format!("{marker} {}", label.0.label());
        text.sections[0].style.color = if is_selected {
            theme::ui_selected_text()
        } else {
            theme::ui_label_text()
        };

        if let Ok(mut background) = button_bg_query.get_mut(parent.get()) {
            background.0 = if is_selected {
                theme::ui_button_selected_background()
            } else {
                theme::ui_button_background()
            };
        }
    }

    if let Ok(mut description) = text_queries.p1().get_single_mut() {
        description.sections[0].value = selected.description().to_string();
    }

    if let Ok(mut hint) = text_queries.p2().get_single_mut() {
        hint.sections[0].value = state.hint.clone().unwrap_or_default();
    }
}

pub(super) fn update_menu_layout(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut panel_query: Query<&mut Style, With<MainMenuPanel>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let compact = window.resolution.width() < MENU_STACK_BREAKPOINT;

    if let Ok(mut panel_style) = panel_query.get_single_mut() {
        panel_style.width = if compact {
            Val::Percent(92.0)
        } else {
            Val::Px(460.0)
        };
        panel_style.padding = if compact {
            UiRect::all(Val::Px(12.0))
        } else {
            UiRect::all(Val::Px(18.0))
        };
    }
}

fn _activate_item(
    item: MenuItem,
    state: &mut MainMenuState,
    next_screen: &mut NextState<AppScreen>,
    app_exit_events: &mut EventWriter<AppExit>,
) {
    match item {
        MenuItem::NewGame => {
            state.hint = None;
            next_screen.set(AppScreen::CharacterCreation);
        }
        MenuItem::LoadGame => {
            state.hint = Some("Sistema de save/load sera adicionado na proxima etapa.".to_string());
        }
        MenuItem::Options => {
            state.hint =
                Some("Menu de opcoes sera implementado apos o gameplay base.".to_string());
        }
        MenuItem::Quit => {
            app_exit_events.send(AppExit::Success);
        }
    }
}
