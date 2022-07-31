use druid::{Widget, WidgetExt, Data, Lens};
use druid::widget::{Button, Checkbox, CrossAxisAlignment, Flex, Label, RadioGroup};
use crate::AppState;
use crate::data::{Settings, Theme};
use crate::screens::main::MainState;
use crate::screens::Screen;

#[derive(Clone, Data, Lens)]
pub struct SettingsState {
    pub previous: MainState,
    pub settings: Settings
}

impl From<MainState> for SettingsState {
    fn from(state: MainState) -> Self {
        SettingsState {
            settings: state.settings.clone(),
            previous: state
        }
    }
}

impl Into<AppState> for SettingsState {
    fn into(self) -> AppState {
        AppState::Settings(self)
    }
}

impl Screen for SettingsState {
    fn widget() -> Box<dyn Widget<Self>> {
        Box::new(build_settings_ui())
    }

    fn theme(&self) -> Theme {
        self.settings.theme
    }

    fn previous(&self) -> Option<AppState> {
        Some(self.previous.clone().into())
    }

    fn make_permanent(&mut self) {
        self.previous.settings = self.settings.clone();
    }
}

fn build_settings_ui() -> impl Widget<SettingsState> {
    Flex::column()
        .with_child(Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(Checkbox::new("close on login")
                .lens(Settings::close_on_login))
            .with_spacer(6.0)
            .with_child(
                Flex::row()
                    .with_child(Label::new("Theme:"))
                    .with_spacer(6.0)
                    .with_child(RadioGroup::row([("Light", Theme::Light), ("Dark", Theme::Dark)]))
                    .lens(Settings::theme)
                )
            .lens(SettingsState::settings))
        .with_child(Button::new("back")
            .on_click(|ctx, state: &mut SettingsState, _| state.back(ctx, true)))
        .center()

}