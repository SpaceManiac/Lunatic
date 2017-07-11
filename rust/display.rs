use libc::c_int;
extern {
    pub fn GetCamera(cx: *mut c_int, cy: *mut c_int);
    pub fn ShakeScreen(howlong: u8);
}
