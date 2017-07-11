#![allow(non_snake_case)]

extern crate libc;

// jamultypes.h
pub const FIXSHIFT: libc::c_int = 16;
pub const FIXAMT: libc::c_int = 65536;

// clock.h
pub mod clock;
// cossin.h
pub mod cossin;

// main.cpp
#[no_mangle]
pub unsafe extern fn parseCmdLine(_argv: *const libc::c_char, windowed: *mut bool) {
    for arg in std::env::args_os() {
        if arg == std::ffi::OsStr::new("window") {
            *windowed = true;
        }
    }
}
