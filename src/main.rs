#![windows_subsystem = "windows"]

mod screens;
mod os;
mod data;
mod util;
mod widgets;

use druid::{AppDelegate, AppLauncher, DelegateCtx, Env, LocalizedString, WindowDesc, WindowHandle, WindowId};
use crate::screens::{AppState, ui};
use crate::os::set_window_icon;

pub fn main() {
    let window = WindowDesc::new(ui())
        .window_size((400.0, 600.0))
        .title(LocalizedString::new("scroll-demo-window-title").with_placeholder("LoL Account Manager"));
    AppLauncher::with_window(window)
        .delegate(ProgramDelegate)
        .log_to_console()
        //.launch(AppState::Setup(SetupState::new(Settings::load().unwrap())))
        .launch(AppState::load().unwrap())
        .expect("launch failed");
}

struct ProgramDelegate;

impl AppDelegate<AppState> for ProgramDelegate {
    fn window_added(&mut self, _id: WindowId, handle: WindowHandle, _data: &mut AppState, _env: &Env, _ctx: &mut DelegateCtx) {
        set_window_icon(&handle);
    }
}
