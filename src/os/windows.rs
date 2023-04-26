use std::mem::size_of;
use std::time::Duration;

use anyhow::ensure;
use windows::core::PCWSTR;
use windows::w;
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::data::Account;

#[allow(clippy::vec_init_then_push)]
pub fn login_account(account: &Account) -> anyhow::Result<()> {
    unsafe {
        println!("Logging in...");

        let window = FindWindowW(PCWSTR::null(), w!("Riot Client Main"));
        ensure!(window.0 != 0);
        BringWindowToTop(window).ok()?;

        let mut rct = RECT::default();
        GetWindowRect(window, &mut rct).ok()?;
        IsWindowVisible(window).ok()?;

        std::thread::sleep(Duration::from_millis(100));

        let sx = 65536 / GetSystemMetrics(SM_CXSCREEN);
        let sy = 65536 / GetSystemMetrics(SM_CYSCREEN);

        let mut input = Vec::new();

        input.push(get_mouse_event(
            sx * mix(rct.left, rct.right, 0.14),
            sy * mix(rct.top, rct.bottom, 0.13),
            MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE
        ));

        input.push(get_mouse_event(0, 0, MOUSEEVENTF_LEFTDOWN));
        input.push(get_mouse_event(0, 0, MOUSEEVENTF_LEFTUP));

        input.push(get_keyboard_event(VK_TAB, 0, None));
        input.push(get_keyboard_event(VK_TAB, 0, KEYEVENTF_KEYUP));

        input.push(get_keyboard_event(VK_LCONTROL, 0, None));
        input.push(get_keyboard_event(VK_A, 0, None));
        input.push(get_keyboard_event(VK_A, 0, KEYEVENTF_KEYUP));
        input.push(get_keyboard_event(VK_LCONTROL, 0, KEYEVENTF_KEYUP));

        for c in account.username.encode_utf16() {
            input.push(get_keyboard_event(VIRTUAL_KEY::default(), c, KEYEVENTF_UNICODE));
            input.push(get_keyboard_event(VIRTUAL_KEY::default(), c, KEYEVENTF_UNICODE | KEYEVENTF_KEYUP));
        }

        input.push(get_keyboard_event(VK_TAB, 0, None));
        input.push(get_keyboard_event(VK_TAB, 0, KEYEVENTF_KEYUP));

        input.push(get_keyboard_event(VK_LCONTROL, 0, None));
        input.push(get_keyboard_event(VK_A, 0, None));
        input.push(get_keyboard_event(VK_A, 0, KEYEVENTF_KEYUP));
        input.push(get_keyboard_event(VK_LCONTROL, 0, KEYEVENTF_KEYUP));

        for c in account.password.encode_utf16() {
            input.push(get_keyboard_event(VIRTUAL_KEY::default(), c, KEYEVENTF_UNICODE));
            input.push(get_keyboard_event(VIRTUAL_KEY::default(), c, KEYEVENTF_UNICODE | KEYEVENTF_KEYUP));
        }

        input.push(get_keyboard_event(VK_RETURN, 0, None));
        input.push(get_keyboard_event(VK_RETURN, 0, KEYEVENTF_KEYUP));

        let sent = SendInput(&input, size_of::<INPUT>() as i32) as usize;
        ensure!(sent == input.len());

        Ok(())
    }
}

fn mix(a: i32, b: i32, v: f32) -> i32 {
    a + ((b - a) as f32 * v) as i32
}

fn get_mouse_event(x: i32, y: i32, m: MOUSE_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: x,
                dy: y,
                mouseData: 0,
                dwFlags: m,
                time: 0,
                dwExtraInfo: 0
            }
        }
    }
}

fn get_keyboard_event(vk: VIRTUAL_KEY, scan: u16, flags: impl Into<Option<KEYBD_EVENT_FLAGS>>) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: scan,
                dwFlags: flags.into().unwrap_or_default(),
                time: 0,
                dwExtraInfo: 0
            }
        }
    }
}
