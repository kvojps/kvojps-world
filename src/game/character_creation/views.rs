use super::components::*;
use super::styles::*;
use bevy::prelude::*;

pub(super) fn setup_character_creation(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: root_container_style(),
                ..default()
            },
            CharacterCreationUiRoot,
        ))
        .with_children(|root| {
            _spawn_creation_header(root);
            _spawn_character_creation_area(root);
        });
}

fn _spawn_creation_header(root: &mut ChildBuilder) {
    root.spawn(TextBundle::from_section("Sala da Guilda", title_style()));
    root.spawn(TextBundle::from_section(
        "Registre seu aventureiro antes de cruzar os portões",
        subtitle_style(),
    ));
}

fn _spawn_character_creation_area(root: &mut ChildBuilder) {
    root.spawn(NodeBundle {
        style: character_creation_area_style(),
        background_color: character_creation_area_bg_color(),
        border_color: character_creation_area_border_color(),
        ..default()
    })
    .with_children(|area| {
        _spawn_character_creation_area_row(area);
    });
}

fn _spawn_character_creation_area_row(creation_area: &mut ChildBuilder) {
    creation_area
        .spawn((
            NodeBundle {
                style: character_area_creation_area_row_style(),
                ..default()
            },
            // CreationAreaRow,
        ))
        .with_children(|content| {
            _spawn_creation_form_column(content);
            // spawn_creation_portrait_column(content);
        });
}

fn _spawn_creation_form_column(creation_area: &mut ChildBuilder) {
    creation_area
        .spawn(NodeBundle {
            style: character_area_creation_form_column_style(),
            ..default()
        })
        .with_children(|left| {
            left.spawn(TextBundle::from_section(
                "Nome do personagem",
                character_title_text_style(),
            ));

            _spawn_creation_name_input(left);

            _spawn_selector_row(left, "Gênero");
            _spawn_selector_row(left, "Classe");
        });
}

fn _spawn_creation_name_input(left: &mut ChildBuilder) {
    left.spawn((
        ButtonBundle {
            style: character_name_style(),
            background_color: BackgroundColor(Color::srgba_u8(18, 12, 8, 220)),
            border_color: BorderColor(Color::srgb_u8(150, 106, 64)),
            ..default()
        },
        // CreationButtonAction::NameInput,
        // NameInputButton,
    ))
    .with_children(|name_button| {
        name_button.spawn((
            TextBundle::from_section("", character_name_text_style()),
            // NameValueText,
        ));
    });
}

fn _spawn_selector_row(parent: &mut ChildBuilder, label: &str) {
    parent.spawn(TextBundle::from_section(
        label,
        selector_row_title_text_style(),
    ));

    parent
        .spawn(NodeBundle {
            style: selector_row_style(),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                ButtonBundle {
                    style: selector_row_prev_action_style(),
                    background_color: selector_row_prev_action_bg_style(),
                    ..default()
                },
                // prev_action,
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "<",
                    selector_row_prev_action_text_style(),
                ));
            });

            row.spawn((
                TextBundle::from_section("", selector_row_marker_text_style()),
                // marker,
            ));

            row.spawn((
                ButtonBundle {
                    style: selector_row_next_action_style(),
                    background_color: selector_row_next_action_bg_style(),
                    ..default()
                },
                // next_action,
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    ">",
                    selector_row_next_action_text_style(),
                ));
            });
        });
}

pub(super) fn cleanup_character_creation(
    mut commands: Commands,
    roots: Query<Entity, With<CharacterCreationUiRoot>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn_recursive();
    }
}
