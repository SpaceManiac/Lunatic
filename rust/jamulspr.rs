use libc::{c_char, c_int, free, malloc, FILE};
use mgldraw::MGLDraw;

// the sprites are 12 bytes, not including the data itself
// note that the value here is 16 - there are four bytes of
// garbage between each sprite header
const SPRITE_INFO_SIZE: c_int = 16;

/*
Jamul Sprite - JSP

header:
count		1 word	how many frames in this sprite
data:
count structures:
	width	1 word		width of sprite in pixels
	height	1 word		height of sprite in pixels
	ofsX	1 short		x-coord of hotspot relative to left
	ofsY	1 short		y-coord of hotspot relative to top
	size	1 dword		how big the sprite data is in bytes

count data chunks:
	data	size bytes	transparency RLE'd sprite data

	The RLE format is as follows:

	count	1 byte	if count is positive, this is considered
			a run of data, negative is a run of
			transparency.  If the run is data, it is
			followed by count bytes of data.  If
			it is transparent, the next RLE tag
			simply follows it.
			Runs do not cross line boundaries.
 */

#[repr(C)]
pub struct sprite_t {
    width: u16,
    height: u16,
    ofsx: i16,
    ofsy: i16,
    size: u32,
    data: *mut u8,
}

#[repr(C)]
pub struct sprite_set_t {
    count: u16,
    spr: *mut *mut sprite_t,
}

impl sprite_t {
    // from_header

    pub unsafe fn LoadData(&mut self, f: *mut FILE) -> bool {
        let size = self.size as usize;
        if size == 0 {
            return true;
        }

        self.data = malloc(size) as *mut u8;
        if self.data.is_null() {
            return false;
        }

        ::libc::fread(decay!(self.data), 1, size, f) == size
    }

    pub unsafe fn SaveData(&mut self, f: *mut FILE) -> bool {
        let size = self.size as usize;
        if size == 0 || self.data.is_null() {
            return true;
        }

        ::libc::fwrite(decay!(self.data), 1, size, f) == size
    }

    // GetHeader

    pub fn Draw(&self, x: c_int, y: c_int, mgl: &mut MGLDraw) {
        sprite_draw(self, x, y, mgl, |src, _| src);
    }

    // bright: how much to darken or lighten the whole thing (-16 to +16 reasonable)
    pub fn DrawBright(&self, x: c_int, y: c_int, mgl: &mut MGLDraw, bright: i8) {
        if bright == 0 {
            return self.Draw(x, y, mgl); // don't waste time
        }

        sprite_draw(self, x, y, mgl, |src, _| SprModifyLight(src, bright));
    }

    // color:  which hue (0-7) to use for the entire thing, ignoring its real hue
    // bright: how much to darken or lighten the whole thing (-16 to +16 reasonable)
    pub fn DrawColored(&self, x: c_int, y: c_int, mgl: &mut MGLDraw, hue: u8, bright: i8) {
        sprite_draw(self, x, y, mgl, |src, _| SprModifyLight(SprModifyColor(src, hue), bright));
    }

    pub fn DrawOffColor(&self, x: c_int, y: c_int, mgl: &mut MGLDraw, from: u8, to: u8, bright: i8) {
        if from == to && bright == 0 {
            return self.Draw(x, y, mgl); // don't waste time
        }

        sprite_draw(self, x, y, mgl, |src, _| {
            SprModifyLight(if SprGetColor(src) == from {
                SprModifyColor(src, to)
            } else {
                src
            }, bright)
        });
    }

    // a ghost sprite is rather special.  It is drawn normally (except lightened
    // or darkened according to the brightness parameter), except where it is grey
    // (color 1-31).  Wherever those colors occur, they are instead used as the
    // degree to which the background should be brightened instead of drawn over.
    //   bright: how much to darken or lighten the whole thing (-16 to +16 reasonable)
    pub fn DrawGhost(&self, x: c_int, y: c_int, mgl: &mut MGLDraw, bright: i8) {
        sprite_draw(self, x, y, mgl, |src, dst| SprModifyGhost(src, dst, bright));
    }

    pub fn DrawGlow(&self, x: c_int, y: c_int, mgl: &mut MGLDraw, bright: i8) {
        sprite_draw(self, x, y, mgl, |src, dst| SprModifyGlow(src, dst, bright));
    }

    /// this makes half-height tilted black shadows (they darken by 4)
    pub fn DrawShadow(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw) {
        sprite_draw_shadow(self, x, y, mgl);
    }

    pub fn get_coords(&self, x: c_int, y: c_int) -> (c_int, c_int, c_int, c_int) {
        let (rx, ry) = (x - self.ofsx as c_int, y - self.ofsy as c_int);
        (rx, ry, rx + self.width as c_int, ry + self.height as c_int)
    }

    pub fn data(&self) -> &[u8] {
        assert!(!self.data.is_null());
        unsafe { ::std::slice::from_raw_parts(self.data, self.size as usize) }
    }
}

impl Drop for sprite_t {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe { free(self.data as *mut _); }
        }
    }
}

impl sprite_set_t {
    pub unsafe fn from_fname(fname: *const c_char) -> *mut sprite_set_t {
        cpp!([fname as "char*"] -> *mut sprite_set_t as "sprite_set_t*" {
            return new sprite_set_t(fname);
        })
    }

    pub unsafe fn delete(me: *mut sprite_set_t) {
        cpp!([me as "sprite_set_t*"] {
            delete me;
        })
    }

    // Save(fname: *const c_char) -> bool
    // Load(fname: *const c_char) -> bool

    pub fn GetSprite(&mut self, which: c_int) -> &mut sprite_t {
        self.sprites_mut()[which as usize]
    }

    pub fn sprites(&self) -> &[&sprite_t] {
        assert!(!self.spr.is_null());
        unsafe {
            ::std::slice::from_raw_parts(self.spr as *const &sprite_t, self.count as usize)
        }
    }

    pub fn sprites_mut(&mut self) -> &mut [&mut sprite_t] {
        assert!(!self.spr.is_null());
        unsafe {
            ::std::slice::from_raw_parts_mut(self.spr as *mut &mut sprite_t, self.count as usize)
        }
    }
}

#[no_mangle]
pub extern fn SprModifyColor(color: u8, hue: u8) -> u8 {
    (hue << 5) | (color & 31)
}

#[no_mangle]
pub extern fn SprGetColor(color: u8) -> u8 {
    color >> 5
}

#[no_mangle]
pub extern fn SprModifyLight(color: u8, bright: i8) -> u8 {
    let mut value = (color & 31).wrapping_add(bright as u8);
    if value > 128 { value = 0; } // since byte is unsigned...
    else if value > 31 { value = 31; }
    (color & !31) | value
}

#[no_mangle]
pub extern fn SprModifyGhost(src: u8, dst: u8, bright: i8) -> u8 {
    if src < 31 {
        SprModifyLight(dst, src as i8)
    } else {
        SprModifyLight(src, bright)
    }
}

#[no_mangle]
pub extern fn SprModifyGlow(src: u8, dst: u8, bright: i8) -> u8 {
    SprModifyLight(src, (dst & 31) as i8 + bright)
}

const MIN_X: c_int = 0;
const MIN_Y: c_int = 0;
const MAX_X: c_int = 640;
const MAX_Y: c_int = 480;

fn sprite_draw<F: Fn(u8, u8) -> u8>(
    spr: &sprite_t, mut x: c_int, mut y: c_int, mgl: &mut MGLDraw, f: F
) {
    x -= spr.ofsx as c_int;
    y -= spr.ofsy as c_int;

    if x >= MAX_X || y >= MAX_Y {
        return; // whole sprite is offscreen
    }

    let (pitch, _) = mgl.get_size();
    let mut src = spr.data();

    let screen = mgl.get_screen();
    let mut dst_idx = x + y * pitch;

    let mut srcx = x;
    let mut srcy = y;
    let mut drawing = srcy >= MIN_Y;

    macro_rules! apply {
        ($len:expr) => { if drawing {
            apply(&mut screen[dst_idx as usize..], src, $len, &f);
        }}
    }

    let end_y = ::std::cmp::min(MAX_Y, spr.height as c_int + y);
    while srcy < end_y {
        if src[0] & 128 != 0 { // transparent run
            let b = (src[0] & 127) as c_int;
            srcx += b;
            src.advance(1);
            dst_idx += b;
        } else { // solid run
            let mut b = src[0] as c_int;
            src.advance(1);
            if srcx < MIN_X - b || srcx >= MAX_X {
                // don't draw this line
            } else if srcx < MIN_X {
                // skip some of the beginning
                let skip = MIN_X - srcx;
                srcx += skip;
                src.advance(skip);
                dst_idx += skip;
                b -= skip;
                if srcx >= MAX_X - b {
                    apply!(MAX_X - srcx);
                } else {
                    apply!(b);
                }
            } else if srcx >= MAX_X - b {
                // skip some of the end
                apply!(MAX_X - srcx);
            } else {
                // do it all!
                apply!(b);
            }
            srcx += b;
            src.advance(b);
            dst_idx += b;
        }
        if srcx >= spr.width as c_int + x {
            srcx = x;
            srcy += 1;
            dst_idx += pitch - spr.width as c_int;
            if srcy >= MIN_Y {
                drawing = true;
            }
        }
    }
}

fn sprite_draw_shadow(
    spr: &sprite_t, mut x: c_int, mut y: c_int, mgl: &mut MGLDraw
) {
    x -= spr.ofsx as c_int + spr.height as c_int / 2;
    y -= spr.ofsy as c_int / 2;

    if x >= MAX_X || y >= MAX_Y {
        return; // whole sprite is offscreen
    }

    let (pitch, _) = mgl.get_size();
    let mut src = spr.data();

    let screen = mgl.get_screen();
    let mut dst_idx = x + y * pitch;

    let mut srcx = x;
    let mut srcy = y;
    let mut drawing = srcy >= MIN_Y;

    let mut alternate = true; // shadow
    let mut x2 = x; // shadow

    macro_rules! apply {
        ($len:expr) => { if drawing && alternate {
            apply(&mut screen[dst_idx as usize..], src, $len, &|_, dst| SprModifyLight(dst, -4));
        }}
    }

    let end_y = ::std::cmp::min(MAX_Y, spr.height as c_int / 2 + y);
    while srcy < end_y {
        if src[0] & 128 != 0 { // transparent run
            let b = (src[0] & 127) as c_int;
            srcx += b;
            src.advance(1);
            dst_idx += b;
        } else { // solid run
            let mut b = src[0] as c_int;
            src.advance(1);
            if srcx < MIN_X - b || srcx >= MAX_X {
                // don't draw this line
            } else if srcx < MIN_X {
                // skip some of the beginning
                let skip = MIN_X - srcx;
                srcx += skip;
                src.advance(skip);
                dst_idx += skip;
                b -= skip;
                if srcx >= MAX_X - b {
                    apply!(MAX_X - srcx);
                } else {
                    apply!(b);
                }
            } else if srcx >= MAX_X - b {
                // skip some of the end
                apply!(MAX_X - srcx);
            } else {
                // do it all!
                apply!(b);
            }
            srcx += b;
            src.advance(b);
            dst_idx += b;
        }
        if srcx >= spr.width as c_int + x2 {
            alternate = !alternate;
            if alternate { x2 += 1; }
            srcx -= spr.width as c_int - if alternate { 1 } else { 0 };
            srcy += if alternate { 1 } else { 0 };
            dst_idx += if alternate { pitch } else { 1 } - spr.width as c_int;
            if srcy >= MIN_Y {
                drawing = true;
            }
        }
    }
}

#[inline]
fn apply<F: Fn(u8, u8) -> u8>(dst: &mut [u8], src: &[u8], len: i32, f: &F) {
    for i in 0..(len as usize) {
        dst[i] = f(src[i], dst[i]);
    }
}

cpp_alloc! {
    sprite_t: Sprite_Alloc, Sprite_Destruct, Sprite_Dealloc;
}
cpp_methods! {
    sprite_t;
    fn Sprite_LoadData = LoadData(f: *mut FILE) -> bool;
    fn Sprite_SaveData = SaveData(f: *mut FILE) -> bool;
    fn Sprite_Draw = Draw(x: c_int, y: c_int, mgl: &mut MGLDraw) -> ();
    fn Sprite_DrawBright = DrawBright(x: c_int, y: c_int, mgl: &mut MGLDraw, bright: i8) -> ();
}

trait Advance {
    fn advance(&mut self, by: c_int);
}

impl<'a, T> Advance for &'a [T] {
    fn advance(&mut self, by: c_int) {
        *self = &self[by as usize..];
    }
}

impl<'a, T> Advance for &'a mut [T] {
    fn advance(&mut self, by: c_int) {
        *self = &mut ::std::mem::replace(self, &mut [])[by as usize..];
    }
}
