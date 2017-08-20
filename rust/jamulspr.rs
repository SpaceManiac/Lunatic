use libc::{c_char, c_int};
use mgldraw::MGLDraw;

#[repr(C)]
pub struct sprite_t {
    pub width: u16,
    pub height: u16,
    pub ofsx: i16,
    pub ofsy: i16,
    pub size: u32,
    pub data: *mut u8,
}

#[repr(C)]
pub struct sprite_set_t {
    count: u16,
    spr: *mut *mut sprite_t,
}

impl sprite_t {
    // new()
    // from_header(header: *mut u8)
    // delete()

    // LoadData
    // SaveData
    // GetHeader

    pub unsafe fn Draw(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw) {
        sprite_draw(self, x, y, mgl, |src, _| src);
    }

    // bright: how much to darken or lighten the whole thing (-16 to +16 reasonable)
    pub unsafe fn DrawBright(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, bright: i8) {
        if bright == 0 {
            return self.Draw(x, y, mgl); // don't waste time
        }

        sprite_draw(self, x, y, mgl, |src, _| SprModifyLight(src, bright));
    }

    // color:  which hue (0-7) to use for the entire thing, ignoring its real hue
    // bright: how much to darken or lighten the whole thing (-16 to +16 reasonable)
    pub unsafe fn DrawColored(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, hue: u8, bright: i8) {
        sprite_draw(self, x, y, mgl, |src, _| SprModifyLight(SprModifyColor(src, hue), bright));
    }

    pub unsafe fn DrawOffColor(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, from: u8, to: u8, bright: i8) {
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
    pub unsafe fn DrawGhost(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, bright: i8) {
        sprite_draw(self, x, y, mgl, |src, dst| SprModifyGhost(src, dst, bright));
    }

    pub unsafe fn DrawGlow(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, bright: i8) {
        sprite_draw(self, x, y, mgl, |src, dst| SprModifyGlow(src, dst, bright));
    }

    /// this makes half-height tilted black shadows (they darken by 4)
    pub unsafe fn DrawShadow(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw) {
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
pub unsafe extern fn SprModifyColor(color: u8, hue: u8) -> u8 {
    (hue << 5) | (color & 31)
}

#[no_mangle]
pub unsafe extern fn SprGetColor(color: u8) -> u8 {
    color >> 5
}

#[no_mangle]
pub unsafe extern fn SprModifyLight(color: u8, bright: i8) -> u8 {
    let mut value = (color & 31).wrapping_add(bright as u8);
    if value > 128 { value = 0; } // since byte is unsigned...
    else if value > 31 { value = 31; }
    (color & !31) | value
}

#[no_mangle]
pub unsafe extern fn SprModifyGhost(src: u8, dst: u8, bright: i8) -> u8 {
    if src < 31 {
        SprModifyLight(dst, src as i8)
    } else {
        SprModifyLight(src, bright)
    }
}

#[no_mangle]
pub unsafe extern fn SprModifyGlow(src: u8, dst: u8, bright: i8) -> u8 {
    SprModifyLight(src, (dst & 31) as i8 + bright)
}

const MIN_X: c_int = 0;
const MIN_Y: c_int = 0;
const MAX_X: c_int = 639;
const MAX_Y: c_int = 479;

unsafe fn sprite_draw<F: Fn(u8, u8) -> u8>(
    spr: &sprite_t, mut x: c_int, mut y: c_int, mgl: &mut MGLDraw, f: F
) {
    x -= spr.ofsx as c_int;
    y -= spr.ofsy as c_int;

    if x > MAX_X || y > MAX_Y {
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

    let end_y = ::std::cmp::min(MAX_Y + 1, spr.height as c_int + y);
    while srcy < end_y {
        if src[0] & 128 != 0 { // transparent run
            let b = (src[0] & 127) as c_int;
            srcx += b;
            src.advance(1);
            dst_idx += b;
        } else { // solid run
            let mut b = src[0] as c_int;
            src.advance(1);
            if srcx < MIN_X - b || srcx > MAX_X {
                // don't draw this line
            } else if srcx < MIN_X {
                // skip some of the beginning
                let skip = MIN_X - srcx;
                srcx += skip;
                src.advance(skip);
                dst_idx += skip;
                b -= skip;
                if srcx > MAX_X - b {
                    let skip = (b - (MAX_X - srcx)) - 1;
                    apply!(b - skip);
                } else {
                    apply!(b);
                }
            } else if srcx > MAX_X - b {
                // skip some of the end
                let skip = (srcx - (MAX_X - b)) - 1;
                apply!(b - skip);
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

unsafe fn sprite_draw_shadow(
    spr: &sprite_t, mut x: c_int, mut y: c_int, mgl: &mut MGLDraw
) {
    x -= spr.ofsx as c_int + spr.height as c_int / 2;
    y -= spr.ofsy as c_int / 2;

    if x > MAX_X || y > MAX_Y {
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

    let end_y = ::std::cmp::min(MAX_Y + 1, spr.height as c_int / 2 + y);
    while srcy < end_y {
        if src[0] & 128 != 0 { // transparent run
            let b = (src[0] & 127) as c_int;
            srcx += b;
            src.advance(1);
            dst_idx += b;
        } else { // solid run
            let mut b = src[0] as c_int;
            src.advance(1);
            if srcx < MIN_X - b || srcx > MAX_X {
                // don't draw this line
            } else if srcx < MIN_X {
                // skip some of the beginning
                let skip = MIN_X - srcx;
                srcx += skip;
                src.advance(skip);
                dst_idx += skip;
                b -= skip;
                if srcx > MAX_X - b {
                    let skip = (b - (MAX_X - srcx)) - 1;
                    apply!(b - skip);
                } else {
                    apply!(b);
                }
            } else if srcx > MAX_X - b {
                // skip some of the end
                let skip = (srcx - (MAX_X - b)) - 1;
                apply!(b - skip);
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

cpp_export! {
    Sprite_Draw: Draw(spr: &mut sprite_t, x: c_int, y: c_int, mgl: &mut MGLDraw) -> ();
    Sprite_DrawBright: DrawBright(spr: &mut sprite_t, x: c_int, y: c_int, mgl: &mut MGLDraw, bright: i8) -> ();
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
