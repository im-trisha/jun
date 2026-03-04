#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod language;
mod message;
mod state;
mod views;

use iced::Theme;
pub use language::Language;
pub use message::Message;
pub use state::{App, Dependencies};

fn main() -> iced::Result {
    fn build_app() -> App {
        let deps = Dependencies {};
        let t = Language::En;
        App::new(deps, t)
    }

    iced::application(build_app, App::update, App::view)
        .centered()
        .decorations(true)
        .title(App::title)
        .theme(Theme::Oxocarbon)
        .run()
}
