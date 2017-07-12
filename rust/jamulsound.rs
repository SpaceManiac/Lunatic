//! here's the fun and easy sound manager
//! it assumes there is a subdirectory "\sounds" that contains snd000.wav - sndXXX.wav,
//! for as many sounds as you'll try to play.  It will load them if they aren't in memory already.
use libc::{c_int, c_long};
use allegro_sys::*;
use std::ptr;

bitflags! {
    /// external fun sound playing flags for everyone to use
    pub struct SoundFlags: u8 {
        /// cut off same sound if needed
        const SND_CUTOFF = 1;
        /// ignore priority value, this sound is a must-have
        const SND_MAXPRIORITY = 2;
        /// only one copy may play at once
        const SND_ONE = 4;
        /// well, it's not for everyone, but it goes here
        const SND_PLAYING = 8;
        /// only allow MAX_FEW_SOUNDS copies to play at once
        const SND_FEW = 16;
    }
}

pub const MAX_SNDPRIORITY: c_int = 65536;

/// most copies of a SND_FEW sound that can play at once
const MAX_FEW_SOUNDS: c_int = 2;
const MAX_SOUNDS_AT_ONCE: usize = 16;

bitflags! {
    /// internal sound playing flags
    struct InternalFlags: u8 {
        /// loop the sound indefinitely (actually does nothing)
        const SOUND_LOOP = 1;
        /// if the copy of the sound is busy, cut it off and restart
        const SOUND_CUTOFF = 2;
    }
}

/// a loaded sound buffer
#[repr(C)]
struct soundbuf_t {
    sample: *mut SAMPLE,
}

/*static mut soundbufSize: c_int = 0;
static mut soundbuf: *mut soundbuf_t = 0 as *mut soundbuf_t;*/

/// a sound currently playing
#[derive(Copy, Clone)]
#[repr(C)]
struct sound_t {
    /// allegro handle
    voice: c_int,
    /// which game sound number
    soundNum: c_int,
    priority: c_int,
    pan: c_long,
    vol: c_long,
    flags: u8,
}

/*static mut playBuffer: [sound_t; MAX_SOUNDS_AT_ONCE]  = [sound_t {
    voice: 0,
    soundNum: 0,
    priority: 0,
    pan: 0,
    vol: 0,
    flags: 0,
}; MAX_SOUNDS_AT_ONCE];*/

extern {
    static mut soundbufSize: c_int;
    static mut soundbuf: *mut soundbuf_t;
    static mut playBuffer: [sound_t; MAX_SOUNDS_AT_ONCE];

    /// call this fairly often to free up unused buffers, otherwise no new sounds can be played
    pub fn JamulSoundUpdate();
    /// call this a lot, it plays sounds
    pub fn GoPlaySound(snd: c_int, pan: c_int, vol: c_int, flags: u8, priority: c_int);
}

cpp! {{
    #include <allegro.h>
    struct soundbuf_t { SAMPLE *sample; };
}}

#[no_mangle]
pub unsafe extern fn JamulSoundInit(numBuffers: c_int) -> bool {
    soundbufSize = numBuffers;
    soundbuf = cpp!([numBuffers as "int"] -> *mut soundbuf_t as "soundbuf_t*" {
        return new soundbuf_t[numBuffers];
    });
    for i in 0..numBuffers {
        (*soundbuf.offset(i as isize)).sample = ptr::null_mut();
    }
    for playing in playBuffer.iter_mut() {
        playing.soundNum = -1;
        playing.voice = -1;
        playing.flags = 0;
    }
    true
}

#[no_mangle]
pub unsafe extern fn JamulSoundDestroyBuffer(which: c_int) {
    let buf = &mut *soundbuf.offset(which as isize);
    if !buf.sample.is_null() {
        destroy_sample(buf.sample);
        buf.sample = ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern fn JamulSoundExit() {
    if !soundbuf.is_null() {
        JamulSoundPurge();
        cpp!([soundbuf as "soundbuf_t*"] {
            delete[] soundbuf;
        });
        soundbuf = ptr::null_mut();
    }
}

/// call this to wipe the sounds from memory
#[no_mangle]
pub unsafe extern fn JamulSoundPurge() {
    for i in 0..soundbufSize {
        JamulSoundDestroyBuffer(i);
    }
}
