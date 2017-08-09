use libc::{c_char, c_int};
use mgldraw::MGLDraw;

opaque!(sprite_t);
opaque!(sprite_set_t);

impl sprite_t {
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

    pub unsafe fn DrawGlow(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, bright: i8) {
        let me = self;
        cpp!([me as "sprite_t*", x as "int", y as "int", mgl as "MGLDraw*", bright as "char"] {
            me->DrawGlow(x, y, mgl, bright);
        })
    }

    pub unsafe fn DrawGhost(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, bright: i8) {
        let me = self;
        cpp!([me as "sprite_t*", x as "int", y as "int", mgl as "MGLDraw*", bright as "char"] {
            me->DrawGhost(x, y, mgl, bright);
        })
    }

    pub unsafe fn DrawOffColor(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, from: u8, to: u8, bright: i8) {
        let me = self;
        cpp!([me as "sprite_t*", x as "int", y as "int", mgl as "MGLDraw*", from as "byte", to as "byte", bright as "char"] {
            me->DrawOffColor(x, y, mgl, from, to, bright);
        })
    }

    pub unsafe fn DrawColored(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw, hue: u8, bright: i8) {
        let me = self;
        cpp!([me as "sprite_t*", x as "int", y as "int", mgl as "MGLDraw*", hue as "byte", bright as "char"] {
            me->DrawColored(x, y, mgl, hue, bright);
        })
    }

    pub unsafe fn DrawShadow(&mut self, x: c_int, y: c_int, mgl: &mut MGLDraw) {
        let me = self;
        cpp!([me as "sprite_t*", x as "int", y as "int", mgl as "MGLDraw*"] {
            me->DrawShadow(x, y, mgl);
        })
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

    pub unsafe fn GetSprite(&mut self, which: c_int) -> &mut sprite_t {
        let me = self;
        cpp!([me as "sprite_set_t*", which as "int"] -> &mut sprite_t as "sprite_t*" {
            //if (which < 0 || which >= me->GetCount()) abort();
            return me->GetSprite(which);
        })
    }
}
