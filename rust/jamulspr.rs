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
        let me = self;
        cpp!([me as "sprite_t*", x as "int", y as "int", mgl as "MGLDraw*"] {
            me->Draw(x, y, mgl);
        })
    }

    pub unsafe fn DrawBright(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, bright: i8) {
        let me = self;
        cpp!([me as "sprite_t*", x as "int", y as "int", mgl as "MGLDraw*", bright as "char"] {
            me->DrawBright(x, y, mgl, bright);
        })
    }

    pub unsafe fn DrawColored(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, hue: u8, bright: i8) {
        let me = self;
        cpp!([me as "sprite_t*", x as "int", y as "int", mgl as "MGLDraw*", hue as "byte", bright as "char"] {
            me->DrawColored(x, y, mgl, hue, bright);
        })
    }

    pub unsafe fn DrawOffColor(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, from: u8, to: u8, bright: i8) {
        let me = self;
        cpp!([me as "sprite_t*", x as "int", y as "int", mgl as "MGLDraw*", from as "byte", to as "byte", bright as "char"] {
            me->DrawOffColor(x, y, mgl, from, to, bright);
        })
    }

    pub unsafe fn DrawGhost(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, bright: i8) {
        let me = self;
        cpp!([me as "sprite_t*", x as "int", y as "int", mgl as "MGLDraw*", bright as "char"] {
            me->DrawGhost(x, y, mgl, bright);
        })
    }

    pub unsafe fn DrawGlow(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, bright: i8) {
        let me = self;
        cpp!([me as "sprite_t*", x as "int", y as "int", mgl as "MGLDraw*", bright as "char"] {
            me->DrawGlow(x, y, mgl, bright);
        })
    }

    /// this makes half-height tilted black shadows (they darken by 4)
    pub unsafe fn DrawShadow(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw) {
        let me = self;
        cpp!([me as "sprite_t*", x as "int", y as "int", mgl as "MGLDraw*"] {
            me->DrawShadow(x, y, mgl);
        })
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
    // new()

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
    // Free()

    pub fn GetSprite(&mut self, which: c_int) -> &mut sprite_t {
        self.sprites_mut()[which as usize]
    }

    pub unsafe fn GetCount(&self) -> u16 {
        self.count
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
