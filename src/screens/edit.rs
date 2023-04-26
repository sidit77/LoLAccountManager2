use std::thread::spawn;

use druid::theme::{BACKGROUND_LIGHT, BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{AspectRatioBox, Container, Controller, Flex, Label, List, MainAxisAlignment};
use druid::{lens, Data, Env, Event, EventCtx, Lens, LensExt, Selector, Widget, WidgetExt};
use druid_material_icons::normal::action::DELETE;
use druid_material_icons::normal::content::{ADD, SAVE};
use druid_material_icons::normal::image::EDIT;
use druid_material_icons::normal::navigation::{ARROW_DROP_DOWN, ARROW_DROP_UP, CLOSE};

use crate::data::{Account, Database};
use crate::screens::account::AccountState;
use crate::screens::main::MainState;
use crate::screens::popup::PopupState;
use crate::screens::{AppState, Navigator};
use crate::util::{icon_text_button, IndexWrapper, Indexed};
use crate::widgets::{Icon, WidgetButton};

const EDIT_ACCOUNT: Selector<usize> = Selector::new("lol_account_manager_v2.edit.edit");
const DELETE_ACCOUNT: Selector<usize> = Selector::new("lol_account_manager_v2.edit.delete");
const MOVE_ACCOUNT: Selector<(usize, i32)> = Selector::new("lol_account_manager_v2.edit.move");

#[derive(Clone, Data, Lens)]
pub struct EditState {
    pub previous: MainState,
    pub database: Database
}

impl From<MainState> for EditState {
    fn from(ms: MainState) -> Self {
        EditState {
            database: ms.database.clone(),
            previous: ms
        }
    }
}

impl EditState {
    fn unsaved_changes(&self) -> bool {
        self.database != self.previous.database
    }

    pub fn widget() -> impl Widget<Self> + 'static {
        build_edit_ui()
    }
}

impl From<EditState> for AppState {
    fn from(value: EditState) -> Self {
        AppState::Editor(value)
    }
}

fn build_edit_ui() -> impl Widget<EditState> {
    Flex::column()
        .with_child(
            Flex::row()
                .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
                .with_flex_child(
                    icon_text_button(ADD, "New")
                        .on_click(|ctx, state: &mut EditState, _| ctx.open(AccountState::new(state.clone())))
                        .expand(),
                    1.0
                )
                .with_spacer(3.0)
                .with_flex_child(
                    icon_text_button(SAVE, "Save")
                        .on_click(|ctx, state: &mut EditState, _| {
                            ctx.open_popup(PopupState::Saving(()));
                            let handle = ctx.get_external_handle();
                            let db = state.database.clone();
                            state.previous.database = db.clone();
                            spawn(move || {
                                db.save().unwrap();
                                handle.close_popup();
                                handle.back();
                            });
                        })
                        .disabled_if(|state: &EditState, _| !state.unsaved_changes())
                        .expand(),
                    1.0
                )
                .with_spacer(3.0)
                .with_flex_child(
                    icon_text_button(CLOSE, "Discard")
                        .on_click(|ctx, state: &mut EditState, _| match state.unsaved_changes() {
                            false => ctx.back(),
                            true => ctx.open_popup(PopupState::Leave(()))
                        })
                        .expand(),
                    1.0
                ) //Button::new("O").expand()
                .expand_width()
                .fix_height(50.0)
        )
        .with_spacer(3.0)
        .with_flex_child(
            List::new(item_ui)
                .with_spacing(3.0)
                .scroll()
                .vertical()
                .lens(lens::Identity.map(
                    |d: &EditState| IndexWrapper::from(d.database.accounts.clone()),
                    |d: &mut EditState, x: IndexWrapper<Account>| d.database.accounts = x.into()
                ))
                .controller(ListController)
                .expand()
                .padding(3.0)
                .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
                .rounded(TEXTBOX_BORDER_RADIUS),
            1.0
        )
        .padding(5.0)
}

fn item_ui() -> impl Widget<Indexed<Account>> {
    Container::new(
        Flex::row()
            .with_flex_child(
                Label::new(|entry: &Indexed<Account>, _: &_| entry.name.to_string())
                    .center()
                    .expand()
                    .padding(3.0),
                1.0
            )
            .with_spacer(3.0)
            .with_child(
                WidgetButton::new(Icon::new(EDIT).expand_height().padding(3.0))
                    .on_click(|ctx, entry: &mut Indexed<Account>, _| ctx.submit_command(EDIT_ACCOUNT.with(entry.index())))
            )
            .with_spacer(3.0)
            .with_child(AspectRatioBox::new(
                Flex::column()
                    .with_flex_child(
                        WidgetButton::new(Icon::new(ARROW_DROP_UP).expand_height().center())
                            .disabled_if(|entry: &Indexed<Account>, _: &_| entry.is_first())
                            .on_click(|ctx, entry: &mut Indexed<Account>, _| ctx.submit_command(MOVE_ACCOUNT.with((entry.index(), -1)))),
                        1.0
                    )
                    .with_spacer(3.0)
                    .with_flex_child(
                        WidgetButton::new(Icon::new(ARROW_DROP_DOWN).expand_height().center())
                            .disabled_if(|entry: &Indexed<Account>, _: &_| entry.is_last())
                            .on_click(|ctx, entry: &mut Indexed<Account>, _| ctx.submit_command(MOVE_ACCOUNT.with((entry.index(), 1)))),
                        1.0
                    ),
                1.0
            ))
            .with_spacer(3.0)
            .with_child(
                WidgetButton::new(Icon::new(DELETE).expand_height().padding(3.0))
                    .on_click(|ctx, entry: &mut Indexed<Account>, _| ctx.submit_command(DELETE_ACCOUNT.with(entry.index())))
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
        if let Event::Command(cmd) = event {
            if let Some(index) = cmd.get(EDIT_ACCOUNT).cloned() {
                ctx.open(AccountState::existing(data.clone(), index));
            }
            if let Some(index) = cmd.get(DELETE_ACCOUNT).cloned() {
                data.database.accounts.remove(index);
            }
            if let Some((index, offset)) = cmd.get(MOVE_ACCOUNT).cloned() {
                let target = match offset.is_negative() {
                    true => index.saturating_sub(offset.unsigned_abs() as usize),
                    false => index.saturating_add(offset.unsigned_abs() as usize)
                };
                data.database.accounts.swap(index, target);
            }
        }
        child.event(ctx, event, data, env)
    }
}
