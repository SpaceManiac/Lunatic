use libc::c_int;

cpp! {{
    #include "options.h"
}}

extern {
    fn GoPlaySound(snd: c_int, pan: c_int, vol: c_int, flags: u8, priority: c_int);
    fn GetCamera(cx: *mut c_int, cy: *mut c_int);
}

static mut SOUND_AVAILABLE: bool = false;

#[no_mangle]
pub unsafe extern fn SoundSystemExists() {
    SOUND_AVAILABLE = true;
}

#[no_mangle]
pub unsafe extern fn InitSound() {
    cpp!([] { JamulSoundPurge(); });
}

#[no_mangle]
pub unsafe extern fn ExitSound() {
    // don't need to do nothing, it's handled by jamulsoundexit
}

#[no_mangle]
pub unsafe extern fn MakeSound(snd: c_int, mut x: c_int, mut y: c_int, flags: c_int, priority: c_int) {
    if !SOUND_AVAILABLE { return }
    if !cpp!([] -> bool as "bool" { return opt.sound; }) { return }

    x >>= ::FIXSHIFT;
    y >>= ::FIXSHIFT;

    let (mut cx, mut cy) = (0, 0);
    GetCamera(&mut cx, &mut cy);

    let pan = 127 + (x - cx) * 127 / 800; // (x-cx)*2 in range -1600 to 1600, this is 0-255
    let vol = -((x - cx)*(x - cx)+(y - cy)*(y - cy)) / 128;
    if vol < -5000 { return } // too quiet to play
    let vol = vol * 255 / 5000 + 255;
    GoPlaySound(snd, pan, vol, flags as u8, priority);
}

#[no_mangle]
pub unsafe extern fn MakeNormalSound(snd: c_int) {
    if !SOUND_AVAILABLE { return }
    if !cpp!([] -> bool as "bool" { return opt.sound; /**/ }) { return }

    GoPlaySound(snd, 128, 255, ::SND_MAXPRIORITY | ::SND_CUTOFF | ::SND_ONE, ::MAX_SNDPRIORITY);
}
