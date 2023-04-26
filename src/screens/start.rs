use std::thread::spawn;
use druid::{Data, Env, LifeCycle, LifeCycleCtx, Widget, WidgetExt};
use druid::widget::{Controller, Flex, Label, Spinner};
use crate::data::{Database, Password};
use crate::screens::{AppState, MainUi, Navigator, Screen};
use crate::screens::main::MainState;
use crate::screens::setup::SetupState;

#[derive(Clone, Data)]
pub struct StartupState {
}

impl StartupState {
    
    pub fn new() -> Self {
        StartupState {}
    }

    pub fn widget() -> impl Widget<Self> + 'static {
        Flex::column()
            .with_child(Spinner::new())
            .with_child(Label::new("Loading..."))
            .controller(LoadDatabase)
            .center()
    }

}

impl From<StartupState> for AppState {
    fn from(value: StartupState) -> Self {
        AppState::Start(value)
    }
}

impl Screen for StartupState {

}

struct LoadDatabase;

impl<W: Widget<StartupState>> Controller<StartupState, W> for LoadDatabase {
    fn lifecycle(&mut self, child: &mut W, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &StartupState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            let handle = ctx.get_external_handle();
            handle.clone().add_idle_callback(move |ui: &mut MainUi| {
                match ui.settings.last_database.clone() {
                    None => ui.open(SetupState::new()),
                    Some(path) => {
                        spawn(move || {
                            println!("Start loading database");
                            let password = Password::get(&path).unwrap();
                            let database = Database::load(&path, &password).unwrap();
                            handle.add_idle_callback(move |main: &mut MainUi| {
                                main.state = AppState::Main(MainState::new(database));
                            });
                            println!("Finished loading database");
                        });
                    }
                }
            });
        }
        child.lifecycle(ctx, event, data, env)
    }
}