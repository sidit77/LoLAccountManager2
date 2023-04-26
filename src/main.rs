#![windows_subsystem = "windows"]

mod data;
pub mod os;
mod screens;
mod util;
mod widgets;

use druid::{AppLauncher, LocalizedString, WindowDesc};

use crate::screens::{AppState, Screen};

pub fn main() {
    let window = WindowDesc::new(AppState::widget())
        .window_size((400.0, 600.0))
        .title(LocalizedString::new("scroll-demo-window-title").with_placeholder("LoL Account Manager"));
    AppLauncher::with_window(window)
        .log_to_console()
        //.launch(AppState::Setup(SetupState::new(Settings::load().unwrap())))
        .launch(AppState::load().unwrap())
        .expect("launch failed");
}
