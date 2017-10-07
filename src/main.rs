#![windows_subsystem = "windows"]
extern crate gdi32;
extern crate kernel32;
extern crate user32;
extern crate winapi;

mod windows;

use std::ptr::{null, null_mut};
use windows::defintions::*;
use windows::edit_control::{init_textview_class, create_textview_ctrl};
use windows::menus::create_menu;
use windows::utils::{error_msgbox, ToWide, WinStruct};

use kernel32::GetModuleHandleW;
use user32::*;
use winapi::basetsd::{INT32, UINT32};
use winapi::minwindef::{LPARAM, LRESULT, TRUE, WPARAM};
use winapi::windef::{HCURSOR, HICON, HWND, RECT};
use winapi::winuser::*;

//Callback function, which processes incoming messages
unsafe extern "system" fn message_handler(
    hwnd_main: HWND, //Handle to main/parent window
    message: UINT32, //System-defined messages (e.g. WM_SIZE)
    wparam: WPARAM, //More message specified information (e.g. User clicked menu item X)
    lparam: LPARAM, //More message specified information
) -> LRESULT {
    let mut rc_client: RECT = WinStruct::initialized();

    let hwnd_textview = FindWindowExW(
        hwnd_main,
        0 as HWND,
        TEXTVIEW_CLASS.to_wide().as_ptr() as *const u16,
        null(),
    );

    match message {
        WM_CREATE => {
            init_textview_class();
            let hwnd_textview = create_textview_ctrl(hwnd_main);

            if hwnd_textview.is_null() {
                error_msgbox("Could not create Edit control!");
                0
            } else {
                0
            }
        }
        WM_SIZE => {
            if hwnd_textview.is_null() {
                error_msgbox("Cannot resize control!");
                0
            } else {
                GetClientRect(hwnd_main, &mut rc_client as *mut RECT);
                MoveWindow(
                    hwnd_textview,
                    0, //Resizes child window with main window
                    0, //when the user resizes window
                    rc_client.right as INT32,
                    rc_client.bottom as INT32,
                    TRUE,
                );
                1
            }
        }
        WM_CLOSE => {
            if MessageBoxW(
                hwnd_main,
                "Are you sure you want to quit?".to_wide().as_ptr() as *const u16,
                "Exit Marmelade".to_wide().as_ptr() as *const u16,
                MB_YESNO,
            ) == IDYES
            {
                DestroyWindow(hwnd_main);
                0
            } else {
                1
            }
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd_main, message, wparam, lparam),
    }
}

//Window application entry function WinMain
//(body could also be put into fn main() instead of creating a new function)
fn create_main_window(class: Vec<u16>, title: Vec<u16>) -> usize {
    unsafe {
        let hinstance = GetModuleHandleW(null_mut());

        //Structure of the window
        let wndclass = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(message_handler), //Calling callback function for processing messages
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: 0 as HICON,
            hCursor: 0 as HCURSOR,
            hbrBackground: null_mut(),
            lpszMenuName: null(),
            lpszClassName: class.as_ptr() as *const u16,
        };

        let hmenubar = CreateMenu();
        create_menu(hmenubar);

        match RegisterClassW(&wndclass) {
            0 => {
                error_msgbox("Failed to register window class!");
                0
            }
            _atom => {
                //Actual main window that is visible
                let hwnd_main = CreateWindowExW(
                    0,
                    class.as_ptr() as *const u16,
                    title.as_ptr() as *const u16,
                    WS_VISIBLE | WS_OVERLAPPEDWINDOW,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    null_mut(),
                    hmenubar,
                    hinstance,
                    null_mut(),
                );

                if hwnd_main.is_null() {
                    error_msgbox("Failed to create window!");
                    PostQuitMessage(0);
                    0
                } else {
                    ShowWindow(hwnd_main, SW_SHOWDEFAULT);
                    UpdateWindow(hwnd_main);
                    0
                }
            }
        }
    }
}

fn main() {
    create_main_window(
        "mainwindow".to_wide(),
        "Project: HotJelly Marmelade".to_wide(),
    );

    unsafe {
        let mut msg: MSG = WinStruct::initialized();

        while GetMessageW(&mut msg as *mut MSG, 0 as HWND, 0, 0) != 0 {
            TranslateMessage(&msg as *const MSG);
            DispatchMessageW(&msg as *const MSG);
        }
        let _app_message = msg.wParam as usize;
    }
}
