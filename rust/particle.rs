use libc::{c_int, c_char};
use mgldraw::{MGL_random, MGL_randoml};
use cossin::{Sine, Cosine};
use map::Map;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ParticleType {
    None = 0,
    /// speedy snow particles, just like hammer particles but white
    Snow2,
    Snow,
    Dirt,
    Hammer,
    Slime,
    Smoke,
    Boom,
    Water,
    Lightning,
    /// stinky lines for garlic
    Stinky,
    /// multicolored stained glass
    Glass,
    /// glowing image of the countess for when she charges
    Countess,
}

#[repr(C)]
#[derive(Clone)]
pub struct Particle {
    pub size: u8,
    pub type_: ParticleType,
    pub color: u8,
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
    pub dx: c_int,
    pub dy: c_int,
    pub dz: c_int,
    pub life: c_int,
}

impl Particle {
    pub fn new() -> Particle {
        Particle {
            size: 0, type_: ParticleType::None, color: 0,
            x: 0, y: 0, z: 0,
            dx: 0, dy: 0, dz: 0,
            life: 0,
        }
    }

    pub fn Alive(&self) -> bool {
        self.life > 0
    }

    pub fn Go(&mut self, type_: ParticleType, x: c_int, y: c_int, z: c_int, angle: u8, force: u8) {
        if force == 0 { return }

        self.type_ = type_;
        self.size = 2;
        let fforce = ::std::cmp::max(1, force as c_int / 4);

        self.x = x + MGL_randoml(32 << ::FIXSHIFT) - (16 << ::FIXSHIFT);
        self.y = y + MGL_randoml(32 << ::FIXSHIFT) - (16 << ::FIXSHIFT);
        self.z = z;
        self.dx = Cosine(angle as c_int) * MGL_random(fforce);
        self.dy = Sine(angle as c_int) * MGL_random(fforce);
        self.dz = MGL_random(fforce * 2) << ::FIXSHIFT;
        self.life = MGL_random(force as c_int) + 10;
    }

    pub fn GoLightning(&mut self, x: c_int, y: c_int, x2: c_int, y2: c_int) {
        self.type_ = ParticleType::Lightning;
        self.x = x;
        self.y = y;
        self.dx = x2;
        self.dy = y2;
        self.life = 4;
        self.color = 64;
        self.size = ((((x2 - x) >> ::FIXSHIFT).abs() + ((y2 - y) >> ::FIXSHIFT).abs()) / 8) as u8;
    }

    pub fn GoRandom(&mut self, type_: ParticleType, x: c_int, y: c_int, z: c_int, force: u8) {
        self.type_ = type_;
        self.size = 2;
        if force == 0 { return }
        let force = force as c_int;

        self.x = x + MGL_randoml(32 << ::FIXSHIFT) - (16 << ::FIXSHIFT);
        self.y = y + MGL_randoml(32 << ::FIXSHIFT) - (16 << ::FIXSHIFT);
        self.z = z;
        self.dx = (MGL_random(force) - force / 2) << ::FIXSHIFT;
        self.dz = (MGL_random(force) - force / 2) << ::FIXSHIFT;
        self.dz = MGL_random(force * 2) << ::FIXSHIFT;
        self.life = MGL_random(force) + 20;
    }

    pub unsafe fn Update(&mut self, map: &mut ::map::Map) {
        use FIXAMT;
        use cossin::Dampen;
        use tile::{TILE_WIDTH, TILE_HEIGHT};
        use std::cmp::{min, max};

        if self.life <= 0 { return }
        self.life -= 1;

        if self.type_ != ParticleType::Lightning {
            self.dz -= FIXAMT;
            self.x += self.dx;
            self.y += self.dy;
            self.z += self.dz;
            if self.z < 0 {
                self.z = 0;
                self.dz = -self.dz / 2;
            }
        }

        match self.type_ {
            ParticleType::None => {}
            ParticleType::Countess => {
                self.dz += FIXAMT; // no gravity
            }
            ParticleType::Smoke => {
                self.dz += FIXAMT; // no gravity
                self.z += FIXAMT;
                self.size = (6 - self.life / 8) as u8;
                self.dx += MGL_random(65535) - FIXAMT / 2;
                self.dy += MGL_random(65535) - FIXAMT / 2;
                Dampen(&mut self.dx, FIXAMT / 8);
                Dampen(&mut self.dy, FIXAMT / 8);
            }
            ParticleType::Stinky => {
                self.dz += FIXAMT; // no gravity
                self.z += FIXAMT + FIXAMT / 2;
                self.size = ((self.life / 2) & 3) as u8;
                if self.size == 3 { self.size = 1; }
                self.dx += MGL_random(65535) - FIXAMT / 2;
                self.dy += MGL_random(65535) - FIXAMT / 2;
                Dampen(&mut self.dx, FIXAMT / 8);
                Dampen(&mut self.dy, FIXAMT / 8);
            }
            ParticleType::Boom => {
                self.dz += FIXAMT;
                self.z += FIXAMT;
                self.size = (7 - self.life) as u8;
            }
            ParticleType::Hammer => {
                self.color = 128 + min(self.life, 31 - 8) as u8;
                self.size = life_to_size(self.life);
            }
            ParticleType::Glass => {
                self.size = life_to_size(self.life);
            }
            ParticleType::Dirt => {
                self.color = 64 + min(self.life, 31 - 8) as u8;
                self.size = life_to_size(self.life);
            }
            ParticleType::Snow2 => {
                self.color = max(31 - 16, min(31, self.life * 2)) as u8;
                self.size = life_to_size(self.life);
            }
            ParticleType::Water => {
                self.color = 96 + max(8, min(31, self.life)) as u8;
                self.size = life_to_size(self.life);
            }
            ParticleType::Slime => {
                self.color = 32 + 4 + min(self.life, 31 - 8) as u8;
                self.size = life_to_size(self.life / 2);
            }
            ParticleType::Snow => {
                self.dx += MGL_random(65535) - FIXAMT / 2;
                self.dy += MGL_random(65535) - FIXAMT / 2;
                Dampen(&mut self.dx, FIXAMT / 8);
                Dampen(&mut self.dy, FIXAMT / 8);
                self.dz += FIXAMT - 256; // not as much gravity as other things
                self.color = 31;
                if self.z == 0 {
                    self.dx = 0;
                    self.dy = 0;
                    self.dz = 0;
                    if self.life < 25 {
                        self.size = 0;
                    } else if self.life < 50 {
                        self.size = 1;
                    }
                    self.color = max(26, min(31, self.life / 2)) as u8;
                    snowCount += 1;
                } else {
                    self.life += 1; // can't die while airborne
                }
            }
            ParticleType::Lightning => {
                self.color /= 2; // get dimmer with each frame
            }
        }

        if self.x < 0 || self.y < 0 ||
            self.x >= (map.width * TILE_WIDTH) << ::FIXSHIFT ||
            self.y >= (map.height * TILE_HEIGHT) << ::FIXSHIFT
        {
            self.life = 0;
            return;
        }

        match self.type_ {
            ParticleType::Smoke | ParticleType::Boom | ParticleType::Stinky => {
                let mapx = (self.x / TILE_WIDTH) >> ::FIXSHIFT;
                let mapy = (self.y / TILE_HEIGHT) >> ::FIXSHIFT;
                let brt = map.get_tile(mapx, mapy).templight;
                self.color = 64u8.wrapping_add(brt as u8);
            }
            ParticleType::Lightning | ParticleType::Glass => {
                // nothing to do
            }
            _ => {
                let mapx = (self.x / TILE_WIDTH) >> ::FIXSHIFT;
                let mapy = (self.y / TILE_HEIGHT) >> ::FIXSHIFT;

                // brighten it appropriately
                let brt = map.get_tile(mapx, mapy).templight;
                let c1 = self.color & !31; // c1 is the color range
                self.color = max(c1, min(c1 + 31, self.color.wrapping_add(brt as u8)));
            }
        }
    }
}

fn life_to_size(life: c_int) -> u8 {
    if life > 20 { 2 } else if life < 10 { 0 } else { 1 }
}

static mut snowCount: c_int = 0;
static mut particleList: *mut *mut Particle = 1 as *mut *mut Particle;
static mut maxParticles: c_int = 0;

#[no_mangle]
pub unsafe extern fn InitParticles(max: c_int) {
    maxParticles = max;

    let mut vec = vec![Box::new(Particle::new()); max as usize];
    particleList = vec.as_mut_ptr() as *mut *mut Particle;
    ::std::mem::forget(vec);
}

#[no_mangle]
pub unsafe extern fn ExitParticles() {
    let len = maxParticles as usize;
    Vec::from_raw_parts(particleList as *mut Box<Particle>, len, len);
}

#[no_mangle]
pub unsafe extern fn UpdateParticles(map: &mut Map) {
    snowCount = 0;
    for particle in particle_list() {
        particle.Update(map);
    }
}

pub unsafe fn RenderParticle(x: c_int, y: c_int, scrn: &mut [u8], color: u8, size: u8) {
    use std::cmp::max;

    let mut scrn = scrn.as_mut_ptr();

    macro_rules! incr {
        ($by:expr) => { scrn = scrn.offset($by) }
    }

    if x < 0 || x > 639 || y < 0 || y > 479 { return }

    match size {
        2 => { // big particle
            if x < 2 || x > 637 || y < 2 || y > 477 { return }

            let c1 = max(color.saturating_sub(2), color & !31); // subtract 2, but only within the same color group
            let c2 = max(c1.saturating_sub(2), c1 & !31); // subtract 2, but only within the same color group

            incr!((x + (y - 2) * 640) as isize);
            *scrn = c2; incr!(639);

            *scrn = c1; incr!(1);
            *scrn = color; incr!(1);
            *scrn = c1; incr!(637);

            *scrn = c2; incr!(1);
            *scrn = c1; incr!(1);
            *scrn = color; incr!(1);
            *scrn = c1; incr!(1);
            *scrn = c2; incr!(637);

            *scrn = c1; incr!(1);
            *scrn = color; incr!(1);
            *scrn = c1; incr!(639);

            *scrn = c2;
        }
        1 => { // normal particle
            if x < 1 || x > 638 || y < 1 || y > 478 { return }

            let c1 = max(color.saturating_sub(2), color & !31); // subtract 2, but only within the same color group

            incr!((x + (y - 1) * 640) as isize);
            *scrn = c1; incr!(639);

            *scrn = c1; incr!(1);
            *scrn = color; incr!(1);
            *scrn = c1; incr!(637);

            *scrn = c1;
        }
        0 => { // tiny particle (1 pixel)
            *scrn.offset((x + y * 640) as isize) = color;
        }
        _ => {}
    }
}

// this was going to be local to renderlightningparticle, but that would've seriously
// chomped up the stack, since that function's recursive.
static ctab: [u8; 25] = [
    8, 3, 2, 3, 8,
    3, 2, 1, 2, 3,
    2, 1, 0, 1, 2,
    3, 2, 1, 2, 3,
    8, 3, 2, 3, 8
];

pub unsafe fn RenderLightningParticle(
    x1: c_int, y1: c_int, x2: c_int, y2: c_int,
    range: c_int, bright: u8, scrn_: &mut [u8],
) {
    let mut scrn = scrn_.as_mut_ptr();
    // base case: draw the (x1, y1) pixel
    if x1 - x2 < 2 && x1 - x2 > -2 && y1 - y2 < 2 && y1 - y2 > -2 {
        if x1 >= 0 && x1 < 635 && y1 >= 0 && y1 < 475 {
            scrn = scrn.offset((x1 + y1 * 640) as isize);
            let mut ctidx = 0;
            for _ in 0..5 { // midy in y1..y1 + 5
                for _ in 0..5 { // midx in x1..x1 + 5
                    let b = *scrn;
                    let brt = bright.checked_shr(ctab[ctidx] as u32).unwrap_or(0);
                    if ((b + brt) & !31) != (b & !31) {
                        *scrn = (b & !31) + 31;
                    } else {
                        *scrn = b + brt;
                    }
                    ctidx += 1;
                    scrn = scrn.offset(1);
                }
                scrn = scrn.offset(640 - 5);
            }
        }
    } else {
        // recursive case, find a (midx,midy) between the other two points
        let range = ::std::cmp::max(1, range);
        let midx = (x1 + x2) / 2 + MGL_random(range) - range / 2;
        let midy = (y1 + y2) / 2 + MGL_random(range) - range / 2;
        RenderLightningParticle(x1, y1, midx, midy, range * 3 / 4, bright, scrn_);
        RenderLightningParticle(midx, midy, x2, y2, range * 3 / 4, bright, scrn_);
    }
}

#[no_mangle]
pub unsafe extern fn RenderParticles() {
    for p in particle_list() {
        use display::*;
        use bullet::{RenderSmoke, RenderBoom, RenderStinky};

        if !p.Alive() { continue }
        match p.type_ {
            ParticleType::Smoke => RenderSmoke(
                p.x >> ::FIXSHIFT, p.y >> ::FIXSHIFT, p.z >> ::FIXSHIFT,
                p.color as c_char, p.size),
            ParticleType::Boom => RenderBoom(
                p.x >> ::FIXSHIFT, p.y >> ::FIXSHIFT, p.z >> ::FIXSHIFT,
                p.color as c_char, p.size),
            ParticleType::Lightning => LightningDraw(
                p.x >> ::FIXSHIFT, p.y >> ::FIXSHIFT,
                p.dx >> ::FIXSHIFT, p.dy >> ::FIXSHIFT,
                p.color, p.size as c_char),
            ParticleType::Stinky => RenderStinky(
                p.x >> ::FIXSHIFT, p.y >> ::FIXSHIFT, p.z >> ::FIXSHIFT,
                p.color as c_char, p.size),
            ParticleType::Countess => SprDraw(
                p.x >> ::FIXSHIFT, p.y >> ::FIXSHIFT, p.z >> ::FIXSHIFT,
                255, (p.life * 4 - 8) as i8,
                ::monster::GetMonsterSprite(::monster::MonsterType::MONS_COUNTESS, ::monster::Animation::ANIM_IDLE, 0, 0),
                DISPLAY_DRAWME | DISPLAY_GLOW),
            _ => ParticleDraw(
                p.x >> ::FIXSHIFT, p.y >> ::FIXSHIFT, p.z >> ::FIXSHIFT,
                p.color, p.size, DISPLAY_DRAWME | DISPLAY_PARTICLE),
        }
    }
}

#[no_mangle]
pub unsafe extern fn BlowSmoke(x: c_int, y: c_int, z: c_int, dz: c_int) {
    make_particle(1, |p| {
        p.x = x;
        p.y = y;
        p.z = z;
        p.dx = 0;
        p.dy = 0;
        p.dz = dz;
        p.life = 6 * 4 - MGL_random(8);
        p.size = 6;
        p.color = 64;
        p.type_ = ParticleType::Smoke;
    })
}

#[no_mangle]
pub unsafe extern fn StinkySteam(x: c_int, y: c_int, z: c_int, dz: c_int) {
    make_particle(1, |p| {
        p.x = x;
        p.y = y;
        p.z = z;
        p.dx = 0;
        p.dy = 0;
        p.dz = dz;
        p.life = 6 * 4 - MGL_random(8);
        p.size = 0;
        p.color = 64;
        p.type_ = ParticleType::Stinky;
    })
}

#[no_mangle]
pub unsafe extern fn CountessGlow(x: c_int, y: c_int) {
    make_particle(1, |p| {
        p.x = x;
        p.y = y;
        p.z = 0;
        p.dx = 0;
        p.dy = 0;
        p.dz = 0;
        p.life = 4;
        p.size = 0;
        p.color = 64;
        p.type_ = ParticleType::Countess;
    })
}

#[no_mangle]
pub unsafe extern fn BlowUpGuy(x: c_int, y: c_int, x2: c_int, y2: c_int, z: c_int, amt: u8) {
    make_particle(amt, |p| {
        p.x = (x + MGL_randoml(x2 - x)) << ::FIXSHIFT;
        p.y = (y + MGL_randoml(y2 - y)) << ::FIXSHIFT;
        p.z = z;
        p.dx = 0;
        p.dy = 0;
        p.dz = 0;
        p.life = 7;
        p.size = 0;
        p.color = 64;
        p.type_ = ParticleType::Boom;
        ::sound::make_sound(::sound::Sound::SND_BOMBBOOM, p.x, p.y, ::sound::SND_CUTOFF, 1800);
    })
}

#[no_mangle]
pub unsafe extern fn GlassShatter(x: c_int, y: c_int, x2: c_int, y2: c_int, z: c_int, amt: u8) {
    let _ = z;
    make_particle(amt, |p| {
        p.GoRandom(
            ParticleType::Glass,
            (x + MGL_randoml(x2 - x)) << ::FIXSHIFT,
            (y + MGL_randoml(y2 - y)) << ::FIXSHIFT,
            MGL_randoml(10 * ::FIXAMT), // maybe should be "z"?
            20,
        );
        p.color = MGL_random(8) as u8 * 32 + 16;
    })
}

#[no_mangle]
pub unsafe extern fn SpurtParticles(type_: ParticleType, left: bool, mut x: c_int, mut y: c_int, z: c_int, angle: u8, force: u8) {
    x += Cosine(angle as c_int) * 10;
    y += Sine(angle as c_int) * 10;
    let ang2 = angle.wrapping_add(if left { 64 } else { 128 + 64 }) as c_int;
    x += Cosine(ang2) * 20;
    y += Sine(ang2) * 20;

    make_particle(force, |p| p.Go(type_, x, y, z, angle, force));
}

#[no_mangle]
pub unsafe extern fn ExplodeParticles(type_: ParticleType, x: c_int, y: c_int, z: c_int, force: u8) {
    make_particle(force, |p| p.GoRandom(type_, x, y, z, force));
}

#[no_mangle]
pub unsafe extern fn ExplodeParticles2(type_: ParticleType, x: c_int, y: c_int, z: c_int, num: u8, force: u8) {
    make_particle(num, |p| p.GoRandom(type_, x, y, z, force));
}

#[no_mangle]
pub unsafe extern fn MakeItSnow(_: &mut Map) {
    // only 25% of particles may be snowflakes
    if MGL_random(100) > 30 || snowCount > maxParticles / 4 {
        return;
    }

    let (mut cx, mut cy) = ::display::get_camera();
    cx -= 320;
    cy -= 240;
    make_particle(1, |p| {
        p.x = (MGL_random(640) + cx) << ::FIXSHIFT;
        p.y = (MGL_random(480) + cy) << ::FIXSHIFT;
        p.z = (300 + MGL_random(300)) << ::FIXSHIFT;
        p.dx = 0;
        p.dy = 0;
        p.dz = 0;
        p.size = 2;
        p.life = 50 + MGL_random(50);
        p.type_ = ParticleType::Snow;
        p.color = 31;
    })
}

#[no_mangle]
pub unsafe extern fn SpecialSnow(x: c_int, y: c_int) {
    make_particle(1, |p| {
        p.x = x;
        p.y = y;
        p.z = (10 + MGL_random(20)) << ::FIXSHIFT;
        p.dx = 0;
        p.dy = 0;
        p.dz = 0;
        p.size = 2;
        p.life = 20 + MGL_random(30);
        p.type_ = ParticleType::Snow;
    })
}

#[no_mangle]
pub unsafe extern fn LightningBolt(x: c_int, y: c_int, x2: c_int, y2: c_int) {
    make_particle(1, |p| p.GoLightning(x, y, x2, y2))
}

unsafe fn make_particle<F: FnMut(&mut Particle)>(mut num: u8, mut f: F) {
    for p in particle_list() {
        if !p.Alive() {
            f(p);
            num -= 1;
            if num == 0 { break }
        }
    }
}

unsafe fn particle_list<'a>() -> &'a mut [&'a mut Particle] {
    ::std::slice::from_raw_parts_mut(particleList as *mut _, maxParticles as usize)
}
