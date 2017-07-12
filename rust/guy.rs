use libc::c_int;

#[repr(C)]
pub enum Action {
    ACTION_IDLE = 0,
    ACTION_BUSY
}

#[repr(C)]
pub struct Guy {
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
    pub oldx: c_int,
    pub oldy: c_int,
    pub dx: c_int,
    pub dy: c_int,
    pub dz: c_int,
    pub mapx: u8,
    pub mapy: u8,
    pub facing: u8,

    /// brain variables for AI
    pub mind: u8,
    pub mind1: u8,
    pub mind2: u8,
    pub mind3: u8,

    pub reload: u8,
    pub poison: u8,

    pub ouch: u8,
    pub action: u8,
    pub frmTimer: u16,
    pub frmAdvance: u16,
    pub frm: u8,
    pub seq: u8,
    pub bright: i8,
    pub friendly: u8,

    pub mindControl: u16,
    pub target: *mut Guy,
    pub parent: *mut Guy,
    pub hp: c_int,
    pub type_: u8,
    /// for collision checks
    pub rectx: c_int,
    pub recty: c_int,
    pub rectx2: c_int,
    pub recty2: c_int,
    /// just a copy of the guy's number
    pub ID: u16,
}

impl Guy {
    pub fn new() -> Guy {
        unsafe { ::std::mem::zeroed() }
    }
}

extern {
    pub static mut goodguy: *mut Guy;
}
