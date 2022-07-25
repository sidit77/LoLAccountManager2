mod widgets;
mod util;
mod settings;
mod main;
mod edit;
mod account;
mod theme;

use std::ops::IndexMut;
use druid::{Data, Event, Widget, WidgetExt, Lens, EventCtx, Env};
use druid::im::Vector;
use druid::theme::BACKGROUND_DARK;
use druid::widget::Controller;
use druid_enums::Matcher;
use crate::gui::main::{build_main_ui, OPEN_EDITOR, OPEN_SETTINGS};
use crate::gui::settings::{build_settings_ui, SETTINGS_SAVE, SettingsState};
use crate::gui::edit::{build_edit_ui, CLOSE_EDITOR, OPEN_ACCOUNT, EditState};
use crate::gui::account::{AccountState, build_account_ui, CLOSE_ACCOUNT, EditMode};


pub use main::MainState;
pub use theme::{Theme};


#[derive(Clone, Data, Lens)]
pub struct Settings {
    pub close_on_login: bool,
    pub theme: Theme
}

#[derive(Clone, Default, Data, Lens)]
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
    Editor(EditState),
    Account(AccountState)
}

impl AppState {
    fn current_theme(&self) -> Theme {
        match self {
            AppState::Settings(state) => state.settings.theme,
            AppState::Main(state) => state.settings.theme,
            AppState::Editor(state) => state.previous.settings.theme,
            AppState::Account(state) => state.previous.previous.settings.theme
        }
    }
}

pub fn ui() -> impl Widget<AppState> {
    App::new()
        .main(build_main_ui())
        .settings(build_settings_ui())
        .editor(build_edit_ui())
        .account(build_account_ui())
        .controller(AppController)
        .background(BACKGROUND_DARK)
        .env_scope(|env,state: &AppState| state.current_theme().setup(env))
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
                ctx.children_changed()
            },
            Event::Command(cmd) if cmd.is(OPEN_EDITOR) => {
                let main_state= cmd.get_unchecked(OPEN_EDITOR);
                *data = AppState::Editor(EditState {
                    previous: main_state.clone(),
                    database: main_state.database.clone()
                });
                ctx.children_changed()
            },
            Event::Command(cmd) if cmd.is(SETTINGS_SAVE) => {
                let settings_state= cmd.get_unchecked(SETTINGS_SAVE);
                let mut main = settings_state.previous.clone();
                main.settings = settings_state.settings.clone();
                *data = AppState::Main(main);
                ctx.children_changed()
            },
            Event::Command(cmd) if cmd.is(CLOSE_EDITOR) => {
                let (state, save) = cmd.get_unchecked(CLOSE_EDITOR);
                let mut main = state.previous.clone();
                if *save {
                    main.database = state.database.clone();
                }
                *data = AppState::Main(main);
                ctx.children_changed()
            },
            Event::Command(cmd) if cmd.is(OPEN_ACCOUNT) => {
                let state= cmd.get_unchecked(OPEN_ACCOUNT);
                *data = AppState::Account(state.clone());
                ctx.children_changed()
            },
            Event::Command(cmd) if cmd.is(CLOSE_ACCOUNT) => {
                let (state, save)= cmd.get_unchecked(CLOSE_ACCOUNT);
                let mut new = state.previous.clone();
                if *save {
                    match state.mode {
                        EditMode::New => new.database.accounts.push_back(state.account.clone()),
                        EditMode::Existing(index) => *new.database.accounts.index_mut(index) = state.account.clone()
                    };
                }
                *data = AppState::Editor(new);
                ctx.children_changed()
            },
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}


