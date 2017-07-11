use std::ptr;
use libc::c_int;
use logg_sys::*;

/// these are CD audio modes for the CDPlayerUpdate
#[repr(C)]
#[derive(FromInt)]
pub enum AudioMode {
    CD_OFF = 0,
    /// continuously loop the current track
    CD_LOOPTRACK,
    /// plays the chosen track, then loops the next one
    CD_INTROLOOP,
    /// after current track, jump to any other at random
    CD_RANDOM,
    /// just keep playing the tracks in order, loops at end of CD to beginning
    CD_NORMAL,
}

static mut currentMode: u8 = 0;
static mut stream: *mut LOGG_Stream = 0 as *mut LOGG_Stream;
static mut isPlaying: bool = false;
static mut trackNum: c_int = 0;

#[no_mangle]
pub unsafe extern fn MusicInit() -> u8 {
    currentMode = AudioMode::CD_OFF as u8;
    stream = ptr::null_mut();
    1
}

#[no_mangle]
pub unsafe extern fn MusicExit() {
    CDStop();
}

#[no_mangle]
pub unsafe extern fn CDPlay(track: c_int) {
    if trackNum == track && !stream.is_null() && isPlaying {
        return; // Already playing that track
    }

    let mut buf = [0u8; 32];
    sprintf!(buf, "sound/mus{:03}.ogg", track);

    trackNum = track;
    if !stream.is_null() { logg_destroy_stream(stream); }
    stream = logg_get_stream(buf.as_ptr() as *const _, 128, 128, 0);
}

#[no_mangle]
pub unsafe extern fn CDPlayerUpdate(mode: u8) {
    isPlaying = false;
    if !stream.is_null() {
        isPlaying = logg_update_stream(stream) != 0;
    }

    let modeChanged = currentMode != mode;
    currentMode = mode;

    if !isPlaying || modeChanged {
        use self::AudioMode::*;
        match AudioMode::from_int(currentMode as usize) {
            Some(CD_LOOPTRACK) => { CDPlay(trackNum); }
            Some(CD_INTROLOOP) => {
                CDPlay(trackNum + 1);
                currentMode = AudioMode::CD_LOOPTRACK as u8;
            }
            Some(CD_RANDOM) => {
                CDPlay(3 + ::mgldraw::MGL_random(15));
            }
            Some(CD_NORMAL) => {
                if !isPlaying {
                    let mut newTrack = trackNum + 1;
                    if newTrack > 18 { newTrack = 3; }
                    CDPlay(newTrack);
                }
            }
            Some(CD_OFF) | None => if isPlaying { CDStop() }
        }
    }
}

#[no_mangle]
pub unsafe extern fn CDNeedsUpdating() {}

#[no_mangle]
pub unsafe extern fn CDStop() {
    if !stream.is_null() {
        logg_destroy_stream(stream);
        stream = ptr::null_mut();
        trackNum = 0;
    }
}

#[no_mangle]
pub unsafe extern fn CDLoaded() -> u8 {
    1
}
