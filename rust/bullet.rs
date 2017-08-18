#![allow(dead_code)]

use libc::c_int;
use jamulspr::sprite_set_t;
use map::Map;
use world::world_t;

#[repr(u8)]
pub enum Bullet {
    BLT_NONE = 0,
    BLT_HAMMER = 1,
    /// this is a hammer with reflection
    BLT_HAMMER2 = 2,
    BLT_MISSILE = 3,
    BLT_FLAME = 4,
    BLT_LASER = 5,
    BLT_ACID = 6,
    BLT_BOMB = 7,
    BLT_BOOM = 8,
    BLT_ENERGY = 9,
    /// this is a ball of energy that launches off a megabeam1
    BLT_MEGABEAM = 10,
    /// this is a huge laser beam (downward)
    BLT_MEGABEAM1 = 11,
    /// this is the laser hitting an object (just a visual effect)
    BLT_MEGABEAM2 = 12,
    /// just like flame, except it is anti-Bouapha
    BLT_FLAME2 = 13,
    BLT_SPORE = 14,
    BLT_SHROOM = 15,
    /// energy grenade, an enemy weapon
    BLT_GRENADE = 16,
    /// yellow explosion made by energy grenade
    BLT_YELBOOM = 17,
    /// purple shockwave, for Super Zombie stomp
    BLT_SHOCKWAVE = 18,
    /// explosion made by missile
    BLT_LILBOOM = 19,
    BLT_SNOWBALL = 20,
    BLT_BIGSNOW = 21,
    /// spike juts out of the ground
    BLT_ICESPIKE = 22,
    BLT_ROCK = 23,
    /// cactus spine
    BLT_SPINE = 24,
    /// a grey hammer that is anti-bouapha
    BLT_EVILHAMMER = 25,
    /// Bouapha's power armor shoots these
    BLT_BIGSHELL = 26,
    /// Bouapha weapon
    BLT_BIGAXE = 27,
    /// Bouapha weapon
    BLT_LIGHTNING = 28,
    /// Bouapha's version of the pygmy spear
    BLT_SPEAR = 29,
    /// Bouapha's machete slash
    BLT_SLASH = 30,
    /// Bouapha's mines
    BLT_MINE = 31,
    /// pygmy-thrown spear
    BLT_BADSPEAR = 32,
    /// guy that flies around Bouapha shooting
    BLT_ORBITER = 33,
    /// friendly green bullets
    BLT_GREEN = 34,
    BLT_BALLLIGHTNING = 35,
    BLT_LIGHTNING2 = 36,
    BLT_MINDWIPE = 37,
    BLT_REFLECT = 38,
    BLT_SWAP = 39,
    BLT_SHARK = 40
}

pub const MAX_BULLETS: usize = 256;

bitflags! {
    /// the special hammer flags for different powerups
    #[repr(C)]
    pub struct HammerFlags: u8 {
        const HMR_REVERSE = 1;
        const HMR_REFLECT = 2;
        const HMR_WATERWALK = 4;
    }
}

#[repr(C)]
pub struct bullet_t {
    x: c_int,
    y: c_int,
    z: c_int,
    dx: c_int,
    dy: c_int,
    dz: c_int,
    timer: c_int,
    target: u16,
    anim: u8,
    facing: u8,
    type_: Bullet,
    bright: i8,
    friendly: u8,
}

extern {
    pub fn RenderBullets();
    pub fn UpdateBullets(map: &mut Map, world: &mut world_t);

    pub fn FireBullet(x: c_int, y: c_int, facing: u8, type_: u8, friendly: u8);
    pub fn HammerLaunch(x: c_int, y: c_int, facing: u8, count: u8, flags: u8);
    pub fn HappyLaunch(x: c_int, y: c_int, facing: u8, count: u8, flags: u8);

    static mut bullet: [bullet_t; MAX_BULLETS];
    static mut bulletSpr: *mut sprite_set_t;
    static mut reflect: u8;
}

const SPR_FLAME: c_int = 0;
const SPR_LASER: c_int = 5;
const SPR_HAMMER: c_int = 21;
const SPR_MISSILE: c_int = 149;
const SPR_SMOKE: c_int = 165;
const SPR_ACID: c_int = 172;
const SPR_BOMB: c_int = 228;
const SPR_ENERGY: c_int = 236;
const SPR_BOOM: c_int = 238;
const SPR_MEGABEAM: c_int = 246;
const SPR_SPORE: c_int = 254;
const SPR_SHROOM: c_int = 258;
const SPR_GRENADE: c_int = 266;
const SPR_YELBOOM: c_int = 268;
const SPR_SHOCKWAVE: c_int = 273;
const SPR_LILBOOM: c_int = 277;
const SPR_SNOWBALL: c_int = 282;
const SPR_BIGSNOW: c_int = 283;
const SPR_ICESPIKE: c_int = 286;
const SPR_ROCK: c_int = 290;
const SPR_SPINE: c_int = 294;
const SPR_BIGAXE: c_int = 310;
const SPR_SPEAR: c_int = 318;
const SPR_SLASH: c_int = 326;
const SPR_MINE: c_int = 350;
const SPR_STINKY: c_int = 355;
const SPR_GREEN: c_int = 358;
const SPR_ORBITER: c_int = 359;

pub unsafe fn fire_bullet(x: i32, y: i32, facing: u8, type_: Bullet, friendly: u8) {
    // TODO: replace friendly with a bool or enum
    FireBullet(x, y, facing, type_ as u8, friendly)
}

pub unsafe fn InitBullets() {
    bulletSpr = sprite_set_t::from_fname(cstr!("graphics/bullets.jsp"));
    for b in bullet.iter_mut() {
        *b = ::std::mem::zeroed();
    }
}

pub unsafe fn ExitBullets() {
    sprite_set_t::delete(bulletSpr);
}

#[no_mangle]
pub unsafe extern fn Bulletable(map: &Map, x: c_int, y: c_int) -> bool {
    let tile = map.get_tile(x, y);
    !(tile.wall != 0 ||
        (tile.item >= ::items::MAX_SHOOTABLE_ITMS as u8 && tile.item < ::items::NEW_PICKUP_ITMS as u8))
}

#[no_mangle]
pub unsafe extern fn OffScreenBulletDie(me: &mut bullet_t, map: &Map) {
    if me.x < 0 || me.y < 0 ||
        me.x >= map.width * ::tile::TILE_WIDTH * ::FIXAMT ||
        me.y >= map.height * ::tile::TILE_HEIGHT * ::FIXAMT
    {
        me.type_ = Bullet::BLT_NONE;
    }
}

#[no_mangle]
pub unsafe extern fn RenderSmoke(x: c_int, y: c_int, z: c_int, bright: i8, frm: u8) {
    ::display::SprDraw(x, y, z, 255, bright - 64,
        (*bulletSpr).GetSprite(SPR_SMOKE + frm as c_int),
        ::display::DISPLAY_DRAWME | ::display::DISPLAY_GHOST);
}

#[no_mangle]
pub unsafe extern fn RenderBoom(x: c_int, y: c_int, z: c_int, bright: i8, frm: u8) {
    ::display::SprDraw(x, y, z, 255, bright - 64,
        (*bulletSpr).GetSprite(SPR_BOOM + frm as c_int),
        ::display::DISPLAY_DRAWME | ::display::DISPLAY_GLOW);
}

#[no_mangle]
pub unsafe extern fn RenderStinky(x: c_int, y: c_int, z: c_int, bright: i8, frm: u8) {
    ::display::SprDraw(x, y, z, 255, bright - 64,
        (*bulletSpr).GetSprite(SPR_STINKY + frm as c_int),
        ::display::DISPLAY_DRAWME | ::display::DISPLAY_GHOST);
}
