use std::ptr::null;

use user32::{AppendMenuW, CreateMenu, CreatePopupMenu};

use windows::utils::{error_msgbox, ToWide};

use winapi::basetsd::UINT_PTR;
//use winapi::minwindef::WPARAM;
use winapi::windef::HMENU;
use winapi::winuser::{MF_POPUP, MF_SEPARATOR, MF_STRING};

/*pub enum Menus {
    File(FileEntries),
    Edit(EditEntries),
    /*Guide(GuideEntries),
    Help(HelpEntries),
    Others(OthersEntries),*/
}*/

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

/*impl Menus {
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
}*/

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
                file_hmenu as UINT_PTR,
                "File".to_wide().as_ptr() as *const u16,
            );

            AppendMenuW(
                file_hmenu,
                MF_STRING,
                FileEntries::New as UINT_PTR,
                "New\tCtrl+N".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                file_hmenu,
                MF_STRING,
                FileEntries::Open as UINT_PTR,
                "Open File\tCtrl+O".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                file_hmenu,
                MF_STRING,
                FileEntries::Save as UINT_PTR,
                "Save\tCtrl+S".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                file_hmenu,
                MF_STRING,
                FileEntries::Export as UINT_PTR,
                "Export".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                file_hmenu,
                MF_STRING,
                FileEntries::Print as UINT_PTR,
                "Print\tCtrl+P".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                file_hmenu,
                MF_STRING,
                FileEntries::Quit as UINT_PTR,
                "Quit\tCtrl+Q".to_wide().as_ptr() as *const u16,
            );

            //Edit
            let edit_hmenu = CreateMenu();
            AppendMenuW(
                hmenubar,
                MF_POPUP,
                edit_hmenu as UINT_PTR,
                "Edit".to_wide().as_ptr() as *const u16,
            );

            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Redo as UINT_PTR,
                "Redo\tCtrl+Z".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Repeat as UINT_PTR,
                "Repeat\tCtrl+Shift+Z".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(edit_hmenu, MF_SEPARATOR, 0, null());
            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Cut as UINT_PTR,
                "Cut\tCtrl+X".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Copy as UINT_PTR,
                "Copy\tCtrl+C".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Paste as UINT_PTR,
                "Paste\tCtrl+V".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(edit_hmenu, MF_SEPARATOR, 0, null());
            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Search as UINT_PTR,
                "Search...\tCtrl+F".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                edit_hmenu,
                MF_STRING,
                EditEntries::Replace as UINT_PTR,
                "Replace\tCtrl+H".to_wide().as_ptr() as *const u16,
            );

            //Guide
            let guide_hmenu = CreateMenu();
            AppendMenuW(
                hmenubar,
                MF_POPUP,
                guide_hmenu as UINT_PTR,
                "Guide".to_wide().as_ptr() as *const u16,
            );

            AppendMenuW(
                guide_hmenu,
                MF_STRING,
                GuideEntries::Run as UINT_PTR,
                "Live Preview\tF5".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(guide_hmenu, MF_SEPARATOR, 0, null());
            AppendMenuW(
                guide_hmenu,
                MF_STRING,
                GuideEntries::Review as UINT_PTR,
                "Review".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                guide_hmenu,
                MF_STRING,
                GuideEntries::Publish as UINT_PTR,
                "Publish Version...".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                guide_hmenu,
                MF_STRING,
                GuideEntries::Info as UINT_PTR,
                "Guide Information".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                guide_hmenu,
                MF_STRING,
                GuideEntries::Options as UINT_PTR,
                "Options".to_wide().as_ptr() as *const u16,
            );

            //Help
            let help_hmenu = CreateMenu();
            AppendMenuW(
                hmenubar,
                MF_POPUP,
                help_hmenu as UINT_PTR,
                "Help".to_wide().as_ptr() as *const u16,
            );

            AppendMenuW(
                help_hmenu,
                MF_STRING,
                HelpEntries::Guide as UINT_PTR,
                "Quick Guide".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                help_hmenu,
                MF_STRING,
                HelpEntries::Doc as UINT_PTR,
                "BBCode Documentation".to_wide().as_ptr() as *const u16,
            );
            let help_submenu = CreatePopupMenu();
            AppendMenuW(
                help_hmenu,
                MF_STRING | MF_POPUP,
                help_submenu as UINT_PTR,
                "League of Legend Wiki".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                help_submenu,
                MF_STRING,
                HelpEntries::Champions as UINT_PTR,
                "Champions".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                help_submenu,
                MF_STRING,
                HelpEntries::Items as UINT_PTR,
                "Items".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                help_submenu,
                MF_STRING,
                HelpEntries::Monster as UINT_PTR,
                "Monster".to_wide().as_ptr() as *const u16,
            );

            //Others
            let others_hmenu = CreateMenu();
            AppendMenuW(
                hmenubar,
                MF_POPUP,
                others_hmenu as UINT_PTR,
                "Others".to_wide().as_ptr() as *const u16,
            );

            AppendMenuW(
                others_hmenu,
                MF_STRING,
                OthersEntries::Version as UINT_PTR,
                "Version".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                others_hmenu,
                MF_STRING,
                OthersEntries::Licence as UINT_PTR,
                "Licence".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(others_hmenu, MF_SEPARATOR, 0, null());
            AppendMenuW(
                others_hmenu,
                MF_STRING,
                OthersEntries::Contribute as UINT_PTR,
                "GitHub - Contribute!".to_wide().as_ptr() as *const u16,
            );
            AppendMenuW(
                others_hmenu,
                MF_STRING,
                OthersEntries::Bug as UINT_PTR,
                "Bug Tracker".to_wide().as_ptr() as *const u16,
            );
        };
    };
}
