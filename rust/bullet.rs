use libc::c_int;

#[repr(C)]
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

pub const MAX_BULLETS: c_int = 256;

bitflags! {
    /// the special hammer flags for different powerups
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
    type_: u8,
    bright: i8,
    friendly: u8,
}

extern {
    pub fn RenderBullets();
    pub fn RenderSmoke(x: c_int, y: c_int, z: c_int, bright: i8, frm: u8);
    pub fn RenderBoom(x: c_int, y: c_int, z: c_int, bright: i8, frm: u8);
    pub fn RenderStinky(x: c_int, y: c_int, z: c_int, bright: i8, frm: u8);

    pub fn FireBullet(x: c_int, y: c_int, facing: u8, type_: u8, friendly: u8);
    pub fn HammerLaunch(x: c_int, y: c_int, facing: u8, count: u8, flags: u8);
    pub fn HappyLaunch(x: c_int, y: c_int, facing: u8, count: u8, flags: u8);
}

pub unsafe fn fire_bullet(x: i32, y: i32, facing: u8, type_: Bullet, friendly: u8) {
    // TODO: replace friendly with a bool or enum
    FireBullet(x, y, facing, type_ as u8, friendly)
}
