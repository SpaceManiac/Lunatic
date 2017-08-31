//! here's the fun and easy sound manager
//! it assumes there is a subdirectory "\sounds" that contains snd000.wav - sndXXX.wav,
//! for as many sounds as you'll try to play.  It will load them if they aren't in memory already.
use libc::{c_int, c_long, c_char};
use ffi::allegro::*;
use std::ptr;
use LocalKeyExt;

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

impl soundbuf_t {
    fn destroy(&mut self) {
        if !self.sample.is_null() {
            unsafe { destroy_sample(self.sample); }
            self.sample = ptr::null_mut();
        }
    }
}

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

impl sound_t {
    unsafe fn play(&mut self, pan: c_long, vol: c_long, flags: SoundFlags, priority: c_int, internal: InternalFlags) {
        self.pan = pan;
        self.vol = vol;
        self.flags = flags | SND_PLAYING;
        self.priority = priority;
        JamulSoundPlay(self.voice, pan, vol, internal);
    }
}

struct JamulSound {
    soundbuf: Box<[soundbuf_t]>,
    playBuffer: [sound_t; MAX_SOUNDS_AT_ONCE],
}

impl JamulSound {
    fn new(numBuffers: usize) -> JamulSound {
        let sounds = (0..numBuffers).map(|_| soundbuf_t {
            sample: ptr::null_mut()
        }).collect::<Vec<_>>().into_boxed_slice();
        JamulSound {
            soundbuf: sounds,
            playBuffer: [sound_t {
                voice: -1,
                soundNum: -1,
                priority: 0,
                pan: 0,
                vol: 0,
                flags: SoundFlags::empty(),
            }; MAX_SOUNDS_AT_ONCE]
        }
    }

    fn destroy_buffer(&mut self, which: usize) {
        self.soundbuf[which].destroy();
    }

    fn purge(&mut self) {
        for buf in self.soundbuf.iter_mut() {
            buf.destroy();
        }
    }

    fn update(&mut self) {
        for buf in self.playBuffer.iter_mut() {
            if buf.voice != -1
                && buf.flags.contains(SND_PLAYING)
                && unsafe { voice_get_position(buf.voice) } == -1
            {
                buf.flags -= SND_PLAYING;
            }
        }
    }

    unsafe fn play(&mut self, num: c_int, pan: c_int, vol: c_int, flags: SoundFlags, mut priority: c_int) {
        // load the sample if it isn't already
        let sound = &mut self.soundbuf[num as usize];
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
            for buf in self.playBuffer.iter_mut() {
                if buf.soundNum == num {
                    // if you want to cut it off, or it isn't playing, then start anew
                    if flags.contains(SND_CUTOFF) || !buf.flags.contains(SND_PLAYING) {
                        buf.play(pan, vol, flags, priority, SOUND_CUTOFF);
                        return; // good job
                    } else {
                        return; // can't be played because can't cut it off
                    }
                }
            }
            // if you fell through to here, it isn't playing, so go ahead as normal
        }

        if flags.contains(SND_FEW) {
            let count = self.playBuffer.iter()
                .filter(|buf| buf.soundNum == num && buf.flags.contains(SND_PLAYING))
                .count();
            if count >= MAX_FEW_SOUNDS {
                for buf in self.playBuffer.iter_mut() {
                    if buf.soundNum == num
                        && flags.contains(SND_CUTOFF)
                        && buf.flags.contains(SND_PLAYING)
                    {
                        buf.play(pan, vol, flags, priority, SOUND_CUTOFF);
                        return; // good job
                    }
                }
                return; // failed for some reason
            }
        }

        let mut best = usize::max_value();
        for (i, buf) in self.playBuffer.iter().enumerate() {
            if buf.soundNum == -1 || !buf.flags.contains(SND_PLAYING) {
                best = i;
                break; // can't beat that
            }
            if buf.priority < priority || (buf.soundNum == num && flags.contains(SND_CUTOFF)) {
                if best == usize::max_value() || buf.priority < self.playBuffer[best].priority {
                    best = i;
                }
            }
        }
        if best == usize::max_value() {
            return; // sound is not worthy to be played
        }

        let buf = &mut self.playBuffer[best];
        if buf.soundNum != num { // if it was already playing that sound, don't waste time
            buf.soundNum = num;
            if buf.voice != -1 {
                deallocate_voice(buf.voice); // slash & burn
            }
            buf.voice = allocate_voice(sound.sample);
        } else {
            voice_set_position(buf.voice, 0);
        }

        if buf.voice == -1 {
            return; // can't play it
        }
        buf.play(pan, vol, flags, priority, InternalFlags::empty());
    }
}

impl Drop for JamulSound {
    fn drop(&mut self) {
        self.purge();
    }
}

global!(static SOUND: JamulSound);

pub fn JamulSoundInit(numBuffers: c_int) -> bool {
    SOUND.init(|| JamulSound::new(numBuffers as usize));
    true
}

pub fn JamulSoundDestroyBuffer(which: usize) {
    SOUND.borrow_mut(|s| s.destroy_buffer(which));
}

pub extern fn JamulSoundExit() {
    SOUND.destroy();
}

/// call this to wipe the sounds from memory
pub fn JamulSoundPurge() {
    SOUND.borrow_mut(JamulSound::purge);
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
    SOUND.borrow_mut(JamulSound::update);
}

/// call this a lot, it plays sounds
pub unsafe fn GoPlaySound(num: c_int, pan: c_int, vol: c_int, flags: SoundFlags, priority: c_int) {
    SOUND.borrow_mut(|s| s.play(num, pan, vol, flags, priority));
}
