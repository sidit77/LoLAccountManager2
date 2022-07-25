use std::ops::Index;
use druid::{Widget, Lens, LensExt, Data, WidgetExt, lens, Selector, EventCtx, Event, Env};
use druid::theme::{BACKGROUND_LIGHT, BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Button, Controller, Flex, Label, List, MainAxisAlignment, Scroll};
use crate::{Account, Database, MainState};
use crate::gui::account::AccountState;
use crate::gui::util::{Indexed, IndexWrapper};

pub const OPEN_ACCOUNT: Selector<AccountState> = Selector::new("lol_account_manager_v2.edit.account");
pub const CLOSE_EDITOR: Selector<(EditState, bool)> = Selector::new("lol_account_manager_v2.edit.close");

const EDIT_ACCOUNT: Selector<usize> = Selector::new("lol_account_manager_v2.edit.edit");
const DELETE_ACCOUNT: Selector<usize> = Selector::new("lol_account_manager_v2.edit.delete");
const MOVE_ACCOUNT: Selector<(usize, i32)> = Selector::new("lol_account_manager_v2.edit.move");

#[derive(Clone, Data, Lens)]
pub struct EditState {
    pub previous: MainState,
    pub database: Database
}

pub fn build_edit_ui() -> impl Widget<EditState> {
    Flex::column()
        .with_child(Flex::row()
            .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
            .with_flex_child(
                Button::new("New")
                    .on_click(|ctx, state: &mut EditState ,_|
                        ctx.submit_command(OPEN_ACCOUNT.with(AccountState::new(state.clone())))).expand(), 1.0)
            .with_spacer(3.0)
            .with_flex_child(
                Button::new("Save")
                    .on_click(|ctx, state: &mut EditState ,_|
                        ctx.submit_command(CLOSE_EDITOR.with((state.clone(), true)))).expand(), 1.0)
            .with_spacer(3.0)
            .with_flex_child(
                Button::new("Discard")
                    .on_click(|ctx, state: &mut EditState ,_|
                        ctx.submit_command(CLOSE_EDITOR.with((state.clone(), false)))).expand(), 1.0) //Button::new("O").expand()
            .expand_width()
            .fix_height(50.0))
        .with_spacer(3.0)
        .with_flex_child(
            account_view()
                .expand()
                .padding(3.0)
                .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
                .rounded(TEXTBOX_BORDER_RADIUS), 1.0)
        .padding(5.0)
}

fn account_view() -> impl Widget<EditState> {
    Scroll::new(List::new(item_ui)
        .with_spacing(3.0))
        .vertical()
        .lens(lens::Identity.map(
            |d: &EditState| IndexWrapper::from(d.database.accounts.clone()),
            |d: &mut EditState, x: IndexWrapper<Account>| d.database.accounts = x.into(),
        ))
        .controller(ListController)
}

fn item_ui() -> impl Widget<Indexed<Account>> {
    Flex::row()
        .with_flex_child(
            Label::new(|entry: &Indexed<Account>, _: &_| format!("{}", entry.name))
                .center()
                .expand()
                .padding(3.0), 1.0)
        .with_spacer(3.0)
        .with_child(Button::new("Edit")
            .on_click(|ctx,entry: &mut Indexed<Account>,_|
                ctx.submit_command(EDIT_ACCOUNT.with(entry.index())))
            .expand_height()
            .padding(3.0))
        .with_child(Flex::column()
            .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
            .with_flex_child(
                Button::new("Up")
                    .disabled_if(|entry: &Indexed<Account>, _: &_| entry.is_first())
                    .on_click(|ctx, entry: &mut Indexed<Account>, _|
                        ctx.submit_command(MOVE_ACCOUNT.with((entry.index(), -1)))), 1.0)
            .with_flex_child(
                Button::new("Down")
                    .disabled_if(|entry: &Indexed<Account>, _: &_| entry.is_last())
                    .on_click(|ctx, entry: &mut Indexed<Account>, _|
                        ctx.submit_command(MOVE_ACCOUNT.with((entry.index(), 1)))), 1.0)
            .expand_height())
        .with_child(Button::new("Delete")
            .on_click(|ctx, entry: &mut Indexed<Account>, _|
                ctx.submit_command(DELETE_ACCOUNT.with(entry.index())))
            .expand_height()
            .padding(3.0))
        .expand_width()
        .fix_height(60.0)
        .background(BACKGROUND_LIGHT)
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
}

struct ListController;
impl<W: Widget<EditState>> Controller<EditState, W> for ListController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut EditState, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(EDIT_ACCOUNT) => {
                let index = *cmd.get_unchecked(EDIT_ACCOUNT);
                let account = data.database.accounts.index(index).clone();
                ctx.submit_command(OPEN_ACCOUNT.with(AccountState::existing(data.clone(), index, account)));
            },
            Event::Command(cmd) if cmd.is(DELETE_ACCOUNT) => {
                let index = *cmd.get_unchecked(DELETE_ACCOUNT);
                data.database.accounts.remove(index);
            },
            Event::Command(cmd) if cmd.is(MOVE_ACCOUNT) => {
                let (index, offset) = *cmd.get_unchecked(MOVE_ACCOUNT);
                let target = match offset.is_negative() {
                    true => index.saturating_sub(offset.abs() as usize),
                    false => index.saturating_add(offset.abs() as usize),
                };
                data.database.accounts.swap(index, target);
            }
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}
