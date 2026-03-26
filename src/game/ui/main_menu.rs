use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::{self, Align, Color32, Layout, RichText, Vec2};

use crate::game::states::AppScreen;

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
            next_screen.set(AppScreen::Overworld);
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
    let pulse = ((t * 0.35).sin() + 1.0) * 0.5;

    let r = 0.03 + 0.02 * pulse;
    let g = 0.02 + 0.015 * pulse;
    let b = 0.06 + 0.03 * pulse;

    clear_color.0 = Color::srgb(r, g, b);
}

fn draw_main_menu(mut egui_contexts: EguiContexts, menu_state: Res<MainMenuState>) {
    let selected_item = menu_state.selected_item();

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(Color32::from_rgba_unmultiplied(5, 5, 10, 0)))
        .show(egui_contexts.ctx_mut(), |ui| {
            _draw_menu_header(ui);
            _draw_menu_options(ui, menu_state.selected);
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
                .color(Color32::from_rgb(224, 206, 145)),
        );

        ui.add_space(4.0);
        ui.label(
            RichText::new("Uma aventura RPG de sobrevivência e exploração")
                .size(18.0)
                .italics()
                .color(Color32::from_rgb(152, 164, 189)),
        );
    });
}

fn _draw_menu_options(ui: &mut egui::Ui, selected_index: usize) {
    ui.add_space(50.0);

    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        egui::Frame::none()
            .fill(Color32::from_rgba_unmultiplied(12, 14, 24, 220))
            .stroke(egui::Stroke::new(1.5, Color32::from_rgb(181, 153, 91)))
            .rounding(egui::Rounding::same(10.0))
            .inner_margin(egui::Margin::same(18.0))
            .show(ui, |ui| {
                ui.set_width(430.0);

                for (index, item) in MenuItem::ALL.iter().enumerate() {
                    let is_selected = index == selected_index;

                    let color = if is_selected {
                        Color32::from_rgb(255, 225, 138)
                    } else {
                        Color32::from_rgb(196, 205, 224)
                    };

                    let marker = if is_selected { "  >" } else { "   " };

                    ui.add_sized(
                        Vec2::new(400.0, 38.0),
                        egui::Label::new(
                            RichText::new(format!("{marker} {}", item.label()))
                                .size(28.0)
                                .color(color)
                                .family(egui::FontFamily::Proportional),
                        ),
                    );
                }
            });
    });
}

fn _draw_menu_description(ui: &mut egui::Ui, selected_item: MenuItem, hint: Option<&str>) {
    ui.add_space(24.0);
    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new(selected_item.description())
                .size(17.0)
                .color(Color32::from_rgb(157, 179, 196)),
        );

        if let Some(hint_text) = hint {
            ui.add_space(8.0);
            ui.label(
                RichText::new(hint_text)
                    .size(15.0)
                    .color(Color32::from_rgb(232, 172, 116)),
            );
        }
    });
}

fn _draw_menu_footer(ui: &mut egui::Ui) {
    ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
        ui.add_space(18.0);
        ui.label(
            RichText::new("Use W/S ou setas para navegar | Enter para confirmar")
                .size(14.0)
                .color(Color32::from_rgb(122, 131, 157)),
        );
    });
}
