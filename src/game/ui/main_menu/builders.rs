use bevy::prelude::*;

use crate::game::ui::theme;

use super::components::{
    MainMenuActionButton, MainMenuDescriptionText, MainMenuHintText, MainMenuOptionLabel,
    MainMenuPanel, MainMenuUiRoot, MenuItem,
};
use super::state::MainMenuState;

// 1. Full screen
pub(super) fn setup_main_menu(mut commands: Commands, mut state: ResMut<MainMenuState>) {
    state.selected = 0;
    state.hint = None;

    let title_style = TextStyle {
        font_size: theme::FONT_SIZE_DISPLAY_TITLE,
        color: theme::ui_title_text(),
        ..default()
    };
    let subtitle_style = TextStyle {
        font_size: theme::FONT_SIZE_LABEL,
        color: theme::ui_subtitle_text(),
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    row_gap: Val::Px(16.0),
                    padding: UiRect::axes(Val::Px(18.0), Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
            MainMenuUiRoot,
        ))
        .with_children(|root| {
            _spawn_header(root, &title_style, &subtitle_style);
            _spawn_menu_panel(root);
            _spawn_description_area(root);
            _spawn_footer(root);
        });
}

// 1.1. Title, subtitle
fn _spawn_header(root: &mut ChildBuilder, title_style: &TextStyle, subtitle_style: &TextStyle) {
    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|header| {
        header.spawn(TextBundle::from_section(
            "Kvojps World",
            title_style.clone(),
        ));
        header.spawn(TextBundle::from_section(
            "Uma aventura RPG de sobrevivencia e exploracao",
            subtitle_style.clone(),
        ));
    });
}

// 1.2 Main Menu
fn _spawn_menu_panel(root: &mut ChildBuilder) {
    root.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(460.0),
                max_width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(18.0)),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                margin: UiRect::top(Val::Px(24.0)),
                ..default()
            },
            background_color: BackgroundColor(theme::ui_panel_background()),
            border_color: BorderColor(theme::ui_panel_border()),
            ..default()
        },
        MainMenuPanel,
    ))
    .with_children(|panel| {
        for item in MenuItem::ALL {
            panel
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(42.0),
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            padding: UiRect::left(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(theme::ui_button_background()),
                        border_color: BorderColor(Color::NONE),
                        ..default()
                    },
                    MainMenuActionButton(item),
                ))
                .with_children(|button| {
                    button.spawn((
                        TextBundle::from_section(
                            item.label(),
                            TextStyle {
                                font_size: theme::FONT_SIZE_MENU_OPTION,
                                color: theme::ui_label_text(),
                                ..default()
                            },
                        ),
                        MainMenuOptionLabel(item),
                    ));
                });
        }
    });
}

fn _spawn_description_area(root: &mut ChildBuilder) {
    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            max_width: Val::Px(840.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(8.0),
            margin: UiRect::top(Val::Px(8.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|description| {
        description.spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font_size: theme::FONT_SIZE_BODY,
                    color: theme::ui_value_text(),
                    ..default()
                },
            ),
            MainMenuDescriptionText,
        ));

        description.spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font_size: theme::FONT_SIZE_HINT,
                    color: theme::ui_hint_text(),
                    ..default()
                },
            ),
            MainMenuHintText,
        ));
    });
}

// 1.3. Footer
fn _spawn_footer(root: &mut ChildBuilder) {
    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            padding: UiRect::bottom(Val::Px(12.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|footer| {
        footer.spawn(TextBundle::from_section(
            "Use W/S ou setas para navegar | Enter ou Espaco para confirmar",
            TextStyle {
                font_size: theme::FONT_SIZE_FOOTER,
                color: theme::ui_muted_text(),
                ..default()
            },
        ));
    });
}

pub(super) fn cleanup_main_menu_ui(
    mut commands: Commands,
    roots: Query<Entity, With<MainMenuUiRoot>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn_recursive();
    }
}
