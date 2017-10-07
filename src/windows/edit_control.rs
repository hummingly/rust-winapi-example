use std::mem::size_of;
use std::ptr::{null, null_mut};
use windows::defintions::{TEXTVIEW_CLASS, TextView};
use windows::utils::{error_msgbox, ToWide, WinStruct};

use gdi32::{GetStockObject, SelectObject, SetBkMode, SetTextColor};
use kernel32::GetModuleHandleW;
use user32::*;
use winapi::basetsd::{INT32, UINT32, LONG_PTR};
use winapi::minwindef::{FALSE, LPARAM, LRESULT, TRUE, WPARAM};
use winapi::windef::{HBRUSH, HFONT, HGDIOBJ, HICON, HWND, RECT};
use winapi::wingdi::{DEFAULT_GUI_FONT, RGB, OPAQUE};
use winapi::winuser::*;

fn get_textview_ctrl(winstruct: &mut TextView) -> *mut TextView {
    winstruct as *mut TextView
}

fn set_textview_ctrl(hwnd: HWND, ctrl_ptr: *mut TextView) {
    unsafe {
        SetWindowLongPtrW(hwnd, 0, ctrl_ptr as LONG_PTR);
    }
}

pub fn init_textview_class() {
    unsafe {
        let wndcl = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as UINT32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(custom_wndproc),
            cbClsExtra: 0,
            cbWndExtra: size_of::<*mut TextView>() as INT32,
            hInstance: GetModuleHandleW(null_mut()),
            hIcon: 0 as HICON,
            hCursor: LoadCursorW(null_mut(), IDC_IBEAM),
            hbrBackground: COLOR_WINDOWFRAME as HBRUSH,
            lpszMenuName: null(),
            lpszClassName: TEXTVIEW_CLASS.to_wide().as_ptr() as *const u16,
            hIconSm: 0 as HICON,
        };

        if RegisterClassExW(&wndcl) == 0 {
            error_msgbox("Could not register TEXTVIEW_CLASS!");
        } else {
            RegisterClassExW(&wndcl);
        };
    }
}

pub fn create_textview_ctrl(hwndparent: HWND) -> HWND {
    unsafe {
        let hwnd_ctrl: HWND = CreateWindowExW(
            0,
            TEXTVIEW_CLASS.to_wide().as_ptr(),
            null_mut(),
            WS_VISIBLE | WS_CHILD | WS_BORDER,
            0,
            0,
            100,
            100,
            hwndparent,
            null_mut(),
            GetModuleHandleW(null_mut()),
            null_mut(),
        );
        hwnd_ctrl
    }
}

fn on_paint_textview(ctrl_ptr: &TextView) {
    let mut ps: PAINTSTRUCT = WinStruct::initialized();
    let mut rect: RECT = WinStruct::initialized();

    unsafe {
        GetClientRect(ctrl_ptr.hwnd, &mut rect as *mut RECT);

        let hdc = BeginPaint(ctrl_ptr.hwnd, &mut ps as *mut PAINTSTRUCT);

        let holdfont = SelectObject(hdc, ctrl_ptr.hfont as HGDIOBJ);

        SetTextColor(hdc, RGB(0, 0, 0));
        SetBkMode(hdc, OPAQUE);
        DrawTextW(
            hdc,
            "Hello World!".to_wide().as_ptr(),
            -1,
            &mut rect as *mut RECT,
            DT_CENTER | DT_SINGLELINE | DT_VCENTER,
        );
        SelectObject(hdc, holdfont);

        EndPaint(ctrl_ptr.hwnd, &ps);
    }
}

unsafe extern "system" fn custom_wndproc(
    hwnd: HWND,
    message: UINT32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let mut control: TextView = WinStruct::initialized();
    let ctrl_ptr = get_textview_ctrl(&mut control);
    match message {
        WM_NCCREATE => {
            if ctrl_ptr.is_null() {
                FALSE as LRESULT
            } else {
                set_textview_ctrl(hwnd, ctrl_ptr);
                TRUE as LRESULT
            }
        }
        WM_PAINT => {
            control.hfont = GetStockObject(DEFAULT_GUI_FONT) as HFONT;
            control.hfont_color = GetSysColor(COLOR_WINDOWTEXT);
            control.hwnd = hwnd;

            on_paint_textview(&control);
            0
        }
        WM_NCDESTROY => {
            UnregisterClassW(
                TEXTVIEW_CLASS.to_wide().as_ptr() as *const u16,
                GetModuleHandleW(null_mut()),
            );
            0
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}
