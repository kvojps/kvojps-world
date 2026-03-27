// Refactor
mod app;

// Refactored
mod main_menu;

// To refactor
mod gameplay;
mod plugin;
mod states;
mod ui;
mod world;

pub fn run() {
    app::build_app().run();
}
