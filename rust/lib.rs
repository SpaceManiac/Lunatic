use std::os::raw::c_char;
use std::ffi::CStr;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern fn parseCmdLine(argv: *const c_char, windowed: *mut bool) {
    let text = CStr::from_ptr(argv);
    if let Ok(text) = text.to_str() {
        for token in text.split(' ') {
            if token == "window" {
                *windowed = true;
            }
        }
    }
}
