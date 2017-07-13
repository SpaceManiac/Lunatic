use libc::c_char;
use mgldraw::MGLDraw;

/// the different plop modes
#[repr(C)]
pub enum PlopMode {
    PLOP_FLOOR = 0,
    PLOP_WALL,
    PLOP_ITEM,
    PLOP_BADGUY,
    PLOP_LIGHT,
    PLOP_DARK,
    PLOP_SMOOTH,
    PLOP_TORCH,
    PLOP_SPECIAL,
    MAXPLOP,
}

#[repr(C)]
pub struct editopt_t {
    pub displayFlags: u8,
    pub plopMode: u8,
    pub brushSize: u8,
    pub curWall: u8,
    pub curWallFloor: u8,
    pub curFloor: u8,
    pub curItem: u8,
    pub curBadguy: u8,
}

extern {
    pub static mut editing: u8;

    pub fn LunaticEditor(mgl: *mut MGLDraw) -> u8;

    pub fn EditorNewWorld();
    pub fn EditorLoadWorld(fname: *const c_char);
    pub fn EditorSaveWorld(fname: *const c_char);
    pub fn EditorSelectMap(w: u8);
    pub fn EditorLoadTiles(fname: *const c_char);
}
