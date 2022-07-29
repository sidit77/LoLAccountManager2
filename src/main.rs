#![windows_subsystem = "windows"]

mod gui;
mod os;

use druid::{AppDelegate, AppLauncher, DelegateCtx, Env, LocalizedString, WindowDesc, WindowHandle, WindowId};
use crate::gui::{Account, AppState, Database, MainState, SetupState, Settings, Theme, ui};

use crate::os::set_window_icon;

pub fn main() {
    let window = WindowDesc::new(ui())
        .window_size((400.0, 600.0))
        .title(LocalizedString::new("scroll-demo-window-title").with_placeholder("LoL Account Manager"));
    AppLauncher::with_window(window)
        .delegate(ProgramDelegate)
        .log_to_console()
        //.launch(AppState::Setup(SetupState::new(Settings::load().unwrap())))
        .launch(create_state().unwrap())
        .expect("launch failed");
}

pub fn create_state() -> anyhow::Result<AppState> {
    let settings = Settings::load()?;
    Ok(match settings.last_database.clone() {
        Some(path) => AppState::Main(MainState {
            settings,
            filter: "".to_string(),
            database: Database::load(&path).unwrap()
        }),
        None => AppState::Setup(SetupState::new(settings))
    })
}

struct ProgramDelegate;

impl AppDelegate<AppState> for ProgramDelegate {
    fn window_added(&mut self, _id: WindowId, handle: WindowHandle, _data: &mut AppState, _env: &Env, _ctx: &mut DelegateCtx) {
        set_window_icon(&handle);
    }
}
