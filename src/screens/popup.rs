use druid::theme::BACKGROUND_DARK;
use druid::widget::{BackgroundBrush, Button, Controller, Flex, Label, LineBreaking, Spinner};
use druid::{Application, Color, Data, Env, Event, EventCtx, FontDescriptor, FontFamily, FontWeight, Widget, WidgetExt};
use druid_widget_nursery::enum_switcher::Switcher;
use druid_widget_nursery::prism::Prism;

use crate::screens::Navigator;

#[derive(Clone, Data, Prism)]
pub enum PopupState {
    Leave(()),
    Saving(bool),
    Error(String)
}

impl PopupState {
    pub fn saving() -> Self {
        Self::Saving(false)
    }

    pub fn confirm_discard() -> Self {
        Self::Leave(())
    }

    pub fn error(content: impl Into<String>) -> Self {
        Self::Error(content.into())
    }

    pub fn widget() -> impl Widget<Self> + 'static {
        Switcher::new()
            .with_variant(PopupStateLeave, leave_popup())
            .with_variant(PopupStateSaving, saving_popup())
            .with_variant(PopupStateError, error_popup())
            .center()
            .background(BackgroundBrush::Color(Color::rgba8(0, 0, 0, 128)))
            .expand()
    }

    pub fn close(self) {
        if let PopupState::Saving(true) = self {
            Application::global().quit()
        }
    }
}

fn error_popup() -> impl Widget<String> + 'static {
    Flex::column()
        .with_child(Label::new("Error")
            .with_font(FontDescriptor::new(FontFamily::SYSTEM_UI)
                .with_weight(FontWeight::SEMI_BOLD))
            .with_text_size(15.0))
        .with_spacer(5.0)
        .with_flex_child(Label::dynamic(|data: &String, _| data.clone())
            .with_line_break_mode(LineBreaking::WordWrap)
            .center(), 1.0)
        .with_spacer(5.0)
        .with_child(Button::new("Close")
            .on_click(|ctx, _, _| ctx.close_popup())
            .expand_width())
        .padding(6.0)
        .fix_size(200.0, 130.0)
        .background(BACKGROUND_DARK)
        .rounded(5.0)
}

fn saving_popup() -> impl Widget<bool> + 'static {
    Flex::column()
        .with_child(Label::new("Saving.."))
        .with_spacer(5.0)
        .with_child(Spinner::new())
        .center()
        .fix_size(200.0, 100.0)
        .background(BACKGROUND_DARK)
        .rounded(5.0)
        .controller(DelayClose)
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

struct DelayClose;

impl<W: Widget<bool>> Controller<bool, W> for DelayClose {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut bool, env: &Env) {
        if let Event::WindowCloseRequested = event {
            ctx.set_handled();
            *data = true;
            println!("Delaying the end of the application until saving is complete!");
        }
        child.event(ctx, event, data, env)
    }
}
