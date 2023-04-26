use druid::theme::BACKGROUND_DARK;
use druid::widget::{BackgroundBrush, Button, Flex, Label, Spinner};
use druid::{Color, Data, Widget, WidgetExt};
use druid_widget_nursery::enum_switcher::Switcher;
use druid_widget_nursery::prism::Prism;

use crate::screens::Navigator;

#[derive(Clone, Data, Prism)]
pub enum PopupState {
    Leave(()),
    Saving(())
}

impl PopupState {
    pub fn widget() -> impl Widget<Self> + 'static {
        Switcher::new()
            .with_variant(PopupStateLeave, leave_popup())
            .with_variant(PopupStateSaving, saving_popup())
            .center()
            .background(BackgroundBrush::Color(Color::rgba8(0, 0, 0, 128)))
            .expand()
    }

    pub fn close(self) {}
}

fn saving_popup() -> impl Widget<()> + 'static {
    Flex::column()
        .with_child(Label::new("Saving.."))
        .with_spacer(5.0)
        .with_child(Spinner::new())
        .center()
        .fix_size(200.0, 100.0)
        .background(BACKGROUND_DARK)
        .rounded(5.0)
}

fn leave_popup() -> impl Widget<()> + 'static {
    Flex::column()
        .with_flex_child(Label::new("Discard unsaved changes?").center(), 1.0)
        .with_child(
            Flex::row()
                .with_flex_child(
                    Button::new("Discard")
                        .on_click(|ctx, _, _| {
                            ctx.close_popup();
                            ctx.back();
                        })
                        .expand_width(),
                    1.0
                )
                .with_spacer(3.0)
                .with_flex_child(
                    Button::new("Back")
                        .on_click(|ctx, _, _| ctx.close_popup())
                        .expand_width(),
                    1.0
                )
        )
        .padding(6.0)
        .fix_size(200.0, 100.0)
        .background(BACKGROUND_DARK)
        .rounded(5.0)
}
