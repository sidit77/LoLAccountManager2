mod widgets;
mod util;
mod settings;

use druid::{Data, Event, Selector, TextAlignment, Widget, WidgetExt, Lens, Color, lens, EventCtx, Env, LensExt};
use druid::im::Vector;
use druid::widget::{Button, Controller, Either, Flex, Label, List, MainAxisAlignment, Scroll, TextBox};
use druid_enums::Matcher;
use crate::gui::settings::{build_settings_ui, SETTINGS_SAVE, SettingsState};
use crate::gui::util::{ListEntry, VectorWrapper};
use crate::gui::widgets::button::IconButton;
use crate::gui::widgets::icons;


const OPEN_SETTINGS: Selector<SettingsState> = Selector::new("lol_account_manager_v2.main.settings");


#[derive(Clone, Data, Lens)]
pub struct Settings {
    pub close_on_login: bool
}

#[derive(Clone, Data, Lens)]
pub struct AppData {
    pub settings: Settings,
    pub filter: String,
    pub edit_mode: bool,
    pub accounts: Vector<String>
}

#[derive(Clone, Data, Matcher)]
#[matcher(matcher_name = App)]
pub enum AppState {
    Settings(SettingsState),
    Main(AppData)
}

pub fn ui() -> impl Widget<AppState> {
    App::new()
        .main(build_widget())
        .settings(build_settings_ui())
        .controller(AppController)
}

struct AppController;
impl Controller<AppState, App> for AppController {
    fn event(&mut self, child: &mut App, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(OPEN_SETTINGS) => {
                let settings_state= cmd.get_unchecked(OPEN_SETTINGS);
                *data = AppState::Settings(settings_state.clone());
            },
            Event::Command(cmd) if cmd.is(SETTINGS_SAVE) => {
                let settings_state= cmd.get_unchecked(SETTINGS_SAVE);
                let mut main = settings_state.previous.clone();
                main.settings = settings_state.settings.clone();
                *data = AppState::Main(main);
            }
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}

fn build_widget() -> impl Widget<AppData> {
    Either::new(|data: &AppData, _ | !data.edit_mode,
                main_layout(
                    TextBox::new()
                        .with_text_alignment(TextAlignment::Center)
                        .with_placeholder("Search...")
                        .lens(AppData::filter)
                        .expand_width(),
                    IconButton::new(&icons::EDIT)
                        .on_click(|_, mode: &mut bool ,_| *mode = true)
                        .lens(AppData::edit_mode),
                    IconButton::new(&icons::PREFERENCES)
                        .on_click(|ctx,state: &mut AppData,_|
                            ctx.submit_command(OPEN_SETTINGS.with(SettingsState {
                                previous: state.clone(),
                                settings: state.settings.clone()
                            }))),
                    standard_account_view()
                ),
                main_layout(
                    Button::new("New Entry")
                        .expand(),
                    Button::new("Y")
                        .on_click(|_, mode: &mut bool ,_| *mode = false)
                        .expand()
                        .lens(AppData::edit_mode),
                    Button::new("N")
                        .on_click(|_, mode: &mut bool ,_| *mode = false)
                        .expand()
                        .lens(AppData::edit_mode),
                    edit_account_view()
                )
    )
}


fn main_layout(h1: impl Widget<AppData> + 'static,
               h2: impl Widget<AppData> + 'static,
               h3: impl Widget<AppData> + 'static,
               body: impl Widget<AppData> + 'static) -> impl Widget<AppData> {
    Flex::column()
        .with_child(Flex::row()
            .with_flex_child(h1
                                 .center()
                                 .expand()
                                 .padding(3.0)
                                 .border(Color::GRAY, 2.0)
                                 .rounded(5.0), 1.0)
            .with_spacer(3.0)
            .with_child(Flex::row()
                .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
                .with_flex_child(h2, 1.0)
                .with_spacer(3.0)
                .with_flex_child(h3, 1.0) //Button::new("O").expand()
                .fix_width(83.0)
                .expand_height()
                .padding(3.0)
                .border(Color::GRAY, 2.0)
                .rounded(5.0))
            .expand_width()
            .fix_height(50.0))
        .with_spacer(3.0)
        .with_flex_child(body
                             .expand()
                             .padding(3.0)
                             .border(Color::GRAY, 2.0)
                             .rounded(5.0), 1.0)
        .padding(5.0)
}

fn standard_account_view() -> impl Widget<AppData> {
    Scroll::new(List::new(|| {
        Button::new(|item: &String, _: &_| format!("{}", item))
            .on_click(|_ctx, acc: &mut String, _env| {
                println!("Login: {}", acc);
            })
            .padding(3.0)
            .expand()
            .height(50.0)
    }))
        .vertical()
        .lens(lens::Identity.map(
            |d: &AppData| d.accounts
                .iter()
                .filter(|s|s
                    .to_lowercase()
                    .contains(&d.filter.to_lowercase()))
                .cloned()
                .collect(),
            |_, _: Vector<String>| {},
        ))
}

fn edit_account_view() -> impl Widget<AppData> {
    Scroll::new(List::new(|| {
        Flex::row()
            .with_flex_child(Label::new(|entry: &ListEntry<String>, _: &_| format!("{}", entry.value()))
                                 .center()
                                 .expand()
                                 .padding(3.0), 1.0)
            .with_spacer(3.0)
            .with_child(Button::new("Edit")
                .on_click(|_,entry: &mut ListEntry<String>,_| entry.value_mut().push('x'))
                .expand_height()
                .padding(3.0))
            .with_child(Flex::column()
                .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
                .with_flex_child(Button::new("Up")
                                     .disabled_if(|entry: &ListEntry<String>, _: &_| entry.is_first())
                                     .on_click(|_, entry: &mut ListEntry<String>, _| entry.move_relative(-1)), 1.0)
                .with_flex_child(Button::new("Down")
                                     .disabled_if(|entry: &ListEntry<String>, _: &_| entry.is_last())
                                     .on_click(|_, entry: &mut ListEntry<String>, _| entry.move_relative(1)), 1.0)
                .expand_height())
            .with_child(Button::new("Delete")
                .on_click(|_, entry: &mut ListEntry<String>, _| entry.delete())
                .expand_height()
                .padding(3.0))
            .expand_width()
            .fix_height(60.0)
            .border(Color::GRAY, 2.0)
            .rounded(5.0)
    }).with_spacing(3.0))
        .vertical()
        .lens(lens::Identity.map(
            |d: &AppData| {
                //println!("get");
                VectorWrapper(d.accounts.clone())
            },
            |d: &mut AppData, x: VectorWrapper<String>| {
                //println!("put");
                d.accounts = x.0
            },
        ))
}

