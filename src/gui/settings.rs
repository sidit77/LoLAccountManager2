use druid::{Selector, Widget, WidgetExt, Data, Lens};
use druid::widget::{Button, Checkbox, CrossAxisAlignment, Flex, Label, RadioGroup};
use crate::{Settings, Theme};
use crate::gui::main::MainState;

pub const SETTINGS_SAVE: Selector<SettingsState> = Selector::new("lol_account_manager_v2.settings.back");

#[derive(Clone, Data, Lens)]
pub struct SettingsState {
    pub previous: MainState,
    pub settings: Settings
}

pub fn build_settings_ui() -> impl Widget<SettingsState> {
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
            .on_click(|ctx, state: &mut SettingsState, _| ctx.submit_command(SETTINGS_SAVE.with(state.clone()))))
        .center()

}