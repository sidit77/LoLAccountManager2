#![windows_subsystem = "windows"]

mod icons;
mod button;

use druid::widget::prelude::*;
use druid::widget::{Button, Flex, Label, MainAxisAlignment, Padding, Scroll};
use druid::{AppLauncher, Color,LocalizedString, theme, WidgetExt, WindowDesc};
use crate::button::IconButton;
use crate::icons::Icon;


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

        })
        .launch(0u32)
        .expect("launch failed");
}

fn build_widget() -> impl Widget<u32> {
    Flex::column()
        .with_child(Flex::row()
            .with_flex_child(Label::new("PLACEHOLDER").center()
                .expand()
                .padding(3.0)
                .border(Color::GRAY, 2.0)
                .rounded(5.0), 1.0)
            .with_spacer(3.0)
            .with_child(Flex::row()
                .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
                .with_flex_child(IconButton::new(&icons::EDIT), 1.0)
                .with_spacer(3.0)
                .with_flex_child(IconButton::new(&icons::PREFERENCES), 1.0) //Button::new("O").expand()
                .fix_width(83.0)
                .expand_height()
                .padding(3.0)
                .border(Color::GRAY, 2.0)
                .rounded(5.0))
            .expand_width()
            .fix_height(50.0))
        .with_spacer(3.0)
        .with_flex_child(account_list()
             .expand()
             .padding(3.0)
             .border(Color::GRAY, 2.0)
             .rounded(5.0), 1.0)
        .padding(5.0)
}

fn account_list() -> impl Widget<u32> {
    let mut col = Flex::column();
    for i in 0..30 {
        col.add_child(Padding::new(3.0, Button::new(format!("Account {}", i + 1)).fix_height(50.0).expand_width()));
    }
    Scroll::new(col.expand_width()).vertical().expand()
}