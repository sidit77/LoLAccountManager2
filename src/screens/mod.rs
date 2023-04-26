mod account;
mod edit;
mod main;
mod settings;
mod setup;
mod start;

use druid::theme::BACKGROUND_DARK;
use druid::widget::Controller;
use druid::{Application, Data, Env, Event, EventCtx, Lens, LifeCycleCtx, Selector, Widget, WidgetExt};
use druid_widget_nursery::enum_switcher::Switcher;
use druid_widget_nursery::prism::Prism;

use crate::data::Settings;
use crate::os;
use crate::screens::account::AccountState;
use crate::screens::edit::EditState;
use crate::screens::main::{MainState, ACCOUNT_LOGIN};
use crate::screens::settings::SettingsState;
use crate::screens::setup::SetupState;
use crate::screens::start::StartupState;
use crate::util::theme::setup_theme;

pub const NAVIGATE: Selector<AppState> = Selector::new("lol_account_manager_v2.navigate");

pub trait Screen: Into<AppState> {
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

    fn open_lifecycle(&self, ctx: &mut LifeCycleCtx, screen: impl Into<AppState>) {
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

#[derive(Clone, Data, Lens)]
pub struct MainUi {
    pub state: AppState
}

impl MainUi {

    pub fn new() -> MainUi {
        MainUi {
            state: StartupState::new().into()//AppState::load().unwrap(),
        }
    }
    
    pub fn widget() -> impl Widget<MainUi> + 'static {
        AppState::widget()
            .lens(MainUi::state)
    }

}


#[derive(Clone, Data, Prism)]
pub enum AppState {
    Start(StartupState),
    Main(MainState),
    Settings(SettingsState),
    Editor(EditState),
    Account(AccountState),
    Setup(SetupState),
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
            AppState::Start(state) => state.settings()
        }
    }

    fn previous(&self) -> Option<AppState> {
        match self {
            AppState::Main(state) => state.previous(),
            AppState::Settings(state) => state.previous(),
            AppState::Editor(state) => state.previous(),
            AppState::Account(state) => state.previous(),
            AppState::Setup(state) => state.previous(),
            AppState::Start(state) => state.previous()
        }
    }

    fn make_permanent(&mut self) -> anyhow::Result<()> {
        match self {
            AppState::Main(state) => state.make_permanent(),
            AppState::Settings(state) => state.make_permanent(),
            AppState::Editor(state) => state.make_permanent(),
            AppState::Account(state) => state.make_permanent(),
            AppState::Setup(state) => state.make_permanent(),
            AppState::Start(state) => state.make_permanent()
        }
    }
}

fn ui() -> impl Widget<AppState> {
    Switcher::new()
        .with_variant(AppStateMain, MainState::widget())
        .with_variant(AppStateSettings, SettingsState::widget())
        .with_variant(AppStateEditor, EditState::widget())
        .with_variant(AppStateAccount, AccountState::widget())
        .with_variant(AppStateSetup, SetupState::widget())
        .with_variant(AppStateStart, StartupState::widget())
        .controller(AppController)
        .background(BACKGROUND_DARK)
        .env_scope(|env, state: &AppState| setup_theme(state.settings().theme, env))
}

struct AppController;
impl Controller<AppState, Switcher<AppState>> for AppController {
    fn event(&mut self, child: &mut Switcher<AppState>, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        if let Event::Command(cmd) = event {
            if let Some(state) = cmd.get(NAVIGATE).cloned() {
                *data = state;
            }
            if let Some(acc) = cmd.get(ACCOUNT_LOGIN).cloned() {
                os::login_account(&acc).unwrap();
                if data.settings().close_on_login {
                    Application::global().quit();
                }
            }
        }
        child.event(ctx, event, data, env)
    }
}
