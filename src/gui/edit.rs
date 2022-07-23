use druid::{Widget, Lens, LensExt, Data, WidgetExt, Color, lens, Selector};
use druid::widget::{Button, Flex, Label, List, MainAxisAlignment, Scroll};
use crate::{Account, Database, MainState};
use crate::gui::util::{ListEntry, VectorWrapper};

pub const CLOSE_EDITOR_SAVE: Selector<EditState> = Selector::new("lol_account_manager_v2.edit.save");
pub const CLOSE_EDITOR_DISCARD: Selector<EditState> = Selector::new("lol_account_manager_v2.edit.discard");

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
                        ctx.submit_command(CLOSE_EDITOR_SAVE.with(state.clone()))).expand(), 1.0)
            .with_spacer(3.0)
            .with_flex_child(
                Button::new("Save")
                    .on_click(|ctx, state: &mut EditState ,_|
                        ctx.submit_command(CLOSE_EDITOR_SAVE.with(state.clone()))).expand(), 1.0)
            .with_spacer(3.0)
            .with_flex_child(
                Button::new("Discard")
                    .on_click(|ctx, state: &mut EditState ,_|
                        ctx.submit_command(CLOSE_EDITOR_DISCARD.with(state.clone()))).expand(), 1.0) //Button::new("O").expand()
            .padding(3.0)
            .border(Color::GRAY, 2.0)
            .rounded(5.0)
            .expand_width()
            .fix_height(50.0))
        .with_spacer(3.0)
        .with_flex_child(
            account_view()
                .expand()
                .padding(3.0)
                .border(Color::GRAY, 2.0)
                .rounded(5.0), 1.0)
        .padding(5.0)
}

fn account_view() -> impl Widget<EditState> {
    Scroll::new(List::new(|| {
        Flex::row()
            .with_flex_child(
                Label::new(|entry: &ListEntry<Account>, _: &_| format!("{}", entry.value().name))
                    .center()
                    .expand()
                    .padding(3.0), 1.0)
            .with_spacer(3.0)
            .with_child(Button::new("Edit")
                .on_click(|_,entry: &mut ListEntry<Account>,_| entry.value_mut().name.push('x'))
                .expand_height()
                .padding(3.0))
            .with_child(Flex::column()
                .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
                .with_flex_child(
                    Button::new("Up")
                        .disabled_if(|entry: &ListEntry<Account>, _: &_| entry.is_first())
                        .on_click(|_, entry: &mut ListEntry<Account>, _| entry.move_relative(-1)), 1.0)
                .with_flex_child(
                    Button::new("Down")
                        .disabled_if(|entry: &ListEntry<Account>, _: &_| entry.is_last())
                        .on_click(|_, entry: &mut ListEntry<Account>, _| entry.move_relative(1)), 1.0)
                .expand_height())
            .with_child(Button::new("Delete")
                .on_click(|_, entry: &mut ListEntry<Account>, _| entry.delete())
                .expand_height()
                .padding(3.0))
            .expand_width()
            .fix_height(60.0)
            .border(Color::GRAY, 2.0)
            .rounded(5.0)
    }).with_spacing(3.0))
        .vertical()
        .lens(lens::Identity.map(
            |d: &EditState| {
                //println!("get");
                VectorWrapper(d.database.accounts.clone())
            },
            |d: &mut EditState, x: VectorWrapper<Account>| {
                //println!("put");
                d.database.accounts = x.0
            },
        ))
}