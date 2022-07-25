use std::ops::Index;
use druid::{Widget, Lens, LensExt, Data, WidgetExt, lens, Selector, EventCtx, Event, Env};
use druid::theme::{BACKGROUND_LIGHT, BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Container, Controller, Flex, Label, List, MainAxisAlignment, Scroll};
use druid_material_icons::IconPaths;
use druid_material_icons::normal::image::EDIT;
use druid_material_icons::normal::action::DELETE;
use druid_material_icons::normal::navigation::ARROW_DROP_UP;
use druid_material_icons::normal::navigation::ARROW_DROP_DOWN;
use druid_material_icons::normal::content::ADD;
use druid_material_icons::normal::content::SAVE;
use druid_material_icons::normal::navigation::CLOSE;
use crate::{Account, Database, MainState};
use crate::gui::account::AccountState;
use crate::gui::util::{Indexed, IndexWrapper};
use crate::gui::widgets::{Icon, WidgetButton};

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
                icon_text_button(ADD, "New")
                    .on_click(|ctx, state: &mut EditState ,_|
                        ctx.submit_command(OPEN_ACCOUNT.with(AccountState::new(state.clone())))).expand(), 1.0)
            .with_spacer(3.0)
            .with_flex_child(
                icon_text_button(SAVE, "Save")
                    .on_click(|ctx, state: &mut EditState ,_|
                        ctx.submit_command(CLOSE_EDITOR.with((state.clone(), true)))).expand(), 1.0)
            .with_spacer(3.0)
            .with_flex_child(
                icon_text_button(CLOSE, "Discard")
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

fn icon_text_button<T: Data>(icon: IconPaths, text: &str) -> impl Widget<T> {
    WidgetButton::new(
        Flex::row()
            .with_child(Icon::new(icon))
            .with_spacer(3.0)
            .with_child(Label::new(text))
            .center()
    )
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
    Container::new(
        Flex::row()
            .with_flex_child(
                Label::new(|entry: &Indexed<Account>, _: &_| entry.name.to_string())
                    .center()
                    .expand()
                    .padding(3.0), 1.0)
            .with_spacer(3.0)
            .with_child(
                WidgetButton::new(Icon::new(EDIT)
                    .expand_height()
                    .padding(3.0))
                    .on_click(|ctx,entry: &mut Indexed<Account>,_|
                        ctx.submit_command(EDIT_ACCOUNT.with(entry.index())))
                    )
            .with_spacer(3.0)
            .with_child(
                Flex::column()
                    .with_flex_child(
                        WidgetButton::new(Icon::new(ARROW_DROP_UP)
                            .expand_height()
                            .center())
                            .disabled_if(|entry: &Indexed<Account>, _: &_| entry.is_first())
                            .on_click(|ctx, entry: &mut Indexed<Account>, _|
                                ctx.submit_command(MOVE_ACCOUNT.with((entry.index(), -1)))), 1.0)
                    .with_spacer(3.0)
                    .with_flex_child(
                        WidgetButton::new(Icon::new(ARROW_DROP_DOWN)
                            .expand_height()
                            .center())
                            .disabled_if(|entry: &Indexed<Account>, _: &_| entry.is_last())
                            .on_click(|ctx, entry: &mut Indexed<Account>, _|
                                ctx.submit_command(MOVE_ACCOUNT.with((entry.index(), 1)))), 1.0)
                    .fix_width(44.0)
            )
            .with_spacer(3.0)
            .with_child(
                WidgetButton::new(Icon::new(DELETE)
                    .expand_height()
                    .padding(3.0))
                    .on_click(|ctx, entry: &mut Indexed<Account>, _|
                        ctx.submit_command(DELETE_ACCOUNT.with(entry.index())))
                    )
    )
        .expand_width()
        .padding(8.0)
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
                    true => index.saturating_sub(offset.unsigned_abs() as usize),
                    false => index.saturating_add(offset.unsigned_abs() as usize),
                };
                data.database.accounts.swap(index, target);
            }
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}
