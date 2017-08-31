use std::ptr;
use libc::c_int;
use ffi::logg::*;

/// these are CD audio modes for the CDPlayerUpdate
#[repr(C)]
#[derive(FromInt, PartialEq)]
pub enum AudioMode {
    Off = 0,
    /// continuously loop the current track
    LoopTrack,
    /// plays the chosen track, then loops the next one
    IntroLoop,
    /// after current track, jump to any other at random
    Random,
    /// just keep playing the tracks in order, loops at end of CD to beginning
    Normal,
}

static mut currentMode: AudioMode = AudioMode::Off;
static mut stream: *mut LOGG_Stream = 0 as *mut LOGG_Stream;
static mut isPlaying: bool = false;
static mut trackNum: c_int = 0;

pub unsafe fn MusicInit() -> u8 {
    currentMode = AudioMode::Off;
    stream = ptr::null_mut();
    1
}

pub unsafe fn MusicExit() {
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

pub unsafe fn CDPlayerUpdate(mode: AudioMode) {
    isPlaying = false;
    if !stream.is_null() {
        isPlaying = logg_update_stream(stream) != 0;
    }

    let modeChanged = currentMode != mode;
    currentMode = mode;

    if !isPlaying || modeChanged {
        use self::AudioMode::*;
        match currentMode {
            LoopTrack => { CDPlay(trackNum); }
            IntroLoop => {
                CDPlay(trackNum + 1);
                currentMode = AudioMode::LoopTrack;
            }
            Random => {
                CDPlay(3 + ::mgldraw::MGL_random(15));
            }
            Normal => {
                if !isPlaying {
                    let mut newTrack = trackNum + 1;
                    if newTrack > 18 { newTrack = 3; }
                    CDPlay(newTrack);
                }
            }
            Off => if isPlaying { CDStop() }
        }
    }
}

pub unsafe fn CDNeedsUpdating() {}

#[no_mangle]
pub unsafe extern fn CDStop() {
    if !stream.is_null() {
        logg_destroy_stream(stream);
        stream = ptr::null_mut();
        trackNum = 0;
    }
}

pub unsafe fn CDLoaded() -> u8 {
    1
}
