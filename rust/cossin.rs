use libc::c_int;
use ::FIXAMT;

const DEGREES: usize = 256;

thread_local!(static TABLES: ([c_int; DEGREES], [c_int; DEGREES]) = {
    let mut costab = [0; DEGREES];
    let mut sintab = [0; DEGREES];
    for i in 0..DEGREES {
        let angle = (i as f32) * 3.14159 * 2. / (DEGREES as f32);
        costab[i] = (angle.cos() * FIXAMT as f32) as c_int;
        sintab[i] = (angle.sin() * FIXAMT as f32) as c_int;
    }
    (costab, sintab)
});

pub fn InitCosSin() {
    TABLES.with(|_| {});
}

#[no_mangle]
pub extern fn Cosine(angle: c_int) -> c_int {
    TABLES.with(|t| t.0[angle as usize])
}

#[no_mangle]
pub extern fn Sine(angle: c_int) -> c_int {
    TABLES.with(|t| t.1[angle as usize])
}

#[no_mangle]
pub extern fn Dampen(value: &mut c_int, amt: c_int) {
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
pub extern fn Clamp(value: &mut c_int, amt: c_int) {
    if *value > amt {
        *value = amt;
    }
    if *value < -amt {
        *value = -amt;
    }
}
