use druid::{Widget, Lens, Data, WidgetExt, Selector};
use druid::text::{EditableText, TextStorage};
use druid::theme::{BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment, TextBox};
use druid_material_icons::IconPaths;
use druid_material_icons::normal::navigation::CLOSE;
use druid_material_icons::normal::content::SAVE;
use crate::Account;
use crate::gui::edit::EditState;
use crate::gui::widgets::{Icon, WidgetButton};

pub const CLOSE_ACCOUNT: Selector<(AccountState, bool)> = Selector::new("lol_account_manager_v2.account.close");

#[derive(Copy, Clone, Data)]
pub enum EditMode {
    New,
    Existing(usize)
}

#[derive(Clone, Data, Lens)]
pub struct AccountState {
    pub previous: EditState,
    pub account: Account,
    pub mode: EditMode
}

impl AccountState {

    pub fn new(previous: EditState) -> Self {
        Self {
            previous,
            account: Account::default(),
            mode: EditMode::New
        }
    }

    pub fn existing(previous: EditState, index: usize, account: Account) -> Self {
        Self {
            previous,
            account,
            mode: EditMode::Existing(index)
        }
    }

}

pub fn build_account_ui() -> impl Widget<AccountState> {
    Flex::column()
        .with_flex_child(Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(field("Name:").lens(Account::name))
            .with_spacer(3.0)
            .with_child(field("Username:").lens(Account::username))
            .with_spacer(3.0)
            .with_child(field("Password:").lens(Account::password))
            .with_spacer(3.0)
            .with_flex_child(multiline_field("Notes:").lens(Account::notes), 1.0)
            .lens(AccountState::account), 1.0)
        .with_spacer(3.0)
        .with_child(Flex::row()
            .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
            .with_flex_child(
                icon_text_button(CLOSE, "Discard")
                    .on_click(|ctx, state: &mut AccountState, _|
                        ctx.submit_command(CLOSE_ACCOUNT.with((state.clone(),false)))), 1.0)
            .with_spacer(3.0)
            .with_flex_child(
                icon_text_button(SAVE, "Save")
                    .on_click(|ctx, state: &mut AccountState, _|
                        ctx.submit_command(CLOSE_ACCOUNT.with((state.clone(), true)))), 1.0)
            .expand_width()
            .fix_height(50.0))
        .padding(6.0)
        .expand()
}

fn icon_text_button<T: Data>(icon: IconPaths, text: &str) -> impl Widget<T> {
    WidgetButton::new(
        Flex::row()
            .with_child(Icon::new(icon))
            .with_spacer(3.0)
            .with_child(Label::new(text))
            .center()
    )
}


fn field<T: EditableText + TextStorage>(name: &str) -> impl Widget<T> {
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

fn multiline_field<T: EditableText + TextStorage>(name: &str) -> impl Widget<T> {
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
