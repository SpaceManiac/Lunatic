use libc::{c_int, c_char};
extern {
    pub fn GetCamera(cx: *mut c_int, cy: *mut c_int);
    pub fn Print(x: c_int, y: c_int, s: *const c_char, bright: c_char, font: u8);
    pub fn GetStrLength(s: *const c_char) -> c_int;
    pub fn ShakeScreen(howlong: u8);
}

pub unsafe fn get_camera() -> (c_int, c_int) {
    let (mut cx, mut cy) = (0, 0);
    GetCamera(&mut cx, &mut cy);
    (cx, cy)
}
