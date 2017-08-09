use libc::c_int;

extern {
    pub fn RenderParticle(x: c_int, y: c_int, scrn: *mut u8, color: u8, size: u8);
    pub fn RenderLightningParticle(x1: c_int, y1: c_int, x2: c_int, y2: c_int, range: c_int, bright: u8, scrn: *mut u8);
}
