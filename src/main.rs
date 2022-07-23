#![windows_subsystem = "windows"]

mod gui;

use druid::{AppLauncher, Color, LocalizedString, theme, WindowDesc};
use crate::gui::{Account, AppState, Database, MainState, Settings, ui};

pub fn main() {
    let window = WindowDesc::new(ui())
        .window_size((400.0, 600.0))
        .title(LocalizedString::new("scroll-demo-window-title").with_placeholder("Scroll demo"));
    AppLauncher::with_window(window)
        .log_to_console()
        .configure_env(|env, _| {
            env.set(theme::BUTTON_DARK, Color::AQUA);
            env.set(theme::BUTTON_LIGHT, Color::AQUA);
            env.set(theme::DISABLED_BUTTON_DARK, Color::WHITE);
            env.set(theme::DISABLED_BUTTON_LIGHT, Color::WHITE);
            env.set(theme::TEXT_COLOR, Color::BLACK);
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::WHITE);
            env.set(theme::BACKGROUND_LIGHT, Color::WHITE);
            env.set(theme::CURSOR_COLOR, Color::BLACK);
            env.set(theme::TEXTBOX_BORDER_WIDTH, 0.0);

        })
        .launch(AppState::Main(MainState {
            settings: Settings { close_on_login: false },
            filter: "".to_string(),
            database: get_test_database()
        }))
        .expect("launch failed");
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
