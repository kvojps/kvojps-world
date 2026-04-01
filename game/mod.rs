mod app;
mod character_creation;
mod main_menu;
mod plugin;
mod states;

pub fn run() {
    app::build_app().run();
}
