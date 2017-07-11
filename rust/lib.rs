#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#[macro_use] extern crate enum_derive;
#[macro_use] extern crate cpp;
extern crate libc;
use libc::*;

macro_rules! cstr {
    ($e:expr) => { concat!($e, "\0").as_ptr() as *const _ };
	() => { "\0".as_ptr() as *const _ };
}

// imports
pub mod logg_sys;

// jamultypes.h
pub const FIXSHIFT: c_int = 16;
pub const FIXAMT: c_int = 65536;

// modules
pub mod clock;
pub mod cossin;
pub mod display;
pub mod editor;
pub mod game;
pub mod jamulsound;
pub mod mgldraw;
pub mod music;
pub mod options;
pub mod sound;
pub mod title;

// main.cpp
#[no_mangle]
pub unsafe extern fn parseCmdLine(_argv: *const c_char, windowed: *mut bool) {
    for arg in std::env::args_os() {
        if arg == std::ffi::OsStr::new("window") {
            *windowed = true;
        }
    }
}

// int PASCAL WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR cmdLine, int nCmdShow)
#[no_mangle]
pub unsafe extern "system" fn WinMain(_: *const c_void, _: *const c_void, _: *const c_char, _: c_int) -> c_int {
	let mut windowedGame = false;
	parseCmdLine(0 as *const _, &mut windowedGame);
    let mainmgl = cpp!([windowedGame as "bool"] -> *mut mgldraw::MGLDraw as "MGLDraw*" {
		return new MGLDraw("Dr. Lunatic", 640, 480, windowedGame);
	});

	game::LunaticInit(mainmgl);
	title::SplashScreen(mainmgl, cstr!("graphics\\hamumu.bmp"), 128, 2);

    loop {
        match title::MainMenu(mainmgl) {
            0 => game::LunaticGame(mainmgl, 0), // new game
            1 => game::LunaticGame(mainmgl, 1), // continue
            3 => { editor::LunaticEditor(mainmgl); } // editor
            4 | 255 => {
                game::LunaticExit();
				mgldraw::delete(mainmgl);
				return 0;
            }
			_ => {}
        }
    }
}
