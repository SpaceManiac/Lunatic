#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use] extern crate cpp;
extern crate libc;
use libc::*;

// jamultypes.h
pub const FIXSHIFT: c_int = 16;
pub const FIXAMT: c_int = 65536;

// jamulsound.h
/// cut off same sound if needed
pub const SND_CUTOFF: u8 = 1;
/// ignore priority value, this sound is a must-have
pub const SND_MAXPRIORITY: u8 = 2;
/// only one copy may play at once
pub const SND_ONE: u8 = 4;
/// well, it's not for everyone, but it goes here
pub const SND_PLAYING: u8 = 8;
/// only allow MAX_FEW_SOUNDS copies to play at once
pub const SND_FEW: u8 = 16;
pub const MAX_SNDPRIORITY: c_int = 65536;

// modules
pub mod clock;
pub mod cossin;
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
