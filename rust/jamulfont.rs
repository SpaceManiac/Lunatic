use libc::*;
use mgldraw::MGLDraw;
use std::ffi::CStr;

pub const FONT_MAX_CHARS: usize = 128;

#[repr(C)]
pub struct mfont_t {
    /// # of characters in the font
    numChars: u8,
    /// the first character's ASCII value (they ascend from there)
    firstChar: u8,
    /// height in pixels of the font
    height: u8,
    /// # of pixels wide to make spaces
    spaceSize: u8,
    /// # of pixels between adjacent letters
    gapSize: u8,
    /// # of pixels to descend for a carriage return
    gapHeight: u8,
    /// the size in bytes of the data of the characters themselves
    dataSize: usize,
    // pointer to the character data
    data: *mut u8,
    /// pointers to each character's data (can't have more than FONT_MAX_CHARS)
    chars: [*mut u8; FONT_MAX_CHARS],
}

impl Drop for mfont_t {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe { ::libc::free(self.data as *mut ::libc::c_void); }
        }
    }
}

// each character in the font is stored as:
// width    1 byte       width of the character in pixels
// data     width*height bytes of actual data

/// error codes
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FontError {
    FONT_OK = 0,
    FONT_FILENOTFOUND,
    FONT_CANTALLOC,
    FONT_INVALIDFILE,
}

static mut fontmgl_: *mut MGLDraw = 0 as *mut MGLDraw;

/// this is sort of a palette translation table for the font
static mut fontPal: [u8; 256] = [0; 256];

#[no_mangle]
pub unsafe extern fn FontInit(mgl: *mut MGLDraw) {
    fontmgl_ = mgl;
    // default translation is none for the font palette
    for i in 0..256 {
        fontPal[i] = i as u8;
    }
}

#[no_mangle]
pub unsafe extern fn FontExit() {}

#[no_mangle]
pub unsafe extern fn FontFree(font: *mut mfont_t) {
    ::std::ptr::drop_in_place(font);
}

pub unsafe fn load_font(fname: *const c_char) -> Result<Box<mfont_t>, FontError> {
    let mut font = Box::new(mfont_t {
        numChars: 0,
        firstChar: 0,
        height: 0,
        spaceSize: 0,
        gapSize: 0,
        gapHeight: 0,
        dataSize: 0,
        data: 0 as *mut u8,
        chars: [0 as *mut u8; FONT_MAX_CHARS],
    });
    let r = FontLoad(fname, &mut *font);
    if r == FontError::FONT_OK {
        Ok(font)
    } else {
        Err(r)
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

    if fwrite(decay!(font), szof!(mfont_t), 1, f) != 1 {
        return FontError::FONT_INVALIDFILE;
    }
    if fwrite(decay!(font.data), font.dataSize, 1, f) != 1 {
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

unsafe fn print_char<F>(mut x: c_int, mut y: c_int, c: u8, font: &mfont_t, f: F)
    where F: Fn(*mut u8, *mut u8) // dst, src
{
    let fontmgl = &mut *fontmgl_;
    let (scrWidth, scrHeight) = fontmgl.get_size();
    let mut dst = fontmgl.get_screen().as_mut_ptr().offset(x as isize + y as isize * scrWidth as isize);
    if c < font.firstChar || c >= (font.firstChar + font.numChars) {
        return; // unprintable
    }

    let c = (c - font.firstChar) as usize;
    let chrWidth = *font.chars[c];
    let mut src = font.chars[c].offset(1);
    for _ in 0 .. font.height {
        for _ in 0 .. chrWidth {
            if *src != 0 && x >= 0 && x < scrWidth && y >= 0 && y < scrHeight {
                f(dst, src);
            }
            dst = dst.offset(1);
            src = src.offset(1);
            x += 1;
        }
        y += 1;
        x -= chrWidth as c_int;
        dst = dst.offset(scrWidth as isize - chrWidth as isize);
    }
}

#[no_mangle]
pub unsafe extern fn FontPrintChar(x: c_int, y: c_int, c: u8, font: &mfont_t) {
    print_char(x, y, c, font, |dst, src| *dst = fontPal[*src as usize]);
}

#[no_mangle]
pub unsafe extern fn FontPrintCharSolid(x: c_int, y: c_int, c: u8, font: &mfont_t, color: u8) {
    print_char(x, y, c, font, |dst, _| *dst = color);
}

#[no_mangle]
pub unsafe extern fn FontPrintCharColor(x: c_int, y: c_int, c: u8, color: u8, font: &mfont_t) {
    let color = color * 32;
    print_char(x, y, c, font, |dst, src| {
        if (*src >= 64 && *src < 64 + 32) || (*src >= 128 && *src < 128 + 32) {
            *dst = ((*src) & 31) + color;
        } else {
            *dst = *src;
        }
    });
}

#[no_mangle]
pub unsafe extern fn FontPrintCharBright(x: c_int, y: c_int, c: u8, bright: i8, font: &mfont_t) {
    print_char(x, y, c, font, |dst, src| {
        *dst = (*src as i8).wrapping_add(bright) as u8;
        if *dst > (*src & !31) + 31 {
            *dst = (*src & !31) + 31;
        } else if *dst < (*src & !31) {
            *dst = *src & !31;
        }
    })
}

unsafe fn print_string<F: Fn(c_int, u8)>(mut x: c_int, s: *const c_char, font: &mfont_t, f: F) {
    for &byte in CStr::from_ptr(s).to_bytes() {
        f(x, byte);
        x += CharWidth(byte, font) as c_int + font.gapSize as c_int;
    }
}

#[no_mangle]
pub unsafe extern fn FontPrintString(x: c_int, y: c_int, s: *const c_char, font: &mfont_t) {
    print_string(x, s, font, |x, byte| FontPrintChar(x, y, byte, font));
}

#[no_mangle]
pub unsafe extern fn FontPrintStringColor(x: c_int, y: c_int, s: *const c_char, font: &mfont_t, color: u8) {
    print_string(x, s, font, |x, byte| FontPrintCharColor(x, y, byte, color, font));
}

#[no_mangle]
pub unsafe extern fn FontPrintStringBright(x: c_int, y: c_int, s: *const c_char, font: &mfont_t, bright: i8) {
    print_string(x, s, font, |x, byte| FontPrintCharBright(x, y, byte, bright, font));
}

#[no_mangle]
pub unsafe extern fn FontPrintStringSolid(x: c_int, y: c_int, s: *const c_char, font: &mfont_t, color: u8) {
    print_string(x, s, font, |x, byte| FontPrintCharSolid(x, y, byte, font, color));
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
    memcpy(decay!(&mut fontPal[first as usize]), decay!(data), count as usize);
}

#[no_mangle]
pub unsafe extern fn FontStrLen(s: *const c_char, font: &mfont_t) -> c_int {
    CStr::from_ptr(s).to_bytes().iter().map(|&byte| {
        CharWidth(byte, font) as c_int + font.gapSize as c_int
    }).sum()
}
