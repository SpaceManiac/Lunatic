use libc::{c_int, c_char};
use mgldraw::MGLDraw;
extern {
    pub fn SplashScreen(mgl: *mut MGLDraw, fname: *const c_char, delay: c_int, sound: u8);
    pub fn MainMenu(mgl: *mut MGLDraw) -> u8;
}
