use libc::{c_int, c_char, c_long};

/// Replacement for missing palette_t
#[repr(C)]
#[derive(Copy, Clone)]
pub struct palette_t {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// For appdata storage of stuff
#[no_mangle]
pub unsafe extern fn AppdataOpen(file: *const c_char, mode: *const c_char) -> *mut ::libc::FILE {
    use libc::{mkdir, strlen, fopen};
    use std::ptr::null_mut;
    use ffi::misc::{SHGetFolderPath, CSIDL_APPDATA};

    let mut buffer = [0; 260]; // MAX_PATH
    SHGetFolderPath(null_mut(), CSIDL_APPDATA, null_mut(), 0, decay!(&mut buffer));

    let len = strlen(decay!(&buffer));
    sprintf!(buffer[len..], "\\Hamumu",);
    mkdir(decay!(&buffer));

    let len = strlen(decay!(&buffer));
    sprintf!(buffer[len..], "\\DrLunatic",);
    mkdir(decay!(&buffer));

    let len = strlen(decay!(&buffer));
    sprintf!(buffer[len..], "\\{}", ::PctS(file));
    mkdir(decay!(&buffer));

    fopen(decay!(&buffer), mode)
}

// Replacements for missing MGL functions
use rand::{SeedableRng, Rng};
use mersenne_twister::MT19937_64;
use std::cell::RefCell;

thread_local!(static mersenne: RefCell<MT19937_64> = RefCell::new(MT19937_64::new_unseeded()));

#[no_mangle]
pub extern fn MGL_srand(seed: c_int) {
    mersenne.with(|m| m.borrow_mut().reseed(seed as u64));
}

#[no_mangle]
pub extern fn MGL_random(max: c_int) -> c_int {
    mersenne.with(|m| m.borrow_mut().gen_range(0, max))
}

#[no_mangle]
pub extern fn MGL_randoml(max: c_long) -> c_long {
    mersenne.with(|m| m.borrow_mut().gen_range(0, max))
}

#[no_mangle]
pub unsafe extern fn MGL_fatalError(txt: *const c_char) {
    /*
    The old Allegro way doesn't seem to actually do what I thought it did,
    which is show a pretty error message box:
        set_gfx_mode(GFX_TEXT, 0, 0, 0, 0);
        allegro_message(txt);
        exit(0);
    so instead panic. This isn't really going to be able to unwind with all the
    C flying around, and `-C panic=abort` should really be set, but in the
    meantime, maybe a backtrace will be useful.
    */
    panic!("{}", ::PctS(txt));
}

// MGLDraw class
opaque!(MGLDraw);

impl MGLDraw {
    pub unsafe fn new(name: *const c_char, xRes: c_int, yRes: c_int, window: bool) -> *mut MGLDraw {
        cpp!([name as "const char*", xRes as "int", yRes as "int", window as "bool"] -> *mut MGLDraw as "MGLDraw*" {
            return new MGLDraw(name, xRes, yRes, window);
        })
    }

    pub unsafe fn delete(mgl: *mut MGLDraw) {
        cpp!([mgl as "MGLDraw*"] { delete mgl; });
    }

    pub unsafe fn LastKeyPressed(&mut self) -> u8 {
        let mgl = self;
        cpp!([mgl as "MGLDraw*"] -> u8 as "char" {
            return mgl->LastKeyPressed();
        })
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

    pub unsafe fn SetPalette(&mut self, pal2: &[palette_t]) {
        assert_eq!(pal2.len(), 256);
        let mgl = self;
        let pal2 = pal2.as_ptr();
        cpp!([mgl as "MGLDraw*", pal2 as "const palette_t*"] {
            mgl->SetPalette(pal2);
        })
    }

    pub unsafe fn ClearScreen(&mut self) {
        let mgl = self;
        cpp!([mgl as "MGLDraw*"] {
            mgl->ClearScreen();
        })
    }

    pub unsafe fn Process(&mut self) -> bool {
        let mgl = self;
        cpp!([mgl as "MGLDraw*"] -> bool as "bool" {
            return mgl->Process();
        })
    }

    pub unsafe fn Flip(&mut self) {
        let mgl = self;
        cpp!([mgl as "MGLDraw*"] {
            mgl->Flip();
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
