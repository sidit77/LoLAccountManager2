#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
pub use windows::login_account;