use std::ptr::null;

use user32::{AppendMenuW, CreateMenu, CreatePopupMenu};

use utility::{error_msgbox, win32_string};

use winapi::windef::HMENU;
use winapi::minwindef::WPARAM;
use winapi::winuser::{MF_POPUP, MF_SEPARATOR, MF_STRING};

pub enum Menus {
    File(FileEntries),
    Edit(EditEntries),
    /*Guide(GuideEntries),
    Help(HelpEntries),
    Others(OthersEntries),*/
}

pub enum FileEntries {
    New = 0x000,
    Open = 0x001,
    Save = 0x002,
    Export = 0x003,
    Print = 0x004,
    Quit = 0x005,
}

pub enum EditEntries {
    Redo = 0x100,
    Repeat = 0x101,
    Cut = 0x102,
    Copy = 0x103,
    Paste = 0x104,
    Search = 0x105,
    Replace = 0x106,
}

pub enum GuideEntries {
    Run = 0x200,
    Review = 0x201,
    Publish = 0x202,
    Info = 0x203,
    Options = 0x204,
}

pub enum HelpEntries {
    Guide = 0x300,
    Doc = 0x301,
    Champions = 0x302,
    Items = 0x303,
    Monster = 0x304,
}

pub enum OthersEntries {
    Version = 0x400,
    Licence = 0x401,
    Contribute = 0x402,
    Bug = 0x403,
}

impl Menus {
    pub fn from_wparam(wparam: WPARAM) -> Option<Self> {
        match wparam {
            //File
            0x000 => Some(Menus::File(FileEntries::New)),
            0x001 => Some(Menus::File(FileEntries::Open)),
            0x002 => Some(Menus::File(FileEntries::Save)),
            0x003 => Some(Menus::File(FileEntries::Export)),
            0x004 => Some(Menus::File(FileEntries::Print)),
            0x005 => Some(Menus::File(FileEntries::Quit)),
            //Edit
            0x100 => Some(Menus::Edit(EditEntries::Redo)),
            0x101 => Some(Menus::Edit(EditEntries::Repeat)),
            0x102 => Some(Menus::Edit(EditEntries::Cut)),
            0x103 => Some(Menus::Edit(EditEntries::Copy)),
            0x104 => Some(Menus::Edit(EditEntries::Paste)),
            0x105 => Some(Menus::Edit(EditEntries::Search)),
            0x106 => Some(Menus::Edit(EditEntries::Replace)),
            //Guide
            /*0x200 => Some(Menus::Guide(GuideEntries::Run)),
            0x201 => Some(Menus::Guide(GuideEntries::Publish)),
            //Help
            0x300 => Some(Menus::Help(HelpEntries::Guide)),
            0x301 => Some(Menus::Help(HelpEntries::Doc)),
            //Others
            0x400 => Some(Menus::Others(OthersEntries::Version)),
            0x401 => Some(Menus::Others(OthersEntries::Licence)),
            */
            _ => None,
        }
    }
}

pub fn create_menu(hmenubar: HMENU) {
    unsafe {
        if hmenubar.is_null() {
            error_msgbox("Failed to create menu!");
        } else {
            //File
            let file_hmenu = CreateMenu();
            AppendMenuW(
                hmenubar,
                MF_POPUP,
                file_hmenu as u64,
                win32_string("File").as_ptr(),
            );

            AppendMenuW(
                file_hmenu,
                MF_STRING,
                FileEntries::New as u64,
                win32_string("New\tCtrl+N").as_ptr(),
            );
            AppendMenuW(
                file_hmenu,
                MF_STRING,
                FileEntries::Open as u64,
                win32_string("Open File\tCtrl+O").as_ptr(),
            );
            AppendMenuW(
                file_hmenu,
                MF_STRING,
                FileEntries::Save as u64,
                win32_string("Save\tCtrl+S").as_ptr(),
            );
            AppendMenuW(
                file_hmenu,
                MF_STRING,
                FileEntries::Export as u64,
                win32_string("Export").as_ptr(),
            );
            AppendMenuW(
                file_hmenu,
                MF_STRING,
                FileEntries::Print as u64,
                win32_string("Print\tCtrl+P").as_ptr(),
            );
            AppendMenuW(
                file_hmenu,
                MF_STRING,
                FileEntries::Quit as u64,
                win32_string("Quit\tCtrl+Q").as_ptr(),
            );

            //Edit
            let edit_hmenu = CreateMenu();
            AppendMenuW(
                hmenubar,
                MF_POPUP,
                edit_hmenu as u64,
                win32_string("Edit").as_ptr(),
            );

            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Redo as u64,
                win32_string("Redo\tCtrl+Z").as_ptr(),
            );
            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Repeat as u64,
                win32_string("Repeat\tCtrl+Shift+Z").as_ptr(),
            );
            AppendMenuW(edit_hmenu, MF_SEPARATOR, 0, null());
            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Cut as u64,
                win32_string("Cut\tCtrl+X").as_ptr(),
            );
            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Copy as u64,
                win32_string("Copy\tCtrl+C").as_ptr(),
            );
            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Paste as u64,
                win32_string("Paste\tCtrl+V").as_ptr(),
            );
            AppendMenuW(edit_hmenu, MF_SEPARATOR, 0, null());
            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Search as u64,
                win32_string("Search...\tCtrl+F").as_ptr(),
            );
            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Replace as u64,
                win32_string("Replace\tCtrl+H").as_ptr(),
            );

            //Guide
            let guide_hmenu = CreateMenu();
            AppendMenuW(
                hmenubar,
                MF_POPUP,
                guide_hmenu as u64,
                win32_string("Guide").as_ptr(),
            );

            AppendMenuW(
                guide_hmenu,
                MF_STRING,
                GuideEntries::Run as u64,
                win32_string("Live Preview\tF5").as_ptr(),
            );
            AppendMenuW(guide_hmenu, MF_SEPARATOR, 0, null());
            AppendMenuW(
                guide_hmenu,
                MF_STRING,
                GuideEntries::Review as u64,
                win32_string("Review").as_ptr(),
            );
            AppendMenuW(
                guide_hmenu,
                MF_STRING,
                GuideEntries::Publish as u64,
                win32_string("Publish Version...").as_ptr(),
            );
            AppendMenuW(
                guide_hmenu,
                MF_STRING,
                GuideEntries::Info as u64,
                win32_string("Guide Information").as_ptr(),
            );
            AppendMenuW(
                guide_hmenu,
                MF_STRING,
                GuideEntries::Options as u64,
                win32_string("Options").as_ptr(),
            );

            //Help
            let help_hmenu = CreateMenu();
            AppendMenuW(
                hmenubar,
                MF_POPUP,
                help_hmenu as u64,
                win32_string("Help").as_ptr(),
            );

            AppendMenuW(
                help_hmenu,
                MF_STRING,
                HelpEntries::Guide as u64,
                win32_string("Quick Guide").as_ptr(),
            );
            AppendMenuW(
                help_hmenu,
                MF_STRING,
                HelpEntries::Doc as u64,
                win32_string("BBCode Documentation").as_ptr(),
            );
            let help_submenu = CreatePopupMenu();
            AppendMenuW(
                help_hmenu,
                MF_STRING | MF_POPUP,
                help_submenu as u64,
                win32_string("League of Legend Wiki").as_ptr(),
            );
            AppendMenuW(
                help_submenu,
                MF_STRING,
                HelpEntries::Champions as u64,
                win32_string("Champions").as_ptr(),
            );
            AppendMenuW(
                help_submenu,
                MF_STRING,
                HelpEntries::Items as u64,
                win32_string("Items").as_ptr(),
            );
            AppendMenuW(
                help_submenu,
                MF_STRING,
                HelpEntries::Monster as u64,
                win32_string("Monster").as_ptr(),
            );

            //Others
            let others_hmenu = CreateMenu();
            AppendMenuW(
                hmenubar,
                MF_POPUP,
                others_hmenu as u64,
                win32_string("Others").as_ptr(),
            );

            AppendMenuW(
                others_hmenu,
                MF_STRING,
                OthersEntries::Version as u64,
                win32_string("Version").as_ptr(),
            );
            AppendMenuW(
                others_hmenu,
                MF_STRING,
                OthersEntries::Licence as u64,
                win32_string("Licence").as_ptr(),
            );
            AppendMenuW(others_hmenu, MF_SEPARATOR, 0, null());
            AppendMenuW(
                others_hmenu,
                MF_STRING,
                OthersEntries::Contribute as u64,
                win32_string("GitHub - Contribute!").as_ptr(),
            );
            AppendMenuW(
                others_hmenu,
                MF_STRING,
                OthersEntries::Bug as u64,
                win32_string("Bug Tracker").as_ptr(),
            );
        };
    };
}
