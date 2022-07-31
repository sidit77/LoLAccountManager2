use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::time::Duration;
use druid::{HasRawWindowHandle, RawWindowHandle, WindowHandle};
use crate::data::Account;

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
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{DWORD, UINT, WORD};
use winapi::shared::windef::RECT;
use winapi::um::winnt::LONG;
use winapi::um::winuser::{BringWindowToTop, FindWindowW, GetSystemMetrics, GetWindowRect, INPUT, INPUT_KEYBOARD, INPUT_MOUSE, IsWindowVisible, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE, MOUSEINPUT, SendInput, SM_CXSCREEN, SM_CYSCREEN, VK_LCONTROL, VK_RETURN, VK_TAB};

const VK_KEY_A: c_int = 0x41;

pub fn login_account(account: &Account) -> anyhow::Result<()> {
    unsafe {
        println!("Logging in...");
        //assert_winapi_success();

        let window = FindWindowW(ptr::null(), to_wstring("Riot Client Main").as_ptr());
        //assert_winapi_success();

        assert!(!window.is_null());
        assert_ne!(BringWindowToTop(window), 0);

        let mut rct: RECT = std::mem::zeroed();
        assert_ne!(GetWindowRect(window, &mut rct), 0);
        assert_ne!(IsWindowVisible(window), 0);
        //assert_winapi_success();

        std::thread::sleep(Duration::from_millis(100));

        let sx = 65536 / GetSystemMetrics(SM_CXSCREEN);
        let sy = 65536 / GetSystemMetrics(SM_CYSCREEN);

       let mut input = Vec::new();

        input.push(get_mouse_event(
            sx * mix(rct.left, rct.right, 0.14),
            sy * mix(rct.top, rct.bottom, 0.13),
            MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE
        ));

        input.push(get_mouse_event(0,0, MOUSEEVENTF_LEFTDOWN));
        input.push(get_mouse_event(0,0, MOUSEEVENTF_LEFTUP));

        input.push(get_keyboard_event(VK_TAB, 0, 0));
        input.push(get_keyboard_event(VK_TAB, 0, KEYEVENTF_KEYUP));

        input.push(get_keyboard_event(VK_LCONTROL, 0, 0));
        input.push(get_keyboard_event(VK_KEY_A, 0, 0));
        input.push(get_keyboard_event(VK_KEY_A, 0, KEYEVENTF_KEYUP));
        input.push(get_keyboard_event(VK_LCONTROL, 0, KEYEVENTF_KEYUP));

        for c in account.username.encode_utf16() {
            input.push(get_keyboard_event(0, c, KEYEVENTF_UNICODE | 0));
            input.push(get_keyboard_event(0, c, KEYEVENTF_UNICODE | KEYEVENTF_KEYUP));
        }

        input.push(get_keyboard_event(VK_TAB, 0, 0));
        input.push(get_keyboard_event(VK_TAB, 0, KEYEVENTF_KEYUP));

        input.push(get_keyboard_event(VK_LCONTROL, 0, 0));
        input.push(get_keyboard_event(VK_KEY_A, 0, 0));
        input.push(get_keyboard_event(VK_KEY_A, 0, KEYEVENTF_KEYUP));
        input.push(get_keyboard_event(VK_LCONTROL, 0, KEYEVENTF_KEYUP));

        for c in account.password.encode_utf16() {
            input.push(get_keyboard_event(0, c, KEYEVENTF_UNICODE | 0));
            input.push(get_keyboard_event(0, c, KEYEVENTF_UNICODE | KEYEVENTF_KEYUP));
        }

        input.push(get_keyboard_event(VK_RETURN, 0, 0));
        input.push(get_keyboard_event(VK_RETURN, 0, KEYEVENTF_KEYUP));

        assert_eq!(
            input.len() as UINT,
            SendInput(input.len() as UINT, input.as_mut_ptr(), std::mem::size_of::<INPUT>() as c_int),
        );

        Ok(())
    }
}

fn mix(a: LONG, b: LONG, v: f32) -> LONG {
    a + ((b - a) as f32 * v) as LONG
}

fn get_mouse_event(x: LONG, y: LONG, m: DWORD) -> INPUT {
    unsafe {
        let mut input = INPUT {
            type_: INPUT_MOUSE,
            u: std::mem::zeroed()
        };
        *input.u.mi_mut() = MOUSEINPUT {
            dx: x,
            dy: y,
            mouseData: 0,
            dwFlags: m,
            time: 0,
            dwExtraInfo: 0
        };
        input
    }
}

fn get_keyboard_event(vk: c_int, scan: WORD, flags: DWORD) -> INPUT {
    unsafe {
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: std::mem::zeroed()
        };
        *input.u.ki_mut() = KEYBDINPUT {
            wVk: vk as WORD,
            wScan: scan,
            dwFlags: flags,
            time: 0,
            dwExtraInfo: 0
        };
        input
    }
}

pub fn set_window_icon(handle: &WindowHandle) {
    let raw_handle = handle.raw_window_handle();
    #[allow(clippy::single_match)]
    match raw_handle {
        RawWindowHandle::Win32(win_handle) => unsafe {
            let program_icon: isize = {
                let h_instance = GetModuleHandleW(ptr::null());

                LoadImageW(h_instance, to_wstring("icon").as_ptr(),
                    IMAGE_ICON, 0, 0, LR_SHARED | LR_DEFAULTSIZE | LR_VGACOLOR,
                ).cast::<HICON>() as isize
            };

            assert_winapi_success();

            SendMessageW(win_handle.hwnd.cast::<HWND__>(),
                WM_SETICON, ICON_SMALL as usize, program_icon);
            //assert_winapi_success();
            SendMessageW(win_handle.hwnd.cast::<HWND__>(),
                         WM_SETICON, ICON_BIG as usize, program_icon, );
            //assert_winapi_success();
        },
        _ => {}
    }
}

fn assert_winapi_success() {
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