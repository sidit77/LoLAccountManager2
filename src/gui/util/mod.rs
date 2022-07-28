mod indexed;
pub mod string_list;

use druid::{Data, Widget, WidgetExt};
use druid::text::{EditableText, TextStorage};
use druid::theme::{BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{CrossAxisAlignment, Flex, Label, TextBox};
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