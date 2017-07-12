use libc::{c_char, c_int};

pub const OGG_PAGES_TO_BUFFER: usize = 2;

opaque!(LOGG_Stream);
opaque!(SAMPLE);

extern {
    pub fn logg_load(filename: *const c_char) -> *mut SAMPLE;
    pub fn logg_get_buffer_size() -> c_int;
    pub fn logg_set_buffer_size(size: c_int);
    pub fn logg_get_stream(filename: *const c_char, volume: c_int, pan: c_int, loop_: c_int) -> *mut LOGG_Stream;
    pub fn logg_update_stream(s: *mut LOGG_Stream) -> c_int;
    pub fn logg_destroy_stream(s: *mut LOGG_Stream);
    pub fn logg_stop_stream(s: *mut LOGG_Stream);
    pub fn logg_restart_stream(s: *mut LOGG_Stream);
}
