use winapi::basetsd::INT32;
use winapi::windef::{COLORREF, HFONT, HWND};

pub const TEXTVIEW_CLASS: &'static str = "TextView_Class";
pub const IDYES: INT32 = 6;
/*pub const IDOK: INT32 = 1;
pub const IDCANCEL: INT32 = 2;*/

pub struct TextView {
    pub hfont: HFONT,
    pub hfont_color: COLORREF,
    pub hwnd: HWND,
}
