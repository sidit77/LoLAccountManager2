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
    let window = WindowDesc::new(build_widget())
        .window_size((400.0, 600.0))
        .title(LocalizedString::new("scroll-demo-window-title").with_placeholder("Scroll demo"));
    AppLauncher::with_window(window)
        .configure_env(|env, _| {
            env.set(theme::BUTTON_DARK, Color::AQUA);
            env.set(theme::BUTTON_LIGHT, Color::AQUA);
            env.set(theme::DISABLED_BUTTON_DARK, Color::WHITE);
            env.set(theme::DISABLED_BUTTON_LIGHT, Color::WHITE);
            env.set(theme::TEXT_COLOR, Color::BLACK);
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
    Either::new(|data: &AppData, _ | !data.edit_mode,
        main_layout(
            TextBox::new()
                .with_text_alignment(TextAlignment::Center)
                .with_placeholder("Search...")
                .lens(AppData::filter)
                .expand_width(),
            IconButton::new(&icons::EDIT)
                .on_click(|_, mode: &mut bool ,_| *mode = true)
                .lens(AppData::edit_mode),
            IconButton::new(&icons::PREFERENCES)
                .on_click(|_,_,_| println!("settings")),
            standard_account_view()
        ),
        main_layout(
            Button::new("New Entry")
                .expand(),
            Button::new("Y")
                .on_click(|_, mode: &mut bool ,_| *mode = false)
                .expand()
                .lens(AppData::edit_mode),
            Button::new("N")
                .on_click(|_, mode: &mut bool ,_| *mode = false)
                .expand()
                .lens(AppData::edit_mode),
            edit_account_view()
        )
    )
}


fn main_layout(h1: impl Widget<AppData> + 'static,
               h2: impl Widget<AppData> + 'static,
               h3: impl Widget<AppData> + 'static,
               body: impl Widget<AppData> + 'static) -> impl Widget<AppData> {
    Flex::column()
        .with_child(Flex::row()
            .with_flex_child(h1
                .center()
                .expand()
                .padding(3.0)
                .border(Color::GRAY, 2.0)
                .rounded(5.0), 1.0)
            .with_spacer(3.0)
            .with_child(Flex::row()
                .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
                .with_flex_child(h2, 1.0)
                .with_spacer(3.0)
                .with_flex_child(h3, 1.0) //Button::new("O").expand()
                .fix_width(83.0)
                .expand_height()
                .padding(3.0)
                .border(Color::GRAY, 2.0)
                .rounded(5.0))
            .expand_width()
            .fix_height(50.0))
        .with_spacer(3.0)
        .with_flex_child(body
            .expand()
            .padding(3.0)
            .border(Color::GRAY, 2.0)
            .rounded(5.0), 1.0)
        .padding(5.0)
}

fn standard_account_view() -> impl Widget<AppData> {
    Scroll::new(List::new(|| {
        Button::new(|item: &String, _: &_| format!("{}", item))
            .on_click(|_ctx, acc: &mut String, _env| {
                println!("Login: {}", acc);
            })
            .padding(3.0)
            .expand()
            .height(50.0)
    }))
    .vertical()
    .lens(lens::Identity.map(
        |d: &AppData| d.accounts
                 .iter()
                 .filter(|s|s
                     .to_lowercase()
                     .contains(&d.filter.to_lowercase()))
                 .cloned()
                 .collect(),
        |_, _: Vector<String>| {},
    ))
}

fn edit_account_view() -> impl Widget<AppData> {
    Scroll::new(List::new(|| {
        Flex::row()
            .with_flex_child(Label::new(|(_, item): &(Vector<String>, String), _: &_| format!("{}", item))
                                 .center()
                                 .expand()
                                 .padding(3.0), 1.0)
            .with_spacer(3.0)
            .with_child(Button::new("Edit")
                .on_click(|_,_,_| println!("edit"))
                .expand_height()
                .padding(3.0))
            .with_child(Flex::column()
                .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
                .with_flex_child(Button::new("Up")
                    .disabled_if(|(list, acc): &(Vector<String>, String), _: &_| list.front().map(|v| v == acc).unwrap_or(false))
                    .on_click(|_, (list, acc): &mut (Vector<String>, String), _| {
                        let i = list.iter().position(|v| v == acc).unwrap();
                        list.swap(i, i - 1);
                    }), 1.0)
                .with_flex_child(Button::new("Down")
                    .disabled_if(|(list, acc): &(Vector<String>, String), _: &_| list.back().map(|v| v == acc).unwrap_or(false))
                    .on_click(|_, (list, acc): &mut (Vector<String>, String), _| {
                        let i = list.iter().position(|v| v == acc).unwrap();
                        list.swap(i, i + 1);
                    }), 1.0)
                .expand_height())
            .with_child(Button::new("Delete")
                .on_click(|_, (list, acc): &mut (Vector<String>, String), _| list.retain(|v| v != acc))
                .expand_height()
                .padding(3.0))
            .expand_width()
            .fix_height(60.0)
            .border(Color::GRAY, 2.0)
            .rounded(5.0)
    }).with_spacing(3.0))
    .vertical()
    .lens(lens::Identity.map(
        |d: &AppData| (d.accounts.clone(), d.accounts.clone()),
        |d: &mut AppData, x: (Vector<String>, Vector<String>)| d.accounts = x.0,
    ))
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
