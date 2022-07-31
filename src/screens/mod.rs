mod account;
mod edit;
mod main;
mod popup;
mod settings;
mod setup;

use crate::data::{Database, Settings, Theme};
use crate::screens::account::{build_account_ui, AccountState, EditMode, CLOSE_ACCOUNT};
use crate::screens::edit::{build_edit_ui, EditState, CLOSE_EDITOR, OPEN_ACCOUNT};
use crate::screens::main::{build_main_ui, MainState, ACCOUNT_LOGIN, OPEN_EDITOR, OPEN_SETTINGS};
use crate::screens::popup::{build_popup_ui, PopupState, POPUP_CLOSE};
use crate::screens::settings::{build_settings_ui, SettingsState, SETTINGS_SAVE};
use crate::screens::setup::{build_setup_ui, SetupState, SETUP_CONFIRM};
use crate::util::theme::setup_theme;
use druid::theme::BACKGROUND_DARK;
use druid::widget::Controller;
use druid::{Data, Env, Event, EventCtx, Widget, WidgetExt};
use druid_widget_nursery::enum_switcher::Switcher;
use druid_widget_nursery::prism::Prism;
use std::ops::{Deref, IndexMut};
use std::sync::Arc;

#[derive(Clone, Data, Prism)]
pub enum AppState {
    Settings(SettingsState),
    Main(MainState),
    Editor(EditState),
    Account(AccountState),
    Setup(SetupState),
    Popup(PopupState),
}

impl AppState {
    pub fn load() -> anyhow::Result<AppState> {
        let settings = Settings::load()?;
        Ok(match settings.last_database.clone() {
            Some(path) => AppState::Main(MainState {
                settings,
                filter: "".to_string(),
                database: Database::load(&path, "").unwrap(),
            }),
            None => AppState::Setup(SetupState::new(settings)),
        })
    }

    fn current_theme(&self) -> Theme {
        match self {
            AppState::Settings(state) => state.settings.theme,
            AppState::Main(state) => state.settings.theme,
            AppState::Editor(state) => state.previous.settings.theme,
            AppState::Account(state) => state.previous.previous.settings.theme,
            AppState::Setup(state) => state.settings.theme,
            AppState::Popup(state) => state.previous.current_theme(),
        }
    }
}

pub fn ui() -> impl Widget<AppState> {
    Switcher::new()
        .with_variant(AppStateMain, build_main_ui())
        .with_variant(AppStateSettings, build_settings_ui())
        .with_variant(AppStateEditor, build_edit_ui())
        .with_variant(AppStateAccount, build_account_ui())
        .with_variant(AppStateSetup, build_setup_ui())
        .with_variant(AppStatePopup, build_popup_ui())
        .controller(AppController)
        .background(BACKGROUND_DARK)
        .env_scope(|env, state: &AppState| setup_theme(state.current_theme(), env))
}

struct AppController;
impl Controller<AppState, Switcher<AppState>> for AppController {
    fn event(&mut self, child: &mut Switcher<AppState>, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(ACCOUNT_LOGIN) => {
                let acc = cmd.get_unchecked(ACCOUNT_LOGIN);
                let new = PopupState {
                    previous: Arc::new(data.clone()),
                    message: format!("{:#?}", acc),
                };
                *data = AppState::Popup(new);
                ctx.children_changed()
            }
            Event::Command(cmd) if cmd.is(OPEN_SETTINGS) => {
                let main_state = cmd.get_unchecked(OPEN_SETTINGS);
                *data = AppState::Settings(SettingsState {
                    previous: main_state.clone(),
                    settings: main_state.settings.clone(),
                });
                ctx.children_changed()
            }
            Event::Command(cmd) if cmd.is(OPEN_EDITOR) => {
                let main_state = cmd.get_unchecked(OPEN_EDITOR);
                *data = AppState::Editor(EditState {
                    previous: main_state.clone(),
                    database: main_state.database.clone(),
                });
                ctx.children_changed()
            }
            Event::Command(cmd) if cmd.is(SETTINGS_SAVE) => {
                let settings_state = cmd.get_unchecked(SETTINGS_SAVE);
                settings_state.settings.save().unwrap();
                let mut main = settings_state.previous.clone();
                main.settings = settings_state.settings.clone();
                *data = AppState::Main(main);
                ctx.children_changed()
            }
            Event::Command(cmd) if cmd.is(CLOSE_EDITOR) => {
                let (state, save) = cmd.get_unchecked(CLOSE_EDITOR);
                let mut main = state.previous.clone();
                if *save {
                    let db = state.database.clone();
                    db.save().unwrap();
                    main.database = db;
                }
                *data = AppState::Main(main);
                ctx.children_changed()
            }
            Event::Command(cmd) if cmd.is(OPEN_ACCOUNT) => {
                let state = cmd.get_unchecked(OPEN_ACCOUNT);
                *data = AppState::Account(state.clone());
                ctx.children_changed()
            }
            Event::Command(cmd) if cmd.is(CLOSE_ACCOUNT) => {
                let (state, save) = cmd.get_unchecked(CLOSE_ACCOUNT);
                let mut new = state.previous.clone();
                if *save {
                    match state.mode {
                        EditMode::New => new.database.accounts.push_back(state.account.clone()),
                        EditMode::Existing(index) => {
                            *new.database.accounts.index_mut(index) = state.account.clone()
                        }
                    };
                }
                *data = AppState::Editor(new);
                ctx.children_changed()
            }
            Event::Command(cmd) if cmd.is(SETUP_CONFIRM) => {
                let state = cmd.get_unchecked(SETUP_CONFIRM);
                let new = MainState {
                    settings: state.settings.clone(),
                    filter: "".to_string(),
                    database: Default::default(),
                };
                *data = AppState::Main(new);
                ctx.children_changed()
            }
            Event::Command(cmd) if cmd.is(POPUP_CLOSE) => {
                let state = cmd.get_unchecked(POPUP_CLOSE);
                *data = state.previous.deref().clone();
                ctx.children_changed()
            }
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}
