use libc::{c_int, c_char};

pub const MAX_LIGHT: i8 = 16;
pub const MIN_LIGHT: i8 = -32;

/// the maximum # of monsters you can put on the map
pub const MAX_MAPMONS: usize = 128;
pub const MAX_SPECIAL: usize = 32;

/// flags for calling map render
pub mod RenderFlags {
    pub const MAP_SHOWLIGHTS: u8 = 1;
    pub const MAP_SHOWWALLS: u8 = 2;
    pub const MAP_SHOWITEMS: u8 = 4;
    pub const MAP_SHOWBADGUYS: u8 = 8;
    pub const MAP_SHOWSPECIALS: u8 = 16;
}

/// special trigger flags
pub mod Trigger {
    pub const TRG_STEP: u16 = 1;
    pub const TRG_ENEMYSTEP: u16 = 2;
    pub const TRG_NEAR: u16 = 4;
    pub const TRG_PASSLEVELS: u16 = 8;
    pub const TRG_KEYCHAINS: u16 = 16;
    pub const TRG_KILLALL: u16 = 32;
    pub const TRG_HAVEBRAINS: u16 = 64;
    pub const TRG_SHOOT: u16 = 128;
    /// works more than once
    pub const TRG_REPEATABLE: u16 = 256;
    /// displays a message
    pub const TRG_MESSAGE: u16 = 512;
    /// goes off if any special in an adjacent square goes off
    pub const TRG_CHAIN: u16 = 1024;
    /// occurs every N seconds
    pub const TRG_TIMED: u16 = 2048;
    /// occurs randomly, with an N% chance each second
    pub const TRG_RANDOM: u16 = 4096;
    /// has loony key for this world
    pub const TRG_HAVELOONY: u16 = 8192;
    /// kill just one of the chosen monster type
    pub const TRG_KILLONE: u16 = 16384;
    /// the floor/roof here is N
    pub const TRG_FLOORHERE: u16 = 32768;
}

/// special effect choices
#[repr(C)]
pub enum Effect {
    SPC_NONE = 0,
    SPC_SUMMON,
    SPC_ZAPWALL,
    SPC_RAISEWALL,
    SPC_TOGGLEWALL,
    SPC_TELEPORT,
    SPC_LIGHT,
    SPC_GOTOMAP,
    SPC_EXIT,
    SPC_PICTURE,
    SPC_PLAYSONG,
    SPC_PLAYSOUND,
    SPC_DROPITEM,
    SPC_TEMPLIGHT,
    SPC_SWAPMAP,
    SPC_CHGTILE,
    SPC_PLAYSOUND2,
    SPC_WINANDGO,
    SPC_COPYMAP,
    SPC_KILLMONS,
    SPC_CHGMONS,
    SPC_RMVSPCL,
    SPC_TOGGLEITEM,

    SPC_MAXEFFECTS
}

/// Map flags
pub mod MapFlags {
    pub const MAP_SNOWING: u8 = 1;
    pub const MAP_MANYITEMS: u8 = 2;
    pub const MAP_SECRET: u8 = 4;
    pub const MAP_TORCHLIT: u8 = 8;
    pub const MAP_STARS: u8 = 16;
}

/// map updating modes
#[repr(C)]
pub enum UpdateMode {
    UPDATE_GAME = 0,
    UPDATE_EDIT,
    UPDATE_FADE,
    UPDATE_FADEIN,
}

#[repr(C)]
pub struct special_t {
    pub trigger: u16,
    pub trigValue: u8,
    pub effect: u8,
    pub x: u8,
    pub y: u8,
    pub effectX: u8,
    pub effectY: u8,
    pub value: c_int,
    pub msg: [c_char; 32],
}

#[repr(C)]
pub struct mapTile_t {
    pub floor: u8,
    pub wall: u8,
    pub item: u8,
    pub light: i8,
    pub templight: i8,
    pub opaque: u8,
}

#[repr(C)]
pub struct mapBadguy_t {
    pub x: u8,
    pub y: u8,
    pub type_: u8,
}

opaque!(Map);

cpp! {{
    #include "map.h"
}}

impl Map {
    pub unsafe fn new(size: u8, name: *const c_char) -> *mut Map {
        cpp!([size as "byte", name as "const char*"] -> *mut Map as "Map*" {
            return new Map(size, name);
        })
    }

    pub unsafe fn from_file(mut f: *mut ::libc::FILE) -> *mut Map {
        cpp!([mut f as "FILE*"] -> *mut Map as "Map*" {
            return new Map(f);
        })
    }

    pub unsafe fn delete(me: *mut Map) {
        cpp!([me as "Map*"] { delete me; });
    }

    pub unsafe fn flags(me: *mut Map) -> *mut u8 {
        cpp!([me as "Map*"] -> *mut u8 as "byte*" {
            return &me->flags;
        })
    }

    pub unsafe fn Save(&mut self, mut f: *mut ::libc::FILE) {
        let me = self;
        cpp!([me as "Map*", mut f as "FILE*"] {
            me->Save(f);
        })
    }
}
