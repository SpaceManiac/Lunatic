#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use] extern crate cpp;
extern crate libc;
use libc::*;

// jamultypes.h
pub const FIXSHIFT: c_int = 16;
pub const FIXAMT: c_int = 65536;

// modules
pub mod clock;
pub mod cossin;
pub mod display;
pub mod jamulsound;
pub mod options;
pub mod sound;

// main.cpp
#[no_mangle]
pub unsafe extern fn parseCmdLine(_argv: *const c_char, windowed: *mut bool) {
    for arg in std::env::args_os() {
        if arg == std::ffi::OsStr::new("window") {
            *windowed = true;
        }
    }
}
