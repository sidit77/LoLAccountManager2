use std::fmt::{Display, Formatter};
use druid::{Widget, Data, Lens, WidgetExt, EventCtx, Event, Env, FileDialogOptions, FileSpec};
use druid::theme::{BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Button, Controller, CrossAxisAlignment, Flex, Label, Maybe, RadioGroup};
use druid_widget_nursery::ComputedWidget;
use druid_widget_nursery::enum_switcher::Switcher;
use druid_widget_nursery::prism::Prism;
use crate::AppState;
use crate::data::{Settings, Theme};
use crate::screens::main::MainState;
use crate::screens::Screen;
use crate::util::{password_field, path_field, PathOptions};

const YAML: FileSpec = FileSpec::new("yaml file", &[".yml", ".yaml"]);

#[derive(Clone, Data, Lens)]
pub struct SetupState {
    pub settings: Settings,
    state: ActionState
}

impl Into<AppState> for SetupState {
    fn into(self) -> AppState {
        AppState::Setup(self)
    }
}

impl Screen for SetupState {
    fn widget() -> Box<dyn Widget<Self>> {
        Box::new(build_setup_ui())
    }

    fn theme(&self) -> Theme {
        self.settings.theme
    }
}

impl SetupState {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            state: ActionState::Create(Default::default())
        }
    }
}

#[derive(Clone, Data, Prism)]
enum ActionState {
    Create(CreateState),
    Open(OpenState),
    Import(ImportState)
}

impl PartialEq for ActionState {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

#[derive(Clone, Data, Default, Eq, PartialEq, Lens)]
struct CreateState {
    path: String,
    password1: String,
    password2: String
}

fn build_create_ui() -> impl Widget<CreateState> {
    let options = FileDialogOptions::new()
        .allowed_types(vec![YAML])
        .default_name("database.yml");
    Flex::column()
        .with_child(path_field("Destination:", PathOptions::Save(options)).lens(CreateState::path))
        .with_spacer(3.0)
        .with_child(password_field("Password:").lens(CreateState::password1))
        .with_spacer(3.0)
        .with_child(password_field("Repeat Password:").lens(CreateState::password2))
}

#[derive(Clone, Data, Default, Eq, PartialEq, Lens)]
struct OpenState {
    path: String,
    password: String,
}

fn build_open_ui() -> impl Widget<OpenState> {
    let options = FileDialogOptions::new()
        .allowed_types(vec![YAML])
        .default_name("database.yml");
    Flex::column()
        .with_child(path_field("Location:", PathOptions::Open(options)).lens(OpenState::path))
        .with_spacer(3.0)
        .with_child(password_field("Password:").lens(OpenState::password))
}

#[derive(Clone, Data, Default, Eq, PartialEq, Lens)]
struct ImportState {
    input_path: String,
    output_path: String,
    password1: String,
    password2: String
}

fn build_import_ui() -> impl Widget<ImportState> {
    let source = FileDialogOptions::new()
        .allowed_types(vec![YAML])
        .default_name("database.yml");
    let destination = FileDialogOptions::new()
        .allowed_types(vec![YAML])
        .default_name("database.yml");
    Flex::column()
        .with_child(path_field("Source:", PathOptions::Open(source)).lens(ImportState::input_path))
        .with_spacer(3.0)
        .with_child(path_field("Path:", PathOptions::Save(destination)).lens(ImportState::output_path))
        .with_spacer(3.0)
        .with_child(password_field("Password:").lens(ImportState::password1))
        .with_spacer(3.0)
        .with_child(password_field("Repeat Password:").lens(ImportState::password2))
}

fn build_setup_ui() -> impl Widget<SetupState> {
    Flex::column()
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Label::new("Action:"))
                .with_spacer(6.0)
                .with_child(RadioGroup::column([
                    ("Create a new database", ActionState::Create(Default::default())),
                    ("Open an exising database", ActionState::Open(Default::default())),
                    ("Import an unencrypted database", ActionState::Import(Default::default()))])
                    .padding((6.0, 0.0)))
                .padding(3.0)
                .expand_width()
                .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
                .rounded(TEXTBOX_BORDER_RADIUS)
                .lens(SetupState::state)
        )
        .with_spacer(3.0)
        .with_child(
            Switcher::new()
                .with_variant(ActionStateCreate, build_create_ui())
                .with_variant(ActionStateOpen, build_open_ui())
                .with_variant(ActionStateImport, build_import_ui())
                .lens(SetupState::state)
        )
        .with_flex_spacer(1.0)
        .with_child(
            ComputedWidget::new(
                Maybe::or_empty(|| Label::dynamic(|msg: &VerificationError, _| msg.to_string())),
                |state: &ActionState| state.check().err()
            )
            .lens(SetupState::state)
        )
        .with_spacer(3.9)
        .with_child(
            Button::new("Confirm")
                .expand_width()
                .fix_height(50.0)
                .on_click(|ctx, state: &mut SetupState, _| {
                    let new = MainState {
                        settings: state.settings.clone(),
                        filter: "".to_string(),
                        database: Default::default(),
                    };
                    state.open(ctx, new);
                })
                .disabled_if(|state: &SetupState, _| state.state.check().is_err())
        )
        .padding(6.0)
        .expand()
        .controller(SetupController)
}

struct SetupController;
impl<W: Widget<SetupState>> Controller<SetupState, W> for SetupController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut SetupState, env: &Env) {
        //match event {
        //    Event::Command(cmd) if cmd.is(SETUP_ACTION_CHANGED) => {
        //        //event.should_propagate_to_hidden();
        //        //ctx.children_changed();
        //    }
        //    _ => {}
        //}
        child.event(ctx, event, data, env)
    }
}

#[derive(Debug, Copy, Clone, Data, Eq, PartialEq)]
enum VerificationError {
    EmptyPath,
    EmptyPassword,
    MismatchedPasswords
}

impl Display for VerificationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationError::EmptyPath => f.write_str("Paths can't be empty!"),
            VerificationError::EmptyPassword => f.write_str("Passwords can't be empty!"),
            VerificationError::MismatchedPasswords => f.write_str("The passwords don't match!"),
        }
    }
}

impl ActionState {
    fn check(&self) ->  Result<(), VerificationError> {
        match self {
            ActionState::Create(state) => {
                check_path(&state.path)?;
                check_password(&state.password1)?;
                check_passwords(&state.password1, &state.password2)?;
                Ok(())
            }
            ActionState::Open(state) => {
                check_path(&state.path)?;
                check_password(&state.password)?;
                Ok(())
            }
            ActionState::Import(state) => {
                check_path(&state.input_path)?;
                check_path(&state.output_path)?;
                check_password(&state.password1)?;
                check_passwords(&state.password1, &state.password2)?;
                Ok(())
            }
        }
    }
}

fn check_path(path: &str) -> Result<(), VerificationError> {
    if path.is_empty() {
        return Err(VerificationError::EmptyPath);
    }
    Ok(())
}

fn check_password(password: &str) ->  Result<(), VerificationError> {
    if password.is_empty() {
        return Err(VerificationError::EmptyPassword);
    }
    Ok(())
}

fn check_passwords(password1: &str, password2: &str) ->  Result<(), VerificationError> {
    if password1.ne(password2) {
        return Err(VerificationError::MismatchedPasswords);
    }
    Ok(())
}