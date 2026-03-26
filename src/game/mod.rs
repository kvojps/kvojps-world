mod app;
mod plugin;
mod states;
mod ui;
mod world;

pub fn run() {
    app::build_app().run();
}
