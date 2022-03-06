#![windows_subsystem = "windows"]

mod icons;
mod button;

use druid::widget::prelude::*;
use druid::widget::{Button, Either, Flex, Label, List, MainAxisAlignment, Scroll, TextBox};
use druid::{AppLauncher, Color, LocalizedString, theme, WidgetExt, WindowDesc, Lens, lens, LensExt, TextAlignment};
use druid::im::{Vector};
use crate::button::IconButton;
use crate::icons::Icon;

#[derive(Clone, Data, Lens)]
struct AppData {
    filter: String,
    edit_mode: bool,
    accounts: Vector<String>
}

pub fn main() {
    let window = WindowDesc::new(build_widget)
        .window_size((300.0, 600.0))
        .title(LocalizedString::new("scroll-demo-window-title").with_placeholder("Scroll demo"));
    AppLauncher::with_window(window)
        .configure_env(|env, _| {
            env.set(theme::BUTTON_DARK, Color::AQUA);
            env.set(theme::BUTTON_LIGHT, Color::AQUA);
            env.set(theme::LABEL_COLOR, Color::BLACK);
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::WHITE);
            env.set(theme::BACKGROUND_LIGHT, Color::WHITE);
            env.set(theme::CURSOR_COLOR, Color::BLACK);
            env.set(theme::TEXTBOX_BORDER_WIDTH, 0.0);

        })
        .launch(AppData {
            filter: "".to_string(),
            edit_mode: false,
            accounts: NAMES.iter().map(|s| s.to_string()).collect()
        })
        .expect("launch failed");
}

fn build_widget() -> impl Widget<AppData> {
    Flex::column()
        .with_child(Flex::row()
            .with_flex_child(TextBox::new()
                .with_text_alignment(TextAlignment::Center)
                .with_placeholder("Search...")
                .lens(AppData::filter)
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
                    .on_click(|_, mode: &mut bool ,_| *mode = !*mode)
                    .lens(AppData::edit_mode), 1.0)
                .with_spacer(3.0)
                .with_flex_child(IconButton::new(&icons::PREFERENCES)
                    .on_click(|_,_,_| println!("settings")), 1.0) //Button::new("O").expand()
                .fix_width(83.0)
                .expand_height()
                .padding(3.0)
                .border(Color::GRAY, 2.0)
                .rounded(5.0))
            .expand_width()
            .fix_height(50.0))
        .with_spacer(3.0)
        .with_flex_child(Either::new(|data: &AppData, _ | !data.edit_mode, standard_account_view(), edit_account_view())
             .expand()
             .padding(3.0)
             .border(Color::GRAY, 2.0)
             .rounded(5.0), 1.0)
        .padding(5.0)
}

fn standard_account_view() -> impl Widget<AppData> {
    Scroll::new(List::new(|| {
        Button::new(|(_, item): &(Vector<String>, String), _: &_| format!("{}", item))
            .on_click(|_ctx, (_, acc): &mut (Vector<String>, String), _env| {
                println!("Login: {}", acc);
            })
            .padding(3.0)
            .expand()
            .height(50.0)
    }))
    .vertical()
    .lens(account_filter_lens())
}

fn edit_account_view() -> impl Widget<AppData> {
    Scroll::new(
        Flex::column()
            .with_flex_child(
                List::new(|| {
                        Button::new(|(_, item): &(Vector<String>, String), _: &_| format!("{}", item))
                            .on_click(|_ctx, (shared, item): &mut (Vector<String>, String), _env| {
                                shared.retain(|v| v != item);
                            })
                            .padding(3.0)
                            .expand()
                            .height(50.0)
                    }), 1.0)
            .with_child(
                Button::new("Test")
                    //.padding(3.0)
                    //.expand_width()
                    //.fix_height(50.0)
                    //.background(Color::BLACK)
            )
    )
    .vertical()
    .lens(account_filter_lens())
}

fn account_filter_lens() -> impl Lens<AppData, (Vector<String>, Vector<String>)> {
    lens::Identity.map(
        |d: &AppData| {
            (d.accounts.clone(),
             d.accounts
                 .iter()
                 .filter(|s|s
                     .to_lowercase()
                     .contains(&d.filter.to_lowercase()))
                 .cloned()
                 .collect())
        },
        |d: &mut AppData, x: (Vector<String>, Vector<String>)| {
            d.accounts = x.0
        },
    )
}


const NAMES: &[&str] = &[
    "Allegra",
    "Bree",
    "Bryna",
    "Carrissa",
    "Clair",
    "Cleopatra",
    "Corinna",
    "Dacia",
    "Dinah",
    "Dionis",
    "Ermentrude",
    "Felicdad",
    "Flori",
    "Geneva",
    "Gussie",
    "Jazmin",
    "Jeannette",
    "Jenine",
    "Joann",
    "Layney",
    "Leona",
    "Lizzy",
    "Lucita",
    "Lyndsey",
    "Marcelia",
    "Marlene",
    "Mirabelle",
    "Nanci",
    "Nyssa",
    "Ora",
    "Paula",
    "Phyllis",
    "Prissie",
    "Riannon",
    "Roby",
    "Salomi",
    "Shaylyn",
    "Shela",
    "Siobhan",
    "Sioux",
    "Susanetta",
    "Tallulah",
    "Tiffi",
    "Valentine",
    "Van",
    "Verine",
    "Wallie",
    "Yvonne",
];