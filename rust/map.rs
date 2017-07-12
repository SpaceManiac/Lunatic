use libc::{c_int, c_char};

pub const MAX_LIGHT: i8 = 16;
pub const MIN_LIGHT: i8 = -32;

/// the maximum # of monsters you can put on the map
pub const MAX_MAPMONS: usize = 128;
pub const MAX_SPECIAL: usize = 32;

bitflags! {
    /// flags for calling map render
    pub struct RenderFlags: u8 {
        const MAP_SHOWLIGHTS = 1;
        const MAP_SHOWWALLS = 2;
        const MAP_SHOWITEMS = 4;
        const MAP_SHOWBADGUYS = 8;
        const MAP_SHOWSPECIALS = 16;
    }
}

bitflags! {
    /// special trigger flags
    pub struct TriggerFlags: u16 {
        const TRG_STEP = 1;
        const TRG_ENEMYSTEP = 2;
        const TRG_NEAR = 4;
        const TRG_PASSLEVELS = 8;
        const TRG_KEYCHAINS = 16;
        const TRG_KILLALL = 32;
        const TRG_HAVEBRAINS = 64;
        const TRG_SHOOT = 128;
        /// works more than once
        const TRG_REPEATABLE = 256;
        /// displays a message
        const TRG_MESSAGE = 512;
        /// goes off if any special in an adjacent square goes off
        const TRG_CHAIN = 1024;
        /// occurs every N seconds
        const TRG_TIMED = 2048;
        /// occurs randomly, with an N% chance each second
        const TRG_RANDOM = 4096;
        /// has loony key for this world
        const TRG_HAVELOONY = 8192;
        /// kill just one of the chosen monster type
        const TRG_KILLONE = 16384;
        /// the floor/roof here is N
        const TRG_FLOORHERE = 32768;
    }
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

bitflags! {
    /// Map flags
    pub struct MapFlags: u8 {
        const MAP_SNOWING = 1;
        const MAP_MANYITEMS = 2;
        const MAP_SECRET = 4;
        const MAP_TORCHLIT = 8;
        const MAP_STARS = 16;
    }
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
