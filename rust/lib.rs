#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate enum_derive;
#[macro_use] extern crate cpp;
extern crate libc;
use libc::*;

macro_rules! cstr {
    ($e:expr) => { concat!($e, "\0").as_ptr() as *const _ };
    () => { "\0".as_ptr() as *const _ };
}

macro_rules! opaque {
    ($name:ident) => { #[repr(C)] pub struct $name { _opaque: [u8; 0] } };
}

macro_rules! sprintf {
    ($buf:expr, $text:expr, $($rest:tt)*) => {{
        use std::io::Write;
        match write!(&mut $buf[..], concat!($text, "\0"), $($rest)*) {
            Err(ref e) if e.kind() == ::std::io::ErrorKind::WriteZero => {
                let len = $buf.len();
                $buf[len - 1] = 0;
            }
            other => other.unwrap(),
        }
    }}
}

macro_rules! szof {
    ($t:ty) => { ::std::mem::size_of::<$t>() }
}

struct PctS { ptr: *const c_char }
unsafe fn PctS(ptr: *const c_char) -> PctS { PctS { ptr } }
impl std::fmt::Display for PctS {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe {
            let cstr = std::ffi::CStr::from_ptr(self.ptr);
            fmt.write_str(&cstr.to_string_lossy())
        }
    }
}

// imports
pub mod allegro_sys;
pub mod misc_sys;
pub mod logg_sys;

// jamultypes.h
pub const FIXSHIFT: c_int = 16;
pub const FIXAMT: c_int = 65536;

// modules
pub mod bullet;
pub mod cheat;
pub mod clock;
pub mod cossin;
pub mod display;
pub mod editor;
pub mod filedialog;
pub mod game;
pub mod guy;
pub mod intface;
pub mod items;
pub mod jamulsound;
pub mod map;
pub mod message;
pub mod mgldraw;
pub mod music;
pub mod options;
pub mod player;
pub mod rage;
pub mod sound;
pub mod tile;
pub mod tiledialog;
pub mod title;
pub mod world;

cpp! {{
    #include "guy.h"
}}

// int PASCAL WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR cmdLine, int nCmdShow)
#[no_mangle]
pub unsafe extern "system" fn WinMain(_: *const c_void, _: *const c_void, _: *const c_char, _: c_int) -> c_int {
    use mgldraw::MGLDraw;

    let mut windowedGame = false;
    for arg in std::env::args_os() {
        if arg == std::ffi::OsStr::new("window") {
            windowedGame = true;
        }
    }

    let mainmgl = MGLDraw::new(cstr!("Dr. Lunatic"), 640, 480, windowedGame);

    game::LunaticInit(mainmgl);
    title::SplashScreen(mainmgl, cstr!("graphics\\hamumu.bmp"), 128, 2);

    loop {
        match title::MainMenu(mainmgl) {
            0 => game::LunaticGame(mainmgl, 0), // new game
            1 => game::LunaticGame(mainmgl, 1), // continue
            3 => { editor::LunaticEditor(mainmgl); } // editor
            4 | 255 => {
                game::LunaticExit();
                MGLDraw::delete(mainmgl);
                return 0;
            }
            _ => {}
        }
    }
}
