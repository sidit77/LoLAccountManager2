use std::thread::spawn;

use druid::widget::{Controller, Flex, Label, Spinner};
use druid::{Data, Env, Event, EventCtx, LifeCycle, LifeCycleCtx, Selector, Target, Widget, WidgetExt};

use crate::data::{Database, Password};
use crate::screens::main::MainState;
use crate::screens::setup::SetupState;
use crate::screens::{AppState, MainUi, Navigator};

#[derive(Clone, Data)]
pub struct StartupState {}

impl StartupState {
    pub fn new() -> Self {
        StartupState {}
    }

    pub fn widget() -> impl Widget<Self> + 'static {
        Flex::column()
            .with_child(Label::new("Decrypting Database..."))
            .with_spacer(5.0)
            .with_child(Spinner::new())
            .controller(LoadDatabase)
            .center()
    }
}

impl From<StartupState> for AppState {
    fn from(value: StartupState) -> Self {
        AppState::Start(value)
    }
}

struct LoadDatabase;

const BRING_TO_FRONT: Selector = Selector::new("lam.focus");

impl<W: Widget<StartupState>> Controller<StartupState, W> for LoadDatabase {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut StartupState, env: &Env) {
        if let Event::Command(cmd) = event {
            if cmd.is(BRING_TO_FRONT) {
                ctx.window().set_always_on_top(true);
                ctx.window().set_always_on_top(false);
                ctx.set_handled();
            }
        }
        child.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, child: &mut W, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &StartupState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            let handle = ctx.get_external_handle();
            handle
                .clone()
                .add_idle_callback(move |ui: &mut MainUi| match ui.settings.last_database.clone() {
                    None => ui.open(SetupState::new()),
                    Some(path) => {
                        let force_focus = ui.settings.force_focus;
                        spawn(move || {
                            println!("Start loading database");
                            let database = Password::get(&path)
                                .map_err(anyhow::Error::from)
                                .and_then(|pw| Database::load(&path, &pw));
                            match database {
                                Ok(database) => handle.open(MainState::new(database)),
                                Err(err) => {
                                    handle.open_popup(err.into());
                                    handle.open(SetupState::new());
                                }
                            }
                            if force_focus {
                                handle
                                    .submit_command(BRING_TO_FRONT, (), Target::Auto)
                                    .unwrap_or_else(|err| println!("Could not bring window to front: {}", err));
                            }
                            println!("Finished loading database");
                        });
                    }
                });
        }
        child.lifecycle(ctx, event, data, env)
    }
}
