use libc::c_int;

opaque!(MGLDraw);

extern {
    pub fn MGL_random(max: c_int) -> c_int;
}

impl MGLDraw {
    pub unsafe fn Box(&mut self, x: c_int, y: c_int, x2: c_int, y2: c_int, c: u8) {
        let mut mgl = self as *mut MGLDraw;
        cpp!([mut mgl as "MGLDraw*", x as "int", y as "int", x2 as "int", y2 as "int", c as "byte"] {
            mgl->Box(x, y, x2, y2, c);
        })
    }

    pub unsafe fn FillBox(&mut self, x: c_int, y: c_int, x2: c_int, y2: c_int, c: u8) {
        let mut mgl = self as *mut MGLDraw;
        cpp!([mut mgl as "MGLDraw*", x as "int", y as "int", x2 as "int", y2 as "int", c as "byte"] {
            mgl->FillBox(x, y, x2, y2, c);
        })
    }
}

pub unsafe fn delete(mgl: *mut MGLDraw) {
    cpp!([mgl as "MGLDraw*"] { delete mgl; });
}
