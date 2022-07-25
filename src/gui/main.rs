use druid::{Selector, Data, Lens, Widget, WidgetExt, TextAlignment, lens, LensExt, theme};
use druid::im::Vector;
use druid::theme::{BACKGROUND_LIGHT, BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Button, Flex, List, Scroll, TextBox};
use druid_material_icons::normal::image::EDIT;
use druid_material_icons::normal::action::SETTINGS;
use crate::{Account, Database, Settings};
use crate::gui::widgets::{WidgetButton, Icon};

pub const OPEN_SETTINGS: Selector<MainState> = Selector::new("lol_account_manager_v2.main.settings");
pub const OPEN_EDITOR: Selector<MainState> = Selector::new("lol_account_manager_v2.main.editor");

#[derive(Clone, Data, Lens)]
pub struct MainState {
    pub settings: Settings,
    pub filter: String,
    pub database: Database
}

pub fn build_main_ui() -> impl Widget<MainState> {
    Flex::column()
        .with_child(Flex::row()
            .with_flex_child(
                TextBox::new()
                .with_text_alignment(TextAlignment::Center)
                .with_placeholder("Search...")
                .lens(MainState::filter)
                .env_scope(|env,_| env.set(theme::TEXTBOX_BORDER_WIDTH, 0.0))
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
                .on_click(|ctx, state: &mut MainState ,_|
                    ctx.submit_command(OPEN_EDITOR.with(state.clone()))))
            .with_spacer(3.0)
            .with_child(WidgetButton::new(Icon::new(SETTINGS)
                .expand_height()
                .padding(3.0))
                .on_click(|ctx, state: &mut MainState, _|
                    ctx.submit_command(OPEN_SETTINGS.with(state.clone()))))
            .expand_width()
            .fix_height(50.0))
        .with_spacer(3.0)
        .with_flex_child(
            account_view()
                .expand()
                .padding(3.0)
                //.background(BACKGROUND_LIGHT)
                .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
                .rounded(TEXTBOX_BORDER_RADIUS), 1.0)
        .padding(5.0)
}

fn account_view() -> impl Widget<MainState> {
    Scroll::new(List::new(item_ui))
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
}

fn item_ui() -> impl Widget<Account> {
    Button::new(|item: &Account, _: &_| item.name.to_string())
        .on_click(|_ctx, acc: &mut Account, _env| {
            println!("Login: {}", acc.name);
        })
        .padding(3.0)
        .expand()
        .height(50.0)
}