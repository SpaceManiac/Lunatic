use libc::{c_char, c_int, c_uint};
use misc_sys::*;
use std::mem;

bitflags! {
    /// the various control flags
    #[repr(C)]
    pub struct Controls: u8 {
        const EMPTY = 0;
        const CONTROL_UP = 1;
        const CONTROL_DN = 2;
        const CONTROL_LF = 4;
        const CONTROL_RT = 8;
        const CONTROL_B1 = 16;
        const CONTROL_B2 = 32;
        const CONTROL_B3 = 64;
        const CONTROL_B4 = 128;
    }
}

/// source of control
#[repr(C)]
pub enum ControlSource {
    CONTROL_KB1 = 0,
    CONTROL_KB2,
    CONTROL_JOY
}

static mut keyState: Controls = EMPTY;
static mut keyTap: Controls = EMPTY;
static mut arrowState: Controls = EMPTY;
static mut arrowTap: Controls = EMPTY;

static mut joyMinX: c_uint = 2000000000; // why?
static mut joyMinY: c_uint = 2000000000;
static mut joyMaxX: c_uint = 0;
static mut joyMaxY: c_uint = 0;
static mut joyCX: c_uint = 0;
static mut joyCY: c_uint = 0;
static mut joyDeadX: c_int = 0;
static mut joyDeadY: c_int = 0;

static mut joystickOn: u8 = 1;
static mut oldJoy: Controls = EMPTY;

static mut lastScanCode: u8 = 0;

static mut kb: [[u8; 4]; 8] = [[0; 4]; 8];
static mut joyBtn: [u32; 2] = [0; 2];

#[no_mangle]
pub unsafe extern fn InitControls() {
    lastScanCode = 0;
    keyState = EMPTY;
    keyTap = EMPTY;
    arrowState = EMPTY;
    arrowTap = EMPTY;

    if joystickOn != 0 {
        let mut joyCaps = mem::zeroed();
        let result = joyGetDevCaps(JOYSTICKID1, &mut joyCaps, szof!(JOYCAPS) as u32);
        if result != JOYERR_NOERROR {
            joystickOn = 0;
            return;
        }

        joyCX = (joyCaps.wXmax - joyCaps.wXmin) / 2 + joyCaps.wXmin;
        joyCY = (joyCaps.wYmax - joyCaps.wYmin) / 2 + joyCaps.wXmin;
        joyMinX = joyCaps.wXmin;
        joyMinY = joyCaps.wYmin;
        joyMaxX = joyCaps.wYmax;
        joyMaxY = joyCaps.wYmax;
        oldJoy = EMPTY;
    }
}

#[no_mangle]
pub unsafe extern fn ControlKeyDown(k: u8) {
    lastScanCode = k;

    for i in 0..4 {
        let mut bit = 1;
        for j in 0..8 {
            if k == kb[j][i] {
                keyState |= Controls::from_bits_truncate(bit);
                keyTap |= Controls::from_bits_truncate(bit);
            }
            bit <<= 1;
        }
    }
    // always track arrows, no matter what the keys are, for menus
    if k == 84 {
        arrowState |= CONTROL_UP;
        arrowTap |= CONTROL_UP;
    }
    if k == 85 {
        arrowState |= CONTROL_DN;
        arrowTap |= CONTROL_DN;
    }
    if k == 82 {
        arrowState |= CONTROL_LF;
        arrowTap |= CONTROL_LF;
    }
    if k == 83 {
        arrowState |= CONTROL_RT;
        arrowTap |= CONTROL_RT;
    }
    if k == 67 {
        arrowState |= CONTROL_B1;
        arrowTap |= CONTROL_B1;
    }
}

#[no_mangle]
pub unsafe extern fn ControlKeyUp(k: u8) {
    for i in 0..4 {
        let mut bit = 1;
        for j in 0..8 {
            if k == kb[j][i] {
                keyState &= !Controls::from_bits_truncate(bit);
            }
            bit <<= 1;
        }
    }
	// always track arrows, no matter what the keys are, for menus
    arrowState &= !match k {
        84 => CONTROL_UP,
        85 => CONTROL_DN,
        82 => CONTROL_LF,
        83 => CONTROL_RT,
        67 => CONTROL_B1,
        _ => EMPTY,
    };
}

#[no_mangle]
pub unsafe extern fn GetJoyState() -> Controls {
    let mut joyInfo: JOYINFOEX = mem::zeroed();
    joyInfo.dwSize = szof!(JOYINFOEX) as u32;
    joyInfo.dwFlags = JOY_RETURNBUTTONS | JOY_RETURNX | JOY_RETURNY;
    let result = joyGetPosEx(JOYSTICKID1, &mut joyInfo);
    if result != JOYERR_NOERROR {
        return EMPTY;
    }
    let joyX = joyInfo.dwXpos as c_uint;
    let joyY = joyInfo.dwYpos as c_uint;
    joyMinX = ::std::cmp::min(joyMinX, joyX);
    joyMaxX = ::std::cmp::max(joyMaxX, joyX);
    joyMinY = ::std::cmp::min(joyMinY, joyY);
    joyMaxY = ::std::cmp::max(joyMaxY, joyY);
    joyDeadX = (joyMaxX - joyMinX) as i32 / 8;
    joyDeadY = (joyMaxY - joyMinY) as i32 / 8;
    let mut joyState = EMPTY;
    if (joyX as i32 - joyCX as i32) < -joyDeadX {
        joyState |= CONTROL_LF;
    }
    if (joyX as i32 - joyCX as i32) > joyDeadX {
        joyState |= CONTROL_RT;
    }
    if (joyY as i32 - joyCY as i32) < -joyDeadY {
        joyState |= CONTROL_UP;
    }
    if (joyY as i32 - joyCY as i32) > joyDeadY {
        joyState |= CONTROL_DN;
    }
    if (joyInfo.dwButtons & joyBtn[0]) != 0 {
        if !oldJoy.contains(CONTROL_B1) {
            keyTap |= CONTROL_B1;
        }
        joyState |= CONTROL_B1;
    }
    if (joyInfo.dwButtons & joyBtn[1]) != 0 {
        if !oldJoy.contains(CONTROL_B2) {
            keyTap |= CONTROL_B2;
        }
        joyState |= CONTROL_B2;
    }
    oldJoy = joyState;
    joyState
}

#[no_mangle]
pub unsafe extern fn GetJoyButtons() -> u32 {
    if joystickOn == 0 {
        return 0;
    }

    let mut joyInfo: JOYINFOEX = mem::zeroed();
    joyInfo.dwSize = szof!(JOYINFOEX) as u32;
    joyInfo.dwFlags = JOY_RETURNBUTTONS | JOY_RETURNX | JOY_RETURNY;
    if joyGetPosEx(JOYSTICKID1, &mut joyInfo) == JOYERR_NOERROR {
        joyInfo.dwButtons
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern fn GetControls() -> u8 {
    (keyState | if joystickOn != 0 { GetJoyState() } else { EMPTY }).bits()
}

#[no_mangle]
pub unsafe extern fn GetTaps() -> u8 {
    if joystickOn != 0 {
        GetJoyState();
    }
    let tapState = keyTap | arrowTap;
    keyTap = EMPTY;
    arrowTap = EMPTY;
    tapState.bits()
}

#[no_mangle]
pub unsafe extern fn GetArrows() -> u8 {
    arrowState.bits()
}

#[no_mangle]
pub unsafe extern fn LastScanCode() -> u8 {
    mem::replace(&mut lastScanCode, 0)
}

#[no_mangle]
pub unsafe extern fn JoystickAvailable() -> u8 {
    joystickOn
}

#[no_mangle]
pub unsafe extern fn ApplyControlSettings() {
    use options::opt;

    for i in 0..6 {
        kb[i][0] = opt.control[0][i];
        kb[i][1] = opt.control[1][i];
        kb[i][2] = 0;
        kb[i][3] = 0;
    }
    joyBtn[0] = 1 << opt.joyCtrl[0];
    joyBtn[1] = 1 << opt.joyCtrl[1];
}

#[no_mangle]
pub unsafe extern fn SetKeys(keys: [u8; 8]) {
    // memcpy(kb, keys, 8
    kb[0][0] = keys[0];
    kb[0][1] = keys[1];
    kb[0][2] = keys[2];
    kb[0][3] = keys[3];
    kb[1][0] = keys[4];
    kb[2][0] = keys[5];
    kb[3][0] = keys[6];
    kb[4][0] = keys[7];
}

#[no_mangle]
pub extern fn ScanCodeText(s: u8) -> *const c_char {
    macro_rules! table {
        ($($t:expr,)*) => (&[$(cstr!($t),)*])
    }
    // text strings corresponding to scan codes 0-88
    let scancodes = table! {
        // 0
        "Null", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
        // 16
        "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1", "2", "3", "4",
        // 32
        "5", "6", "7", "8", "9", "Numpad0", "Numpad1", "Numpad2", "Numpad3", "Numpad4", "Numpad5", "Numpad6", "Numpad7", "Numpad8", "Numpad9", "F1",
        // 48
        "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", "Escape", "~", "-", "=", "Backspace",
        // 64
        "Tab", "[", "]", "Enter", ":", "\"", "\\", "\\ 2", ",", "Stop", "/", "Space", "Insert", "Delete", "Home", "End",
        // 80
        "PageUp", "PageDown", "Left", "Right", "Up", "Down", "Numpad/", "Numpad*", "Numpad-", "Numpad+", "NumpadDel", "NumpadEnter", "PrintScreen", "Pause", "ABNT-C1", "Yen",
        // 96
        "Kana", "Convert", "NoConvert", "@", "Circumflex", ": 2", "Kanji", "Numpad=", "`", ";", "Command", "Unknown1", "Unknown2", "Unknown3", "Unknown4", "Unknown5",
        // 112
        "Unknown6", "Unknown7", "Unknown8", "LShift", "RShift", "LControl", "RControl", "Alt", "AltGr", "LWin", "RWin", "Menu", "ScrollLock", "NumLock", "CapsLock", "Maximum",
    };
    scancodes.get(s as usize).cloned().unwrap_or(cstr!("Unknown"))
}
