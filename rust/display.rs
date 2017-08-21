/* This file handles all screen displaying.  It's a go-between so that you
   don't have to pass the mgldraw object everywhere, and also handles the display
   list and camera, so everything is drawn in sorted order (or not drawn). */

use libc::{c_int, c_char};
use mgldraw::MGLDraw;
use jamulfont::*;
use jamulspr::sprite_t;
use map::{Map, RenderFlags};
use tile::TILE_HEIGHT;

pub const MAX_DISPLAY_OBJS: usize = 1024;
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

static mut gameFont: [*mut mfont_t; 2] = [0 as *mut mfont_t; 2];
static mut mgl: *mut MGLDraw = 0 as *mut MGLDraw;

static mut scrx: c_int = 0;
static mut scry: c_int = 0;
static mut scrdx: c_int = 0;
static mut scrdy: c_int = 0;
static mut rscrx: c_int = 0;
static mut rscry: c_int = 0;

static mut shakeTimer: u8 = 0;

static mut dispList: *mut DisplayList = 0 as *mut DisplayList;

static mut gammaCorrection: u8 = 0;

pub unsafe fn get_camera() -> (c_int, c_int) {
    (scrx, scry)
}

#[no_mangle]
pub unsafe extern fn InitDisplay(mainmgl: *mut MGLDraw) -> bool {
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

    dispList = Box::into_raw(Box::new(DisplayList::new()));
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
    Box::from_raw(dispList);
}

#[no_mangle] // iffy
pub unsafe extern fn GetDisplayMGL<'a>() -> &'a mut MGLDraw {
    assert!(!mgl.is_null());
    &mut *mgl
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

#[no_mangle]
pub unsafe extern fn ShowVictoryAnim(world: u8) {
    use ffi::win::timeGetTime;
    use jamulfmv::play_flic;
    use music::CDPlay;

    let mgl_ = &mut *mgl;

    let start = timeGetTime();
    let music_on = ::player::PlayerGetMusicSettings() == ::options::Music::On;
    if music_on {
        if world < 4 {
            CDPlay(19); // standard victory theme
        } else if world > 10 {
            CDPlay(16); // the asylum hub music.  Play it for the asylum intro anim.
        } else if world == 10 {
            CDPlay(18); // switch to asylum boss music when Lunatic transforms
        }
    }
    match world {
        0 => play_flic("graphics/caverns.flc", false, 80, mgl_),
        1 => play_flic("graphics/icy.flc", false, 60, mgl_),
        2 => play_flic("graphics/forest.flc", false, 60, mgl_),
        3 => play_flic("graphics/desert.flc", false, 60, mgl_),
        4 => { // the final victory!
            if music_on {
                CDPlay(22); // ending music, deedeleedo
            }
            play_flic("graphics/asylum.flc", false, 60, mgl_);
        }
        10 => play_flic("graphics/transfrm.flc", false, 60, mgl_),
        11 => play_flic("graphics/asylumno.flc", false, 40, mgl_),
        12 => play_flic("graphics/asylumys.flc", false, 40, mgl_),
        _ => {}
    }
    mgl_.LoadBMP(cstr!("graphics/title.bmp"));
    ::game::AddGarbageTime(timeGetTime() - start);
}

unsafe fn LoadText(nm: *const c_char) {
    use libc::*;

    let f = fopen(nm, cstr!("rt"));
    if f.is_null() { return; }

    let mgl_ = &mut *mgl;
    mgl_.ClearScreen();
    for y in 0..32 {
        mgl_.FillBox(0, y, 639, y, (31 - y as u8) + 32 * 5);
        mgl_.FillBox(0, 479 - y, 639, 479 - y, (31 - y as u8) + 32 * 5);
    }

    let mut y = 10;
    let mut line = [0; 256];
    while !fgets(decay!(&mut line), 256, f).is_null() && y < 480 - 50 {
        CenterPrint(320, y, decay!(&line), 0, 0);
        y += 50;
    }
    fclose(f);
}

#[no_mangle]
pub unsafe extern fn ShowImageOrFlic(input: *const c_char) {
    use std::ascii::AsciiExt;
    use sound::{make_normal_sound, Sound};
    use ffi::win::timeGetTime;

    let input = ::std::ffi::CStr::from_ptr(input).to_string_lossy();
    let mut split = input.splitn(2, ",");
    let fname = split.next().unwrap();
    let speed = split.next().and_then(|v| v.parse::<u16>().ok()).unwrap_or(60);

    let mut nm = [0; 64];
    sprintf!(nm, "graphics/{}", fname);

    let ext = match fname.rfind(".") {
        Some(i) => &fname[i + 1..],
        None => return, // no extension, what is it?
    };

    if ext.eq_ignore_ascii_case("bmp") {
        // BMP loading
        ::game::EnterPictureDisplay();
        make_normal_sound(Sound::SND_MESSAGE);
        (*mgl).LoadBMP(decay!(&nm));
    } else if ext.eq_ignore_ascii_case("txt") {
        // Text files
        ::game::EnterPictureDisplay();
        make_normal_sound(Sound::SND_MESSAGE);
        LoadText(decay!(&nm));
    } else {
        // assume it's an flc for now
        let start = timeGetTime();
        ::jamulfmv::play_flic(&format!("graphics/{}", fname), false, speed, &mut *mgl);
        (*mgl).LoadBMP(cstr!("graphics/title.bmp"));
        ::game::AddGarbageTime(timeGetTime() - start);
    }
}

#[no_mangle]
pub unsafe extern fn UpdateCamera(x: c_int, y: c_int, facing: u8, map: &Map) {
    use {FIXSHIFT, FIXAMT};
    use cossin::*;
    use tile::{TILE_WIDTH, TILE_HEIGHT};
    use std::cmp::{min, max};

    let desiredX = ((x << FIXSHIFT) + Cosine(facing as c_int) * 80) >> FIXSHIFT;
    let desiredY = ((y << FIXSHIFT) + Sine(facing as c_int) * 60) >> FIXSHIFT;

    rscrx += scrdx;
    rscry += scrdy;

    rscrx = max(rscrx, 320 << FIXSHIFT);
    rscrx = min(rscrx, (map.width * TILE_WIDTH - 320) << FIXSHIFT);
    rscry = max(rscry, (240 - TILE_HEIGHT) << FIXSHIFT);
    rscry = min(rscry, (map.height * TILE_HEIGHT - 240) << FIXSHIFT);

    if scrx > desiredX + 20 {
        scrdx = (scrx - (desiredX + 20)) * FIXAMT / -16;
    } else if scrx < desiredX - 20 {
        scrdx = (desiredX - 20 - scrx) * FIXAMT / 16;
    }
    if scry > desiredY + 20 {
        scrdy = (scry - (desiredY + 20)) * FIXAMT / -16;
    } else if scry < desiredY - 20 {
        scrdy = (desiredY - 20 - scry) * FIXAMT / 16;
    }

    Dampen(&mut scrdx, 1 << FIXSHIFT);
    Dampen(&mut scrdy, 1 << FIXSHIFT);

    scrx = rscrx >> FIXSHIFT;
    scry = rscry >> FIXSHIFT;
}

#[no_mangle]
pub unsafe extern fn RenderItAll(world: *mut ::world::world_t, map: &mut Map, flags: RenderFlags) {
    if shakeTimer > 0 {
        shakeTimer -= 1;
        scrx -= 2 + ::mgldraw::MGL_random(5);
        scry -= 2 + ::mgldraw::MGL_random(5);
    }
    map.Render(world, scrx, scry, flags);

    scrx -= 320;
    scry -= 240;
    (*dispList).Render(&mut *mgl);
    (*dispList).ClearList();
    scrx += 320;
    scry += 240;
}

// these calls return whether they worked or not, but frankly, we don't care
#[no_mangle]
pub unsafe extern fn SprDraw(x: c_int, y: c_int, z: c_int, hue: u8, bright: i8, spr: *const sprite_t, flags: DisplayFlags) {
    (*dispList).DrawSprite(x, y, z, 0, hue, bright, spr, flags);
}

#[no_mangle]
pub unsafe extern fn SprDrawOff(x: c_int, y: c_int, z: c_int, fromHue: u8, hue: u8, bright: i8, spr: *const sprite_t, flags: DisplayFlags) {
    (*dispList).DrawSprite(x, y, z, fromHue as i32, hue, bright, spr, flags | DISPLAY_OFFCOLOR);
}

#[no_mangle]
pub unsafe extern fn WallDraw(x: c_int, y: c_int, wall: u8, floor: u8, map: *mut Map, flags: DisplayFlags) {
    (*dispList).DrawSprite(x, y, 0, wall as c_int, floor, 0, map as *mut sprite_t, flags);
}

#[no_mangle]
pub unsafe extern fn RoofDraw(x: c_int, y: c_int, roof: u8, map: *mut Map, flags: DisplayFlags) {
    (*dispList).DrawSprite(x, y, TILE_HEIGHT, 0, roof, 0, map as *mut sprite_t, flags);
}

#[no_mangle]
pub unsafe extern fn ParticleDraw(x: c_int, y: c_int, z: c_int, color: u8, size: u8, flags: DisplayFlags) {
    (*dispList).DrawSprite(x, y, z, 0, color, size as i8, 1 as *mut sprite_t, flags);
}

#[no_mangle]
pub unsafe extern fn LightningDraw(x: c_int, y: c_int, x2: c_int, y2: c_int, bright: u8, range: i8) {
    (*dispList).DrawSprite(x, y, x2, y2, bright, range, 1 as *mut sprite_t, DISPLAY_DRAWME | DISPLAY_LIGHTNING);
}

// ---------------------------------------------------------------------------------------
// from here on out it's class DISPLAYLIST

#[repr(C)]
#[derive(Copy, Clone)]
struct displayObj_t {
    x: c_int,
    y: c_int,
    z: c_int,
    z2: c_int,
    spr: *const sprite_t,
    hue: u8,
    bright: i8,
    flags: DisplayFlags,
    prev: usize,
    next: usize,
}

const NONE: usize = ::std::usize::MAX;

#[repr(C)]
pub struct DisplayList {
    dispObj: [displayObj_t; MAX_DISPLAY_OBJS],
    head: usize,
    nextfree: usize,
}

impl DisplayList {
    fn new() -> DisplayList {
        DisplayList {
            dispObj: [displayObj_t {
                prev: NONE, next: NONE, flags: DisplayFlags::empty(),
                x: 0, y: 0, z: 0, z2: 0, spr: 0 as *mut sprite_t,
                hue: 0, bright: 0,
            }; MAX_DISPLAY_OBJS],
            head: NONE,
            nextfree: 0,
        }
    }

    pub fn ClearList(&mut self) {
        *self = DisplayList::new();
    }

    fn GetOpenSlot(&self) -> usize {
        self.dispObj.iter().position(|i| i.flags.is_empty()).unwrap_or(NONE)
    }

    fn HookIn(&mut self, me: usize) {
        let dispObj = &mut self.dispObj;
        if self.head == NONE {
            self.head = me;
            dispObj[me].prev = NONE;
            dispObj[me].next = NONE;
            return;
        }

        // shadows go on the head of the list always, drawn before anything else
        // (and the order of shadows doesn't matter, of course)
        if dispObj[me].flags.contains(DISPLAY_SHADOW) {
            dispObj[me].next = self.head;
            dispObj[self.head].prev = me;
            dispObj[me].prev = NONE;
            self.head = me;
            return;
        }

        let mut i = self.head;
        loop {
            if !dispObj[i].flags.contains(DISPLAY_SHADOW) &&
                (dispObj[i].y > dispObj[me].y ||
                (dispObj[i].y == dispObj[me].y && dispObj[i].z > dispObj[me].z))
            {
                dispObj[me].prev = dispObj[i].prev;
                dispObj[me].next = i;
                if dispObj[me].prev != NONE {
                    dispObj[dispObj[me].prev].next = me;
                }
                dispObj[i].prev = me;
                if self.head == i {
                    self.head = me;
                }
                return;
            }
            if dispObj[i].next == NONE {
                dispObj[i].next = me;
                dispObj[me].prev = i;
                dispObj[me].next = NONE;
                return;
            }
            i = dispObj[i].next;
        }
    }

    fn DrawSprite(&mut self,
        x: c_int, y: c_int, z: c_int, z2: c_int,
        hue: u8, bright: i8, spr: *const sprite_t, flags: DisplayFlags
    ) -> bool {
        let (scrx_, scry_) = unsafe { (scrx, scry) };
        if (x - scrx_ + 320) < -DISPLAY_XBORDER ||
            (x - scrx_ + 320) > 640 + DISPLAY_XBORDER ||
            (y - scry_ + 240) < -DISPLAY_YBORDER ||
            (y - scry_ + 240) > 480 + DISPLAY_YBORDER
        {
            return true;
        }

        let i = self.GetOpenSlot();
        if i == NONE { return false; }

        {
            let o = &mut self.dispObj[i];
            o.hue = hue;
            o.bright = bright;
            o.flags = flags;
            o.spr = spr;
            o.x = x;
            o.y = y;
            o.z = z;
            o.z2 = z2;
        }
        self.HookIn(i);
        true
    }

    unsafe fn Render(&mut self, mgl_: &mut MGLDraw) {
        use particle::{RenderParticle, RenderLightningParticle};
        use tile::{RenderWallTileFancy, RenderRoofTileFancy};

        let mut i = self.head;
        while i != NONE {
            let o = self.dispObj[i];
            i = o.next;

            if !o.flags.contains(DISPLAY_DRAWME) || o.spr.is_null() {
                continue
            }

            let x = o.x - scrx;
            let y = o.y - o.z - scry;
            //let mgl_ = &mut *mgl;

            if o.flags.contains(DISPLAY_WALLTILE) {
                // for tiles, DISPLAY_GHOST means lighting is disabled
                let bright = (*(o.spr as *mut Map)).MakeSmoothLighting(o.flags.contains(DISPLAY_GHOST), o.x / 32, o.y / 24);
                RenderWallTileFancy(x, o.y - scry, 199 + o.z2, bright);
                RenderRoofTileFancy(x, o.y - scry - TILE_HEIGHT, o.hue as i32, o.flags.contains(DISPLAY_TRANSTILE), 0, bright);
            } else if o.flags.contains(DISPLAY_ROOFTILE) {
                let bright = (*(o.spr as *mut Map)).MakeSmoothLighting(o.flags.contains(DISPLAY_GHOST), o.x / 32, o.y / 24);
                RenderRoofTileFancy(x, o.y - scry - TILE_HEIGHT, o.hue as i32, o.flags.contains(DISPLAY_TRANSTILE), 0, bright);
            } else if o.flags.contains(DISPLAY_SHADOW) {
                (*o.spr).DrawShadow(x, y, mgl_);
            } else if o.flags.contains(DISPLAY_PARTICLE) {
                RenderParticle(x, y, mgl_.get_screen(), o.hue, o.bright as u8);
            } else if o.flags.contains(DISPLAY_LIGHTNING) {
                RenderLightningParticle(x, o.y - scry, o.z - scrx, o.z2 - scry, o.bright as c_int, o.hue, mgl_.get_screen());
            } else if o.flags.contains(DISPLAY_GHOST) {
                (*o.spr).DrawGhost(x, y, mgl_, o.bright);
            } else if o.flags.contains(DISPLAY_GLOW) {
                (*o.spr).DrawGlow(x, y, mgl_, o.bright);
            } else if o.flags.contains(DISPLAY_OFFCOLOR) {
                (*o.spr).DrawOffColor(x, y, mgl_, o.z2 as u8, o.hue, o.bright);
            } else if o.hue == 255 { // no special coloring
                (*o.spr).DrawBright(x, y, mgl_, o.bright);
            } else { // draw special color
                (*o.spr).DrawColored(x, y, mgl_, o.hue, o.bright);
            }
        }
    }
}
