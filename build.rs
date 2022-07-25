
fn main() {

    #[cfg(windows)]
    {
        winres::WindowsResource::new()
            .set_icon_with_id("icon.ico", "icon")
            .set_language(0x0)
            .compile()
            .expect("Could not build windows resources");
    }

}