/* This file handles all screen displaying.  It's a go-between so that you
   don't have to pass the mgldraw object everywhere, and also handles the display
   list and camera, so everything is drawn in sorted order (or not drawn). */

use libc::{c_int, c_char};
use mgldraw::MGLDraw;
use jamulfont::*;
use jamulspr::sprite_t;
use map::Map;
use tile::TILE_HEIGHT;

pub const MAX_DISPLAY_OBJS: c_int = 1024;
pub const DISPLAY_XBORDER: c_int = 128;
pub const DISPLAY_YBORDER: c_int = 128;

bitflags! {
    /// display object flags
    #[repr(C)]
    pub struct DisplayFlags : u16 {
        const DISPLAY_DRAWME = 1;
        const DISPLAY_SHADOW = 2;
        const DISPLAY_WALLTILE = 4;
        const DISPLAY_ROOFTILE = 8;
        const DISPLAY_PARTICLE = 16;
        const DISPLAY_GHOST = 32;
        const DISPLAY_GLOW = 64;
        const DISPLAY_TRANSTILE = 128;
        const DISPLAY_LIGHTNING = 256;
        const DISPLAY_OFFCOLOR = 512;
    }
}

#[repr(C)]
pub struct displayObj_t {
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
    pub z2: c_int,
    pub spr: *mut sprite_t,
    pub hue: u8,
    pub bright: i8,
    pub flags: u16,
    pub prev: c_int,
    pub next: c_int,
}

extern {
    static mut gameFont: [*mut mfont_t; 2];
    static mut mgl: *mut MGLDraw;

    static mut scrx: c_int;
    static mut scry: c_int;
    static mut scrdx: c_int;
    static mut scrdy: c_int;
    static mut rscrx: c_int;
    static mut rscry: c_int;

    static mut shakeTimer: u8;

    static mut dispList: *mut DisplayList;
    #[link_name="display_gammaCorrection"]
    static mut gammaCorrection: u8;
}

pub unsafe fn get_camera() -> (c_int, c_int) {
    (scrx, scry)
}

#[no_mangle]
pub unsafe extern fn InitDisplay(mainmgl: &mut MGLDraw) -> bool {
    mgl = mainmgl;
    if mgl.is_null() { return false; }

    FontInit(mgl);
    gameFont[0] = Box::into_raw(match load_font(cstr!("graphics/girlsrweird.jft")) {
        Ok(font) => font,
        Err(_) => { return false; }
    });
    gameFont[1] = Box::into_raw(match load_font(cstr!("graphics/verdana.jft")) {
        Ok(font) => font,
        Err(_) => { return false; }
    });

    dispList = DisplayList::new();
    true
}

#[no_mangle]
pub unsafe extern fn ExitDisplay() {
    if !gameFont[0].is_null() {
        FontFree(gameFont[0]);
        Box::from_raw(gameFont[0]);
        gameFont[0] = 0 as *mut _;
    }
    if !gameFont[1].is_null() {
        FontFree(gameFont[1]);
        Box::from_raw(gameFont[1]);
        gameFont[0] = 1 as *mut _;
    }
    DisplayList::delete(dispList);
}

#[no_mangle] // iffy
pub unsafe extern fn GetDisplayMGL<'a>() -> &'a mut MGLDraw {
    assert!(!mgl.is_null());
    &mut *mgl
}

#[no_mangle]
pub unsafe extern fn GetDisplayScreen() -> *mut u8 {
    (*mgl).GetScreen()
}

#[no_mangle]
pub unsafe extern fn GetGamma() -> u8 {
    gammaCorrection
}

#[no_mangle]
pub unsafe extern fn SetGamma(g: u8) {
    gammaCorrection = g;
}

#[no_mangle]
pub unsafe extern fn GetCamera(x: *mut c_int, y: *mut c_int) {
    *x = scrx;
    *y = scry;
}

#[no_mangle]
pub unsafe extern fn PutCamera(x: c_int, y: c_int) {
    rscrx = x;
    rscry = y;
    scrdx = 0;
    scrdy = 0;

    scrx = rscrx >> ::FIXSHIFT;
    scry = rscry >> ::FIXSHIFT;
}

#[no_mangle]
pub unsafe extern fn GetStrLength(s: *const c_char) -> c_int {
    FontStrLen(s, &*gameFont[0])
}

#[no_mangle]
pub unsafe extern fn DrawMouseCursor(x: c_int, y: c_int) {
    FontPrintStringSolid(x - 1, y, cstr!("}"), &*gameFont[1], 0);
    FontPrintStringSolid(x + 1, y, cstr!("}"), &*gameFont[1], 0);
    FontPrintStringSolid(x, y - 1, cstr!("}"), &*gameFont[1], 0);
    FontPrintStringSolid(x, y + 1, cstr!("}"), &*gameFont[1], 0);
    FontPrintStringSolid(x, y, cstr!("}"), &*gameFont[1], 31);
}

#[no_mangle]
pub unsafe extern fn Print(x: c_int, y: c_int, s: *const c_char, bright: i8, font: u8) {
    if font == 0 {
        FontPrintStringBright(x, y, s, &*gameFont[0], bright);
    } else if bright == 0 {
        FontPrintString(x, y, s, &*gameFont[1]);
    } else {
        FontPrintStringSolid(x, y, s, &*gameFont[1], 0);
    }
}

#[no_mangle]
pub unsafe extern fn CenterPrint(x: c_int, y: c_int, s: *const c_char, bright: c_char, font: u8) {
    if font == 0 {
        let x = x - FontStrLen(s, &*gameFont[0]) / 2;
        FontPrintStringBright(x, y, s, &*gameFont[0], bright);
    } else {
        let f = &*gameFont[1];
        let x = x - FontStrLen(s, f) / 2;
        if bright == 0 {
            FontPrintString(x, y, s, f);
        } else if bright != 16 {
            FontPrintStringSolid(x, y, s, f, 0);
        } else {
            FontPrintStringSolid(x, y, s, f, 16);
        }
    }
}

#[no_mangle]
pub unsafe extern fn ShakeScreen(howlong: u8) {
    shakeTimer = howlong;
}

#[no_mangle]
pub unsafe extern fn DrawBox(x: c_int, y: c_int, x2: c_int, y2: c_int, c: u8) {
    (*mgl).Box(x, y, x2, y2, c);
}

#[no_mangle]
pub unsafe extern fn DrawDebugBox(x: c_int, y: c_int, x2: c_int, y2: c_int) {
    let x = x - scrx + 320;
    let y = y - scry + 240;
    let x2 = x2 - scrx + 320;
    let y2 = y2 - scry + 240;
    (*mgl).Box(x, y, x2, y2, 255);
    (*mgl).Flip();
}

#[no_mangle]
pub unsafe extern fn DrawFillBox(x: c_int, y: c_int, x2: c_int, y2: c_int, c: u8) {
    (*mgl).FillBox(x, y, x2, y2, c);
}

// these calls return whether they worked or not, but frankly, we don't care
#[no_mangle]
pub unsafe extern fn SprDraw(x: c_int, y: c_int, z: c_int, hue: u8, bright: i8, spr: *mut sprite_t, flags: u16) {
    (*dispList).DrawSprite(x, y, z, 0, hue, bright, spr, flags);
}

#[no_mangle]
pub unsafe extern fn SprDrawOff(x: c_int, y: c_int, z: c_int, hue: u8, bright: i8, spr: *mut sprite_t, flags: u16) {
    (*dispList).DrawSprite(x, y, z, 0, hue, bright, spr, flags | DISPLAY_OFFCOLOR.bits());
}

#[no_mangle]
pub unsafe extern fn WallDraw(x: c_int, y: c_int, wall: u8, floor: u8, map: *mut Map, flags: u16) {
    (*dispList).DrawSprite(x, y, 0, wall as c_int, floor, 0, map as *mut sprite_t, flags);
}

#[no_mangle]
pub unsafe extern fn RoofDraw(x: c_int, y: c_int, roof: u8, map: *mut Map, flags: u16) {
    (*dispList).DrawSprite(x, y, TILE_HEIGHT, 0, roof, 0, map as *mut sprite_t, flags);
}

#[no_mangle]
pub unsafe extern fn ParticleDraw(x: c_int, y: c_int, z: c_int, color: u8, size: u8, flags: u16) {
    (*dispList).DrawSprite(x, y, z, 0, color, size as i8, 1 as *mut sprite_t, flags);
}

#[no_mangle]
pub unsafe extern fn LightningDraw(x: c_int, y: c_int, x2: c_int, y2: c_int, bright: u8, range: i8) {
    (*dispList).DrawSprite(x, y, x2, y2, bright, range, 1 as *mut sprite_t, (DISPLAY_DRAWME | DISPLAY_LIGHTNING).bits());
}

// ---------------------------------------------------------------------------------------
// from here on out it's class DISPLAYLIST

cpp! {{
    #include "display.h"
}}

opaque!(DisplayList);

impl DisplayList {
    pub unsafe fn new() -> *mut DisplayList {
        cpp!([] -> *mut DisplayList as "DisplayList*" {
            return new DisplayList();
        })
    }

    pub unsafe fn delete(me: *mut DisplayList) {
        cpp!([me as "DisplayList*"] {
            delete me;
        })
    }

    pub unsafe fn DrawSprite(&mut self,
        x: c_int, y: c_int, z: c_int, z2: c_int,
        hue: u8, bright: i8, spr: *mut sprite_t, flags: u16
    ) -> bool {
        let me = self;
        cpp!([me as "DisplayList*", x as "int", y as "int", z as "int", z2 as "int",
            hue as "byte", bright as "char", spr as "sprite_t*", flags as "word"
        ] -> bool as "bool" {
            return me->DrawSprite(x, y, z, z2, hue, bright, spr, flags);
        })
    }
}
