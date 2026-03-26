use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::{self, Align, Layout, RichText, Vec2};

use crate::game::states::AppScreen;
use crate::game::ui::theme;

// Plugin
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainMenuState>()
            .add_systems(OnEnter(AppScreen::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                (animate_menu_background, handle_menu_input, draw_main_menu)
                    .run_if(in_state(AppScreen::MainMenu)),
            );
    }
}

// States
#[derive(Resource, Default)]
struct MainMenuState {
    selected: usize,
    hint: Option<String>,
}

#[derive(Clone, Copy)]
enum MenuItem {
    NewGame,
    LoadGame,
    Options,
    Quit,
}

impl MenuItem {
    const ALL: [Self; 4] = [Self::NewGame, Self::LoadGame, Self::Options, Self::Quit];

    fn label(self) -> &'static str {
        match self {
            MenuItem::NewGame => "Novo Jogo",
            MenuItem::LoadGame => "Carregar Jornada",
            MenuItem::Options => "Opções",
            MenuItem::Quit => "Sair",
        }
    }

    fn description(self) -> &'static str {
        match self {
            MenuItem::NewGame => "Inicie uma nova aventura no reino de Kvojps.",
            MenuItem::LoadGame => "Retome a jornada a partir do último acampamento.",
            MenuItem::Options => "Ajuste áudio, controles e preferências visuais.",
            MenuItem::Quit => "Fechar o jogo e voltar ao desktop.",
        }
    }
}

impl MainMenuState {
    fn selected_item(&self) -> MenuItem {
        MenuItem::ALL[self.selected]
    }
}

fn setup_main_menu(mut menu_state: ResMut<MainMenuState>) {
    menu_state.selected = 0;
    menu_state.hint = None;
}

fn handle_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MainMenuState>,
    mut next_screen: ResMut<NextState<AppScreen>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
        _select_previous_item(&mut menu_state);
    }

    if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        _select_next_item(&mut menu_state);
    }

    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        _activate_selected_item(&mut menu_state, &mut next_screen, &mut app_exit_events);
    }
}

fn _select_previous_item(menu_state: &mut MainMenuState) {
    menu_state.selected = if menu_state.selected == 0 {
        MenuItem::ALL.len() - 1
    } else {
        menu_state.selected - 1
    };
}

fn _select_next_item(menu_state: &mut MainMenuState) {
    menu_state.selected = (menu_state.selected + 1) % MenuItem::ALL.len();
}

fn _activate_selected_item(
    menu_state: &mut MainMenuState,
    next_screen: &mut NextState<AppScreen>,
    app_exit_events: &mut EventWriter<AppExit>,
) {
    match menu_state.selected_item() {
        MenuItem::NewGame => {
            next_screen.set(AppScreen::CharacterCreation);
        }
        MenuItem::LoadGame => {
            menu_state.hint =
                Some("Sistema de save/load será adicionado na proxima etapa.".to_string());
        }
        MenuItem::Options => {
            menu_state.hint =
                Some("Menu de opções será implementado após o gameplay base.".to_string());
        }
        MenuItem::Quit => {
            app_exit_events.send(AppExit::Success);
        }
    }
}

// UI
fn animate_menu_background(time: Res<Time>, mut clear_color: ResMut<ClearColor>) {
    let t = time.elapsed_seconds();
    let pulse = ((t * theme::MENU_BACKGROUND_SPEED).sin() + 1.0) * 0.5;
    clear_color.0 = theme::ember_background(pulse);
}

fn draw_main_menu(
    mut egui_contexts: EguiContexts,
    mut menu_state: ResMut<MainMenuState>,
    mut next_screen: ResMut<NextState<AppScreen>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    let selected_item = menu_state.selected_item();

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(theme::surface_fill()))
        .show(egui_contexts.ctx_mut(), |ui| {
            _draw_menu_header(ui);
            if let Some(clicked_index) = _draw_menu_options(ui, menu_state.selected) {
                menu_state.selected = clicked_index;
                _activate_selected_item(&mut menu_state, &mut next_screen, &mut app_exit_events);
            }
            _draw_menu_description(ui, selected_item, menu_state.hint.as_deref());
            _draw_menu_footer(ui);
        });
}

fn _draw_menu_header(ui: &mut egui::Ui) {
    ui.add_space(40.0);
    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new("Kvojps World")
                .size(56.0)
                .strong()
                .color(theme::title_gold()),
        );

        ui.add_space(4.0);
        ui.label(
            RichText::new("Uma aventura RPG de sobrevivência e exploração")
                .size(18.0)
                .italics()
                .color(theme::subtitle_parchment()),
        );
    });
}

fn _draw_menu_options(ui: &mut egui::Ui, selected_index: usize) -> Option<usize> {
    let mut clicked_index = None;

    ui.add_space(50.0);

    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        egui::Frame::none()
            .fill(theme::panel_fill())
            .stroke(egui::Stroke::new(1.8, theme::panel_stroke()))
            .rounding(egui::Rounding::same(10.0))
            .inner_margin(egui::Margin::same(18.0))
            .show(ui, |ui| {
                ui.set_width(430.0);

                for (index, item) in MenuItem::ALL.iter().enumerate() {
                    let is_selected = index == selected_index;

                    let color = if is_selected {
                        theme::selection_gold()
                    } else {
                        theme::text_primary()
                    };

                    let marker = if is_selected { "  >" } else { "   " };

                    let response = ui.add_sized(
                        Vec2::new(400.0, 38.0),
                        egui::Button::new(
                            RichText::new(format!("{marker} {}", item.label()))
                                .size(28.0)
                                .color(color)
                                .family(egui::FontFamily::Proportional),
                        )
                        .fill(theme::surface_fill())
                        .stroke(egui::Stroke::NONE),
                    );

                    if response.clicked() {
                        clicked_index = Some(index);
                    }
                }
            });
    });

    clicked_index
}

fn _draw_menu_description(ui: &mut egui::Ui, selected_item: MenuItem, hint: Option<&str>) {
    ui.add_space(24.0);
    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new(selected_item.description())
                .size(17.0)
                .color(theme::text_body()),
        );

        if let Some(hint_text) = hint {
            ui.add_space(8.0);
            ui.label(
                RichText::new(hint_text)
                    .size(15.0)
                    .color(theme::accent_ember()),
            );
        }
    });
}

fn _draw_menu_footer(ui: &mut egui::Ui) {
    ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
        ui.add_space(18.0);
        ui.label(
            RichText::new("Use W/S ou setas para navegar | Enter ou clique para confirmar")
                .size(14.0)
                .color(theme::text_muted()),
        );
    });
}
