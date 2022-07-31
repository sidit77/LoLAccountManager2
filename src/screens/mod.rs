mod account;
mod edit;
mod main;
mod popup;
mod settings;
mod setup;

use crate::data::{Database, Settings, Password};
use crate::screens::account::{AccountState};
use crate::screens::edit::{EditState};
use crate::screens::main::{ACCOUNT_LOGIN, MainState};
use crate::screens::popup::{PopupState};
use crate::screens::settings::{SettingsState};
use crate::screens::setup::{SetupState};
use crate::util::theme::setup_theme;
use druid::theme::BACKGROUND_DARK;
use druid::widget::Controller;
use druid::{Application, Data, Env, Event, EventCtx, Selector, Widget, WidgetExt};
use druid_widget_nursery::enum_switcher::Switcher;
use druid_widget_nursery::prism::Prism;
use crate::os;

pub const NAVIGATE: Selector<AppState> = Selector::new("lol_account_manager_v2.navigate");

pub trait Screen : Into<AppState> {

    fn back(&mut self, ctx: &mut EventCtx, save: bool) {
        if save {
            self.make_permanent().unwrap();
        }
        if let Some(previous) = self.previous() {
            ctx.submit_command(NAVIGATE.with(previous))
        }
    }

    fn open(&self, ctx: &mut EventCtx, screen: impl Into<AppState>) {
        ctx.submit_command(NAVIGATE.with(screen.into()))
    }

    fn widget() -> Box<dyn Widget<Self>>;

    fn settings(&self) -> Settings;

    fn previous(&self) -> Option<AppState> {
        None
    }

    fn make_permanent(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Data, Prism)]
pub enum AppState {
    Main(MainState),
    Settings(SettingsState),
    Editor(EditState),
    Account(AccountState),
    Setup(SetupState),
    Popup(PopupState),
}

impl Screen for AppState {
    fn widget() -> Box<dyn Widget<Self>> {
        Box::new(ui())
    }

    fn settings(&self) -> Settings {
        match self {
            AppState::Main(state) => state.settings(),
            AppState::Settings(state) => state.settings(),
            AppState::Editor(state) => state.settings(),
            AppState::Account(state) => state.settings(),
            AppState::Setup(state) => state.settings(),
            AppState::Popup(state) => state.settings()
        }
    }

    fn previous(&self) -> Option<AppState> {
        match self {
            AppState::Main(state) => state.previous(),
            AppState::Settings(state) => state.previous(),
            AppState::Editor(state) => state.previous(),
            AppState::Account(state) => state.previous(),
            AppState::Setup(state) => state.previous(),
            AppState::Popup(state) => state.previous()
        }
    }

    fn make_permanent(&mut self) -> anyhow::Result<()> {
        match self {
            AppState::Main(state) => state.make_permanent(),
            AppState::Settings(state) => state.make_permanent(),
            AppState::Editor(state) => state.make_permanent(),
            AppState::Account(state) => state.make_permanent(),
            AppState::Setup(state) => state.make_permanent(),
            AppState::Popup(state) => state.make_permanent()
        }
    }
}

impl AppState {

    pub fn load() -> anyhow::Result<AppState> {
        let settings = Settings::load()?;
        Ok(match settings.last_database.clone() {
            Some(path) => {
                let password = Password::get()?;
                AppState::Main(MainState::new(
                    settings,
                    Database::load(&path, &password)?
                ))
            },
            None => AppState::Setup(SetupState::new(settings)),
        })
    }

}

fn ui() -> impl Widget<AppState> {
    Switcher::new()
        .with_variant(AppStateMain, MainState::widget())
        .with_variant(AppStateSettings, SettingsState::widget())
        .with_variant(AppStateEditor, EditState::widget())
        .with_variant(AppStateAccount, AccountState::widget())
        .with_variant(AppStateSetup, SetupState::widget())
        .with_variant(AppStatePopup, PopupState::widget())
        .controller(AppController)
        .background(BACKGROUND_DARK)
        .env_scope(|env, state: &AppState| setup_theme(state.settings().theme, env))
}

struct AppController;
impl Controller<AppState, Switcher<AppState>> for AppController {
    fn event(&mut self, child: &mut Switcher<AppState>, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        if let Event::Command(cmd) = event {
            if let Some(state) = cmd.get(NAVIGATE).cloned(){
                *data = state;
            }
            if let Some(acc) = cmd.get(ACCOUNT_LOGIN).cloned() {
                //let new = PopupState {
                //    previous: Arc::new(data.clone()),
                //    message: format!("{:#?}", acc),
                //};
                //*data = AppState::Popup(new);
                os::login_account(&acc).unwrap();
                if data.settings().close_on_login {
                    Application::global().quit();
                }
            }
        }
        child.event(ctx, event, data, env)
    }
}
