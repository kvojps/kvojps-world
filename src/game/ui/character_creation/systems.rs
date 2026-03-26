use bevy::input::{
    ButtonState,
    keyboard::{Key, KeyboardInput},
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::states::AppScreen;
use crate::game::ui::theme;

use super::components::{
    CHARACTER_CLASSES, CHARACTER_GENDERS, CREATION_STACK_BREAKPOINT, ClassValueText,
    CreationActionsRow, CreationButtonAction, CreationContentRow, CreationPortraitCard,
    ErrorTextLabel, GenderValueText, NAME_MAX_LEN, NameInputButton, NameValueText,
    PortraitClassText, PortraitImageNode, PortraitStatusText,
};
use super::state::{CharacterCreationState, CharacterPortraitCatalog};

pub(super) fn handle_creation_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_screen: ResMut<NextState<AppScreen>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_screen.set(AppScreen::MainMenu);
    }
}

pub(super) fn handle_creation_button_interactions(
    mut interactions: Query<
        (&Interaction, &CreationButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<CharacterCreationState>,
    mut next_screen: ResMut<NextState<AppScreen>>,
) {
    for (interaction, action) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            CreationButtonAction::Back => {
                state.error_text = None;
                state.name_input_active = false;
                next_screen.set(AppScreen::MainMenu);
            }
            CreationButtonAction::Begin => {
                state.name_input_active = false;
                if state.character_name.trim().is_empty() {
                    state.error_text = Some("Informe um nome para iniciar a jornada.".to_string());
                } else {
                    state.error_text = None;
                    next_screen.set(AppScreen::Overworld);
                }
            }
            CreationButtonAction::NameInput => {
                state.name_input_active = true;
            }
            CreationButtonAction::GenderPrev => {
                state.name_input_active = false;
                state.selected_gender = if state.selected_gender == 0 {
                    CHARACTER_GENDERS.len() - 1
                } else {
                    state.selected_gender - 1
                };
            }
            CreationButtonAction::GenderNext => {
                state.name_input_active = false;
                state.selected_gender = (state.selected_gender + 1) % CHARACTER_GENDERS.len();
            }
            CreationButtonAction::ClassPrev => {
                state.name_input_active = false;
                state.selected_class = if state.selected_class == 0 {
                    CHARACTER_CLASSES.len() - 1
                } else {
                    state.selected_class - 1
                };
            }
            CreationButtonAction::ClassNext => {
                state.name_input_active = false;
                state.selected_class = (state.selected_class + 1) % CHARACTER_CLASSES.len();
            }
        }
    }
}

pub(super) fn handle_creation_text_input(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut state: ResMut<CharacterCreationState>,
) {
    if !state.name_input_active {
        return;
    }

    for keyboard_event in keyboard_events.read() {
        if keyboard_event.state != ButtonState::Pressed {
            continue;
        }

        match &keyboard_event.logical_key {
            Key::Backspace => {
                state.character_name.pop();
                state.error_text = None;
            }
            Key::Enter => {
                state.name_input_active = false;
            }
            Key::Space => {
                if state.character_name.len() < NAME_MAX_LEN {
                    state.character_name.push(' ');
                    state.error_text = None;
                }
            }
            Key::Character(input) => {
                for ch in input.chars() {
                    if state.character_name.len() >= NAME_MAX_LEN {
                        break;
                    }

                    if is_allowed_name_char(ch) {
                        state.character_name.push(ch);
                        state.error_text = None;
                    }
                }
            }
            _ => {}
        }
    }
}

fn is_allowed_name_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == ' ' || ch == '-' || ch == '\''
}

pub(super) fn sync_creation_ui(
    state: Res<CharacterCreationState>,
    portraits: Res<CharacterPortraitCatalog>,
    asset_server: Res<AssetServer>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<NameValueText>>,
        Query<&mut Text, With<GenderValueText>>,
        Query<&mut Text, With<ClassValueText>>,
        Query<&mut Text, With<ErrorTextLabel>>,
        Query<&mut Text, With<PortraitClassText>>,
        Query<&mut Text, With<PortraitStatusText>>,
    )>,
    mut portrait_image_query: Query<&mut UiImage, With<PortraitImageNode>>,
    mut name_field_bg_query: Query<&mut BackgroundColor, With<NameInputButton>>,
) {
    let name_display = if state.character_name.is_empty() {
        if state.name_input_active {
            "Digite o nome do heroi...".to_string()
        } else {
            "Clique para editar o nome".to_string()
        }
    } else if state.name_input_active {
        format!("{}|", state.character_name)
    } else {
        state.character_name.clone()
    };

    if let Ok(mut text) = text_queries.p0().get_single_mut() {
        text.sections[0].value = name_display;
    }

    if let Ok(mut bg) = name_field_bg_query.get_single_mut() {
        bg.0 = if state.name_input_active {
            Color::srgba_u8(94, 58, 36, 245)
        } else {
            Color::srgba_u8(18, 12, 8, 220)
        };
    }

    if let Ok(mut text) = text_queries.p1().get_single_mut() {
        text.sections[0].value = CHARACTER_GENDERS[state.selected_gender].to_string();
    }

    if let Ok(mut text) = text_queries.p2().get_single_mut() {
        text.sections[0].value = CHARACTER_CLASSES[state.selected_class].to_string();
    }

    if let Ok(mut text) = text_queries.p3().get_single_mut() {
        text.sections[0].value = state.error_text.clone().unwrap_or_default();
    }

    if let Ok(mut text) = text_queries.p4().get_single_mut() {
        text.sections[0].value = CHARACTER_CLASSES[state.selected_class].to_string();
    }

    let selected_portrait = portraits
        .handle_for_class(state.selected_class)
        .clone_weak();
    if let Ok(mut image) = portrait_image_query.get_single_mut() {
        image.texture = selected_portrait.clone();
    }

    let status_message = match asset_server.get_load_state(selected_portrait.id()) {
        Some(bevy::asset::LoadState::Loaded) => "",
        Some(bevy::asset::LoadState::Failed(_)) => "Falha ao carregar retrato",
        _ => "Carregando retrato...",
    };

    if let Ok(mut text) = text_queries.p5().get_single_mut() {
        text.sections[0].value = status_message.to_string();
    }
}

pub(super) fn animate_creation_background(time: Res<Time>, mut clear_color: ResMut<ClearColor>) {
    let t = time.elapsed_seconds();
    let pulse = ((t * theme::CREATION_BACKGROUND_SPEED).sin() + 1.0) * 0.5;
    clear_color.0 = theme::ember_background(pulse);
}

pub(super) fn update_creation_layout(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut style_queries: ParamSet<(
        Query<&mut Style, With<CreationContentRow>>,
        Query<&mut Style, With<CreationActionsRow>>,
        Query<&mut Style, With<CreationPortraitCard>>,
        Query<&mut Style, With<PortraitImageNode>>,
    )>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let compact_layout = window.resolution.width() < CREATION_STACK_BREAKPOINT;

    if let Ok(mut style) = style_queries.p0().get_single_mut() {
        style.flex_direction = if compact_layout {
            FlexDirection::Column
        } else {
            FlexDirection::Row
        };
        style.row_gap = if compact_layout {
            Val::Px(16.0)
        } else {
            Val::Px(0.0)
        };
    }

    if let Ok(mut style) = style_queries.p1().get_single_mut() {
        style.flex_direction = if compact_layout {
            FlexDirection::Column
        } else {
            FlexDirection::Row
        };
        style.align_items = if compact_layout {
            AlignItems::Stretch
        } else {
            AlignItems::FlexStart
        };
    }

    if let Ok(mut style) = style_queries.p2().get_single_mut() {
        style.width = if compact_layout {
            Val::Percent(100.0)
        } else {
            Val::Px(260.0)
        };
    }

    if let Ok(mut style) = style_queries.p3().get_single_mut() {
        style.width = if compact_layout {
            Val::Px(180.0)
        } else {
            Val::Px(200.0)
        };
        style.height = if compact_layout {
            Val::Px(220.0)
        } else {
            Val::Px(240.0)
        };
    }
}
