//! here's the fun and easy sound manager
//! it assumes there is a subdirectory "\sounds" that contains snd000.wav - sndXXX.wav,
//! for as many sounds as you'll try to play.  It will load them if they aren't in memory already.
use libc::{c_int, c_long, c_char};
use allegro_sys::*;
use std::ptr;

bitflags! {
    /// external fun sound playing flags for everyone to use
    #[repr(C)]
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
const MAX_FEW_SOUNDS: usize = 2;
const MAX_SOUNDS_AT_ONCE: usize = 16;

bitflags! {
    /// internal sound playing flags
    #[repr(C)]
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

static mut soundbufSize: usize = 0;
static mut soundbuf: *mut soundbuf_t = 0 as *mut soundbuf_t;

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
    flags: SoundFlags,
}

static mut playBuffer: [sound_t; MAX_SOUNDS_AT_ONCE]  = [sound_t {
    voice: 0,
    soundNum: 0,
    priority: 0,
    pan: 0,
    vol: 0,
    flags: SoundFlags { bits: 0 },
}; MAX_SOUNDS_AT_ONCE];

#[no_mangle]
pub unsafe extern fn JamulSoundInit(numBuffers: c_int) -> bool {
    let mut vec = Vec::with_capacity(numBuffers as usize);
    soundbufSize = vec.capacity();
    soundbuf = vec.as_mut_ptr();
    ::std::mem::forget(vec);

    for i in 0..soundbufSize {
        (*soundbuf.offset(i as isize)).sample = ptr::null_mut();
    }
    for playing in playBuffer.iter_mut() {
        playing.soundNum = -1;
        playing.voice = -1;
        playing.flags = SoundFlags::empty();
    }
    true
}

#[no_mangle]
pub unsafe extern fn JamulSoundDestroyBuffer(which: usize) {
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
        Vec::from_raw_parts(soundbuf, 0, soundbufSize);
        soundbuf = ptr::null_mut();
        soundbufSize = 0;
    }
}

/// call this to wipe the sounds from memory
#[no_mangle]
pub unsafe extern fn JamulSoundPurge() {
    for i in 0..soundbufSize {
        JamulSoundDestroyBuffer(i);
    }
}

unsafe fn JamulSoundPlay(voice: c_int, pan: c_long, vol: c_long, playFlags: InternalFlags) -> bool {
	// if this copy is in use, can't play it
	if voice_get_position(voice) > 0 {
		if playFlags.contains(SOUND_CUTOFF) {
			voice_set_position(voice, 0);
			// keep going to handle the rest of the stuff
		} else {
            return false;
        }
	}

	// set the pan and volume and start the voice
	voice_set_volume(voice, vol);
	voice_set_pan(voice, pan);
	voice_start(voice);
    true
}

// now here is all the big sound manager stuff, that allows multiple sounds at once

/// call this fairly often to free up unused buffers, otherwise no new sounds can be played
#[no_mangle]
pub unsafe extern fn JamulSoundUpdate() {
    for buf in playBuffer.iter_mut() {
        if buf.voice != -1
            && buf.flags.contains(SND_PLAYING)
            && voice_get_position(buf.voice) == -1
        {
            buf.flags -= SND_PLAYING;
        }
    }
}

/// call this a lot, it plays sounds
#[no_mangle]
pub unsafe extern fn GoPlaySound(num: c_int, pan: c_int, vol: c_int, flags: u8, mut priority: c_int) {
    let flags = SoundFlags::from_bits_truncate(flags);

	// load the sample if it isn't already
    let sound = &mut *soundbuf.offset(num as isize);
    if sound.sample.is_null() {
        let mut txt = [0; 32];
        sprintf!(txt, "sound\\snd{:03}.wav", num);
        sound.sample = load_sample(txt.as_ptr() as *const c_char);
        if sound.sample.is_null() {
            return; // can't play the sound, it won't load for some reason
        }
    }

    priority += vol; // the quieter a sound, the lower the priority
    if flags.contains(SND_MAXPRIORITY) {
        priority = MAX_SNDPRIORITY;
    }

    if flags.contains(SND_ONE) {
        for buf in playBuffer.iter_mut() {
            if buf.soundNum == num {
                // if you want to cut it off, or it isn't playing, then start anew
                if flags.contains(SND_CUTOFF) || !buf.flags.contains(SND_PLAYING) {
                    buf.pan = pan;
                    buf.vol = vol;
                    buf.flags = flags | SND_PLAYING;
                    buf.priority = priority;
                    JamulSoundPlay(buf.voice, pan, vol, SOUND_CUTOFF);
                    return; // good job
                } else {
                    return; // can't be played because can't cut it off
                }
            }
        }
        // if you fell through to here, it isn't playing, so go ahead as normal
    }

    if flags.contains(SND_FEW) {
        let count = playBuffer.iter()
            .filter(|buf| buf.soundNum == num && buf.flags.contains(SND_PLAYING))
            .count();
        if count >= MAX_FEW_SOUNDS {
            for buf in playBuffer.iter_mut() {
                if buf.soundNum == num
                    && flags.contains(SND_CUTOFF)
                    && buf.flags.contains(SND_PLAYING)
                {
                    buf.pan = pan;
                    buf.vol = vol;
                    buf.flags = flags | SND_PLAYING;
                    buf.priority = priority;
                    JamulSoundPlay(buf.voice, pan, vol, SOUND_CUTOFF);
                    return; // good job
                }
            }
            return; // failed for some reason
        }
    }

    let mut best = usize::max_value();
    for (i, buf) in playBuffer.iter().enumerate() {
        if buf.soundNum == -1 || !buf.flags.contains(SND_PLAYING) {
            best = i;
            break; // can't beat that
        }
        if buf.priority < priority || (buf.soundNum == num && flags.contains(SND_CUTOFF)) {
            if best == usize::max_value() || buf.priority < playBuffer[best].priority {
                best = i;
            }
        }
    }
    if best == usize::max_value() {
        return; // sound is not worthy to be played
    }

    let buf = &mut playBuffer[best];
    if buf.soundNum != num { // if it was already playing that sound, don't waste time
        buf.soundNum = num;
        if buf.voice != -1 {
            deallocate_voice(buf.voice); // slash & burn
        }
        buf.voice = allocate_voice((*soundbuf.offset(num as isize)).sample);
    } else {
        voice_set_position(buf.voice, 0);
    }

    if buf.voice == -1 {
        return; // can't play it
    }
    buf.priority = priority;
    buf.pan = pan;
    buf.vol = vol;
    buf.flags = flags | SND_PLAYING;
    JamulSoundPlay(buf.voice, pan, vol, InternalFlags::empty());
}
