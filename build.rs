
#[cfg(windows)]
fn main() {
    tauri_winres::WindowsResource::new()
        .set_icon_with_id("icon.ico", "1")
        .set_language(0x0)
        .compile()
        .expect("Could not build windows resources");
}

#[cfg(not(windows))]
fn main() {

}