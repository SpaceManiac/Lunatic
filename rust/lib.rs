#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate enum_derive;
#[macro_use] extern crate cpp;
extern crate libc;
extern crate flic;
extern crate mersenne_twister;
extern crate rand;
extern crate byteorder;
use libc::*;

macro_rules! cstr {
    ($e1:expr, $($e2:expr),*$(,)*) => {
        [cstr!($e1), $(cstr!($e2),)*]
    };
    ($e:expr) => { concat!($e, "\0") as *const str as *const _ };
    () => { &0i8 as *const _ };
}

macro_rules! opaque {
    ($name:ident) => { #[repr(C)] pub struct $name { _opaque: [u8; 0] } };
}

macro_rules! sprintf {
    ($buf:expr, $text:expr, $($rest:tt)*) => {{
        use std::io::Write;
        let buf = &mut $buf[..];
        match write!(&mut *buf, concat!($text, "\0"), $($rest)*) {
            Err(ref e) if e.kind() == ::std::io::ErrorKind::WriteZero => {
                let len = buf.len();
                buf[len - 1] = 0;
            }
            other => other.unwrap(),
        }
    }}
}

macro_rules! szof {
    ($t:ty) => { ::std::mem::size_of::<$t>() }
}

macro_rules! decay {
    (&mut $e:expr) => { (&mut $e) as *mut _ as *mut _ };
    (&$e:expr) => { (&$e) as *const _ as *const _ };
    ($e:expr) => { $e as *const _ as *mut _ };
}

macro_rules! cpp_alloc {
    ($t:ty: $alloc:ident, $destruct:ident, $dealloc:ident;
        $(fn $new:ident = $new2:ident($( $arg:ident: $argT:ty ),*);)*
    ) => {
        #[no_mangle]
        pub unsafe extern fn $alloc() -> *mut $t {
            Box::into_raw(Box::new(::std::mem::uninitialized()))
        }
        #[no_mangle]
        pub unsafe extern fn $destruct(this: *mut $t) {
            ::std::ptr::drop_in_place(this);
        }
        #[no_mangle]
        pub unsafe extern fn $dealloc(this: *mut $t) {
            ::std::mem::forget(*Box::from_raw(this));
        }
        $(
            #[no_mangle]
            pub unsafe extern fn $new(this: *mut $t $(, $arg: $argT)*) {
                ::std::ptr::write(this, <$t>::$new2($($arg),*))
            }
        )*
    }
}

macro_rules! cpp_methods {
    ($ty1:ty; $(fn $cname:ident = $method:ident($($id2:ident: $ty2:ty),*) -> $o:ty;)*) => (
        $(
            #[no_mangle]
            pub unsafe extern fn $cname(this: &mut $ty1 $(, $id2: $ty2)*) -> $o {
                this.$method($($id2),*)
            }
        )*
    )
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

trait LocalKeyExt<T> {
    fn init<F: FnOnce() -> T>(&'static self, f: F);
    fn destroy(&'static self) -> T;
    fn borrow<F: FnOnce(&T) -> R, R>(&'static self, f: F) -> R;
    fn borrow_mut<F: FnOnce(&mut T) -> R, R>(&'static self, f: F) -> R;
}
impl<T: 'static> LocalKeyExt<T> for std::thread::LocalKey<std::cell::RefCell<Option<T>>> {
    fn init<F: FnOnce() -> T>(&'static self, f: F) {
        self.with(|t| {
            let t = &mut *t.borrow_mut();
            assert!(t.is_none(), "double-init");
            *t = Some(f());
        })
    }
    fn destroy(&'static self) -> T {
        self.with(|t| t.borrow_mut().take().unwrap())
    }
    fn borrow<F: FnOnce(&T) -> R, R>(&'static self, f: F) -> R {
        self.with(|t| f(t.borrow().as_ref().unwrap()))
    }
    fn borrow_mut<F: FnOnce(&mut T) -> R, R>(&'static self, f: F) -> R {
        self.with(|t| f(t.borrow_mut().as_mut().unwrap()))
    }
}
macro_rules! global {
    (static $name:ident: $t:ty) => {
        thread_local!(static $name: ::std::cell::RefCell<Option<$t>> = ::std::cell::RefCell::new(None));
    }
}

macro_rules! check_size {
    ($cover:ident, $t:ident, $sz:expr) => {
        mod $cover {
            unsafe fn _check() {
                const N: usize = $sz;
                ::std::mem::transmute::<[u8; N], super::$t>([0; N]);
            }
        }
    }
}

macro_rules! move_towards {
    ($v:expr, $target:expr, $by:expr) => {{
        let t = $target;
        if t > $v {
            $v = $v.saturating_add($by);
            if t < $v { $v = t }
        } else if t < $v {
            $v = $v.saturating_sub($by);
            if t > $v { $v = t }
        }
    }}
}

fn memset<T: Copy>(dest: &mut [T], val: T, len: usize) {
    for p in dest[..len].iter_mut() {
        *p = val;
    }
}

// imports
pub mod ffi;

// jamultypes.h
pub const FIXSHIFT: c_int = 16;
pub const FIXAMT: c_int = 65536;

// modules
pub mod bullet;
pub mod cheat;
pub mod control;
pub mod cossin;
pub mod display;
pub mod editor;
pub mod filedialog;
pub mod game;
pub mod guy;
pub mod intface;
pub mod items;
pub mod jamulfmv;
pub mod jamulfont;
pub mod jamulsound;
pub mod jamulspr;
pub mod map;
pub mod mapdialog;
pub mod message;
pub mod mgldraw;
pub mod monster;
pub mod music;
pub mod options;
pub mod particle;
pub mod pause;
pub mod player;
pub mod rage;
pub mod sound;
pub mod spcldialog;
pub mod tile;
pub mod tiledialog;
pub mod title;
pub mod world;

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

    let mainmgl = &mut MGLDraw::new(cstr!("Dr. Lunatic"), 640, 480, windowedGame);

    game::LunaticInit(mainmgl);
    title::SplashScreen(mainmgl, cstr!("graphics/hamumu.bmp"), 128, 2);

    loop {
        match title::MainMenu(mainmgl) {
            0 => game::LunaticGame(mainmgl, false), // new game
            1 => game::LunaticGame(mainmgl, true), // continue
            3 => { editor::LunaticEditor(mainmgl); } // editor
            4 | 255 => {
                game::LunaticExit();
                return 0;
            }
            _ => {}
        }
    }
}
