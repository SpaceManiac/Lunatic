use libc::*;

pub type __time64_t = i64;
pub type _fsize_t = c_ulong;

// _finddata64i32_t
#[repr(C)]
pub struct _finddata_t {
    pub attrib: c_uint,
    pub time_create: __time64_t,
    pub time_access: __time64_t,
    pub time_write: __time64_t,
    pub size: _fsize_t,
    pub name: [c_char; 260],
}

extern {
    #[link_name="_findfirst64i32"]
    pub fn _findfirst(filespec: *const c_char, fileinfo: *mut _finddata_t) -> intptr_t;
    #[link_name="_findnext64i32"]
    pub fn _findnext(handle: intptr_t, fileinfo: *mut _finddata_t) -> c_int;
    #[link_name="_findclose"]
    pub fn _findclose(handle: intptr_t) -> c_int;
}

pub type MMRESULT = c_uint;
pub const JOYSTICKID1: c_uint = 0;
pub const JOYERR_NOERROR: MMRESULT = 0;
pub const JOY_RETURNX: c_ulong = 0x1;
pub const JOY_RETURNY: c_ulong = 0x2;
pub const JOY_RETURNBUTTONS: c_ulong = 0x80;

#[repr(C)]
pub struct JOYCAPSA {
    pub wMid: c_ushort,
    pub wPid: c_ushort,
    pub szPname: [c_char; 32], // MAXPNAMELEN
    pub wXmin: c_uint,
    pub wXmax: c_uint,
    pub wYmin: c_uint,
    pub wYmax: c_uint,
    pub wZmin: c_uint,
    pub wZmax: c_uint,
    pub wNumButtons: c_uint,
    pub wPeriodMin: c_uint,
    pub wPeriodMax: c_uint,
    pub wRmin: c_uint,
    pub wRmax: c_uint,
    pub wUmin: c_uint,
    pub wUmax: c_uint,
    pub wVmin: c_uint,
    pub wVmax: c_uint,
    pub wCaps: c_uint,
    pub wMaxAxes: c_uint,
    pub wNumAxes: c_uint,
    pub wMaxButtons: c_uint,
    pub szRegKey: [c_char; 32], // MAXPNAMELEN
    pub szOEMVxD: [c_char; 260], // MAX_JOYSTICKOEMVXDNAME
}

#[repr(C)]
pub struct JOYINFOEX {
    pub dwSize: c_ulong,
    pub dwFlags: c_ulong,
    pub dwXpos: c_ulong,
    pub dwYpos: c_ulong,
    pub dwZpos: c_ulong,
    pub dwRpos: c_ulong,
    pub dwUpos: c_ulong,
    pub dwVpos: c_ulong,
    pub dwButtons: c_ulong,
    pub dwButtonNumber: c_ulong,
    pub dwPOV: c_ulong,
    pub dwReserved1: c_ulong,
    pub dwReserved2: c_ulong,
}

extern "system" {
    pub fn joyGetDevCapsA(uJoyID: c_uint, pjc: *mut JOYCAPSA, cbjc: c_uint) -> MMRESULT;
    pub fn joyGetPosEx(uJoyId: c_uint, pji: *mut JOYINFOEX) -> MMRESULT;
}

pub use self::joyGetDevCapsA as joyGetDevCaps;
pub use self::JOYCAPSA as JOYCAPS;
