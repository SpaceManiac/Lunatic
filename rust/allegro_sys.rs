use libc::{c_int, c_char};

opaque!(SAMPLE);

extern {
    pub fn load_sample(filename: *const c_char) -> *mut SAMPLE;
    pub fn destroy_sample(spl: *mut SAMPLE);

    pub fn allocate_voice(spl: *const SAMPLE) -> c_int;
    pub fn deallocate_voice(voice: c_int);
    pub fn voice_start(voice: c_int);
    pub fn voice_get_position(voice: c_int) -> c_int;
    pub fn voice_set_position(voice: c_int, position: c_int);
    pub fn voice_set_volume(voice: c_int, volume: c_int);
    pub fn voice_set_pan(voice: c_int, pan: c_int);
}
