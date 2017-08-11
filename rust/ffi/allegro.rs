use libc::{c_int, c_char};
use ffi::win::HWND;

opaque!(BITMAP);
opaque!(SAMPLE);

pub const KEY_MAX: usize = 127;

pub const DIGI_AUTODETECT: c_int = -1;
pub const MIDI_AUTODETECT: c_int = -1;
pub const GFX_AUTODETECT_FULLSCREEN: c_int = 1;
pub const GFX_AUTODETECT_WINDOWED: c_int = 2;
pub const SWITCH_BACKGROUND: c_int = 3;
pub const SWITCH_IN: c_int = 0;
pub const SWITCH_OUT: c_int = 1;

extern {
    pub static mut allegro_error: [c_char; 256];
    #[link_name="screen"]
    pub static mut al_screen: *mut BITMAP;
    #[link_name="key"]
    pub static mut al_key: [c_char; KEY_MAX];
    #[link_name="mouse_x"]
    pub static mut al_mouse_x: c_int;
    #[link_name="mouse_y"]
    pub static mut al_mouse_y: c_int;
    #[link_name="mouse_b"]
    pub static mut al_mouse_b: c_int;

    pub fn install_keyboard() -> c_int;
    pub fn install_mouse() -> c_int;
    pub fn install_sound(digi: c_int, midi: c_int, cfg_path: *const c_char) -> c_int;
    pub fn set_color_depth(depth: c_int);

    pub fn set_gfx_mode(card: c_int, w: c_int, h: c_int, v_w: c_int, v_h: c_int) -> c_int;
    pub fn set_window_title(title: *const c_char);
    pub fn set_close_button_callback(proc_: unsafe extern fn());
    pub fn set_display_switch_mode(mode: c_int) -> c_int;
    pub fn set_display_switch_callback(dir: c_int, proc_: unsafe extern fn()) -> c_int;
    pub fn win_get_window() -> HWND;

    pub fn create_bitmap(width: c_int, height: c_int) -> *mut BITMAP;
    pub fn destroy_bitmap(bitmap: *mut BITMAP);
    pub fn blit(source: *mut BITMAP, dest: *mut BITMAP, sx: c_int, sy: c_int, dx: c_int, dy: c_int, w: c_int, h: c_int);
    pub fn makecol(r: c_int, g: c_int, b: c_int) -> c_int;
    pub fn putpixel(bmp: *mut BITMAP, x: c_int, y: c_int, color: c_int);
    pub fn _putpixel32(bmp: *mut BITMAP, x: c_int, y: c_int, color: c_int);

    pub fn keypressed() -> c_int;
    pub fn readkey() -> c_int;

    pub fn load_sample(filename: *const c_char) -> *mut SAMPLE;
    pub fn destroy_sample(spl: *mut SAMPLE);

    pub fn allocate_voice(spl: *const SAMPLE) -> c_int;
    pub fn deallocate_voice(voice: c_int);
    pub fn voice_start(voice: c_int);
    pub fn voice_get_position(voice: c_int) -> c_int;
    pub fn voice_set_position(voice: c_int, position: c_int);
    pub fn voice_set_volume(voice: c_int, volume: c_int);
    pub fn voice_set_pan(voice: c_int, pan: c_int);

    fn _install_allegro_version_check(system_id: c_int, errno_ptr: *mut c_int,
        atexit_ptr: unsafe extern fn(_: extern fn()) -> c_int, version: c_int) -> c_int;
}

#[cfg(windows)]
extern {
    fn _errno() -> *mut c_int;
}
#[cfg(not(windows))]
unsafe fn _errno() -> *mut c_int {
    ::libc::__errno_location()
}

pub unsafe fn allegro_init() -> c_int {
    _install_allegro_version_check(0, _errno(), ::libc::atexit,
        (4 << 16) | (4 << 8) | 1) // v 4.4.1
}
