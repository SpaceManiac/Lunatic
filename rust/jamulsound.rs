use libc::c_int;

bitflags! {
    pub struct SoundFlags: u8 {
        /// external fun sound playing flags for everyone to use
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

extern {
    pub fn GoPlaySound(snd: c_int, pan: c_int, vol: c_int, flags: u8, priority: c_int);
}
