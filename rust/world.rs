use libc::{c_int, c_char};
use mgldraw::MGLDraw;
use map::Map;

pub const MAX_MAPS: usize = 24;

/// terrain flags
pub mod Terrain {
    pub const TF_SOLID: u16 = 1;
    pub const TF_ICE: u16 = 2;
    pub const TF_MUD: u16 = 4;
    pub const TF_WATER: u16 = 8;
    pub const TF_LAVA: u16 = 16;
    /// if this is the roof of a wall, the wall is pushable
    pub const TF_PUSHY: u16 = 32;
    /// only PUSHON terrain can have things pushed over it
    pub const TF_PUSHON: u16 = 64;
    pub const TF_ANIM: u16 = 128;
    pub const TF_STEP: u16 = 256;
    pub const TF_DESTRUCT: u16 = 512;
    pub const TF_TRANS: u16 = 1024;
    pub const TF_MINECART: u16 = 2048;
    pub const TF_BUNNY: u16 = 4096;
}

#[repr(C)]
pub struct terrain_t {
    pub flags: u16,
    pub next: u8,
}

#[repr(C)]
pub struct world_t {
    numMaps: u8,
    totalPoints: c_int,
    map: [*mut Map; MAX_MAPS],
    terrain: [terrain_t; 200],
}

extern {
    pub fn WorldLoadBMP(name: *mut c_char, dst: *mut u8);

    pub fn NewWorld(world: *mut world_t, mgl: *mut MGLDraw) -> u8;
    pub fn LoadWorld(world: *mut world_t, fname: *const c_char) -> u8;
    pub fn SaveWorld(world: *mut world_t, fname: *const c_char) -> u8;
    pub fn FreeWorld(world: *mut world_t);

    pub fn InitWorld(world: *mut world_t, worldNum: u8);
    pub fn GetWorldName(fname: *mut c_char, buf: *mut c_char);
    pub fn GetWorldPoints(fname: *const c_char) -> c_int;
}
