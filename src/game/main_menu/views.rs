use super::components::*;
use super::states::MainMenuState;
use super::styles::*;
use crate::themes::*;
use bevy::prelude::*;

pub(super) fn setup_main_menu(mut commands: Commands, mut state: ResMut<MainMenuState>) {
    state.selected = 0;

    commands
        .spawn((
            NodeBundle {
                style: root_container_style(),
                background_color: ember_background(),
                ..default()
            },
            MainMenuUiRoot,
        ))
        .with_children(|root| {
            _spawn_header(root);
            _spawn_menu_panel(root);
            _spawn_description_area(root);
        });
}

fn _spawn_header(root: &mut ChildBuilder) {
    root.spawn(NodeBundle {
        style: header_container_style(),
        ..default()
    })
    .with_children(|header| {
        header.spawn(TextBundle::from_section("Kvojps World", title_style()));
        header.spawn(TextBundle::from_section(
            "Uma aventura RPG de sobrevivência e exploracão",
            subtitle_style(),
        ));
    });
}

fn _spawn_menu_panel(root: &mut ChildBuilder) {
    root.spawn((
        NodeBundle {
            style: menu_panel_container_style(),
            ..default()
        },
        MainMenuPanel,
    ))
    .with_children(|panel| {
        for item in MenuItem::ALL {
            panel
                .spawn((
                    ButtonBundle {
                        style: menu_item_style(),
                        background_color: menu_item_colors().0,
                        border_color: menu_item_colors().1,
                        ..default()
                    },
                    MenuItemActionButton(item),
                ))
                .with_children(|button| {
                    button.spawn((
                        TextBundle::from_section(item.label(), menu_item_label_style()),
                        MenuItemLabel(item),
                    ));
                });
        }
    });
}

fn _spawn_description_area(root: &mut ChildBuilder) {
    root.spawn(NodeBundle {
        style: menu_item_desc_container_style(),
        ..default()
    })
    .with_children(|description| {
        description.spawn((
            TextBundle::from_section("", menu_item_desc_text_style()),
            MenuItemDescriptionText,
        ));
        // description.spawn((
        //     TextBundle::from_section(
        //         "",
        //         TextStyle {
        //             font_size: theme::FONT_SIZE_HINT,
        //             color: theme::ui_hint_text(),
        //             ..default()
        //         },
        //     ),
        //     MainMenuHintText,
        // ));
    });
}

pub(super) fn cleanup_main_menu(
    mut commands: Commands,
    roots: Query<Entity, With<MainMenuUiRoot>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn_recursive();
    }
}
