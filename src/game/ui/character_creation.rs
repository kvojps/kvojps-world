use bevy::input::{
    ButtonState,
    keyboard::{Key, KeyboardInput},
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::states::AppScreen;
use crate::game::ui::theme;

// Plugin
pub struct CharacterCreationPlugin;

impl Plugin for CharacterCreationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterCreationState>()
            .init_resource::<CharacterPortraitCatalog>()
            .add_systems(
                OnEnter(AppScreen::CharacterCreation),
                setup_character_creation,
            )
            .add_systems(
                OnExit(AppScreen::CharacterCreation),
                cleanup_character_creation_ui,
            )
            .add_systems(
                Update,
                (
                    animate_creation_background,
                    handle_creation_input,
                    handle_creation_button_interactions,
                    handle_creation_text_input,
                    sync_creation_ui,
                    update_creation_layout,
                )
                    .run_if(in_state(AppScreen::CharacterCreation)),
            );
    }
}

// States
#[derive(Resource, Default)]
pub struct CharacterCreationState {
    pub character_name: String,
    pub selected_gender: usize,
    pub selected_class: usize,
    pub error_text: Option<String>,
    pub name_input_active: bool,
}

#[derive(Resource, Default)]
struct CharacterPortraitCatalog {
    warrior: Handle<Image>,
    ranger: Handle<Image>,
    mage: Handle<Image>,
    cleric: Handle<Image>,
}

impl CharacterPortraitCatalog {
    fn handle_for_class(&self, class_index: usize) -> &Handle<Image> {
        match class_index {
            0 => &self.warrior,
            1 => &self.ranger,
            2 => &self.mage,
            _ => &self.cleric,
        }
    }
}

const CHARACTER_GENDERS: [&str; 2] = ["Masculino", "Feminino"];
const CHARACTER_CLASSES: [&str; 4] = ["Guerreiro", "Ranger", "Mago", "Clerigo"];
const CREATION_STACK_BREAKPOINT: f32 = 900.0;
const NAME_MAX_LEN: usize = 24;

// UI - Components
#[derive(Component)]
struct CharacterCreationUiRoot;

#[derive(Component)]
struct CreationContentRow;

#[derive(Component)]
struct CreationActionsRow;

#[derive(Component)]
struct CreationPortraitCard;

#[derive(Component)]
struct NameInputButton;

#[derive(Component)]
struct NameValueText;

#[derive(Component)]
struct GenderValueText;

#[derive(Component)]
struct ClassValueText;

#[derive(Component)]
struct ErrorTextLabel;

#[derive(Component)]
struct PortraitImageNode;

#[derive(Component)]
struct PortraitStatusText;

#[derive(Component)]
struct PortraitClassText;

#[derive(Component, Clone, Copy)]
enum CreationButtonAction {
    Back,
    Begin,
    NameInput,
    GenderPrev,
    GenderNext,
    ClassPrev,
    ClassNext,
}

fn setup_character_creation(
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
            root.spawn(TextBundle::from_section(
                "Sala da Guilda",
                title_style.clone(),
            ));
            root.spawn(TextBundle::from_section(
                "Registre seu aventureiro antes de cruzar os portoes",
                subtitle_style.clone(),
            ));

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
                                    background_color: BackgroundColor(Color::srgba_u8(
                                        18, 12, 8, 220,
                                    )),
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
                                _spawn_action_button(
                                    actions,
                                    "Voltar ao Menu",
                                    CreationButtonAction::Back,
                                    180.0,
                                );
                                _spawn_action_button(
                                    actions,
                                    "Forjar Destino",
                                    CreationButtonAction::Begin,
                                    220.0,
                                );
                            });
                        });

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
                            portrait
                                .spawn(TextBundle::from_section("Retrato", label_style.clone()));

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
                                    background_color: BackgroundColor(Color::srgba_u8(
                                        18, 10, 6, 170,
                                    )),
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
                });
            });

            root.spawn(TextBundle::from_section(
                "ESC para voltar ao menu principal",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb_u8(152, 128, 102),
                    ..default()
                },
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

fn cleanup_character_creation_ui(
    mut commands: Commands,
    roots: Query<Entity, With<CharacterCreationUiRoot>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_creation_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_screen: ResMut<NextState<AppScreen>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_screen.set(AppScreen::MainMenu);
    }
}

fn handle_creation_button_interactions(
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

fn handle_creation_text_input(
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

fn sync_creation_ui(
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

// UI
fn animate_creation_background(time: Res<Time>, mut clear_color: ResMut<ClearColor>) {
    let t = time.elapsed_seconds();
    let pulse = ((t * theme::CREATION_BACKGROUND_SPEED).sin() + 1.0) * 0.5;
    clear_color.0 = theme::ember_background(pulse);
}

fn update_creation_layout(
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
