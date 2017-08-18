use libc::{c_int, c_char, strncpy};
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
pub enum Init {
    Game = 2,
    World = 1,
    Level = 0,
}

/// vehicles you could be on
#[repr(u8)]
pub enum Vehicle {
    None = 0,
    Minecart = 1,
    Raft = 2,
}

/// the most custom worlds it will handle
pub const MAX_CUSTOM: usize = 128;

#[repr(C)]
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

extern {
    pub static mut player: player_t;
    static mut playerGlow: u8; // for torch-lit levels, and for exciting moments
    static mut tportclock: u8;

    pub fn InitPlayer(initWhat: Init, world: u8, level: u8);

    pub fn PlayerWinLevel(w: u8, l: u8, isSecret: bool);

    pub fn PlayerGetItem(itm: u8, x: c_int, y: c_int) -> u8;
    pub fn PlayerHeal(amt: u8);

    pub fn PlayerLoadGame(which: u8);
    pub fn PlayerSaveGame(which: u8);
}

// InitPlayer

#[no_mangle]
pub unsafe extern fn ExitPlayer() {
}

// PlayerLoadGame
// PlayerSaveGame

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

// PlayerWinLevel

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

// PlayerThrowHammer
// PlayerHeal

#[no_mangle]
pub unsafe extern fn GetTportClock() -> u8 {
    tportclock
}

#[no_mangle]
pub unsafe extern fn SetTportClock(tp: u8) {
    tportclock = tp;
}

// DoPlayerFacing
// PlayerFireWeapon
// PlayerFirePowerArmor
// PlayerControlMe
// PlayerControlPowerArmor
// StealWeapon
