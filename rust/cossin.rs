use libc::c_int;
use ::FIXAMT;

const DEGREES: usize = 256;

static mut COSTAB: [c_int; DEGREES] = [0; DEGREES];
static mut SINTAB: [c_int; DEGREES] = [0; DEGREES];

#[no_mangle]
pub unsafe extern fn InitCosSin() {
    for i in 0..DEGREES {
        let angle = (i as f32) * 3.14159 * 2. / (DEGREES as f32);
        COSTAB[i] = (angle.cos() * FIXAMT as f32) as c_int;
        SINTAB[i] = (angle.sin() * FIXAMT as f32) as c_int;
    }
}

#[no_mangle]
pub unsafe extern fn Cosine(angle: c_int) -> c_int {
    COSTAB[angle as usize]
}

#[no_mangle]
pub unsafe extern fn Sine(angle: c_int) -> c_int {
    SINTAB[angle as usize]
}

#[no_mangle]
pub unsafe extern fn Dampen(value: *mut c_int, amt: c_int) {
    if *value > 0 {
        *value -= amt;
        if *value < 0 { *value = 0 }
    }
    if *value < 0 {
        *value += amt;
        if *value > 0 { *value = 0}
    }
}

#[no_mangle]
pub unsafe extern fn Clamp(value: *mut c_int, amt: c_int) {
    if *value > amt {
        *value = amt;
    }
    if *value < -amt {
        *value = -amt;
    }
}
