use libc::*;
use mgldraw::MGLDraw;
use std::ffi::CStr;
use std::io;
use byteorder::LittleEndian as BO;

pub const FONT_MAX_CHARS: usize = 128;

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
    /// pointer to the character data
    data: Box<[u8]>,
    /// offsets to each character's data (can't have more than FONT_MAX_CHARS)
    chars: [usize; FONT_MAX_CHARS],
}

// each character in the font is stored as:
// width    1 byte       width of the character in pixels
// data     width*height bytes of actual data

static mut fontmgl: *mut MGLDraw = 0 as *mut MGLDraw;

pub unsafe fn FontInit(mgl: *mut MGLDraw) {
    fontmgl = mgl;
}

pub fn FontExit() {}

impl mfont_t {
    pub fn load(fname: &str) -> io::Result<mfont_t> {
        use byteorder::ReadBytesExt;
        use std::io::{Read, BufReader};

        let mut f = BufReader::new(::std::fs::File::open(fname)?);
        let numChars = f.read_u8()?;
        let firstChar = f.read_u8()?;
        let height = f.read_u8()?;
        let spaceSize = f.read_u8()?;
        let gapSize = f.read_u8()?;
        let gapHeight = f.read_u8()?;
        f.read_u16::<BO>()?; // padding
        let dataSize = f.read_u32::<BO>()? as usize;
        f.read_u32::<BO>()?; // data pointer
        for _ in 0..FONT_MAX_CHARS {
            f.read_u32::<BO>()?; // chars array
        }

        let mut data = vec![0; dataSize].into_boxed_slice();
        f.read_exact(&mut data)?;

        let mut chars = [0; FONT_MAX_CHARS];
        for i in 1..(numChars as usize) {
            chars[i] = chars[i - 1] + 1 + data[chars[i - 1]] as usize * height as usize;
        }

        Ok(mfont_t {
            numChars, firstChar, height, spaceSize, gapSize, gapHeight,
            data, chars,
        })
    }

    pub fn char_width(&self, c: u8) -> u8 {
        if c < self.firstChar || c >= (self.firstChar + self.numChars) {
            return self.spaceSize; // unprintable
        }

        self.data[self.chars[(c - self.firstChar) as usize]]
    }

    pub fn save(&self, fname: &str) -> io::Result<()> {
        use byteorder::WriteBytesExt;
        use std::io::{Write, BufWriter};

        let mut f = BufWriter::new(::std::fs::File::create(fname)?);
        f.write_all(&[
            self.numChars,
            self.firstChar,
            self.height,
            self.spaceSize,
            self.gapSize,
            self.gapHeight,
            0,
            0,
        ])?;
        f.write_u32::<BO>(self.data.len() as u32)?;
        f.write_u32::<BO>(0)?; // data pointer
        for _ in 0..FONT_MAX_CHARS {
            f.write_u32::<BO>(0)?; // chars array
        }
        f.write_all(&self.data)?;
        Ok(())
    }
}

fn print_char<F>(mgl: &mut MGLDraw, mut x: c_int, mut y: c_int, c: u8, font: &mfont_t, f: F)
    where F: Fn(&mut u8, u8) // dst, src
{
    let (scrWidth, scrHeight) = mgl.get_size();
    let screen = mgl.get_screen();
    let mut dst_index = x + y * scrWidth;
    if c < font.firstChar || c >= (font.firstChar + font.numChars) {
        return; // unprintable
    }

    let c = (c - font.firstChar) as usize;
    let chrWidth = font.data[font.chars[c]];
    let mut src_idx = font.chars[c] + 1;
    for _ in 0 .. font.height {
        for _ in 0 .. chrWidth {
            let src = font.data[src_idx];
            if src != 0 && x >= 0 && x < scrWidth && y >= 0 && y < scrHeight {
                f(&mut screen[dst_index as usize], src);
            }
            dst_index += 1;
            src_idx += 1;
            x += 1;
        }
        y += 1;
        x -= chrWidth as c_int;
        dst_index += scrWidth - chrWidth as c_int;
    }
}

pub unsafe fn FontPrintChar(x: c_int, y: c_int, c: u8, font: &mfont_t) {
    print_char(&mut *fontmgl, x, y, c, font, |dst, src| *dst = src);
}

pub unsafe fn FontPrintCharSolid(x: c_int, y: c_int, c: u8, font: &mfont_t, color: u8) {
    print_char(&mut *fontmgl, x, y, c, font, |dst, _| *dst = color);
}

pub unsafe fn FontPrintCharColor(x: c_int, y: c_int, c: u8, color: u8, font: &mfont_t) {
    let color = color * 32;
    print_char(&mut *fontmgl, x, y, c, font, |dst, src| {
        if (src >= 64 && src < 64 + 32) || (src >= 128 && src < 128 + 32) {
            *dst = (src & 31) + color;
        } else {
            *dst = src;
        }
    });
}

pub unsafe fn FontPrintCharBright(x: c_int, y: c_int, c: u8, bright: i8, font: &mfont_t) {
    print_char(&mut *fontmgl, x, y, c, font, |dst, src| {
        *dst = (src as i8).wrapping_add(bright) as u8;
        if *dst > (src & !31) + 31 {
            *dst = (src & !31) + 31;
        } else if *dst < (src & !31) {
            *dst = src & !31;
        }
    })
}

unsafe fn print_string<F: Fn(c_int, u8)>(mut x: c_int, s: *const c_char, font: &mfont_t, f: F) {
    for &byte in CStr::from_ptr(s).to_bytes() {
        f(x, byte);
        x += font.char_width(byte) as c_int + font.gapSize as c_int;
    }
}

pub unsafe fn FontPrintString(x: c_int, y: c_int, s: *const c_char, font: &mfont_t) {
    print_string(x, s, font, |x, byte| FontPrintChar(x, y, byte, font));
}

pub unsafe fn FontPrintStringColor(x: c_int, y: c_int, s: *const c_char, font: &mfont_t, color: u8) {
    print_string(x, s, font, |x, byte| FontPrintCharColor(x, y, byte, color, font));
}

pub unsafe fn FontPrintStringBright(x: c_int, y: c_int, s: *const c_char, font: &mfont_t, bright: i8) {
    print_string(x, s, font, |x, byte| FontPrintCharBright(x, y, byte, bright, font));
}

pub unsafe fn FontPrintStringSolid(x: c_int, y: c_int, s: *const c_char, font: &mfont_t, color: u8) {
    print_string(x, s, font, |x, byte| FontPrintCharSolid(x, y, byte, font, color));
}

pub unsafe fn FontPrintStringDropShadow(
    mut x: c_int, y: c_int,
    s: *const c_char,
    font: &mfont_t,
    shadowColor: u8,
    shadowOffset: u8,
) {
    for &byte in CStr::from_ptr(s).to_bytes() {
        FontPrintCharSolid(x + shadowOffset as c_int, y + shadowOffset as c_int, byte, font, shadowColor);
        FontPrintChar(x, y, byte, font);
        x += font.char_width(byte) as c_int + font.gapSize as c_int;
    }
}

pub unsafe fn FontStrLen(s: *const c_char, font: &mfont_t) -> c_int {
    CStr::from_ptr(s).to_bytes().iter().map(|&byte| {
        font.char_width(byte) as c_int + font.gapSize as c_int
    }).sum()
}
