use druid::{Selector, Data, Lens, Widget, WidgetExt, TextAlignment, lens, LensExt};
use druid::im::Vector;
use druid::theme::{BACKGROUND_LIGHT, BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Button, Flex, List, TextBox};
use druid_material_icons::normal::image::EDIT;
use druid_material_icons::normal::action::SETTINGS;
use crate::AppState;
use crate::data::{Account, Database, Settings};
use crate::screens::edit::EditState;
use crate::screens::Screen;
use crate::screens::settings::SettingsState;
use crate::widgets::{WidgetButton, Icon};

pub const ACCOUNT_LOGIN: Selector<Account> = Selector::new("lol_account_manager_v2.main.login");

#[derive(Clone, Data, Lens)]
pub struct MainState {
    pub settings: Settings,
    pub filter: String,
    pub database: Database
}

impl MainState {
    pub fn new(settings: Settings, database: Database) -> Self {
        Self {
            settings,
            filter: "".to_string(),
            database
        }
    }
}

impl From<MainState> for AppState {
    fn from(value: MainState) -> Self {
        Self::Main(value)
    }
}

impl Screen for MainState {
    fn widget() -> Box<dyn Widget<Self>> {
        Box::new(build_main_ui())
    }

    fn settings(&self) -> Settings {
        self.settings.clone()
    }

    fn previous(&self) -> Option<AppState> {
        None
    }

}

fn build_main_ui() -> impl Widget<MainState> {
    Flex::column()
        .with_child(Flex::row()
            .with_flex_child(
                TextBox::new()
                .with_text_alignment(TextAlignment::Center)
                .with_placeholder("Search...")
                .lens(MainState::filter)
                .env_scope(|env,_| env.set(TEXTBOX_BORDER_WIDTH, 0.0))
                .expand_width()
                .center()
                .expand()
                .padding(3.0)
                .background(BACKGROUND_LIGHT)
                .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
                .rounded(TEXTBOX_BORDER_RADIUS), 1.0)
            .with_spacer(3.0)
            .with_child(WidgetButton::new(Icon::new(EDIT)
                .expand_height()
                .padding(3.0))
                .on_click(|ctx, state: &mut MainState ,_| {
                    state.open(ctx, EditState::from(state.clone()))
                }))
            .with_spacer(3.0)
            .with_child(WidgetButton::new(Icon::new(SETTINGS)
                .expand_height()
                .padding(3.0))
                .on_click(|ctx, state: &mut MainState, _|{
                    state.open(ctx, SettingsState::from(state.clone()))
                }))
            .expand_width()
            .fix_height(50.0))
        .with_spacer(3.0)
        .with_flex_child(
            List::new(item_ui)
                .with_spacing(3.0)
                .scroll()
                .vertical()
                .lens(lens::Identity.map(
                    |d: &MainState| d.database.accounts
                        .iter()
                        .filter(|acc|acc
                            .name
                            .to_lowercase()
                            .contains(&d.filter.to_lowercase()))
                        .cloned()
                        .collect(),
                    |_, _: Vector<Account>| {},
                ))
                .expand()
                .padding(3.0)
                .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
                .rounded(TEXTBOX_BORDER_RADIUS), 1.0)
        .padding(5.0)
}

fn item_ui() -> impl Widget<Account> {
    Button::new(|item: &Account, _: &_| item.name.to_string())
        .on_click(|ctx, acc: &mut Account, _env| {
            ctx.submit_command(ACCOUNT_LOGIN.with(acc.clone()))
        })
        .expand()
        .height(50.0)
}