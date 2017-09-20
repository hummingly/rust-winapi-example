use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use kernel32::GetLastError;
use user32::MessageBoxW;
use winapi::winuser::MB_OK;

pub fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

pub fn error_msgbox(error_message: &str) -> String {
    unsafe {
        let error_code = "Error: ".to_string() + &GetLastError().to_string();
        MessageBoxW(
            null_mut(),
            win32_string(error_message).as_ptr() as *const u16,
            win32_string(&error_code).as_ptr() as *const u16,
            MB_OK,
        );
        return error_code;
    };
}
