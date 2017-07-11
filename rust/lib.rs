#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#[macro_use] extern crate enum_derive;
#[macro_use] extern crate cpp;
extern crate libc;
use libc::*;

// imports
pub mod logg_sys;

// jamultypes.h
pub const FIXSHIFT: c_int = 16;
pub const FIXAMT: c_int = 65536;

// modules
pub mod clock;
pub mod cossin;
pub mod display;
pub mod jamulsound;
pub mod mgldraw;
pub mod music;
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
