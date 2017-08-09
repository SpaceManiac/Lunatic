use libc::{c_int, c_char, c_long};
use ffi::allegro::*;

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

// Allegro shenanigans
static mut prevKey: [i8; KEY_MAX] = [0; KEY_MAX];
static mut closeButtonPressed: bool = false;

unsafe extern fn closeButtonCallback() {
    closeButtonPressed = true;
}
unsafe extern fn switchInCallback() {
    ::game::SetGameIdle(0);
}
unsafe extern fn switchOutCallback() {
    ::game::SetGameIdle(1);
}

// MGLDraw class
#[repr(C)]
pub struct MGLDraw {
    xRes: c_int,
    yRes: c_int,
    pitch: c_int,
    mousex: c_int,
    mousey: c_int,
    scrn: *mut u8,
    buffer: *mut BITMAP,
    pal: [c_int; 256], // palette_t -> c_int
    readyToQuit: bool,
    lastKeyPressed: u8,
    mouseDown: u8,
}
check_size!(_check_MGLDraw, MGLDraw, 1056);

impl Drop for MGLDraw {
    fn drop(&mut self) {
        unsafe {
            ::sound::JamulSoundExit();
            destroy_bitmap(self.buffer);
            Vec::from_raw_parts(self.scrn, 0, (self.xRes * self.yRes) as usize);
        }
    }
}

impl MGLDraw {
    pub unsafe fn new(name: *const c_char, xRes: c_int, yRes: c_int, window: bool) -> *mut MGLDraw {
        Box::into_raw(Box::new(MGLDraw::inner_new(name, xRes, yRes, window)))
    }

    unsafe fn inner_new(name: *const c_char, xRes: c_int, yRes: c_int, window: bool) -> MGLDraw {
        allegro_init();
        install_keyboard();
        install_mouse();
        install_sound(DIGI_AUTODETECT, MIDI_AUTODETECT, cstr!("donotuse.cfg"));
        set_color_depth(32);

        if set_gfx_mode(if window { GFX_AUTODETECT_WINDOWED } else { GFX_AUTODETECT_FULLSCREEN }, xRes, yRes, 0, 0) != 0 {
            panic!("Unable to set graphics mode: {}", ::PctS(decay!(&allegro_error)));
        }
        set_window_title(name);
        set_close_button_callback(closeButtonCallback);
        set_display_switch_mode(SWITCH_BACKGROUND);
        set_display_switch_callback(SWITCH_IN, switchInCallback);
        set_display_switch_callback(SWITCH_OUT, switchOutCallback);

        if ::jamulsound::JamulSoundInit(512) {
            ::sound::SoundSystemExists();
        }

        let mut vec = vec![0u8; (xRes * yRes) as usize];
        let scrn = vec.as_mut_ptr();
        ::std::mem::forget(vec);
        MGLDraw {
            xRes: xRes,
            yRes: yRes,
            pitch: xRes,
            mousex: xRes / 2,
            mousey: yRes / 2,
            scrn: scrn,
            buffer: create_bitmap(xRes, yRes),
            pal: [0; 256],
            readyToQuit: false,
            lastKeyPressed: 0,
            mouseDown: 0,
        }
    }

    pub unsafe fn delete(mgl: *mut MGLDraw) {
        Box::from_raw(mgl);
    }

    pub unsafe fn Process(&mut self) -> bool {
        blit(self.buffer, al_screen, 0, 0, 0, 0, self.xRes, self.yRes);

        while keypressed() != 0 {
            self.SetLastKey(readkey() as u8);
        }

        for i in 0..KEY_MAX {
            if al_key[i] != 0 && prevKey[i] == 0 {
                ::control::ControlKeyDown(i as u8);
            } else if al_key[i] == 0 && prevKey[i] != 0 {
                ::control::ControlKeyUp(i as u8);
            }
            prevKey[i] = al_key[i];
        }

        self.mousex = al_mouse_x;
        self.mousey = al_mouse_y;
        self.mouseDown = al_mouse_b as u8 & 3;
        self.readyToQuit |= closeButtonPressed;
        !self.readyToQuit
    }

    pub unsafe fn Flip(&mut self) {
        if ::game::GetGameIdle() != 0 {
            ::game::GameIdle();
        }

        // This is nice and fast, thankfully
        {
            let screen = ::std::slice::from_raw_parts_mut(self.scrn, (self.pitch * self.yRes) as usize);
            let (mut x, mut y) = (0, 0);
            for &v in screen.iter() {
                _putpixel32(self.buffer, x, y, self.pal[v as usize]);
                x += 1;
                if x >= self.xRes { x = 0; y += 1; }
            }
        }
        self.Process();
    }

    pub unsafe fn ClearScreen(&mut self) {
        for i in 0..self.xRes * self.yRes {
            *self.scrn.offset(i as isize) = 0u8;
        }
    }

    pub unsafe fn GetScreen(&mut self) -> *mut u8 {
        self.scrn
    }

    pub fn get_screen(&mut self) -> &mut [u8] {
        // scrn points to a bytebuffer of size self.pitch * self.yRes
        unsafe {
            ::std::slice::from_raw_parts_mut(self.scrn, (self.pitch * self.yRes) as usize)
        }
    }

    pub fn get_size(&mut self) -> (c_int, c_int) {
        (self.pitch, self.yRes)
    }

    pub fn Quit(&mut self) {
        self.readyToQuit = true;
    }

    // LoadPalette

    pub fn set_palette(&mut self, pal2: &[palette_t]) {
        assert_eq!(pal2.len(), 256);
        for (p, &c) in self.pal.iter_mut().zip(pal2.iter()) {
            *p = unsafe { makecol(c.red as c_int, c.green as c_int, c.blue as c_int) };
        }
    }

    pub fn Box(&mut self, x: c_int, y: c_int, x2: c_int, y2: c_int, c: u8) {
        use std::cmp::{min, max};

        if x2 < 0 || y2 < 0 { return; }
        let mut x = max(0, min(self.xRes - 1, x));
        let mut y = max(0, min(self.yRes - 1, y));
        let mut x2 = min(self.xRes - 1, x2);
        let mut y2 = min(self.yRes - 1, y2);

        if x > x2 { ::std::mem::swap(&mut x, &mut x2); }
        if y > y2 { ::std::mem::swap(&mut y, &mut y2); }

        let pitch = self.pitch;
        let screen = self.get_screen();
        ::memset(&mut screen[(x + y * pitch) as usize..], c, (x2 - x + 1) as usize);
        ::memset(&mut screen[(x + y2 * pitch) as usize..], c, (x2 - x + 1) as usize);
        for i in y..y2 + 1 {
            screen[(x + i * pitch) as usize] = c;
            screen[(x2 + i * pitch) as usize] = c;
        }
    }

    pub fn FillBox(&mut self, x: c_int, y: c_int, x2: c_int, y2: c_int, c: u8) {
        use std::cmp::{min, max};

        if x2 < 0 || y2 < 0 || y >= self.yRes { return; }
        let x = max(0, min(self.xRes - 1, x));
        let y = max(0, min(self.yRes - 1, y));
        let x2 = min(self.xRes - 1, x2);
        let y2 = min(self.yRes - 1, y2);

        let pitch = self.pitch;
        let screen = self.get_screen();
        for i in y..y2 + 1 {
            ::memset(&mut screen[(x + i * pitch) as usize..], c, (x2 - x + 1) as usize);
        }
    }

    pub fn SetLastKey(&mut self, c: u8) {
        self.lastKeyPressed = c;
    }

    pub fn LastKeyPressed(&mut self) -> u8 {
        ::std::mem::replace(&mut self.lastKeyPressed, 0)
    }

    pub fn LastKeyPeek(&mut self) -> u8 {
        self.lastKeyPressed
    }

    pub fn SetMouseDown(&mut self, w: u8) {
        self.mouseDown = w;
    }

    pub fn MouseDown(&mut self) -> u8 {
        self.mouseDown
    }

    pub fn SetMouse(&mut self, x: c_int, y: c_int) {
        self.mousex = x;
        self.mousey = y;
    }

    pub fn get_mouse(&mut self) -> (c_int, c_int) {
        (self.mousex, self.mousey)
    }

    pub unsafe fn TeleportMouse(&mut self, x: c_int, y: c_int) {
        use ffi::misc::{POINT, ClientToScreen, SetCursorPos};

        let mut pt = POINT { x: x as c_long, y: y as c_long };
        ClientToScreen(win_get_window(), &mut pt);
        SetCursorPos(pt.x, pt.y);
        self.SetMouse(x, y);
    }

    pub unsafe fn LoadBMP(&mut self, name: *const c_char) -> bool {
        /*let me = self;
        cpp!([me as "MGLDraw*", name as "const char*"] -> bool as "bool" {
            return me->LoadBMP(name);
        })*/
        use libc::{fopen, fread, fclose};
        use ffi::misc::*;

        let mut bmpFHead: BITMAPFILEHEADER = ::std::mem::zeroed();
        let mut bmpIHead: BITMAPINFOHEADER = ::std::mem::zeroed();
        let mut pal2: [RGBQUAD; 256] = ::std::mem::zeroed();

        let f = fopen(name, cstr!("rb"));
        if f.is_null() { return false; }

        fread(decay!(&mut bmpFHead), szof!(BITMAPFILEHEADER), 1, f);
        fread(decay!(&mut bmpIHead), szof!(BITMAPINFOHEADER), 1, f);

        // 8-bit BMPs only
        if bmpIHead.biBitCount != 8 { return false; }

        // Non-RLE BMPs only
        if bmpIHead.biCompression != 0 {
            println!("bitmap {} is compressed ({})", ::PctS(name), bmpIHead.biCompression);
            return false;
        }

        fread(decay!(&mut pal2), 256 * szof!(RGBQUAD), 1, f);
        for i in 0..256 {
            self.pal[i] = makecol(pal2[i].rgbRed as c_int, pal2[i].rgbGreen as c_int, pal2[i].rgbBlue as c_int);
        }

        let pitch = self.pitch;
        let screen = self.get_screen();
        for i in 0..bmpIHead.biHeight {
            fread(
                decay!(&mut screen[((bmpIHead.biHeight - 1 - i) * pitch) as usize]),
                1, bmpIHead.biWidth as usize, f);
        }
        fclose(f);
        true
    }

    pub fn GammaCorrect(&mut self, _gamma: u8) {
        // TODO: add back this logic if it makes sense to do so.
        // The C implementation was broken to the point of doing nothing.

        // for (r,g,b) in palette:
        //   r = min(255, (r * (gamma + 4)) / 4)
        //   g = min(255, (g * (gamma + 4)) / 4)
        //   b = min(255, (b * (gamma + 4)) / 4)
    }
}

#[no_mangle]
pub unsafe extern fn MGLDraw_Process(mgl: &mut MGLDraw) -> bool {
    mgl.Process()
}

#[no_mangle]
pub unsafe extern fn MGLDraw_Flip(mgl: &mut MGLDraw) {
    mgl.Flip()
}

#[no_mangle]
pub unsafe extern fn MGLDraw_SetPalette(mgl: &mut MGLDraw, palette: *const palette_t) {
    mgl.set_palette(::std::slice::from_raw_parts(palette, 256));
}

#[no_mangle]
pub unsafe extern fn MGLDraw_Box(mgl: &mut MGLDraw, x: c_int, y: c_int, x2: c_int, y2: c_int, c: u8) {
    mgl.Box(x, y, x2, y2, c)
}

#[no_mangle]
pub unsafe extern fn MGLDraw_FillBox(mgl: &mut MGLDraw, x: c_int, y: c_int, x2: c_int, y2: c_int, c: u8) {
    mgl.FillBox(x, y, x2, y2, c)
}

#[no_mangle]
pub unsafe extern fn MGLDraw_TeleportMouse(mgl: &mut MGLDraw, x: c_int, y: c_int) {
    mgl.TeleportMouse(x, y)
}

#[no_mangle]
pub unsafe extern fn MGLDraw_LoadBMP(mgl: &mut MGLDraw, name: *const c_char) -> bool {
    mgl.LoadBMP(name)
}

#[no_mangle]
pub unsafe extern fn MGLDraw_GammaCorrect(mgl: &mut MGLDraw, gamma: u8) {
    mgl.GammaCorrect(gamma)
}
