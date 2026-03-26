use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_egui::egui::{self, Align, Frame, Layout, RichText, Stroke};

use crate::game::states::AppScreen;
use crate::game::ui::theme;

// Plugin
pub struct CharacterCreationPlugin;

impl Plugin for CharacterCreationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterCreationState>()
            .add_systems(
                OnEnter(AppScreen::CharacterCreation),
                setup_character_creation,
            )
            .add_systems(
                Update,
                (
                    animate_creation_background,
                    handle_creation_input,
                    draw_character_creation,
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
}

const CHARACTER_GENDERS: [&str; 2] = ["Masculino", "Feminino"];
const CHARACTER_CLASSES: [&str; 4] = ["Guerreiro", "Ranger", "Mago", "Clerigo"];

fn setup_character_creation(mut state: ResMut<CharacterCreationState>) {
    state.error_text = None;
    if state.character_name.is_empty() {
        state.selected_gender = 0;
        state.selected_class = 0;
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

// UI
fn animate_creation_background(time: Res<Time>, mut clear_color: ResMut<ClearColor>) {
    let t = time.elapsed_seconds();
    let pulse = ((t * theme::CREATION_BACKGROUND_SPEED).sin() + 1.0) * 0.5;
    clear_color.0 = theme::ember_background(pulse);
}

fn draw_character_creation(
    mut egui_contexts: EguiContexts,
    mut state: ResMut<CharacterCreationState>,
    mut next_screen: ResMut<NextState<AppScreen>>,
) {
    let ctx = egui_contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(Frame::none().fill(theme::surface_fill()))
        .show(ctx, |ui| {
            _draw_creation_header(ui);
            _draw_creation_form_panel(ui, &mut state, &mut next_screen);
            _draw_creation_footer(ui);
        });
}

fn _draw_creation_header(ui: &mut egui::Ui) {
    ui.add_space(16.0);
    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new("Sala da Guilda")
                .size(50.0)
                .strong()
                .color(theme::title_gold()),
        );
        ui.label(
            RichText::new("Registre seu aventureiro antes de cruzar os portoes")
                .size(17.0)
                .color(theme::subtitle_parchment()),
        );
    });
}

fn _draw_creation_form_panel(
    ui: &mut egui::Ui,
    state: &mut CharacterCreationState,
    next_screen: &mut NextState<AppScreen>,
) {
    ui.add_space(28.0);

    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        Frame::none()
            .fill(theme::panel_fill())
            .stroke(Stroke::new(2.0, theme::panel_stroke()))
            .rounding(egui::Rounding::same(8.0))
            .inner_margin(egui::Margin::same(22.0))
            .show(ui, |ui| {
                ui.set_width(560.0);

                _draw_creation_identity_fields(ui, state);
                _draw_creation_error(ui, state.error_text.as_deref());
                _draw_creation_actions(ui, state, next_screen);
            });
    });
}

fn _draw_creation_identity_fields(ui: &mut egui::Ui, state: &mut CharacterCreationState) {
    ui.label(
        RichText::new("Nome do personagem")
            .size(15.0)
            .color(theme::text_primary()),
    );
    ui.add_space(4.0);
    ui.add_sized(
        [520.0, 30.0],
        egui::TextEdit::singleline(&mut state.character_name),
    );

    ui.add_space(14.0);
    ui.columns(2, |columns| {
        columns[0].label(
            RichText::new("Genero")
                .size(15.0)
                .color(theme::text_primary()),
        );
        egui::ComboBox::from_id_source("guild_gender")
            .selected_text(CHARACTER_GENDERS[state.selected_gender])
            .width(240.0)
            .show_ui(&mut columns[0], |ui| {
                for (index, label) in CHARACTER_GENDERS.iter().enumerate() {
                    ui.selectable_value(&mut state.selected_gender, index, *label);
                }
            });

        columns[1].label(
            RichText::new("Classe")
                .size(15.0)
                .color(theme::text_primary()),
        );
        egui::ComboBox::from_id_source("guild_class")
            .selected_text(CHARACTER_CLASSES[state.selected_class])
            .width(240.0)
            .show_ui(&mut columns[1], |ui| {
                for (index, label) in CHARACTER_CLASSES.iter().enumerate() {
                    ui.selectable_value(&mut state.selected_class, index, *label);
                }
            });
    });
}

fn _draw_creation_error(ui: &mut egui::Ui, error_text: Option<&str>) {
    ui.add_space(12.0);
    if let Some(error_text) = error_text {
        ui.label(
            RichText::new(error_text)
                .size(14.0)
                .color(theme::error_red()),
        );
    }
}

fn _draw_creation_actions(
    ui: &mut egui::Ui,
    state: &mut CharacterCreationState,
    next_screen: &mut NextState<AppScreen>,
) {
    ui.add_space(16.0);
    ui.horizontal(|ui| {
        let back_clicked = ui
            .add_sized([160.0, 34.0], egui::Button::new("Voltar ao Menu"))
            .clicked();
        let begin_clicked = ui
            .add_sized([230.0, 34.0], egui::Button::new("Forjar Destino"))
            .clicked();

        if back_clicked {
            state.error_text = None;
            next_screen.set(AppScreen::MainMenu);
        }

        if begin_clicked {
            if state.character_name.trim().is_empty() {
                state.error_text = Some("Informe um nome para iniciar a jornada.".to_string());
            } else {
                state.error_text = None;
                next_screen.set(AppScreen::Overworld);
            }
        }
    });
}

fn _draw_creation_footer(ui: &mut egui::Ui) {
    ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
        ui.add_space(8.0);
        ui.label(
            RichText::new("ESC para voltar ao menu principal")
                .size(13.0)
                .color(theme::text_muted()),
        );
    });
}
