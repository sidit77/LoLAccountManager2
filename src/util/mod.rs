mod indexed;
pub mod string_list;
pub mod theme;

use std::ops::Not;

use druid::commands::{OPEN_FILE, SAVE_FILE_AS, SHOW_OPEN_PANEL, SHOW_SAVE_PANEL};
use druid::text::{EditableText, TextStorage};
use druid::theme::{BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Button, Controller, CrossAxisAlignment, Either, Flex, Label, Scope, TextBox};
use druid::{Command, Data, Env, Event, EventCtx, FileDialogOptions, Lens, Widget, WidgetExt};
use druid_material_icons::IconPaths;
pub use indexed::{IndexWrapper, Indexed};

use crate::widgets::{Icon, WidgetButton};

pub fn icon_text_button<T: Data>(icon: IconPaths, text: &str) -> impl Widget<T> {
    WidgetButton::new(
        Flex::row()
            .with_child(Icon::new(icon))
            .with_spacer(3.0)
            .with_child(Label::new(text))
            .center()
    )
}

pub fn field<T: EditableText + TextStorage>(name: &str) -> impl Widget<T> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new(name))
        .with_spacer(2.0)
        .with_child(TextBox::new().expand_width())
        .padding(3.0)
        .expand_width()
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
}

#[derive(Clone, Data, Lens)]
struct PasswordState<T> {
    password: T,
    visible: bool
}

impl<T> PasswordState<T> {
    fn new(password: T) -> Self {
        Self { password, visible: false }
    }
}

pub fn ternary<T>(a: bool, v1: T, v2: T) -> T {
    if a { v1 } else { v2 }
}

pub fn password_field<T: EditableText + TextStorage>(name: &str) -> impl Widget<T> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new(name))
        .with_spacer(2.0)
        .with_child(
            Scope::from_lens(
                PasswordState::new,
                PasswordState::password,
                Flex::row()
                    .with_flex_child(
                        Either::new(
                            |state: &PasswordState<_>, _| state.visible,
                            TextBox::new().lens(PasswordState::password),
                            TextBox::protected()
                                .fix_height(34.0)
                                .lens(PasswordState::password)
                        )
                        .expand_width(),
                        1.0
                    )
                    .with_spacer(3.0)
                    .with_child(
                        Button::dynamic(|state: &bool, _| ternary(*state, "Hide", "Show").to_string())
                            .on_click(|_, state: &mut bool, _| *state = state.not())
                            .fix_width(62.0)
                            .lens(PasswordState::visible)
                    )
            ) //TextBox::protected()
              //.expand_width()
        )
        .padding(3.0)
        .expand_width()
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
}

pub fn multiline_field<T: EditableText + TextStorage>(name: &str) -> impl Widget<T> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new(name))
        .with_spacer(2.0)
        .with_flex_child(TextBox::multiline().with_line_wrapping(false).expand(), 1.0)
        .padding(3.0)
        .expand_width()
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
}

#[derive(Debug, Clone)]
pub enum PathOptions {
    Save(FileDialogOptions),
    Open(FileDialogOptions)
}

impl From<PathOptions> for Command {
    fn from(options: PathOptions) -> Self {
        match options {
            PathOptions::Save(options) => SHOW_SAVE_PANEL.with(options),
            PathOptions::Open(options) => SHOW_OPEN_PANEL.with(options)
        }
    }
}

pub fn path_field<T: EditableText + TextStorage>(name: &str, options: PathOptions) -> impl Widget<T> {
    let controller = options.clone();
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new(name))
        .with_spacer(2.0)
        .with_child(
            Flex::row()
                .with_flex_child(TextBox::new().expand_width(), 1.0)
                .with_spacer(3.0)
                .with_child(Button::new("Browse").on_click(move |ctx, _, _| ctx.submit_command(options.clone())))
                .expand_width()
        )
        .controller(controller)
        .padding(3.0)
        .expand_width()
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
}

impl<T: EditableText + TextStorage, W: Widget<T>> Controller<T, W> for PathOptions {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::Command(cmd) = event {
            match self {
                PathOptions::Save(_) => {
                    if let Some(file_info) = cmd.get(SAVE_FILE_AS) {
                        data.edit(0..data.len(), file_info.path.to_string_lossy());
                    }
                }
                PathOptions::Open(_) => {
                    if let Some(file_info) = cmd.get(OPEN_FILE) {
                        data.edit(0..data.len(), file_info.path.to_string_lossy());
                    }
                }
            }
        }
        child.event(ctx, event, data, env)
    }
}
