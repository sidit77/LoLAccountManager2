mod account;
mod edit;
mod main;
mod popup;
mod settings;
mod setup;
mod start;

use druid::theme::BACKGROUND_DARK;
use druid::widget::{Maybe, ZStack};
use druid::{Data, EventCtx, ExtEventSink, Lens, Widget, WidgetExt};
use druid_widget_nursery::enum_switcher::Switcher;
use druid_widget_nursery::prism::Prism;

use crate::data::{Settings, Theme};
use crate::screens::account::AccountState;
use crate::screens::edit::EditState;
use crate::screens::main::MainState;
use crate::screens::popup::PopupState;
use crate::screens::settings::SettingsState;
use crate::screens::setup::SetupState;
use crate::screens::start::StartupState;
use crate::util::theme::setup_theme;

pub trait Navigator {
    fn close_popup(self);
    fn open_popup(self, popup: PopupState);
    fn back(self);
    fn open(self, screen: impl Into<AppState>);
}

impl Navigator for &mut MainUi {
    fn close_popup(self) {
        if let Some(popup) = self.popup.take() {
            popup.close()
        }
    }

    fn open_popup(self, popup: PopupState) {
        if self.popup.is_some() {
            self.close_popup();
        }
        self.popup = Some(popup);
    }

    fn back(self) {
        if let Some(previous) = self.state.previous() {
            self.state = previous;
        }
    }

    fn open(self, screen: impl Into<AppState>) {
        self.state = screen.into();
    }
}

impl Navigator for &ExtEventSink {
    fn close_popup(self) {
        self.add_idle_callback(|ui: &mut MainUi| ui.close_popup())
    }

    fn open_popup(self, popup: PopupState) {
        self.add_idle_callback(|ui: &mut MainUi| ui.open_popup(popup))
    }

    fn back(self) {
        self.add_idle_callback(|ui: &mut MainUi| ui.back())
    }

    fn open(self, screen: impl Into<AppState>) {
        let screen = screen.into();
        self.add_idle_callback(|ui: &mut MainUi| ui.open(screen))
    }
}

impl Navigator for &EventCtx<'_, '_> {
    fn close_popup(self) {
        self.get_external_handle().close_popup()
    }

    fn open_popup(self, popup: PopupState) {
        self.get_external_handle().open_popup(popup)
    }

    fn back(self) {
        self.get_external_handle().back()
    }

    fn open(self, screen: impl Into<AppState>) {
        self.get_external_handle().open(screen)
    }
}

#[derive(Clone, Data, Lens)]
pub struct MainUi {
    pub settings: Settings,
    pub state: AppState,
    pub popup: Option<PopupState>
}

impl MainUi {
    pub fn new() -> MainUi {
        let (settings, popup) = Settings::load()
            .map(|settings| (settings, None))
            .unwrap_or_else(|err| (Settings::default(), Some(err.into())));
        MainUi {
            settings,
            state: StartupState::new().into(), //AppState::load().unwrap(),
            popup
        }
    }

    fn current_theme(&self) -> Theme {
        match &self.state {
            AppState::Settings(state) => state.settings.theme,
            _ => self.settings.theme
        }
    }

    pub fn widget() -> impl Widget<MainUi> + 'static {
        let main = AppState::widget().lens(MainUi::state);
        let popup = Maybe::or_empty(PopupState::widget).lens(MainUi::popup);
        ZStack::new(main)
            .with_centered_child(popup)
            .env_scope(|env, ui: &MainUi| setup_theme(ui.current_theme(), env))
    }
}

#[derive(Clone, Data, Prism)]
pub enum AppState {
    Start(StartupState),
    Main(MainState),
    Settings(SettingsState),
    Editor(EditState),
    Account(AccountState),
    Setup(SetupState)
}

impl AppState {
    pub fn widget() -> impl Widget<Self> + 'static {
        Switcher::new()
            .with_variant(AppStateMain, MainState::widget())
            .with_variant(AppStateSettings, SettingsState::widget())
            .with_variant(AppStateEditor, EditState::widget())
            .with_variant(AppStateAccount, AccountState::widget())
            .with_variant(AppStateSetup, SetupState::widget())
            .with_variant(AppStateStart, StartupState::widget())
            .background(BACKGROUND_DARK)
    }

    fn previous(&self) -> Option<AppState> {
        match self {
            AppState::Settings(state) => Some(state.previous.clone().into()),
            AppState::Editor(state) => Some(state.previous.clone().into()),
            AppState::Account(state) => Some(state.previous.clone().into()),
            _ => None
        }
    }
}
