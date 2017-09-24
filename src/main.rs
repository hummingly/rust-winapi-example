#![windows_subsystem = "windows"]
extern crate gdi32;
extern crate kernel32;
extern crate user32;
extern crate winapi;

mod menu;
mod utility;

use menu::{create_menu, EditEntries, FileEntries, Menus};
use utility::{error_msgbox, win32_string};

use std::ptr::{null, null_mut};

use gdi32::GetStockObject;

use user32::{CreateMenu, CreateWindowExW, MessageBoxW, RegisterClassW, ShowWindow, UpdateWindow};
use user32::{DispatchMessageW, GetMessageW, PostQuitMessage, SendMessageW, TranslateMessage};
use user32::{DefWindowProcW, DestroyWindow, FindWindowExW, GetClientRect, MoveWindow, SetWindowPos};

use winapi::minwindef::{HINSTANCE, LPARAM, LRESULT, TRUE, UINT, WPARAM};
use winapi::windef::{HCURSOR, HFONT, HICON, HWND, POINT, RECT};
use winapi::wingdi::DEFAULT_GUI_FONT;
use winapi::winuser::{MB_YESNO, MSG, WNDCLASSW};
use winapi::winuser::{CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, ES_AUTOVSCROLL, ES_MULTILINE,
                      WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE, WS_VSCROLL};
use winapi::winuser::{SWP_NOZORDER, SW_SHOWDEFAULT, WM_CLOSE, WM_COMMAND, WM_COPY, WM_CREATE,
                      WM_CUT, WM_DESTROY, WM_PASTE, WM_SETFONT, WM_SIZE, WM_UNDO};

//Window application entry function WinMain
//(body could also be put into fn main() instead of creating a new function)
fn create_win_main() -> u32 {
    let class = win32_string("mainwindow"); //class name of window
    let title = win32_string("Project: HotJelly Marmelade");

    unsafe {
        let hinstance = 0 as HINSTANCE; //It's the same as calling null_mut() or zeroed()
        let wndclass = WNDCLASSW {
            //Structure of the window
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
                let hwnd_main = CreateWindowExW(
                    //Actual main window that is visible
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
                    //Messaging with OS
                    let mut msg = MSG {
                        //Initialize MSG struct with 0 (unsafer alternative: zeroed(), uninitialized())
                        hwnd: 0 as HWND,
                        message: 0,
                        wParam: 0,
                        lParam: 0,
                        time: 0,
                        pt: POINT { x: 0, y: 0 },
                    };
                    while GetMessageW(&mut msg as *mut MSG, 0 as HWND, 0, 0) != 0 {
                        TranslateMessage(&msg as *const MSG);
                        DispatchMessageW(&msg as *const MSG);
                    }
                    msg.wParam as u32
                }
            }
        }
    }
}

//Callback function, which processes incoming messages
unsafe extern "system" fn message_handler(
    hwnd_main: HWND, //Handle to main/parent window
    message: UINT,   //System-defined messages (e.g. WM_SIZE)
    wparam: WPARAM,  //More message specified information (e.g. User clicked menu item X)
    lparam: LPARAM,  //More message specified information
) -> LRESULT {
    let mut rc_client = RECT {
        //Initialize rect struct with 0
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    let hwnd_main_edit = FindWindowExW(
        hwnd_main,
        0 as HWND,
        win32_string("EDIT").as_ptr() as *const u16,
        null(),
    );

    match message {
        WM_CREATE => {
            let hinstance_child = 0 as HINSTANCE;
            let hwnd_edit = CreateWindowExW(
                0,
                win32_string("EDIT").as_ptr() as *const u16, //Needs to be class "Edit Control" (system defined)
                win32_string("").as_ptr(), //or else it will be a normal window without input
                WS_CHILD | WS_VISIBLE | WS_VSCROLL | ES_MULTILINE | ES_AUTOVSCROLL,
                1,
                1,   //Child of main window with vertical scrollbars and
                100, //edit control with multilines enabled
                100,
                hwnd_main,
                null_mut(),
                hinstance_child,
                null_mut(),
            );

            if hwnd_edit.is_null() {
                error_msgbox("Could not create Edit control!");
                0
            } else {
                GetClientRect(hwnd_main, &mut rc_client as *mut RECT);
                SetWindowPos(
                    hwnd_edit,
                    null_mut(),
                    0,
                    0,
                    rc_client.right,
                    rc_client.bottom,
                    SWP_NOZORDER,
                );

                let hfont = GetStockObject(DEFAULT_GUI_FONT) as HFONT; //Gets pretty font from system
                SendMessageW(hwnd_edit, WM_SETFONT, hfont as WPARAM, i64::from(TRUE)) //and sends as message "Tahoma"
            }
        }
        WM_SIZE => {
            if hwnd_main_edit.is_null() {
                error_msgbox("Failed to resize Edit control!");
                0
            } else {
                GetClientRect(hwnd_main, &mut rc_client as *mut RECT);
                MoveWindow(
                    hwnd_main_edit as HWND,
                    0, //Resizes child window with main window
                    0, //when the user resizes window
                    rc_client.right as i32,
                    rc_client.bottom as i32,
                    TRUE,
                );
                1
            }
        }
        WM_COMMAND => match Menus::from_wparam(wparam) {
            Some(Menus::File(FileEntries::New)) => return 0,
            Some(Menus::File(FileEntries::Open)) => return 0,
            Some(Menus::File(FileEntries::Save)) => return 0,
            Some(Menus::File(FileEntries::Export)) => {
                error_msgbox("Not implemented yet.");
                0
            }
            Some(Menus::File(FileEntries::Print)) => {
                error_msgbox("Not implemented yet.");
                0
            }
            Some(Menus::File(FileEntries::Quit)) => {
                PostQuitMessage(0);
                0
            }
            Some(Menus::Edit(EditEntries::Redo)) => {
                SendMessageW(hwnd_main_edit, WM_UNDO, 0, i64::from(TRUE)); //I need a better undo function...
                0
            }
            Some(Menus::Edit(EditEntries::Repeat)) => {
                SendMessageW(hwnd_main_edit, WM_UNDO, 0, i64::from(TRUE)); //I need a better undo function...
                0
            }
            Some(Menus::Edit(EditEntries::Cut)) => {
                SendMessageW(hwnd_main_edit, WM_CUT, 0, i64::from(TRUE));
                0
            }
            Some(Menus::Edit(EditEntries::Copy)) => {
                SendMessageW(hwnd_main_edit, WM_COPY, 0, i64::from(TRUE));
                0
            }
            Some(Menus::Edit(EditEntries::Paste)) => {
                SendMessageW(hwnd_main_edit, WM_PASTE, 0, i64::from(TRUE));
                0
            }
            Some(Menus::Edit(EditEntries::Search)) => {
                error_msgbox("Not implemented yet.");
                0
            }
            Some(Menus::Edit(EditEntries::Replace)) => {
                error_msgbox("Not implemented yet.");
                0
            }
            None => 1,
        },
        WM_CLOSE => {
            if (MessageBoxW(
                hwnd_main,
                win32_string("Are you sure you want to quit?").as_ptr() as *const u16,
                win32_string("Exit Marmelade").as_ptr() as *const u16,
                MB_YESNO,
            )) == 6
            //Yes Button was pressed (6 = IDYES)
            {
                DestroyWindow(hwnd_main);
                1 //(1 = IDOK)
            } else {
                2 //(2 = IDCANCEL)
            }
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd_main, message, wparam, lparam),
    }
}

fn main() {
    create_win_main();
}
