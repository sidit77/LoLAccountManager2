use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::anyhow;
use druid::commands::{SAVE_FILE_AS, SHOW_SAVE_PANEL};
use druid::theme::{BORDER_DARK, TEXTBOX_BORDER_RADIUS, TEXTBOX_BORDER_WIDTH};
use druid::widget::{Button, Checkbox, Controller, CrossAxisAlignment, Flex, Label, LineBreaking, RadioGroup};
use druid::{Data, Env, Event, EventCtx, FileDialogOptions, FileSpec, Lens, Widget, WidgetExt};

use crate::data::{Settings, Theme};
use crate::screens::main::MainState;
use crate::screens::setup::SetupState;
use crate::screens::{AppState, MainUi, Navigator};

const YAML: FileSpec = FileSpec::new("yaml file", &["yml", "yaml"]);
const TXT: FileSpec = FileSpec::new("text file", &["txt"]);

#[derive(Clone, Data, Lens)]
pub struct SettingsState {
    pub previous: MainState,
    pub settings: Settings
}

impl SettingsState {
    fn save(&self, ctx: &EventCtx) {
        let settings = self.settings.clone();
        ctx.get_external_handle()
            .add_idle_callback(move |ui: &mut MainUi| {
                settings.save().unwrap();
                ui.settings = settings;
            })
    }

    fn export_txt(&self, path: &Path) -> anyhow::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        for account in &self.previous.database.accounts {
            writeln!(writer, "Name: {}", account.name)?;
            writeln!(writer, "Username: {}", account.username)?;
            writeln!(writer, "Password: {}", account.password)?;
            writeln!(writer, "Notes:\n{}", account.notes)?;
            writeln!(writer)?;
        }
        writer.flush()?;
        Ok(())
    }

    fn export_yml(&self, path: &Path) -> anyhow::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_yaml::to_writer(writer, &self.previous.database.accounts)?;
        Ok(())
    }

    pub fn widget() -> impl Widget<Self> + 'static {
        build_settings_ui()
    }
}

impl From<SettingsState> for AppState {
    fn from(value: SettingsState) -> Self {
        Self::Settings(value)
    }
}

fn build_settings_ui() -> impl Widget<SettingsState> {
    Flex::column()
        .with_child(
            Flex::column()
                .with_child(after_login_ui().lens(Settings::close_on_login))
                .with_spacer(3.0)
                .with_child(theme_ui().lens(Settings::theme))
                .lens(SettingsState::settings)
        )
        .with_spacer(3.0)
        .with_child(database_ui())
        .with_spacer(3.0)
        .with_child(info_ui())
        .with_flex_spacer(1.0)
        .with_child(
            Button::new("Back")
                .on_click(|ctx, state: &mut SettingsState, _| {
                    state.save(ctx);
                    ctx.back();
                })
                .expand_width()
                .fix_height(50.0)
        )
        .padding(6.0)
        .expand()
}

fn after_login_ui() -> impl Widget<bool> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("After Login:"))
        .with_spacer(3.0)
        .with_child(Checkbox::new("Close the Program"))
        .padding(5.0)
        .expand_width()
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
}

fn theme_ui() -> impl Widget<Theme> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("Theme:"))
        .with_spacer(6.0)
        .with_child(RadioGroup::column([("Light", Theme::Light), ("Dark", Theme::Dark)]).padding((6.0, 0.0)))
        .padding(5.0)
        .expand_width()
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
}

fn database_ui() -> impl Widget<SettingsState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("Database:"))
        .with_spacer(6.0)
        .with_child(
            Flex::row()
                .with_flex_child(
                    Button::new("Change")
                        .on_click(|ctx, state: &mut SettingsState, _| {
                            state.save(ctx);
                            ctx.open(SetupState::new());
                        })
                        .expand_width(),
                    1.0
                )
                .with_spacer(3.0)
                .with_flex_child(
                    Button::new("Export")
                        .on_click(|ctx, _, _| {
                            ctx.submit_command(
                                SHOW_SAVE_PANEL.with(
                                    FileDialogOptions::new()
                                        .allowed_types(vec![YAML, TXT])
                                        .default_name("accounts")
                                        .default_type(TXT)
                                )
                            )
                        })
                        .expand_width(),
                    1.0
                )
        )
        .padding(5.0)
        .expand_width()
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
        .controller(Exporter)
}

fn info_ui() -> impl Widget<SettingsState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("Information:"))
        .with_spacer(6.0)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Label::new(concat!("Version: ", env!("CARGO_PKG_VERSION"))))
                .with_spacer(3.0)
                .with_child(
                    Flex::row()
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .with_child(Label::new("Database: "))
                        .with_flex_child(
                            Label::dynamic(|state: &SettingsState, _| state.previous.database.path.clone())
                                .with_line_break_mode(LineBreaking::WordWrap),
                            1.0
                        )
                )
                .padding((6.0, 0.0, 0.0, 0.0))
        )
        .padding(5.0)
        .expand_width()
        .border(BORDER_DARK, TEXTBOX_BORDER_WIDTH)
        .rounded(TEXTBOX_BORDER_RADIUS)
}

struct Exporter;

impl<W: Widget<SettingsState>> Controller<SettingsState, W> for Exporter {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut SettingsState, env: &Env) {
        if let Event::Command(cmd) = event {
            if let Some(file) = cmd.get(SAVE_FILE_AS) {
                let spec = file
                    .path
                    .extension()
                    .and_then(OsStr::to_str)
                    .and_then(|ext| {
                        [TXT, YAML]
                            .into_iter()
                            .find(|spec| spec.extensions.contains(&ext))
                    });
                match spec {
                    Some(TXT) => data.export_txt(&file.path),
                    Some(YAML) => data.export_yml(&file.path),
                    _ => Err(anyhow!("Unknown Format"))
                }
                .unwrap_or_else(|err| println!("export error: {}", err)) //TODO error popup
            }
        }
        child.event(ctx, event, data, env)
    }
}
