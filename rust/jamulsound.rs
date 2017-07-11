use libc::c_int;

// external fun sound playing flags for everyone to use

/// cut off same sound if needed
pub const SND_CUTOFF: u8 = 1;
/// ignore priority value, this sound is a must-have
pub const SND_MAXPRIORITY: u8 = 2;
/// only one copy may play at once
pub const SND_ONE: u8 = 4;
/// well, it's not for everyone, but it goes here
pub const SND_PLAYING: u8 = 8;
/// only allow MAX_FEW_SOUNDS copies to play at once
pub const SND_FEW: u8 = 16;

pub const MAX_SNDPRIORITY: c_int = 65536;

extern {
    pub fn GoPlaySound(snd: c_int, pan: c_int, vol: c_int, flags: u8, priority: c_int);
}
