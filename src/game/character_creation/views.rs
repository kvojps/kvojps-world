use super::components::*;
use super::states::*;
use super::styles::*;
use bevy::prelude::*;

pub(super) fn setup_character_creation(
    mut commands: Commands,
    mut state: ResMut<CharacterCreationState>,
    mut portraits: ResMut<CharacterPortraitCatalog>,
    asset_server: Res<AssetServer>,
) {
    state.selected_gender = 0;
    state.selected_class = 0;
    state.error_text = None;
    state.name_input_active = false;

    portraits.warrior = asset_server.load("portraits/warrior.png");
    portraits.ranger = asset_server.load("portraits/ranger.png");
    portraits.mage = asset_server.load("portraits/mage.png");
    portraits.cleric = asset_server.load("portraits/clerico.png");

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
            _spawn_character_creation_area(root, &portraits);
        });
}

fn _spawn_creation_header(root: &mut ChildBuilder) {
    root.spawn(TextBundle::from_section("Sala da Guilda", title_style()));
    root.spawn(TextBundle::from_section(
        "Registre seu aventureiro antes de cruzar os portões",
        subtitle_style(),
    ));
}

fn _spawn_character_creation_area(root: &mut ChildBuilder, portraits: &CharacterPortraitCatalog) {
    root.spawn(NodeBundle {
        style: character_creation_area_style(),
        background_color: character_creation_area_bg_color(),
        border_color: character_creation_area_border_color(),
        ..default()
    })
    .with_children(|area| {
        _spawn_character_creation_area_row(area, portraits);
    });
}

fn _spawn_character_creation_area_row(
    creation_area: &mut ChildBuilder,
    portraits: &CharacterPortraitCatalog,
) {
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
            _spawn_creation_portrait_column(content, portraits);
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

            _spawn_selector_row(
                left,
                "Gênero",
                CreationButtonAction::GenderPrev,
                CreationButtonAction::GenderNext,
                GenderInputValue,
            );
            _spawn_selector_row(
                left,
                "Classe",
                CreationButtonAction::ClassPrev,
                CreationButtonAction::ClassNext,
                ClassInputValue,
            );

            _spawn_creation_actions(left);
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
        CreationButtonAction::NameInput,
        NameInputButton,
    ))
    .with_children(|name_button| {
        name_button.spawn((
            TextBundle::from_section("", character_name_text_style()),
            NameInputValue,
        ));
    });
}

fn _spawn_selector_row<T: Component>(
    parent: &mut ChildBuilder,
    label: &str,
    prev_action: CreationButtonAction,
    next_action: CreationButtonAction,
    marker: T,
) {
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
                prev_action,
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "<",
                    selector_row_prev_action_text_style(),
                ));
            });

            row.spawn((
                TextBundle::from_section("", selector_row_marker_text_style()),
                marker,
            ));

            row.spawn((
                ButtonBundle {
                    style: selector_row_next_action_style(),
                    background_color: selector_row_next_action_bg_style(),
                    ..default()
                },
                next_action,
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    ">",
                    selector_row_next_action_text_style(),
                ));
            });
        });
}

fn _spawn_creation_actions(left: &mut ChildBuilder) {
    left.spawn((NodeBundle {
        style: creation_actions_container_style(),
        ..default()
    },))
        .with_children(|actions| {
            _spawn_action_button(actions, "Voltar ao Menu", CreationButtonAction::Back);
            _spawn_action_button(actions, "Forjar Destino", CreationButtonAction::Begin);
        });
}

fn _spawn_action_button(parent: &mut ChildBuilder, label: &str, action: CreationButtonAction) {
    parent
        .spawn((
            ButtonBundle {
                style: action_button_style(),
                background_color: action_button_bg_style(),
                ..default()
            },
            action,
        ))
        .with_children(|button| {
            button.spawn(TextBundle::from_section(label, action_button_text_style()));
        });
}

fn _spawn_creation_portrait_column(
    content: &mut ChildBuilder,
    portraits: &CharacterPortraitCatalog,
) {
    content
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(260.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                ..default()
            },
            // CreationPortraitCard,
        ))
        .with_children(|portrait| {
            portrait.spawn(TextBundle::from_section(
                "Retrato",
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb_u8(222, 196, 156),
                    ..default()
                },
            ));

            portrait
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(220.0),
                        height: Val::Px(280.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba_u8(18, 10, 6, 170)),
                    border_color: BorderColor(Color::srgb_u8(150, 106, 64)),
                    ..default()
                })
                .with_children(|frame| {
                    frame.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                height: Val::Px(240.0),
                                ..default()
                            },
                            image: UiImage::new(portraits.warrior.clone()),
                            ..default()
                        },
                        // PortraitImageNode,
                    ));
                });

            portrait.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 17.0,
                        color: Color::srgb_u8(252, 204, 118),
                        ..default()
                    },
                ),
                // PortraitClassText,
            ));

            portrait.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb_u8(152, 128, 102),
                        ..default()
                    },
                ),
                // PortraitStatusText,
            ));
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
