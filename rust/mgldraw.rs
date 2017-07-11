use libc::c_int;

pub enum MGLDraw {}

pub unsafe fn delete(mgl: *mut MGLDraw) {
    cpp!([mgl as "MGLDraw*"] { delete mgl; });
}

extern {
    pub fn MGL_random(max: c_int) -> c_int;
}
