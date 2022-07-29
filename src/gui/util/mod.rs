mod indexed;
pub mod string_list;

use druid::{Command, Data, Env, Event, EventCtx, FileDialogOptions, Widget, WidgetExt};
use druid::commands::{OPEN_FILE, SAVE_FILE_AS, SHOW_OPEN_PANEL, SHOW_SAVE_PANEL};
use druid::text::{EditableText, TextStorage};
use druid::theme::{BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Button, Controller, CrossAxisAlignment, Flex, Label, TextBox};
use druid_material_icons::IconPaths;
use crate::gui::widgets::{Icon, WidgetButton};

pub use indexed::{IndexWrapper, Indexed};

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
        .with_child(
            TextBox::new()
                .expand_width()
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
        .with_flex_child(
            TextBox::multiline()
                .with_line_wrapping(false)
                .expand()
            , 1.0)
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
                .with_flex_child(
                    TextBox::new()
                        .expand_width(), 1.0)
                .with_spacer(3.0)
                .with_child(
                    Button::new("Browse")
                        .on_click(move |ctx, _, _| ctx.submit_command(options.clone())))
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
