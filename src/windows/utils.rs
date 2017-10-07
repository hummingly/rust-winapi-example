use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use windows::defintions::TextView;

use kernel32::GetLastError;
use user32::MessageBoxW;
use winapi::windef::{POINT, RECT};
use winapi::winuser::{MB_OK, MSG, PAINTSTRUCT};

pub trait ToWide {
    fn to_wide(&self) -> Vec<u16>;
}

impl ToWide for String {
    fn to_wide(&self) -> Vec<u16> {
        OsStr::new(self).encode_wide().chain(Some(0)).collect()
    }
}

impl ToWide for str {
    fn to_wide(&self) -> Vec<u16> {
        OsStr::new(self).encode_wide().chain(Some(0)).collect()
    }
}

pub fn error_msgbox(error_message: &str) {
    unsafe {
        let error_code = "Error: ".to_string() + &GetLastError().to_string();
        MessageBoxW(
            null_mut(),
            error_message.to_wide().as_ptr() as *const u16,
            error_code.to_wide().as_ptr() as *const u16,
            MB_OK,
        );
    };
}

//Defaults for defined WinAPI structs
pub trait WinStruct {
    fn initialized() -> Self;
}

impl WinStruct for MSG {
    fn initialized() -> Self {
        MSG {
            hwnd: null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        }
    }
}

impl WinStruct for PAINTSTRUCT {
    fn initialized() -> Self {
        PAINTSTRUCT {
            hdc: null_mut(),
            fErase: 0,
            rcPaint: WinStruct::initialized(),
            fRestore: 0,
            fIncUpdate: 0,
            rgbReserved: [0; 32],
        }
    }
}

impl WinStruct for RECT {
    fn initialized() -> Self {
        RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
}

impl WinStruct for TextView {
    fn initialized() -> Self {
        TextView {
            hfont: null_mut(),
            hfont_color: 0,
            hwnd: null_mut(),
        }
    }
}
