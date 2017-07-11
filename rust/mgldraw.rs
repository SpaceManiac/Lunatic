use libc::c_int;
extern {
    pub fn MGL_random(max: c_int) -> c_int;
}
