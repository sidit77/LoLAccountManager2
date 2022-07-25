#![windows_subsystem = "windows"]

mod gui;
mod os;

use druid::{AppDelegate, AppLauncher, DelegateCtx, Env, LocalizedString, WindowDesc, WindowHandle, WindowId};
use crate::gui::{Account, AppState, Database, MainState, Settings, Theme, ui};

use crate::os::set_window_icon;

pub fn main() {
    let window = WindowDesc::new(ui())
        .window_size((400.0, 600.0))
        .title(LocalizedString::new("scroll-demo-window-title").with_placeholder("LoL Account Manager"));
    AppLauncher::with_window(window)
        .delegate(ProgramDelegate)
        .log_to_console()
        .launch(AppState::Main(MainState {
            settings: Settings { close_on_login: false, theme: Theme::Light },
            filter: "".to_string(),
            database: get_test_database()
        }))
        .expect("launch failed");
}

struct ProgramDelegate;

impl AppDelegate<AppState> for ProgramDelegate {
    fn window_added(&mut self, _id: WindowId, handle: WindowHandle, _data: &mut AppState, _env: &Env, _ctx: &mut DelegateCtx) {
        set_window_icon(&handle);
    }
}


pub fn get_test_database() -> Database {
    Database {
        accounts: NAMES.iter().map(|s| Account {
            name: s.to_string()
        }).collect()
    }
}

const NAMES: &[&str] = &[
    "Allegra",
    "Bree",
    "Bryna",
    "Carrissa",
    "Clair",
    "Cleopatra",
    "Corinna",
    "Dacia",
    "Dinah",
    "Dionis",
    "Ermentrude",
    "Felicdad",
    "Flori",
    "Geneva",
    "Gussie",
    "Jazmin",
    "Jeannette",
    "Jenine",
    "Joann",
    "Layney",
    "Leona",
    "Lizzy",
    "Lucita",
    "Lyndsey",
    "Marcelia",
    "Marlene",
    "Mirabelle",
    "Nanci",
    "Nyssa",
    "Ora",
    "Paula",
    "Phyllis",
    "Prissie",
    "Riannon",
    "Roby",
    "Salomi",
    "Shaylyn",
    "Shela",
    "Siobhan",
    "Sioux",
    "Susanetta",
    "Tallulah",
    "Tiffi",
    "Valentine",
    "Van",
    "Verine",
    "Wallie",
    "Yvonne",
];
