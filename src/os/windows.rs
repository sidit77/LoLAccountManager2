use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use druid::{HasRawWindowHandle, RawWindowHandle, WindowHandle};

use winapi::{
    shared::{
        windef::{HICON, HWND__},
    },
    um::{
        libloaderapi::GetModuleHandleW,
        winuser::{
            LoadImageW,
            SendMessageW,
            ICON_BIG,
            ICON_SMALL,
            IMAGE_ICON,
            LR_DEFAULTSIZE,
            LR_SHARED,
            LR_VGACOLOR,
            WM_SETICON,
        },
    },
};

pub fn set_window_icon(handle: &WindowHandle) {
    let raw_handle = handle.raw_window_handle();
    #[allow(clippy::single_match)]
    match raw_handle {
        RawWindowHandle::Win32(win_handle) => unsafe {
            #[cfg(windows)]
            {

                let program_icon: isize = {
                    // Passing NULL means the executable file is selected
                    let h_instance = GetModuleHandleW(ptr::null());

                    LoadImageW(
                        h_instance,
                        to_wstring("icon").as_ptr(),
                        IMAGE_ICON,
                        0,
                        0,
                        LR_SHARED | LR_DEFAULTSIZE | LR_VGACOLOR,
                    ).cast::<HICON>() as isize
                };

                assert_winapi_success();

                // Shown at the top of the window
                SendMessageW(
                    win_handle.hwnd.cast::<HWND__>(),
                    WM_SETICON,
                    ICON_SMALL as usize,
                    program_icon,
                );
                //assert_winapi_success();
                // Shown in the Alt+Tab dialog
                SendMessageW(
                    win_handle.hwnd.cast::<HWND__>(),
                    WM_SETICON,
                    ICON_BIG as usize,
                    program_icon,
                );
                //assert_winapi_success();
            }
        },
        _ => {}
    }
}

pub fn assert_winapi_success() {
    #[cfg(debug_assertions)]
    {
        let last_error = unsafe { winapi::um::errhandlingapi::GetLastError() };
        assert_eq!(
            winapi::shared::winerror::ERROR_SUCCESS, last_error,
            "the last WinAPI call failed with error code: {:#010X}",
            last_error
        );
    }
}

fn to_wstring(str: &str) -> Vec<u16> {
    OsStr::new(str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
}