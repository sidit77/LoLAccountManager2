use std::sync::Arc;
use druid::{Widget, Lens, Data, WidgetExt, Selector};
use druid::theme::{BACKGROUND_LIGHT, BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Button, Flex, Label, LineBreaking};
use druid_material_icons::normal::alert::WARNING_AMBER;
use crate::AppState;
use crate::gui::widgets::Icon;

pub const POPUP_CLOSE: Selector<PopupState> = Selector::new("lol_account_manager_v2.popup.close");

#[derive(Clone, Data, Lens)]
pub struct PopupState {
    pub previous: Arc<AppState>,
    pub message: String
}

pub fn build_popup_ui() -> impl Widget<PopupState> {
    Flex::column()
        .with_child(Icon::new(WARNING_AMBER).fix_height(60.0))
        .with_child(Label::dynamic(|state: &PopupState, _ | state.message.clone()).with_line_break_mode(LineBreaking::WordWrap))
        .with_spacer(3.0)
        .with_child(Button::new("Close").on_click(|ctx, state: &mut PopupState, _| {
            ctx.submit_command(POPUP_CLOSE.with(state.clone()))
        }))
        .padding(6.0)
        .background(BACKGROUND_LIGHT)
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
        .center()
}