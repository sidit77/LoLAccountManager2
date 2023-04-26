use std::thread::spawn;
use druid::{Data, Env, LifeCycle, LifeCycleCtx, Widget, WidgetExt};
use druid::widget::{Controller, Flex, Label, Spinner};
use crate::data::{Database, Password, Settings};
use crate::screens::{AppState, MainUi, Navigator, Screen};
use crate::screens::main::MainState;
use crate::screens::setup::SetupState;

#[derive(Clone, Data)]
pub struct StartupState {
    pub settings: Settings
}

impl StartupState {
    
    pub fn new() -> Self {
        StartupState {
            settings: Settings::load().unwrap(),
        }
    }
    
}

impl From<StartupState> for AppState {
    fn from(value: StartupState) -> Self {
        AppState::Start(value)
    }
}

impl Screen for StartupState {
    fn widget() -> Box<dyn Widget<Self>> {
        Flex::column()
            .with_child(Spinner::new())
            .with_child(Label::new("Loading..."))
            .controller(LoadDatabase)
            .center()
            .boxed()
    }

    fn settings(&self) -> Settings {
        self.settings.clone()
    }
}

struct LoadDatabase;

impl<W: Widget<StartupState>> Controller<StartupState, W> for LoadDatabase {
    fn lifecycle(&mut self, child: &mut W, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &StartupState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            match data.settings.last_database.clone() {
                None => ctx.get_external_handle().open(SetupState::new(data.settings.clone())),
                Some(path) => {
                    let handle = ctx.get_external_handle();
                    let settings = data.settings.clone();
                    spawn(move || {
                        println!("Start loading database");
                        let password = Password::get(&path).unwrap();
                        let database = Database::load(&path, &password).unwrap();
                        handle.add_idle_callback(move |main: &mut MainUi| {
                            main.state = AppState::Main(MainState::new(settings, database));
                        });
                        println!("Finished loading database");
                    });
                }
            }
        }
        child.lifecycle(ctx, event, data, env)
    }
}