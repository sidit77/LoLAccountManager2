#[cfg(windows)]
#[path = "windows.rs"]
pub mod platform;

pub use platform::login_account;
