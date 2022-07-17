#![windows_subsystem = "windows"]

mod icons;
mod button;

use druid::widget::prelude::*;
use druid::widget::{Button, Either, Flex, Label, List, ListIter, MainAxisAlignment, Scroll, TextBox};
use druid::{AppLauncher, Color, LocalizedString, theme, WidgetExt, WindowDesc, Lens, lens, LensExt, TextAlignment};
use druid::im::{Vector};
use crate::button::IconButton;
use crate::icons::Icon;

#[derive(Clone, Data, Lens)]
struct AppData {
    filter: String,
    edit_mode: bool,
    accounts: Vector<String>
}

pub fn main() {
    let window = WindowDesc::new(build_widget())
        .window_size((400.0, 600.0))
        .title(LocalizedString::new("scroll-demo-window-title").with_placeholder("Scroll demo"));
    AppLauncher::with_window(window)
        .configure_env(|env, _| {
            env.set(theme::BUTTON_DARK, Color::AQUA);
            env.set(theme::BUTTON_LIGHT, Color::AQUA);
            env.set(theme::DISABLED_BUTTON_DARK, Color::WHITE);
            env.set(theme::DISABLED_BUTTON_LIGHT, Color::WHITE);
            env.set(theme::TEXT_COLOR, Color::BLACK);
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::WHITE);
            env.set(theme::BACKGROUND_LIGHT, Color::WHITE);
            env.set(theme::CURSOR_COLOR, Color::BLACK);
            env.set(theme::TEXTBOX_BORDER_WIDTH, 0.0);

        })
        .launch(AppData {
            filter: "".to_string(),
            edit_mode: false,
            accounts: NAMES.iter().map(|s| s.to_string()).collect()
        })
        .expect("launch failed");
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
                .on_click(|_,_,_| println!("settings")),
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

#[derive(Clone)]
struct VectorWrapper<T: Data>(Vector<T>);

impl<T: Data> Data for VectorWrapper<T> {
    fn same(&self, other: &Self) -> bool {
        self.0.same(&other.0)
    }
}

impl<T: Data> ListIter<ListEntry<T>> for VectorWrapper<T> {
    fn for_each(&self, mut cb: impl FnMut(&ListEntry<T>, usize)) {
        for (i, item) in self.0.iter().enumerate() {
            let d = ListEntry {
                list: self.0.to_owned(),
                cached_item: item.to_owned(),
                index: i
            };
            cb(&d, i);
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut ListEntry<T>, usize)) {
        for (i, item) in self.0.clone().iter().enumerate() {
            let mut d = ListEntry {
                list: self.0.clone(),
                cached_item: item.to_owned(),
                index: i
            };
            cb(&mut d, i);
            if !self.0.same(&d.list){
                println!("updating right");
                self.0 = d.list;
            }
        }
    }

    fn data_len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Data)]
struct ListEntry<T: Data>{
    list: Vector<T>,
    cached_item: T,
    index: usize
}

impl<T: Data> ListEntry<T> {

    pub fn value(&self) -> &T {
        &self.cached_item
    }

    pub fn is_first(&self) -> bool {
        self.index == 0
    }

    pub fn is_last(&self) -> bool  {
        self.index == self.list.len() - 1
    }

    pub fn value_mut(&mut self) -> &mut T {
        println!("invalidate cache");
        &mut self.list[self.index]
    }

    pub fn delete(&mut self) {
        println!("invalidate item");
        self.list.remove(self.index);
    }

    pub fn swap(&mut self, new_index: usize){
        println!("invalidate cache");
        self.list.swap(self.index, new_index);
    }

    pub fn move_relative(&mut self, offset: i32) {
        if offset.is_negative(){
            self.swap(self.index.saturating_sub(offset.abs() as usize));
        } else {
            self.swap(self.index.saturating_add(offset.abs() as usize));
        }

    }

}

const NAMES: &[&str] = &[
    "Allegra",
    "Bree",
    "Bryna",
    "Carrissa",
    "Clair",
    "Cleopatra",
    "Corinna",
    "Dacia",
    "Dinah",
    "Dionis",
    "Ermentrude",
    "Felicdad",
    "Flori",
    "Geneva",
    "Gussie",
    "Jazmin",
    "Jeannette",
    "Jenine",
    "Joann",
    "Layney",
    "Leona",
    "Lizzy",
    "Lucita",
    "Lyndsey",
    "Marcelia",
    "Marlene",
    "Mirabelle",
    "Nanci",
    "Nyssa",
    "Ora",
    "Paula",
    "Phyllis",
    "Prissie",
    "Riannon",
    "Roby",
    "Salomi",
    "Shaylyn",
    "Shela",
    "Siobhan",
    "Sioux",
    "Susanetta",
    "Tallulah",
    "Tiffi",
    "Valentine",
    "Van",
    "Verine",
    "Wallie",
    "Yvonne",
];
