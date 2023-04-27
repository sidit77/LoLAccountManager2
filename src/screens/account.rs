use std::ops::{Index, IndexMut};

use druid::widget::{CrossAxisAlignment, Flex, MainAxisAlignment};
use druid::{Data, Lens, Widget, WidgetExt};
use druid_material_icons::normal::action::DONE;
use druid_material_icons::normal::navigation::CLOSE;

use crate::data::Account;
use crate::screens::edit::EditState;
use crate::screens::popup::PopupState;
use crate::screens::{AppState, Navigator};
use crate::util::{field, icon_text_button, multiline_field, password_field};

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
    fn valid(&self) -> bool {
        !self.account.name.is_empty() && !self.account.username.is_empty() && !self.account.password.is_empty()
    }

    fn unsaved_changes(&self) -> bool {
        match self.mode {
            EditMode::New => !self.account.name.is_empty() || !self.account.username.is_empty() || !self.account.password.is_empty(),
            EditMode::Existing(i) => self.previous.database.accounts.index(i) != &self.account
        }
    }

    fn save(&mut self) {
        match self.mode {
            EditMode::New => self
                .previous
                .database
                .accounts
                .push_back(self.account.clone()),
            EditMode::Existing(index) => *self.previous.database.accounts.index_mut(index) = self.account.clone()
        }
    }

    pub fn widget() -> impl Widget<Self> + 'static {
        build_account_ui()
    }
}

impl From<AccountState> for AppState {
    fn from(value: AccountState) -> Self {
        AppState::Account(value)
    }
}

impl AccountState {
    pub fn new(previous: EditState) -> Self {
        Self {
            previous,
            account: Account::default(),
            mode: EditMode::New
        }
    }

    pub fn existing(previous: EditState, index: usize) -> Self {
        let account = previous.database.accounts.index(index).clone();
        Self {
            previous,
            account,
            mode: EditMode::Existing(index)
        }
    }
}

fn build_account_ui() -> impl Widget<AccountState> {
    Flex::column()
        .with_flex_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(field("Name:").lens(Account::name))
                .with_spacer(3.0)
                .with_child(field("Username:").lens(Account::username))
                .with_spacer(3.0)
                .with_child(password_field("Password:").lens(Account::password))
                .with_spacer(3.0)
                .with_flex_child(multiline_field("Notes:").lens(Account::notes), 1.0)
                .lens(AccountState::account),
            1.0
        )
        .with_spacer(3.0)
        .with_child(
            Flex::row()
                .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
                .with_flex_child(
                    icon_text_button(DONE, "Ok")
                        .on_click(|ctx, state: &mut AccountState, _| {
                            state.save();
                            ctx.back();
                        })
                        .disabled_if(|state: &AccountState, _| !state.valid() || !state.unsaved_changes()),
                    1.0
                )
                .with_spacer(3.0)
                .with_flex_child(
                    icon_text_button(CLOSE, "Cancel").on_click(|ctx, state: &mut AccountState, _| match state.unsaved_changes() {
                        true => ctx.open_popup(PopupState::confirm_discard()),
                        false => ctx.back()
                    }),
                    1.0
                )
                .expand_width()
                .fix_height(50.0)
        )
        .padding(6.0)
        .expand()
}
