use druid::theme::{BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Button, Checkbox, CrossAxisAlignment, Flex, Label, LineBreaking, RadioGroup};
use druid::{Data, EventCtx, Lens, Widget, WidgetExt};

use crate::data::{Settings, Theme};
use crate::screens::main::MainState;
use crate::screens::setup::SetupState;
use crate::screens::{AppState, MainUi, Navigator, Screen};

#[derive(Clone, Data, Lens)]
pub struct SettingsState {
    pub previous: MainState,
    pub settings: Settings
}

impl SettingsState {

    fn save(&self, ctx: &EventCtx) {
        let settings = self.settings.clone();
        ctx.get_external_handle().add_idle_callback(move |ui: &mut MainUi| {
            settings.save().unwrap();
            ui.settings = settings;
        })
    }

    pub fn widget() -> impl Widget<Self> + 'static {
        build_settings_ui()
    }

}

impl From<SettingsState> for AppState {
    fn from(value: SettingsState) -> Self {
        Self::Settings(value)
    }
}

impl Screen for SettingsState {

    fn previous(&self) -> Option<AppState> {
        Some(self.previous.clone().into())
    }

}

fn build_settings_ui() -> impl Widget<SettingsState> {
    Flex::column()
        .with_child(
            Flex::column()
                .with_child(after_login_ui().lens(Settings::close_on_login))
                .with_spacer(3.0)
                .with_child(theme_ui().lens(Settings::theme))
                .lens(SettingsState::settings)
        )
        .with_spacer(3.0)
        .with_child(database_ui())
        .with_spacer(3.0)
        .with_child(info_ui())
        .with_flex_spacer(1.0)
        .with_child(
            Button::new("Back")
                .on_click(|ctx, state: &mut SettingsState, _| {
                    state.save(ctx);
                    ctx.back();
                })
                .expand_width()
                .fix_height(50.0)
        )
        .padding(6.0)
        .expand()
}

fn after_login_ui() -> impl Widget<bool> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("After Login:"))
        .with_spacer(3.0)
        .with_child(Checkbox::new("Close the Program"))
        .padding(5.0)
        .expand_width()
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
}

fn theme_ui() -> impl Widget<Theme> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("Theme:"))
        .with_spacer(6.0)
        .with_child(RadioGroup::column([("Light", Theme::Light), ("Dark", Theme::Dark)]).padding((6.0, 0.0)))
        .padding(5.0)
        .expand_width()
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
}

fn database_ui() -> impl Widget<SettingsState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("Database:"))
        .with_spacer(6.0)
        .with_child(
            Flex::row()
                .with_flex_child(
                    Button::new("Change")
                        .on_click(|ctx, state: &mut SettingsState, _| {
                            state.save(ctx);
                            ctx.open(SetupState::new());
                        })
                        .expand_width(),
                    1.0
                )
                .with_spacer(3.0)
                .with_flex_child(
                    Button::new("Export as YAML")
                        .disabled_if(|_, _| true)
                        .expand_width(),
                    1.0
                )
                .with_spacer(3.0)
                .with_flex_child(
                    Button::new("Export as Text")
                        .disabled_if(|_, _| true)
                        .expand_width(),
                    1.0
                )
        )
        .padding(5.0)
        .expand_width()
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
}

fn info_ui() -> impl Widget<SettingsState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("Information:"))
        .with_spacer(6.0)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Label::new(concat!("Version: ", env!("CARGO_PKG_VERSION"))))
                .with_spacer(3.0)
                .with_child(
                    Flex::row()
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .with_child(Label::new("Database: "))
                        .with_flex_child(
                            Label::dynamic(|state: &SettingsState, _| state.previous.database.path.clone())
                                .with_line_break_mode(LineBreaking::WordWrap),
                            1.0
                        )
                )
                .padding((6.0, 0.0, 0.0, 0.0))
        )
        .padding(5.0)
        .expand_width()
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
}
