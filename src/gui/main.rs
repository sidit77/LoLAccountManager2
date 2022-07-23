use druid::{Selector, Data, Lens, Widget, WidgetExt, Color, TextAlignment, lens, LensExt, theme};
use druid::im::Vector;
use druid::widget::{Button, Flex, List, MainAxisAlignment, Scroll, TextBox};
use crate::{Account, Database, Settings};
use crate::gui::widgets::{IconButton, icons};

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
                .border(Color::GRAY, 2.0)
                .rounded(5.0), 1.0)
            .with_spacer(3.0)
            .with_child(Flex::row()
                .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
                .with_flex_child(IconButton::new(&icons::EDIT)
                    .on_click(|ctx, state: &mut MainState ,_|
                        ctx.submit_command(OPEN_EDITOR.with(state.clone()))), 1.0)
                .with_spacer(3.0)
                .with_flex_child(IconButton::new(&icons::PREFERENCES)
                    .on_click(|ctx, state: &mut MainState, _|
                        ctx.submit_command(OPEN_SETTINGS.with(state.clone()))), 1.0) //Button::new("O").expand()
                .fix_width(83.0)
                .expand_height()
                .padding(3.0)
                .border(Color::GRAY, 2.0)
                .rounded(5.0))
            .expand_width()
            .fix_height(50.0))
        .with_spacer(3.0)
        .with_flex_child(account_view()
            .expand()
            .padding(3.0)
            .border(Color::GRAY, 2.0)
            .rounded(5.0), 1.0)
        .padding(5.0)
}

fn account_view() -> impl Widget<MainState> {
    Scroll::new(List::new(|| {
        Button::new(|item: &Account, _: &_| format!("{}", item.name))
            .on_click(|_ctx, acc: &mut Account, _env| {
                println!("Login: {}", acc.name);
            })
            .padding(3.0)
            .expand()
            .height(50.0)
    }))
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