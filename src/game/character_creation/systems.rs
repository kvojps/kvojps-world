use super::components::*;
use super::states::*;
use super::styles::{character_name_active_bg_color, character_name_inactive_bg_color};
use bevy::input::{
    ButtonState,
    keyboard::{Key, KeyboardInput},
};
use bevy::prelude::*;

pub(super) fn sync_character_creation_ui(
    state: Res<CharacterCreationState>,
    mut text_query: Query<&mut Text, With<NameInputValue>>,
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

    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = name_display;
    }

    if let Ok(mut bg) = name_field_bg_query.get_single_mut() {
        bg.0 = if state.name_input_active {
            character_name_active_bg_color()
        } else {
            character_name_inactive_bg_color()
        };
    }
}

pub(super) fn handle_character_creation_interactions(
    mut interactions: Query<
        (&Interaction, &CreationButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<CharacterCreationState>,
) {
    for (interaction, action) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            CreationButtonAction::NameInput => {
                state.name_input_active = true;
            }
        }
    }
}

pub(super) const NAME_MAX_LEN: usize = 24;
pub(super) fn handle_character_name_input(
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

                    if _is_allowed_name_char(ch) {
                        state.character_name.push(ch);
                        state.error_text = None;
                    }
                }
            }
            _ => {}
        }
    }
}

fn _is_allowed_name_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == ' ' || ch == '-' || ch == '\''
}
