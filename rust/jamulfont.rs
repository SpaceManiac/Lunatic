use libc::*;
use mgldraw::MGLDraw;
use std::ffi::CStr;

pub const FONT_MAX_CHARS: usize = 128;

#[repr(C)]
pub struct mfont_t {
    /// # of characters in the font
    pub numChars: u8,
    /// the first character's ASCII value (they ascend from there)
    pub firstChar: u8,
    /// height in pixels of the font
    pub height: u8,
    /// # of pixels wide to make spaces
    pub spaceSize: u8,
    /// # of pixels between adjacent letters
    pub gapSize: u8,
    /// # of pixels to descend for a carriage return
    pub gapHeight: u8,
    /// the size in bytes of the data of the characters themselves
    pub dataSize: usize,
    // pointer to the character data
    pub data: *mut u8,
    /// pointers to each character's data (can't have more than FONT_MAX_CHARS)
    pub chars: [*mut u8; FONT_MAX_CHARS],
}

// each character in the font is stored as:
// width    1 byte       width of the character in pixels
// data     width*height bytes of actual data

/// error codes
#[repr(C)]
pub enum FontError {
    FONT_OK = 0,
    FONT_FILENOTFOUND,
    FONT_CANTALLOC,
    FONT_INVALIDFILE,
}

extern {
    static mut fontmgl: *mut MGLDraw;
    /// this is sort of a palette translation table for the font
    static mut fontPal: [u8; 256];

    fn FontPrintChar(x: c_int, y: c_int, c: u8, font: *const mfont_t);
    fn FontPrintCharColor(x: c_int, y: c_int, c: u8, color: u8, font: *const mfont_t);
    fn FontPrintCharSolid(x: c_int, y: c_int, c: u8, font: *const mfont_t, color: u8);
    fn FontPrintCharBright(x: c_int, y: c_int, c: u8, bright: i8, font: *const mfont_t);
}

#[no_mangle]
pub unsafe extern fn FontInit(mgl: *mut MGLDraw) {
    fontmgl = mgl;
	// default translation is none for the font palette
    for i in 0..256 {
        fontPal[i] = i as u8;
    }
}

#[no_mangle]
pub unsafe extern fn FontExit() {}

#[no_mangle]
pub unsafe extern fn FontFree(font: *mut mfont_t) {
    if !(*font).data.is_null() {
        ::libc::free((*font).data as *mut ::libc::c_void);
    }
}

#[no_mangle]
pub unsafe extern fn FontLoad(fname: *const c_char, font: &mut mfont_t) -> FontError {
    let f = fopen(fname, cstr!("rb"));
    if f.is_null() {
        return FontError::FONT_FILENOTFOUND
    }

    if fread(decay!(font), szof!(mfont_t), 1, f) != 1 {
        return FontError::FONT_INVALIDFILE;
    }

    font.data = malloc(font.dataSize) as *mut u8;
    if font.data.is_null() {
        return FontError::FONT_CANTALLOC;
    }

    if fread(decay!(font.data), font.dataSize, 1, f) != 1 {
        return FontError::FONT_INVALIDFILE;
    }

    fclose(f);
    font.chars[0] = font.data;
    for i in 1..(font.numChars as usize) {
        font.chars[i] = font.chars[i - 1].offset(1 + ((*font.chars[i - 1]) as isize * font.height as isize));
    }

    FontError::FONT_OK
}

#[no_mangle]
pub unsafe extern fn FontSave(fname: *const c_char, font: &mfont_t) -> FontError {
    let f = fopen(fname, cstr!("wb"));
    if f.is_null() {
        return FontError::FONT_FILENOTFOUND;
    }

    if fwrite(decay!(const font), szof!(mfont_t), 1, f) != 1 {
        return FontError::FONT_INVALIDFILE;
    }
    if fwrite(decay!(const font.data), font.dataSize, 1, f) != 1 {
        return FontError::FONT_INVALIDFILE;
    }
    fclose(f);
    FontError::FONT_OK
}

#[no_mangle]
pub unsafe extern fn CharWidth(c: u8, font: &mfont_t) -> u8 {
	if c < font.firstChar || c >= (font.firstChar + font.numChars) {
        return font.spaceSize; // unprintable
    }

	*font.chars[(c - font.firstChar) as usize]
}

#[no_mangle]
pub unsafe extern fn FontPrintString(mut x: c_int, y: c_int, s: *const c_char, font: &mfont_t) {
    for &byte in CStr::from_ptr(s).to_bytes() {
        FontPrintChar(x, y, byte, font);
        x += CharWidth(byte, font) as c_int + font.gapSize as c_int;
    }
}

#[no_mangle]
pub unsafe extern fn FontPrintStringColor(mut x: c_int, y: c_int, s: *const c_char, font: &mfont_t, color: u8) {
    for &byte in CStr::from_ptr(s).to_bytes() {
        FontPrintCharColor(x, y, byte, color, font);
        x += CharWidth(byte, font) as c_int + font.gapSize as c_int;
    }
}

#[no_mangle]
pub unsafe extern fn FontPrintStringBright(mut x: c_int, y: c_int, s: *const c_char, font: &mfont_t, bright: i8) {
    for &byte in CStr::from_ptr(s).to_bytes() {
        FontPrintCharBright(x, y, byte, bright, font);
        x += CharWidth(byte, font) as c_int + font.gapSize as c_int;
    }
}

#[no_mangle]
pub unsafe extern fn FontPrintStringSolid(mut x: c_int, y: c_int, s: *const c_char, font: &mfont_t, color: u8) {
    for &byte in CStr::from_ptr(s).to_bytes() {
        FontPrintCharSolid(x, y, byte, font, color);
        x += CharWidth(byte, font) as c_int + font.gapSize as c_int;
    }
}

#[no_mangle]
pub unsafe extern fn FontPrintStringDropShadow(
    mut x: c_int, y: c_int,
    s: *const c_char,
    font: &mfont_t,
    shadowColor: u8,
    shadowOffset: u8,
) {
    for &byte in CStr::from_ptr(s).to_bytes() {
        FontPrintCharSolid(x + shadowOffset as c_int, y + shadowOffset as c_int, byte, font, shadowColor);
        FontPrintChar(x, y, byte, font);
        x += CharWidth(byte, font) as c_int + font.gapSize as c_int;
    }
}

#[no_mangle]
pub unsafe extern fn FontSetColors(first: u8, count: u8, data: *const u8) {
    memcpy(decay!(&mut fontPal[first as usize]), decay!(const data), count as usize);
}

#[no_mangle]
pub unsafe extern fn FontStrLen(s: *const c_char, font: &mfont_t) -> c_int {
    CStr::from_ptr(s).to_bytes().iter().map(|&byte| {
        CharWidth(byte, font) as c_int + font.gapSize as c_int
    }).sum()
}
