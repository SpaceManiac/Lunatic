use libc::{c_int, c_char};

opaque!(MGLDraw);

extern {
    pub fn MGL_random(max: c_int) -> c_int;
}

impl MGLDraw {
    pub unsafe fn new(name: *const c_char, xRes: c_int, yRes: c_int, window: bool) -> *mut MGLDraw {
        cpp!([name as "const char*", xRes as "int", yRes as "int", window as "bool"] -> *mut MGLDraw as "MGLDraw*" {
            return new MGLDraw(name, xRes, yRes, window);
        })
    }

    pub unsafe fn delete(mgl: *mut MGLDraw) {
        cpp!([mgl as "MGLDraw*"] { delete mgl; });
    }

    pub unsafe fn Box(&mut self, x: c_int, y: c_int, x2: c_int, y2: c_int, c: u8) {
        let mgl = self;
        cpp!([mgl as "MGLDraw*", x as "int", y as "int", x2 as "int", y2 as "int", c as "byte"] {
            mgl->Box(x, y, x2, y2, c);
        })
    }

    pub unsafe fn FillBox(&mut self, x: c_int, y: c_int, x2: c_int, y2: c_int, c: u8) {
        let mgl = self;
        cpp!([mgl as "MGLDraw*", x as "int", y as "int", x2 as "int", y2 as "int", c as "byte"] {
            mgl->FillBox(x, y, x2, y2, c);
        })
    }

    pub unsafe fn LoadBMP(&mut self, name: *const c_char) -> bool {
        let mgl = self;
        cpp!([mgl as "MGLDraw*", name as "const char*"] -> bool as "bool" {
            return mgl->LoadBMP(name);
        })
    }

    pub unsafe fn GetScreen(&mut self) -> *mut u8 {
        let mgl = self;
        cpp!([mgl as "MGLDraw*"] -> *mut u8 as "byte*" {
            return mgl->GetScreen();
        })
    }

    pub unsafe fn get_size(&mut self) -> (c_int, c_int) {
        let mgl = self;
        let (mut x, mut y) = (0, 0);
        cpp!([mgl as "MGLDraw*", mut x as "int", mut y as "int"] {
            x = mgl->GetWidth();
            y = mgl->GetHeight();
        });
        (x, y)
    }
}
