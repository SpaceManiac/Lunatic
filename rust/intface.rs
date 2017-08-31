use libc::{c_char, c_int};
use mgldraw::MGLDraw;
use jamulspr::sprite_set_t;
use display::GetDisplayMGL;

const SPR_LIFEMETER: c_int = 0;
const SPR_IFHAMMER: c_int = 1;
const SPR_MINIGAUGE: c_int = 2;
const SPR_NUMBERS: c_int = 3;
const SPR_WEAPONS: c_int = 13;
const SPR_KEYRING: c_int = 29;
const SPR_BRAINOMETER: c_int = 40;
const SPR_RAGE: c_int = 42;
const SPR_RAGEGAUGE: c_int = 47;

const KEYRINGX: c_int = 40;
const KEYRINGY: c_int = 38;

static mut intfaceSpr_: *mut sprite_set_t = 0 as *mut _;
static mut curLife: u8 = 0;
static mut curBrains: u8 = 0;
static mut monsAlive: u8 = 0;
static mut curMonsLife: c_int = 0;

// these are for the enemy life meter
static mut monsName: [c_char; 32] = [0; 32];
static mut monsHP: c_int = 0;
static mut monsTimer: u16 = 0;

pub unsafe fn InitInterface() {
    intfaceSpr_ = sprite_set_t::load_boxed("graphics/intface.jsp");
}

pub unsafe fn ExitInterface() {
    sprite_set_t::delete(intfaceSpr_);
}

pub unsafe fn ResetInterface() {
    curLife = 0;
    monsTimer = 0;
    curBrains = 0;
}

pub unsafe fn RenderRage(size: u8, mgl: *mut MGLDraw) {
    let mgl = &mut *mgl;
    (*intfaceSpr_).GetSprite(SPR_RAGE + size as c_int).Draw(320, 240, mgl);
}

#[no_mangle]
pub unsafe extern fn ShowEnemyLife(name: *const c_char, formerLife: u8, life: u8, alive: u8) {
    curMonsLife = formerLife as c_int;
    monsHP = life as c_int;
    ::libc::strcpy(monsName.as_mut_ptr(), name);
    monsTimer = 60; // 2 seconds
    monsAlive = alive;
}

unsafe fn DrawLifeMeter(x: c_int, y: c_int, amt: u8) {
    if amt == 0 { return }
    let amt = (amt - 1) as c_int;

    let c = if amt > 64 + 16 {
        44 // in the green
    } else if amt > 32 {
        174 // in the yellow
    } else {
        140 // in the red!!
    };

    let mgl = GetDisplayMGL();
    mgl.FillBox(x, y + 1, x, y + 12, c - 6);
    mgl.FillBox(x + amt, y + 1, x + amt, y + 12, c - 6);
    if amt > 1 {
        mgl.FillBox(x + 1, y + 1, x + 1, y + 12, c - 1);
        mgl.FillBox(x + amt - 1, y + 1, x + amt - 1, y + 12, c - 1);
        if amt > 3 {
            // here's the short parts on top and bottom
            mgl.FillBox(x + 2, y, x + (amt - 2), y, c - 3);
            mgl.FillBox(x + 2, y + 13, x + (amt - 2), y + 13, c - 3);
            // and here's the normal part
            mgl.FillBox(x + 2, y + 1, x + amt - 2, y + 1, c);
            mgl.FillBox(x + 2, y + 12, x + amt - 2, y + 12, c);
            mgl.FillBox(x + 2, y + 2, x + amt - 2, y + 2, c + 8);
            mgl.FillBox(x + 2, y + 3, x + amt - 2, y + 11, c + 4);
        }
    }
}

unsafe fn DrawRageMeter(x: c_int, y: c_int, enuf: u8, amt: u8) {
    static mut blink: bool = false;
    blink = !blink;

    if amt <= 1 { return }
    let amt = amt - 1;

    let c = if enuf != 0 {
        if blink { 48 } else { 44 }
    } else {
        if blink { 140 } else { 134 }
    };

    GetDisplayMGL().FillBox(x, y, x + amt as c_int, y + 1, c);
}

unsafe fn DrawScore(mut x: c_int, y: c_int, score: c_int, mgl: &mut MGLDraw) {
    let score = ::std::cmp::max(score, 0); // I don't think you can have a negative score
    let mut j = 100_000_000; // 100 million
    for _ in 0..9 {
        let n = (score / j) % 10;
        (*intfaceSpr_).GetSprite(n + SPR_NUMBERS).Draw(x, y, mgl);
        j /= 10;
        x += 23;
    }
}

unsafe fn DrawHammerSpeed(x: c_int, y: c_int, spd: u8) {
    if spd >= 32 { return }
    let b = ((32 - spd as c_int) * 14 / 32) - 1;

    let mgl = GetDisplayMGL();
    mgl.FillBox(x + 1, y + 13 - b, x + 1, y + 13, 143);
    mgl.FillBox(x + 2, y + 13 - b, x + 2, y + 13, 141);
    if b > 2 {
        mgl.FillBox(x, y + 12 - (b - 2), x, y + 12, 138);
        mgl.FillBox(x + 3, y + 12 - (b - 2), x + 3, y + 2, 138);
    }
}

unsafe fn DrawLitGauge(x: c_int, y: c_int, c: u8) {
    let mgl = GetDisplayMGL();
    mgl.FillBox(x + 1, y, x + 1, y + 13, c + 2);
    mgl.FillBox(x + 2, y, x + 2, y + 13, c);
    mgl.FillBox(x, y + 1, x, y + 12, c - 4);
    mgl.FillBox(x + 3, y + 1, x + 3, y + 12, c - 4);
}

pub unsafe fn RenderInterface(
    life: u8, rage: u8, hmrFlags: ::bullet::HammerFlags, hammers: u8,
    brains: c_int, score: c_int, wpn: u8, ammo: c_int,
    hamSpeed: u8, mgl: &mut MGLDraw,
) {
    use display::Print;
    use player::{PlayerKeys, PlayerKeyChain};

    static mut flip: u8 = 0;
    flip = flip.wrapping_add(1);

    move_towards!(curLife, life, 4);
    move_towards!(curBrains, brains as u8, 2);

    let intfaceSpr = &mut (*intfaceSpr_);
    intfaceSpr.GetSprite(SPR_LIFEMETER).Draw(5, 3, mgl);
    DrawLifeMeter(7, 8, curLife);
    intfaceSpr.GetSprite(SPR_RAGEGAUGE).Draw(5, 3, mgl);
    DrawRageMeter(7, 29, if rage >= life { 1 } else { 0 }, rage);

    // hammer speed gauge
    intfaceSpr.GetSprite(SPR_MINIGAUGE).Draw(139, 3, mgl);
    DrawHammerSpeed(141, 8, hamSpeed * 2);

    // hammer reverse indicator
    intfaceSpr.GetSprite(SPR_MINIGAUGE).Draw(148, 3, mgl);
    if hmrFlags.contains(::bullet::HMR_REVERSE) {
        DrawLitGauge(150, 8, 112);
    }

    // hammer reflect indicator
    intfaceSpr.GetSprite(SPR_MINIGAUGE).Draw(157, 3, mgl);
    if hmrFlags.contains(::bullet::HMR_REFLECT) {
        DrawLitGauge(159, 8, 175);
    }

    // number of hammers
    for i in 0..(hammers as c_int) {
        intfaceSpr.GetSprite(SPR_IFHAMMER).Draw(167 + i * 19, 3, mgl);
    }

    DrawScore(432, 2, score, mgl);

    // Enemy life gauge
    if monsTimer > 0 {
        monsTimer -= 1;
        move_towards!(curMonsLife, monsHP, 4);

        intfaceSpr.GetSprite(SPR_LIFEMETER).Draw(6, 453, mgl);
        DrawLifeMeter(8, 458, curMonsLife as u8);
        // if the monster is dead, the name blinks
        Print(11, 461, monsName.as_ptr(), 1, 1);
        if monsAlive != 0 || (flip & 2) == 0 {
            Print(10, 460, monsName.as_ptr(), 0, 1);
        }
    }

    // secondary weapons
    if wpn != 0 {
        intfaceSpr.GetSprite(SPR_WEAPONS - 1 + wpn as c_int).Draw(595, 30, mgl);
        let mut s = [0; 6];
        if wpn == ::player::Weapon::WPN_PWRARMOR as u8 {
            sprintf!(s, "{:3}", ammo / 10);
            Print(601, 61, s.as_ptr() as *const _, 1, 1);
            Print(600, 60, s.as_ptr() as *const _, 0, 1);
        } else {
            sprintf!(s, "{:02}", ammo);
            Print(621, 61, s.as_ptr() as *const _, 1, 1);
            Print(620, 60, s.as_ptr() as *const _, 0, 1);
        }
    }

    // the almighty keyring
    macro_rules! keych {
        ($c:expr, $p:expr) => {
            if $c { intfaceSpr.GetSprite(SPR_KEYRING + $p).Draw(KEYRINGX, KEYRINGY, mgl) }
        }
    }
    keych!(true, 0);
    keych!(PlayerKeys(3) != 0, 1);
    keych!(PlayerKeys(1) != 0, 2);
    keych!(PlayerKeys(2) != 0, 3);
    let i = PlayerKeys(0);
    keych!(i > 0, 4);
    keych!(i > 1, 5);
    keych!(i > 2, 6);
    keych!(PlayerKeyChain(0), 7);
    keych!(PlayerKeyChain(1), 8);
    keych!(PlayerKeyChain(2), 9);
    keych!(PlayerKeyChain(3), 10);

    // the brainometer
    intfaceSpr.GetSprite(SPR_BRAINOMETER).Draw(617, 342, mgl);
    if curBrains != 0 {
        mgl.FillBox(620, 347 + 127 - (curBrains as c_int - 1), 635, 347 + 127, 96 + 13 + (curBrains / 8));
    }
    intfaceSpr.GetSprite(SPR_BRAINOMETER + 1).Draw(617, 342, mgl);
}
