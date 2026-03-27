mod app;
mod gameplay;
mod plugin;
mod states;
mod ui;
mod world;

pub fn run() {
    app::build_app().run();
}
