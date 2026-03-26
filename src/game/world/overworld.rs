use bevy::prelude::*;
use bevy_egui::egui::{self, Align, Color32, Layout, RichText};
use bevy_egui::EguiContexts;

use crate::game::states::AppScreen;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, return_to_menu.run_if(in_state(AppScreen::Overworld)))
            .add_systems(Update, draw_overworld_overlay.run_if(in_state(AppScreen::Overworld)));
    }
}

fn return_to_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_screen: ResMut<NextState<AppScreen>>,
    mut clear_color: ResMut<ClearColor>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        clear_color.0 = Color::srgb(0.04, 0.03, 0.06);
        next_screen.set(AppScreen::MainMenu);
    }
}

fn draw_overworld_overlay(mut egui_contexts: EguiContexts) {
    egui::TopBottomPanel::top("hud_top").show(egui_contexts.ctx_mut(), |ui| {
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Asterfall Frontier")
                    .size(20.0)
                    .strong()
                    .color(Color32::from_rgb(224, 206, 145)),
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    RichText::new("ESC para voltar ao menu")
                        .size(14.0)
                        .color(Color32::from_rgb(150, 160, 182)),
                );
            });
        });
        ui.add_space(6.0);
    });

    egui::Area::new("overworld_tip".into())
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_contexts.ctx_mut(), |ui| {
            egui::Frame::none()
                .fill(Color32::from_rgba_unmultiplied(18, 24, 35, 212))
                .stroke(egui::Stroke::new(1.2, Color32::from_rgb(98, 124, 160)))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            RichText::new("Template inicial carregado")
                                .size(22.0)
                                .color(Color32::from_rgb(209, 220, 241)),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new("Proximo passo: tilemap, player e colisao.")
                                .size(16.0)
                                .color(Color32::from_rgb(152, 172, 200)),
                        );
                    });
                });
        });
}
