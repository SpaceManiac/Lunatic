use ffi::misc::timeGetTime;

static mut START: u32 = 0;
static mut END: u32 = 0;

#[no_mangle]
pub unsafe extern fn StartClock() {
    START = timeGetTime();
    END = START;
}

#[no_mangle]
pub unsafe extern fn EndClock() {
    END = timeGetTime();
}

#[no_mangle]
pub unsafe extern fn TimeLength() -> u32 {
    END - START
}
