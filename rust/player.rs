use libc::{c_int, c_char, strncpy, fseek, fread, fwrite, fclose};
use world::MAX_MAPS;
use mgldraw::MGLDraw;
use bullet::HammerFlags;
use guy::{Guy, goodguy};

/// secondary weapon defines
#[repr(u8)]
#[derive(FromInt, Eq, PartialEq, Copy, Clone)]
pub enum Weapon {
    WPN_NONE,
    WPN_MISSILES,
    WPN_AK8087,
    WPN_BOMBS,
    WPN_FLAME,
    WPN_PWRARMOR,
    WPN_BIGAXE,
    WPN_LIGHTNING,
    WPN_SPEAR,
    WPN_MACHETE,
    WPN_MINES,
    WPN_TURRET,
    WPN_MINDCONTROL,
    WPN_REFLECTOR,
    WPN_JETPACK,
    WPN_SWAPGUN,
}

/// initializing constants (pass to InitPlayer)
#[repr(C)]
#[derive(PartialEq, PartialOrd)]
pub enum Init {
    Game = 2,
    World = 1,
    Level = 0,
}

/// vehicles you could be on
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Vehicle {
    None = 0,
    Minecart = 1,
    Raft = 2,
}

/// the most custom worlds it will handle
pub const MAX_CUSTOM: usize = 128;

#[repr(C)]
#[derive(Copy)]
pub struct player_t {
    // values for the overall game
    pub musicSettings: ::options::Music,
    /// so you can lose all your points when you die
    pub prevScore: c_int,
    pub score: c_int,
    pub levelPassed: [[u8; MAX_MAPS]; MAX_CUSTOM],
    pub keychain: [[u8; 4]; MAX_CUSTOM],
    /// total completion is how many "points" the world has in it
    pub totalCompletion: [c_int; MAX_CUSTOM],
    /// complete is how many of those points the player has, to create a percentage complete display
    pub complete: [c_int; MAX_CUSTOM],
    pub customName: [[c_char; 32]; MAX_CUSTOM],
    pub lunacyKey: [u8; MAX_CUSTOM],

    // values reset for each world
    pub levelsPassed: u8,
    pub worldNum: u8,

    // values reset for each level
    pub shield: u8,
    pub levelNum: u8,
    pub keys: [u8; 4],
    pub boredom: c_int,
    pub hammers: u8,
    pub hamSpeed: u8,
    pub weapon: Weapon,
    pub ammo: c_int,
    pub reload: u8,
    pub wpnReload: u8,
    pub life: u8,
    pub brains: c_int,
    /// for pushing pushy blocks
    pub pushPower: u8,
    pub hammerFlags: HammerFlags,
    pub vehicle: Vehicle,
    pub garlic: u8,
    /// accelerated
    pub speed: u8,
    pub rageClock: u8,
    pub rage: u16,
    pub invisibility: u8,
    pub jetting: u8,
}

impl Clone for player_t {
    fn clone(&self) -> Self {
        *self
    }
}

extern {
    pub static mut player: player_t;
    static mut playerGlow: u8; // for torch-lit levels, and for exciting moments
    static mut tportclock: u8;

    pub fn PlayerGetItem(itm: ::items::Item, x: c_int, y: c_int) -> u8;
}

#[no_mangle]
pub unsafe extern fn InitPlayer(initWhat: Init, world: u8, level: u8) {
    let wrldName = cstr!["caverns.dlw", "icymount.dlw", "forest.dlw", "desert.dlw", "asylum.dlw"];

    if initWhat == Init::Game { // initialize everything, this is to start a whole new game
        player.score = 0;
        for i in 0..MAX_CUSTOM {
            for j in 0..MAX_MAPS {
                player.levelPassed[i][j] = 0;
            }
            for j in 0..4 {
                player.keychain[i][j] = 0;
            }
            player.totalCompletion[i] = 100;
            player.complete[i] = 0;
            player.lunacyKey[i] = 0;
            if i > 4 {
                player.customName[i][0] = 0;
            } else {
                strncpy(player.customName[i].as_mut_ptr(), wrldName[i], 32);
            }
        }
        ::title::ScanWorldNames();
        player.totalCompletion[0] = ::world::GetWorldPoints(cstr!("caverns.dlw"));
    }
    if initWhat >= Init::World { // initialize the things that go with each world
        player.levelsPassed = 0;
        player.worldNum = world;
    }

    player.levelNum = level;
    player.prevScore = player.score; // back up the score (if you give up or die, it is reset)

    for i in 0..4 {
        player.keys[i] = 0;
    }

    player.brains = 0;
    player.boredom = 0;
    player.hammers = 0;
    player.hamSpeed = 16;
    player.weapon = Weapon::WPN_NONE;
    player.ammo = 0;
    player.reload = 10;
    player.wpnReload = 10;
    player.hammerFlags = ::bullet::HammerFlags::empty();
    player.life = 128;
    player.shield = 0;
    playerGlow = 0;
    player.pushPower = 0;
    player.vehicle = Vehicle::None;
    player.garlic = 0;
    player.speed = 0;
    player.rageClock = 0;
    player.rage = 0;
    player.invisibility = 0;
    player.jetting = 0;

    player.musicSettings = ::options::opt.music;
    if ::music::CDLoaded() == 0 {
        player.musicSettings = ::options::Music::Off;
    }
}

#[no_mangle]
pub unsafe extern fn ExitPlayer() {
}

#[no_mangle]
pub unsafe extern fn PlayerLoadGame(which: u8) {
    let f = ::mgldraw::AppdataOpen(cstr!("loony.sav"), cstr!("rb"));
    if f.is_null() {
        InitPlayer(Init::Game, 0, 0);
    } else {
        fseek(f, which as i32 * szof!(player_t) as i32, ::libc::SEEK_SET);
        fread(decay!(&mut player), szof!(player_t), 1, f);
        fclose(f);
    }
}

#[no_mangle]
pub unsafe extern fn PlayerSaveGame(which: u8) {
    use mgldraw::AppdataOpen;

    let mut p: [player_t; 3] = ::std::mem::zeroed();

    let mut f = AppdataOpen(cstr!("loony.sav"), cstr!("rb"));
    if f.is_null() {
        for i in 0..5 {
            for j in 0..3 {
                p[j].totalCompletion[i] = 100;
            }
        }
        f = AppdataOpen(cstr!("loony.sav"), cstr!("wb"));
        fwrite(decay!(&p), szof!(player_t), 3, f);
        fclose(f);
        f = AppdataOpen(cstr!("loony.sav"), cstr!("rb"));
    }
    fread(decay!(&mut p), szof!(player_t), 3, f);
    fclose(f);
    p[which as usize] = player;
    f = AppdataOpen(cstr!("loony.sav"), cstr!("wb"));
    fwrite(decay!(&p), szof!(player_t), 3, f);
    fclose(f);
}

#[no_mangle]
pub unsafe extern fn PlayerSetWorldWorth(world: u8, amt: c_int) {
    player.totalCompletion[world as usize] = amt;
}

#[no_mangle]
pub unsafe extern fn PlayerRenderInterface(mgl: &mut MGLDraw) {
    let mut b = ::map::TotalBrains();
    if b != 0 {
        b = 128 - (player.brains * 128 / b);
    }
    ::intface::RenderInterface(player.life, (player.rage / 256) as u8,
        player.hammerFlags, player.hammers, b, player.score, player.weapon as u8,
        player.ammo, player.hamSpeed, mgl);
}

#[no_mangle]
pub unsafe extern fn SetCustomName(name: *const c_char) {
    strncpy(player.customName[player.worldNum as usize].as_mut_ptr(), name, 32);
}

#[no_mangle]
pub unsafe extern fn GetCustomName() -> *mut c_char {
    player.customName[player.worldNum as usize].as_mut_ptr()
}

#[no_mangle]
pub unsafe extern fn PlayerGetPercent(world: u8) -> f32 {
    if player.totalCompletion[world as usize] == 0 {
        1.0
    } else {
        player.complete[world as usize] as f32 / player.totalCompletion[world as usize] as f32
    }
}

#[no_mangle]
pub unsafe extern fn PlayerGetGamePercent() -> f32 {
    let (mut amt, mut total) = (0, 0);
    for i in 0..5 {
        total += player.totalCompletion[i];
        amt += player.complete[i];
    }
    amt as f32 / total as f32
}

#[no_mangle]
pub unsafe extern fn PlayerShield() -> u8 {
    player.shield
}

#[no_mangle]
pub unsafe extern fn PlayerHasHammer() -> bool {
    player.hammers > 0
}

#[no_mangle]
pub unsafe extern fn PlayerBrains() -> c_int {
    player.brains
}

#[no_mangle]
pub unsafe extern fn PoisonVictim(me: *mut Guy, amt: u8) {
    if me == goodguy && player.shield > 0 {
        return; // can't be poisoned while invulnerable
    }
    (*me).poison = (*me).poison.saturating_add(amt);
}

#[no_mangle]
pub unsafe extern fn PlayerResetScore() {
    player.score = player.prevScore;
}

#[no_mangle]
pub unsafe extern fn PlayerPassedLevel(world: u8, map: u8) -> u8 {
    player.levelPassed[world as usize][map as usize]
}

#[no_mangle]
pub unsafe extern fn PlayerWinLevel(w: u8, l: u8, isSecret: bool) {
    use options::{opt, SaveOptions};

    if player.levelPassed[w as usize][l as usize] == 0 {
        player.complete[w as usize] += 100; // get some percentage points
        if !isSecret {
            // secret levels aren't counted in this (it's for triggering specials)
            player.levelsPassed += 1;
        }
        if w == 4 && l == 6 && !opt.wonGame {
            opt.wonGame = true;
            SaveOptions();
            ::game::SendMessageToGame(::game::Message::NewFeature, 0);
        }
    } else {
        PlayerResetScore(); // you can't get points for a level you've already passed
    }

    if !opt.gotAllSecrets && PlayerGetGamePercent() > 0.999 {
        opt.gotAllSecrets = true;
        SaveOptions();
        ::game::SendMessageToGame(::game::Message::NewFeature, 0);
    }

    player.levelPassed[w as usize][l as usize] = 1;
}

#[no_mangle]
pub unsafe extern fn GetPlayerWorld() -> u8 {
    player.worldNum
}

#[no_mangle]
pub unsafe extern fn SetPlayerHP(hp: c_int) {
    player.life = hp as u8;
}

#[no_mangle]
pub unsafe extern fn PlayerLevelsPassed() -> u8 {
    player.levelsPassed
}

#[no_mangle]
pub unsafe extern fn KeyChainAllCheck() {
    if player.keychain[player.worldNum as usize].iter().all(|&b| b == 1) {
        ::message::NewBigMessage(cstr!("I collected all four!"), 30);
    }
}

// PlayerGetItem

#[no_mangle]
pub unsafe extern fn ToggleWaterwalk() {
    player.hammerFlags ^= ::bullet::HMR_WATERWALK;
}

#[no_mangle]
pub unsafe extern fn PlayerCanWaterwalk() -> bool {
    player.hammerFlags.contains(::bullet::HMR_WATERWALK)
}

#[no_mangle]
pub unsafe extern fn PlayerPushMore() -> bool {
    player.pushPower += 2;
    if player.pushPower >= 5 {
        player.pushPower = 0;
        true
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern fn PlayerHasLunacyKey(w: u8) -> bool {
    player.lunacyKey[w as usize] != 0
}

#[no_mangle]
pub unsafe extern fn PlayerLoseKey(w: u8) {
    if player.keys[w as usize] > 0 {
        player.keys[w as usize] -= 1;
    }
}

#[no_mangle]
pub unsafe extern fn PlayerKeyChain(w: u8) -> bool {
    player.keychain[player.worldNum as usize][w as usize] != 0
}

#[no_mangle]
pub unsafe extern fn PlayerKeys(w: u8) -> u8 {
    player.keys[w as usize]
}

#[no_mangle]
pub unsafe extern fn PlayerGetPoints(amt: c_int) {
    player.score += amt;
}

#[no_mangle]
pub unsafe extern fn GetPlayerGlow() -> u8 {
    playerGlow
}

#[no_mangle]
pub unsafe extern fn SetPlayerGlow(v: u8) {
    playerGlow = v;
}

#[no_mangle]
pub unsafe extern fn PlayerGetMusicSettings() -> ::options::Music {
    player.musicSettings
}

#[no_mangle]
pub unsafe extern fn PlayerSetMusicSettings(m: ::options::Music) {
    if ::music::CDLoaded() != 0 {
        player.musicSettings = m;
    } else {
        player.musicSettings = ::options::Music::Off;
    }
}

#[no_mangle]
pub unsafe extern fn PlayerThrowHammer(me: &Guy) {
    use options::{opt, PlayAs};
    use bullet::{HammerLaunch, HappyLaunch, Bullet, fire_bullet};

    match opt.playAs {
        PlayAs::Bouapha => {
            HammerLaunch(me.x, me.y, me.facing, player.hammers, player.hammerFlags);
        }
        PlayAs::Lunatic => {
            ::sound::make_sound(::sound::Sound::SND_BALLLIGHTNING, me.x, me.y, ::sound::SND_CUTOFF, 600);
            fire_bullet(me.x, me.y, me.facing, Bullet::BLT_BALLLIGHTNING, 1);
            if player.hammerFlags.contains(::bullet::HMR_REVERSE) {
                fire_bullet(me.x, me.y, ::mgldraw::MGL_random(8) as u8, Bullet::BLT_BALLLIGHTNING, 1);
            }
        }
        PlayAs::Happy => {
            HappyLaunch(me.x, me.y, me.facing, player.hammers, player.hammerFlags);
        }
    }

    player.reload = player.hamSpeed + 2;
}

#[no_mangle]
pub unsafe extern fn PlayerHeal(amt: u8) {
    ::guy::HealGoodguy(amt);

    player.life = ::std::cmp::min(128, player.life.saturating_add(amt));
}

#[no_mangle]
pub unsafe extern fn GetTportClock() -> u8 {
    tportclock
}

#[no_mangle]
pub unsafe extern fn SetTportClock(tp: u8) {
    tportclock = tp;
}

#[no_mangle]
pub unsafe extern fn DoPlayerFacing(c: ::control::Controls, me: &mut Guy) {
    use control::*;

    if c.contains(CONTROL_UP) {
        me.facing = 6;
        if c.contains(CONTROL_LF) {
            me.facing = 5;
        } else if c.contains(CONTROL_RT) {
            me.facing = 7;
        }
    } else if c.contains(CONTROL_DN) {
        me.facing = 2;
        if c.contains(CONTROL_LF) {
            me.facing = 3;
        } else if c.contains(CONTROL_RT) {
            me.facing = 1;
        }
    } else if c.contains(CONTROL_LF) {
        me.facing = 4;
    } else if c.contains(CONTROL_RT) {
        me.facing = 0;
    }
}

// PlayerFireWeapon

#[no_mangle]
pub unsafe extern fn PlayerFirePowerArmor(me: &mut Guy, mode: u8) {
    use bullet::{fire_bullet, Bullet};
    use cossin::{Cosine, Sine};

    match mode {
        1 => {
            ::sound::MakeSound(::sound::Sound::SND_ARMORSHOOT, me.x, me.y, ::sound::SND_CUTOFF, 1200);
            let f = (me.facing * 32).wrapping_sub(64) as c_int;
            let x = me.x + Cosine(me.facing as c_int * 32) * 20;
            let y = me.y + Sine(me.facing as c_int * 32) * 20;

            fire_bullet(x + Cosine(f) * 32, y + Sine(f) * 32,
                me.facing * 32, Bullet::BLT_BIGSHELL, 1);
            fire_bullet(x - Cosine(f) * 32, y - Sine(f) * 32,
                me.facing * 32, Bullet::BLT_BIGSHELL, 1);
            if player.ammo > 2 {
                player.ammo -= 2;
            }
        }
        2 => {
            ::bullet::QuadMissile(me.x, me.y, me.facing, 1);
            player.ammo = player.ammo.saturating_sub(25);
        }
        _ => {}
    }
}

// PlayerControlMe
// PlayerControlPowerArmor

#[no_mangle]
pub unsafe extern fn StealWeapon() -> ::items::Item {
    use items::Item;
    use bullet::{HMR_REVERSE, HMR_REFLECT};

    if player.hammers == 0 &&
        player.hamSpeed == 0 &&
        !player.hammerFlags.intersects(HMR_REVERSE | HMR_REFLECT)
    {
        return Item::ITM_NONE; // player has nothing to steal!
    }

    loop {
        match ::mgldraw::MGL_random(4) {
            0 => if player.hammers > 0 {
                player.hammers -= 1;
                return Item::ITM_HAMMERUP;
            },
            1 => if player.hamSpeed < 16 {
                player.hamSpeed += 4;
                return Item::ITM_PANTS;
            },
            2 => if player.hammerFlags.contains(HMR_REVERSE) {
                player.hammerFlags.remove(HMR_REVERSE);
                return Item::ITM_REVERSE;
            },
            3 => if player.hammerFlags.contains(HMR_REFLECT) {
                player.hammerFlags.remove(HMR_REFLECT);
                return Item::ITM_REFLECT;
            },
            _ => unreachable!()
        }
    }
}
