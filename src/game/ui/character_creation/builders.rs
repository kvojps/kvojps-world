use bevy::prelude::*;

use super::components::{
    CharacterCreationUiRoot, ClassValueText, CreationActionsRow, CreationButtonAction,
    CreationContentRow, CreationPortraitCard, ErrorTextLabel, GenderValueText, NameInputButton,
    NameValueText, PortraitClassText, PortraitImageNode, PortraitStatusText,
};
use super::state::{CharacterCreationState, CharacterPortraitCatalog};

pub(super) fn setup_character_creation(
    mut commands: Commands,
    mut state: ResMut<CharacterCreationState>,
    mut portraits: ResMut<CharacterPortraitCatalog>,
    asset_server: Res<AssetServer>,
) {
    state.error_text = None;
    state.name_input_active = true;
    if state.character_name.is_empty() {
        state.selected_gender = 0;
        state.selected_class = 0;
    }

    portraits.warrior = asset_server.load("portraits/warrior.png");
    portraits.ranger = asset_server.load("portraits/ranger.png");
    portraits.mage = asset_server.load("portraits/mage.png");
    portraits.cleric = asset_server.load("portraits/clerico.png");

    let title_style = TextStyle {
        font_size: 52.0,
        color: Color::srgb_u8(236, 192, 112),
        ..default()
    };
    let subtitle_style = TextStyle {
        font_size: 17.0,
        color: Color::srgb_u8(188, 160, 126),
        ..default()
    };
    let label_style = TextStyle {
        font_size: 18.0,
        color: Color::srgb_u8(222, 196, 156),
        ..default()
    };
    let value_style = TextStyle {
        font_size: 16.0,
        color: Color::srgb_u8(196, 170, 136),
        ..default()
    };

    _spawn_creation_ui(
        &mut commands,
        &portraits,
        &title_style,
        &subtitle_style,
        &label_style,
        &value_style,
    );
}

// 1. Full screen
fn _spawn_creation_ui(
    commands: &mut Commands,
    portraits: &CharacterPortraitCatalog,
    title_style: &TextStyle,
    subtitle_style: &TextStyle,
    label_style: &TextStyle,
    value_style: &TextStyle,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    row_gap: Val::Px(18.0),
                    padding: UiRect::axes(Val::Px(22.0), Val::Px(18.0)),
                    ..default()
                },
                ..default()
            },
            CharacterCreationUiRoot,
        ))
        .with_children(|root| {
            _spawn_creation_header(root, title_style, subtitle_style);
            _spawn_creation_card(root, portraits, label_style, value_style);
            _spawn_creation_footer(root);
        });
}

// 1.1. Title, subtitle
fn _spawn_creation_header(
    root: &mut ChildBuilder,
    title_style: &TextStyle,
    subtitle_style: &TextStyle,
) {
    root.spawn(TextBundle::from_section(
        "Sala da Guilda",
        title_style.clone(),
    ));
    root.spawn(TextBundle::from_section(
        "Registre seu aventureiro antes de cruzar os portoes",
        subtitle_style.clone(),
    ));
}

// 1.2. Main area with form and portrait
fn _spawn_creation_card(
    root: &mut ChildBuilder,
    portraits: &CharacterPortraitCatalog,
    label_style: &TextStyle,
    value_style: &TextStyle,
) {
    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            max_width: Val::Px(940.0),
            min_width: Val::Px(320.0),
            padding: UiRect::all(Val::Px(22.0)),
            border: UiRect::all(Val::Px(2.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: BackgroundColor(Color::srgba_u8(36, 20, 12, 228)),
        border_color: BorderColor(Color::srgb_u8(150, 106, 64)),
        ..default()
    })
    .with_children(|card| {
        _spawn_creation_content_row(card, portraits, label_style, value_style);
    });
}

fn _spawn_creation_content_row(
    card: &mut ChildBuilder,
    portraits: &CharacterPortraitCatalog,
    label_style: &TextStyle,
    value_style: &TextStyle,
) {
    card.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::SpaceBetween,
                column_gap: Val::Px(24.0),
                ..default()
            },
            ..default()
        },
        CreationContentRow,
    ))
    .with_children(|content| {
        _spawn_creation_form_column(content, label_style, value_style);
        spawn_creation_portrait_column(content, portraits, label_style);
    });
}

// 1.2.1. Form column
fn _spawn_creation_form_column(
    content: &mut ChildBuilder,
    label_style: &TextStyle,
    value_style: &TextStyle,
) {
    content
        .spawn(NodeBundle {
            style: Style {
                flex_grow: 1.0,
                min_width: Val::Px(280.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            },
            ..default()
        })
        .with_children(|left| {
            left.spawn(TextBundle::from_section(
                "Nome do personagem",
                label_style.clone(),
            ));

            _spawn_creation_name_input(left, value_style);

            _spawn_selector_row(
                left,
                "Genero",
                CreationButtonAction::GenderPrev,
                CreationButtonAction::GenderNext,
                GenderValueText,
            );

            _spawn_selector_row(
                left,
                "Classe",
                CreationButtonAction::ClassPrev,
                CreationButtonAction::ClassNext,
                ClassValueText,
            );

            left.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb_u8(240, 130, 110),
                        ..default()
                    },
                ),
                ErrorTextLabel,
            ));

            _spawn_creation_actions(left);
        });
}

fn _spawn_creation_name_input(left: &mut ChildBuilder, value_style: &TextStyle) {
    left.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(38.0),
                padding: UiRect::horizontal(Val::Px(10.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba_u8(18, 12, 8, 220)),
            border_color: BorderColor(Color::srgb_u8(150, 106, 64)),
            ..default()
        },
        CreationButtonAction::NameInput,
        NameInputButton,
    ))
    .with_children(|name_button| {
        name_button.spawn((
            TextBundle::from_section("", value_style.clone()),
            NameValueText,
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
        TextStyle {
            font_size: 18.0,
            color: Color::srgb_u8(222, 196, 156),
            ..default()
        },
    ));

    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                column_gap: Val::Px(8.0),
                ..default()
            },
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(36.0),
                        height: Val::Px(32.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba_u8(60, 38, 25, 230)),
                    ..default()
                },
                prev_action,
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "<",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb_u8(222, 196, 156),
                        ..default()
                    },
                ));
            });

            row.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb_u8(196, 170, 136),
                        ..default()
                    },
                ),
                marker,
            ));

            row.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(36.0),
                        height: Val::Px(32.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba_u8(60, 38, 25, 230)),
                    ..default()
                },
                next_action,
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    ">",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb_u8(222, 196, 156),
                        ..default()
                    },
                ));
            });
        });
}

fn _spawn_creation_actions(left: &mut ChildBuilder) {
    left.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                row_gap: Val::Px(10.0),
                margin: UiRect::top(Val::Px(8.0)),
                ..default()
            },
            ..default()
        },
        CreationActionsRow,
    ))
    .with_children(|actions| {
        _spawn_action_button(actions, "Voltar ao Menu", CreationButtonAction::Back, 180.0);
        _spawn_action_button(
            actions,
            "Forjar Destino",
            CreationButtonAction::Begin,
            220.0,
        );
    });
}

fn _spawn_action_button(
    parent: &mut ChildBuilder,
    label: &str,
    action: CreationButtonAction,
    width: f32,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(width),
                    height: Val::Px(34.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba_u8(92, 56, 32, 240)),
                ..default()
            },
            action,
        ))
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 15.0,
                    color: Color::srgb_u8(222, 196, 156),
                    ..default()
                },
            ));
        });
}

// 1.2.2. Portrait column
fn spawn_creation_portrait_column(
    content: &mut ChildBuilder,
    portraits: &CharacterPortraitCatalog,
    label_style: &TextStyle,
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
            CreationPortraitCard,
        ))
        .with_children(|portrait| {
            portrait.spawn(TextBundle::from_section("Retrato", label_style.clone()));

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
                        PortraitImageNode,
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
                PortraitClassText,
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
                PortraitStatusText,
            ));
        });
}

// 1.3 Footer
fn _spawn_creation_footer(root: &mut ChildBuilder) {
    root.spawn(TextBundle::from_section(
        "ESC para voltar ao menu principal",
        TextStyle {
            font_size: 14.0,
            color: Color::srgb_u8(152, 128, 102),
            ..default()
        },
    ));
}

pub(super) fn cleanup_character_creation_ui(
    mut commands: Commands,
    roots: Query<Entity, With<CharacterCreationUiRoot>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn_recursive();
    }
}
