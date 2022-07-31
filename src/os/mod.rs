#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
pub use windows::{set_window_icon, login_account};