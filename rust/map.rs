use libc::{c_int, c_char};
use world::world_t;
use mgldraw::MGLDraw;
use monster::MonsterType;

pub const MAX_LIGHT: i8 = 16;
pub const MIN_LIGHT: i8 = -32;

/// the maximum # of monsters you can put on the map
pub const MAX_MAPMONS: usize = 128;
pub const MAX_SPECIAL: usize = 32;

bitflags! {
    /// flags for calling map render
    #[repr(C)]
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
    #[repr(C)]
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
#[repr(u8)]
#[derive(PartialEq)]
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
    #[repr(C)]
    pub struct MapFlags: u8 {
        const MAP_EMPTY = 0;
        const MAP_SNOWING = 1;
        const MAP_MANYITEMS = 2;
        const MAP_SECRET = 4;
        const MAP_TORCHLIT = 8;
        const MAP_STARS = 16;
    }
}

/// map updating modes
#[repr(u8)]
pub enum UpdateMode {
    Game = 0,
    Edit,
    FadeOut,
    FadeIn,
}

#[repr(C)]
pub struct special_t {
    pub trigger: TriggerFlags,
    pub trigValue: u8,
    pub effect: Effect,
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
    pub type_: MonsterType,
}

#[repr(C)]
pub struct Map {
    pub width: c_int,
    pub height: c_int,
    pub map: *mut mapTile_t,
    pub name: [c_char; 32],
    pub song: u8,
    pub flags: MapFlags,
    pub badguy: [mapBadguy_t; MAX_MAPMONS],
    pub special: [special_t; MAX_SPECIAL],
    /// Gourad stuff
    pub smoothLight: [i8; 9],
}

extern {
    pub fn RenderSpecialXes(mgl: &mut MGLDraw, map: &mut Map, world: u8);
    pub fn SpecialAnytimeCheck(map: &mut Map);

    pub fn TotalBrains() -> c_int;
}

cpp! {{
    #include "map.h"
}}

impl Map {
    pub unsafe fn new(size: u8, name: *const c_char) -> *mut Map {
        cpp!([size as "byte", name as "const char*"] -> *mut Map as "Map*" {
            return new Map(size, name);
        })
    }

    pub unsafe fn from_map(m: &Map) -> *mut Map {
        cpp!([m as "Map*"] -> *mut Map as "Map*" {
            return new Map(m);
        })
    }

    pub unsafe fn from_file(f: *mut ::libc::FILE) -> *mut Map {
        cpp!([f as "FILE*"] -> *mut Map as "Map*" {
            return new Map(f);
        })
    }

    pub unsafe fn delete(me: *mut Map) {
        cpp!([me as "Map*"] { delete me; });
    }

    pub fn get_tile(&self, x: c_int, y: c_int) -> &mapTile_t {
        assert!(!self.map.is_null());
        assert!(x >= 0 && x < self.width);
        assert!(y >= 0 && y < self.height);
        unsafe { &*self.map.offset((x + y * self.width) as isize) }
    }

    pub fn get_tile_mut(&mut self, x: c_int, y: c_int) -> &mut mapTile_t {
        assert!(!self.map.is_null());
        assert!(x >= 0 && x < self.width);
        assert!(y >= 0 && y < self.height);
        unsafe { &mut *self.map.offset((x + y * self.width) as isize) }
    }

    pub unsafe fn Save(&mut self, f: *mut ::libc::FILE) {
        let me = self;
        cpp!([me as "Map*", f as "FILE*"] {
            me->Save(f);
        })
    }

    pub unsafe fn Init(&mut self, wrld: *mut world_t) {
        let me = self;
        cpp!([me as "Map*", wrld as "world_t*"] {
            me->Init(wrld);
        })
    }

    pub unsafe fn Render(&mut self, world: *mut world_t, camX: c_int, camY: c_int, flags: RenderFlags) {
        let me = self;
        cpp!([me as "Map*", world as "world_t*", camX as "int", camY as "int", flags as "byte"] {
            me->Render(world, camX, camY, flags);
        })
    }

    pub unsafe fn Update(&mut self, mode: UpdateMode, world: &mut world_t) {
        let me = self;
        cpp!([me as "Map*", mode as "byte", world as "world_t*"] {
            me->Update(mode, world);
        })
    }

    pub unsafe fn MakeSmoothLighting(&mut self, beZero: bool, x: c_int, y: c_int) -> *mut c_char {
        let me = self;
        cpp!([me as "Map*", beZero as "bool", x as "int", y as "int"] -> *mut c_char as "char*" {
            return me->MakeSmoothLighting(beZero, x, y);
        })
    }
}
