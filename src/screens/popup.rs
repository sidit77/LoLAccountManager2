use std::ops::Deref;
use std::sync::Arc;
use druid::{Widget, Lens, Data, WidgetExt};
use druid::theme::{BACKGROUND_LIGHT, BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Button, Flex, Label, LineBreaking};
use druid_material_icons::normal::alert::WARNING_AMBER;
use crate::AppState;
use crate::data::{Settings};
use crate::screens::Screen;
use crate::widgets::Icon;

#[derive(Clone, Data, Lens)]
pub struct PopupState {
    pub previous: Arc<AppState>,
    pub message: String
}

impl From<PopupState> for AppState {
    fn from(value: PopupState) -> Self {
        Self::Popup(value)
    }
}

impl Screen for PopupState {
    fn widget() -> Box<dyn Widget<Self>> {
        Box::new(build_popup_ui())
    }

    fn settings(&self) -> Settings {
       self.previous.settings()
    }

    fn previous(&self) -> Option<AppState> {
        Some(self.previous.deref().clone())
    }
}

fn build_popup_ui() -> impl Widget<PopupState> {
    Flex::column()
        .with_child(Icon::new(WARNING_AMBER).fix_height(60.0))
        .with_child(Label::dynamic(|state: &PopupState, _ | state.message.clone()).with_line_break_mode(LineBreaking::WordWrap))
        .with_spacer(3.0)
        .with_child(Button::new("Close").on_click(|ctx, state: &mut PopupState, _| {
            state.back(ctx, false);
        }))
        .padding(6.0)
        .background(BACKGROUND_LIGHT)
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
        .center()
}