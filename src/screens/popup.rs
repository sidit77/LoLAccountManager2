use druid::{Color, Widget, WidgetExt, Data};
use druid::widget::{BackgroundBrush, Button};
use druid_widget_nursery::prism::Prism;
use crate::screens::Navigator;

#[derive(Clone, Data, Prism)]
pub enum  PopupState {
    Leave(String)
}

impl PopupState {

    pub fn widget() -> impl Widget<Self> + 'static {
        Button::new("Test")
            .on_click(|ctx, _, _| ctx.close_popup())
            .center()
            .background(BackgroundBrush::Color(Color::rgba8(0, 0, 0, 128)))
            .expand()
    }

    pub fn close(self) {

    }

}