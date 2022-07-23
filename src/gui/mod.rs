mod widgets;
mod util;
mod settings;
mod main;
mod edit;

use druid::{Data, Event, Widget, WidgetExt, Lens, EventCtx, Env};
use druid::im::Vector;
use druid::widget::Controller;
use druid_enums::Matcher;
use crate::gui::main::{build_main_ui, OPEN_EDITOR, OPEN_SETTINGS};
use crate::gui::settings::{build_settings_ui, SETTINGS_SAVE, SettingsState};
use crate::gui::edit::{build_edit_ui, CLOSE_EDITOR_DISCARD, CLOSE_EDITOR_SAVE, EditState};

pub use main::MainState;


#[derive(Clone, Data, Lens)]
pub struct Settings {
    pub close_on_login: bool
}

#[derive(Clone, Data, Lens)]
pub struct Account {
    pub name: String
}

#[derive(Clone, Data, Lens)]
pub struct Database {
    pub accounts: Vector<Account>
}


#[derive(Clone, Data, Matcher)]
#[matcher(matcher_name = App)]
pub enum AppState {
    Settings(SettingsState),
    Main(MainState),
    Editor(EditState)
}

pub fn ui() -> impl Widget<AppState> {
    App::new()
        .main(build_main_ui())
        .settings(build_settings_ui())
        .editor(build_edit_ui())
        .controller(AppController)
}

struct AppController;
impl Controller<AppState, App> for AppController {
    fn event(&mut self, child: &mut App, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(OPEN_SETTINGS) => {
                let main_state= cmd.get_unchecked(OPEN_SETTINGS);
                *data = AppState::Settings(SettingsState {
                    previous: main_state.clone(),
                    settings: main_state.settings.clone()
                });
            },
            Event::Command(cmd) if cmd.is(OPEN_EDITOR) => {
                let main_state= cmd.get_unchecked(OPEN_EDITOR);
                *data = AppState::Editor(EditState {
                    previous: main_state.clone(),
                    database: main_state.database.clone()
                });
            },
            Event::Command(cmd) if cmd.is(SETTINGS_SAVE) => {
                let settings_state= cmd.get_unchecked(SETTINGS_SAVE);
                let mut main = settings_state.previous.clone();
                main.settings = settings_state.settings.clone();
                //? without this the searchbar becomes stuck
                main.filter.clear();
                *data = AppState::Main(main);
            },
            Event::Command(cmd) if cmd.is(CLOSE_EDITOR_SAVE) => {
                let edit_state= cmd.get_unchecked(CLOSE_EDITOR_SAVE);
                let mut main = edit_state.previous.clone();
                main.database = edit_state.database.clone();
                //? without this the searchbar becomes stuck
                main.filter.clear();
                *data = AppState::Main(main);
            },
            Event::Command(cmd) if cmd.is(CLOSE_EDITOR_DISCARD) => {
                let edit_state= cmd.get_unchecked(CLOSE_EDITOR_DISCARD);
                let mut main = edit_state.previous.clone();
                //? without this the searchbar becomes stuck
                main.filter.clear();
                *data = AppState::Main(main);
            },
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}


