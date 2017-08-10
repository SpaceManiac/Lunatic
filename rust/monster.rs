/* this file does the AI and everything else for each monster type.
   It knows all about the different types, as opposed to guy.cpp which
   just sort of keeps track of the list of guys. */

use libc::c_int;
use jamulspr::sprite_t;

/// the monster types
#[repr(u8)]
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

pub const NUM_MONSTERS: c_int = 161;
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

pub const ANIM_LENGTH: usize = 24;

// TODO: flags

extern {
    pub fn GetMonsterSprite(type_: MonsterType, seq: Animation, frm: u8, facing: u8) -> *mut sprite_t;
}
