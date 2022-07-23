use druid::{Widget, Lens, Data, WidgetExt, Selector};
use druid::widget::{Button, Flex, TextBox};
use crate::Account;
use crate::gui::edit::EditState;

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
        .with_child(Flex::column()
            .with_child(
                TextBox::new()
                    .with_placeholder("name")
                    .lens(Account::name))
            .lens(AccountState::account))
        .with_child(Flex::row()
            .with_flex_child(
                Button::new("Save")
                    .on_click(|ctx, state: &mut AccountState, _|
                        ctx.submit_command(CLOSE_ACCOUNT.with((state.clone(), true)))), 1.0)
            .with_flex_child(
                Button::new("Discard")
                    .on_click(|ctx, state: &mut AccountState, _|
                        ctx.submit_command(CLOSE_ACCOUNT.with((state.clone(),false)))), 1.0))
        .center()
}