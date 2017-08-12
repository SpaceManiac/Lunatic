/* this file does the AI and everything else for each monster type.
   It knows all about the different types, as opposed to guy.cpp which
   just sort of keeps track of the list of guys. */

use libc::{c_int, c_char};
use jamulspr::{sprite_t, sprite_set_t};
use guy::Guy;
use map::Map;
use world::world_t;
use player::{player, Weapon};
use options::{opt, PlayAs};
use mgldraw::MGLDraw;

/// the monster types
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MonsterType {
    MONS_NONE = 0,
    MONS_BOUAPHA = 1,
    MONS_BONEHEAD = 2,
    MONS_BAT = 3,
    MONS_SPIDER = 4,
    MONS_BIGSPDR = 5,
    MONS_ZOMBIE = 6,
    MONS_EGGSAC = 7,
    MONS_MAMASPDR = 8,
    MONS_PYGMY = 9,
    MONS_SERPENT = 10,
    // the parts of Mattie
    MONS_MATHEAD = 11,
    MONS_MATSKULL = 12,
    MONS_MATBRAIN = 13,
    MONS_MATBODY = 14,
    MONS_MATCLAW1 = 15,
    MONS_MATCLAW2 = 16,
    MONS_MATTAIL = 17,
    // more normal monsters
    MONS_GINGER = 18,
    MONS_PUMPKIN = 19,
    MONS_BABYTHING = 20,
    MONS_MOSS = 21,
    MONS_MOSSGRANDE = 22,
    MONS_MAGMAZOID = 23,
    MONS_SHROOM = 24,
    MONS_MUSH = 25,
    // the parts of The Thing
    MONS_THING = 26,
    MONS_THINGTENT = 27,
    MONS_THINGTENTTIP = 28,
    // normal monsters again
    MONS_SUPERZOMBIE = 29,
    MONS_STICKMAN = 30,
    MONS_BABYSEAL = 31,
    MONS_ISOZOID = 32,
    MONS_SNOWGUY = 33,
    MONS_PENGUIN = 34,
    MONS_ZOMBONI = 35,
    // the Yeti Bros.
    MONS_SVEN = 36,
    MONS_BJORN = 37,
    // normal monsters again
    MONS_GEOZOID = 38,
    MONS_MUMBLE = 39,
    MONS_DJINNI = 40,
    MONS_LAMP = 41,
    MONS_CACTUS = 42,
    MONS_ROLLER = 43,
    MONS_LICH = 44,
    MONS_DUSTDEVIL = 45,
    MONS_MECHABOUAPHA = 46,
    // Sphinx parts
    MONS_SPHXARM1 = 47,
    MONS_SPHXARM2 = 48,
    MONS_SPHINX = 49,
    // more normal monsters
    MONS_FREAKAZOID = 50,
    MONS_CENTIBODY = 51,
    MONS_CENTIHEAD = 52,
    MONS_WACKO = 53,
    MONS_BOILER = 54,
    MONS_GREATPK = 55,
    MONS_ULTRAZOID = 56,
    // the Asylum bosses
    MONS_DRL = 57,
    MONS_SDZL = 58,

    MONS_SANTA = 59,

    // expansion "monsters"
    MONS_MINECART = 60,		// mine cart Bouapha rides on
    MONS_RAFT = 61,			// raft Bouapha rides on
    MONS_PWRBOUAPHA = 62,	// Bouapha in power armor

    // expansion monsters
    MONS_VAMPIRE = 63,
    MONS_COFFIN = 64,
    MONS_GHOST = 65,
    MONS_BURNER = 66,
    MONS_LEFTY = 67,
    MONS_PYGMY2 = 68,
    MONS_PYGMY3 = 69,
    MONS_PKSTEIN = 70,
    MONS_KNIGHT = 71,
    MONS_TRICEROID = 72,
    MONS_COUNTESS = 73,
    MONS_ALIENEGG = 74,
    MONS_BABYALIEN = 75,
    MONS_ALIEN = 76,
    MONS_ROBOPK = 77,
    MONS_SHOCKTR = 78,
    MONS_ROBOT1 = 79,
    MONS_ROBOT2 = 80,
    MONS_ROBOFACTORY = 81,
    MONS_TURRET = 82,
    MONS_BUNNY = 83,
    MONS_KONGOR = 84,
    MONS_SQUASH = 85,
    MONS_MINIPYGMY = 86,
    MONS_LOONYBOT = 87,
    MONS_LOONYCORE = 88,
    MONS_LOONYGUN = 89,
    MONS_LOONYSHIP = 90,

    // -- fun pack
    MONS_FRIENDLY = 91,
    MONS_GOODTURRET = 92,
    MONS_ROLLER2 = 93,
    MONS_ALIENEGG2 = 94,

    MONS_MAT2HEAD = 95,
    MONS_MAT2SKULL = 96,
    MONS_MAT2BRAIN = 97,
    MONS_MAT2BODY = 98,
    MONS_MAT2TAIL = 99,

    MONS_SHAMAN2 = 100,
    MONS_JALAPENO = 101,

    MONS_GENERATOR1 = 102,
    MONS_GENERATOR2 = 103,
    MONS_GENERATOR3 = 104,
    MONS_GENERATOR4 = 105,

    MONS_SHARK = 106,
    MONS_MADBUG = 107,
    MONS_WIZARD = 108,
    MONS_EVILCLONE = 109,
    MONS_BOB = 110,
    MONS_MULTIMOSS = 111,
    MONS_MOSS2 = 112,
    MONS_SNOWBALL = 113,
    MONS_SNOWBALL2 = 114,
    MONS_SNOWBLOW = 115,
    MONS_BOOMKIN = 116,
    MONS_MUMBLE2 = 117,
    MONS_GOODROBOT = 118,
    MONS_GOODROBOT2 = 119,
    MONS_XENOMAMA = 120,
    MONS_ROLLER3 = 121,
    MONS_ROLLER4 = 122,
    MONS_DARKVAMP = 123,
    MONS_GNOME = 124,
    MONS_NOBODY = 125,
    MONS_FRIENDLY2 = 126,
    MONS_TROOPER2 = 127,
    MONS_PUMPKIN2 = 128,
    MONS_CRAZYBONE = 129,
    MONS_CREEPAZOID = 130
}

pub const NUM_MONSTERS: usize = 131;
// 60 without EXPANDO

/// the animations
#[repr(u8)]
pub enum Animation {
    ANIM_IDLE = 0,
    ANIM_MOVE,
    ANIM_ATTACK,
    ANIM_DIE,
    ANIM_A1,
    ANIM_A2,
    ANIM_A3,
    ANIM_A4,
    ANIM_A5,
    NUM_ANIMS,
}

pub const NUM_ANIMS: usize = Animation::NUM_ANIMS as usize;
pub const ANIM_LENGTH: usize = 24;

bitflags! {
    /// flags
    #[repr(C)]
    pub struct MonsterFlags: u16 {
        const EMPTY = 0;
        const MF_FLYING = 1;
        const MF_WATERWALK = 2;
        const MF_ONEFACE = 4;
        /// other enemies can stomp all over this one (but not Bouapha)
        const MF_ENEMYWALK = 8;
        /// doesn't move when hit
        const MF_NOMOVE = 16;
        /// can ONLY move on water/lava, not land
        const MF_AQUATIC = 32;
        /// totally invulnerable to harm
        const MF_INVINCIBLE = 64;
        /// use the sprite's rect for collision checks instead of standard size-box method
        const MF_SPRITEBOX = 128;
        /// this monster's "facing" value should just be added to the sprite number,
        /// it's calculated by his AI (only useful for MF_ONEFACE monsters)
        const MF_FACECMD = 256;
        const MF_NOGRAV = 512;
        /// Bouapha can walk right through it
        const MF_FREEWALK = 1024;
        /// walk through walls
        const MF_WALLWALK = 2048;
        /// doesn't cast a shadow
        const MF_NOSHADOW = 4096;
        /// draw using ghost draw
        const MF_GHOST = 8192;
        /// bullets pass through it
        const MF_NOHIT = 16384;
        /// draw using glow draw
        const MF_GLOW = 32768;
    }
}

pub type monsterAi_t = unsafe extern fn(me: *mut Guy, map: *mut Map, world: *mut world_t, goodguy: *mut Guy);

#[repr(C)]
pub struct monsterType_t {
    name: [c_char; 32],
    fromCol: u8,
    toCol: u8,
    brtChg: i8,
    size: u8,
    framesPerDir: u8,
    hp: u16,
    points: u16,
    sprName: [c_char; 32],
    spr: *mut sprite_set_t,
    flags: MonsterFlags,
    aiFunc: Option<monsterAi_t>,
    anim: [[u8; ANIM_LENGTH]; NUM_ANIMS],
}

extern {
    static mut monsType: [monsterType_t; NUM_MONSTERS];
}

#[no_mangle]
pub unsafe extern fn InitMonsters() {
    for ty in monsType.iter_mut() {
        ty.spr = ::std::ptr::null_mut();
    }
    // just keep bouapha perma-loaded
    let bouapha = &mut monsType[MonsterType::MONS_BOUAPHA as usize];
    bouapha.spr = sprite_set_t::from_fname(&bouapha.sprName[0]);
}

#[no_mangle]
pub unsafe extern fn ExitMonsters() {
    PurgeMonsterSprites();
    sprite_set_t::delete(monsType[MonsterType::MONS_BOUAPHA as usize].spr);
}

#[no_mangle]
pub unsafe extern fn PurgeMonsterSprites() {
    // note this starts at 2, skipping bouapha
    for ty in monsType[2..].iter_mut() {
        // repeat graphics monsters do not delete their sprites
        if ty.spr.is_null() { continue }
        if ty.sprName[0] as u8 != b'!' {
            sprite_set_t::delete(ty.spr);
        }
        ty.spr = ::std::ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern fn ChangeOffColor(type_: u8, from: u8, to: u8) {
    monsType[type_ as usize].fromCol = from;
    monsType[type_ as usize].toCol = to;
}

unsafe fn power_bouapha(type_: u8) -> &'static monsterType_t {
    if type_ == MonsterType::MONS_BOUAPHA as u8 &&
        player.weapon == Weapon::WPN_PWRARMOR
    {
        &monsType[MonsterType::MONS_PWRBOUAPHA as usize]
    } else {
        &monsType[type_ as usize]
    }
}

#[no_mangle]
pub unsafe extern fn MonsterSize(type_: u8) -> u8 {
    power_bouapha(type_).size
}

#[no_mangle]
pub unsafe extern fn MonsterAnim(type_: u8, anim: u8) -> *const u8 {
    power_bouapha(type_).anim[anim as usize].as_ptr()
}

#[no_mangle]
pub unsafe extern fn MonsterFlags(type_: u8) -> u16 {
    power_bouapha(type_).flags.bits()
}

#[no_mangle]
pub unsafe extern fn MonsterFrames(type_: u8) -> u8 {
    power_bouapha(type_).framesPerDir
}

#[no_mangle]
pub unsafe extern fn MonsterPoints(type_: u8) -> u16 {
    monsType[type_ as usize].points
}

#[no_mangle]
pub unsafe extern fn MonsterHP(type_: u8) -> u16 {
    monsType[type_ as usize].hp
}

#[no_mangle]
pub unsafe extern fn MonsterName(type_: u8) -> *const c_char {
    if type_ >= NUM_MONSTERS as u8 {
        cstr!("NULL")
    } else {
        monsType[type_ as usize].name.as_ptr()
    }
}

#[no_mangle]
pub unsafe extern fn MonsterAi(type_: u8) -> Option<monsterAi_t> {
    monsType.get(type_ as usize).and_then(|m| m.aiFunc)
}

#[no_mangle]
pub unsafe extern fn SetMonsterFlags(type_: u8, flags: u16) {
    monsType[type_ as usize].flags = MonsterFlags::from_bits_truncate(flags);
}

unsafe fn LoadMySprite(type_: MonsterType) {
    use ffi::win::timeGetTime;

    if type_ == MonsterType::MONS_NONE /* || type_ >= NUM_MONSTERS as u8 */ {
        return
    }

    let m = &mut monsType[type_ as usize];
    if !m.spr.is_null() { return }

    let start = timeGetTime();
    if m.sprName[0] as u8 == b'!' {
        // it's a repeat of someone else's sprite
        let v = ::libc::atoi(decay!(&m.sprName[1]));
        let m2 = &mut monsType[v as usize];
        if m2.spr.is_null() {
            m2.spr = sprite_set_t::from_fname(m2.sprName.as_ptr());
        }
        m.spr = m2.spr;
    } else {
        m.spr = sprite_set_t::from_fname(m.sprName.as_ptr());
    }
    ::game::AddGarbageTime(timeGetTime() - start);
}

#[no_mangle]
pub unsafe extern fn GetMonsterSprite(mut type_: MonsterType, seq: Animation, frm: u8, facing: u8) -> *mut sprite_t {
    if type_ == MonsterType::MONS_BOUAPHA {
        if player.weapon == Weapon::WPN_PWRARMOR {
            type_ = MonsterType::MONS_PWRBOUAPHA;
        } else if opt.playAs == PlayAs::PLAYAS_LUNATIC {
            type_ = MonsterType::MONS_DRL;
        } else if opt.playAs == PlayAs::PLAYAS_HAPPY {
            type_ = MonsterType::MONS_STICKMAN;
        }
    }

    // load if not loaded
    LoadMySprite(type_);

    let m = &monsType[type_ as usize];
    let mut v = m.anim[seq as usize][frm as usize] as c_int;
    if v == 254 {
        return ::std::ptr::null_mut(); // 254 means no sprite for this frame
    }

    if !m.flags.contains(MF_ONEFACE) {
        v += facing as c_int * m.framesPerDir as c_int;
    }

    if type_ == MonsterType::MONS_EVILCLONE ||
        (type_ == MonsterType::MONS_BOUAPHA &&
        ::player::PlayerHasHammer())
    {
        v += 8 * m.framesPerDir as c_int;
    }

    if m.flags.contains(MF_FACECMD) {
        v += facing as c_int;
    }

    (*m.spr).GetSprite(v)
}

#[no_mangle]
pub unsafe extern fn MonsterDraw(x: c_int, y: c_int, z: c_int,
    mut mons: MonsterType, seq: Animation, frm: u8,
    facing: u8, bright: i8, ouch: bool, poison: bool)
{
    use display::*;
    use FIXSHIFT;

    let isBouapha = mons == MonsterType::MONS_BOUAPHA;
    if isBouapha {
        if player.weapon == Weapon::WPN_PWRARMOR {
            mons = MonsterType::MONS_PWRBOUAPHA;
        } else if opt.playAs == PlayAs::PLAYAS_LUNATIC {
            mons = MonsterType::MONS_DRL;
        } else if opt.playAs == PlayAs::PLAYAS_HAPPY {
            mons = MonsterType::MONS_STICKMAN;
        }
    }

    // load if not loaded
    LoadMySprite(mons);

    let m = &monsType[mons as usize];
    let mut v = m.anim[seq as usize][frm as usize] as c_int;
    if v == 254 { return; } // don't draw this frame

    if !m.flags.contains(MF_ONEFACE) {
        v += facing as c_int * m.framesPerDir as c_int;
    }

    if isBouapha {
        if mons == MonsterType::MONS_BOUAPHA && ::player::PlayerHasHammer() {
            v += 8 * m.framesPerDir as c_int;
        }
        let mut shld = ::player::PlayerShield();
        if shld < 16 && (shld & 2) != 0 { // it blinks when there is 1/2 second left
            shld = 0;
        }
        if shld > 0 {
            let curSpr = (*monsType[MonsterType::MONS_BOUAPHA as usize].spr).GetSprite(464 + (shld & 7) as c_int);
            SprDraw(x >> FIXSHIFT, (y >> FIXSHIFT) + 1, 1, 255, bright, curSpr, DISPLAY_DRAWME | DISPLAY_GLOW);
        }
        let curSpr = (*m.spr).GetSprite(v); // ...return if none
        if poison {
            if !m.flags.contains(MF_NOSHADOW) {
                SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, 0, 255, 0, curSpr, DISPLAY_DRAWME | DISPLAY_SHADOW);
            }
            if !ouch {
                SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 1, bright - 4, curSpr, DISPLAY_DRAWME); // green
            } else {
                SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 5, bright, curSpr, DISPLAY_DRAWME); // yellow
            }
            return;
        } else if player.invisibility > 0 {
            SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 255, bright, curSpr, DISPLAY_DRAWME | DISPLAY_GLOW);
            return;
        }
    }

    if mons == MonsterType::MONS_EVILCLONE {
        v += 8 * m.framesPerDir as c_int;
    }
    if m.flags.contains(MF_FACECMD) {
        v += facing as c_int;
    }

    let curSpr = (*m.spr).GetSprite(v); // ...return if none
    if !m.flags.contains(MF_NOSHADOW) {
        SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, 0, 255, 0, curSpr, DISPLAY_DRAWME | DISPLAY_SHADOW);
    }

    if !ouch {
        if poison {
            SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 1, bright, curSpr, DISPLAY_DRAWME); // green
        } else if !m.flags.intersects(MF_GHOST | MF_GLOW) {
            if m.fromCol == 255 {
                SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 255, bright + m.brtChg, curSpr, DISPLAY_DRAWME);
            } else {
				SprDrawOff(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, m.fromCol, m.toCol,
						bright + m.brtChg, curSpr, DISPLAY_DRAWME);
            }
        } else if m.flags.contains(MF_GHOST) {
			SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 255, bright + m.brtChg, curSpr, DISPLAY_DRAWME | DISPLAY_GHOST);
        } else if m.flags.contains(MF_GLOW) {
			SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 255, bright + m.brtChg, curSpr, DISPLAY_DRAWME | DISPLAY_GLOW);
        }
    } else {
        if !poison {
            SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 4, bright, curSpr, DISPLAY_DRAWME); // red
        } else {
            SprDraw(x >> FIXSHIFT, y >> FIXSHIFT, z >> FIXSHIFT, 5, bright, curSpr, DISPLAY_DRAWME); // yellow
        }
    }
}

#[no_mangle]
pub unsafe extern fn InstaRenderMonster(x: c_int, y: c_int, mons: MonsterType, bright: i8, mgl: &mut MGLDraw) {
    // load if not loaded
    LoadMySprite(mons);
    let m = &monsType[mons as usize];
    let mut v = m.anim[Animation::ANIM_IDLE as usize][0] as c_int;
    if v == 254 { return }

    if !m.flags.contains(MF_ONEFACE) {
        v += 2 * m.framesPerDir as c_int;
    }

    let curSpr = (*m.spr).GetSprite(v);
    // if !curSpr { return }

    if m.fromCol == 255 {
        curSpr.DrawBright(x, y, mgl, bright + m.brtChg);
    } else {
        curSpr.DrawOffColor(x, y, mgl, m.fromCol, m.toCol, bright + m.brtChg);
    }
}
