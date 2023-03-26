extern crate winapi;

use std::thread;
use std::time::Duration;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::iter::once;
use winapi::um::winnt::LPWSTR;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::LPARAM;
use winapi::um::winuser::{FindWindowW, SendMessageW, WM_KEYDOWN, WM_KEYUP};

fn main() {
    let window_title = "PS Remote Play";
    let hwnd = find_window_by_title(window_title);

    match hwnd {
        Some(handle) => {
            println!("HWND {:?}", handle);
            loop {
                send_enter_key(handle);
                thread::sleep(Duration::from_secs(5));
            } 
        },
        None => println!("Window not found."),
    }

    
}

fn find_window_by_title(title: &str) -> Option<winapi::shared::windef::HWND> {
    let wide_title: Vec<u16> = OsStr::new(title).encode_wide().chain(once(0)).collect();
    let hwnd = unsafe { FindWindowW(null_mut(), wide_title.as_ptr() as LPWSTR) };

    if hwnd.is_null() {
        None
    } else {
        Some(hwnd)
    }
}

fn send_enter_key(hwnd: HWND) {
    let enter_key = 0x20; // VK_RETURN
    let key_down = WM_KEYDOWN;
    let key_up = WM_KEYUP;
    let lparam = 0x1C0001 as LPARAM;

    unsafe {
        SendMessageW(hwnd, key_down, enter_key, lparam);
        SendMessageW(hwnd, key_up, enter_key, lparam);
    }

    println!("Enter key sent to the window.");
}
