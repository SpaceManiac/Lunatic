use libc::{c_int, c_char};
use world::MAX_MAPS;
use mgldraw::MGLDraw;

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
    pub musicSettings: u8,
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
    pub hammerFlags: u8,
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

    pub fn InitPlayer(initWhat: Init, world: u8, level: u8);
    pub fn ExitPlayer();

    pub fn GetCustomName() -> *const c_char;
    pub fn PlayerWinLevel(w: u8, l: u8, isSecret: bool);
    pub fn PlayerResetScore();
    pub fn PlayerRenderInterface(mgl: &mut MGLDraw);

    pub fn PlayerGetItem(itm: u8, x: c_int, y: c_int) -> u8;
    pub fn PlayerSetWorldWorth(world: u8, amt: c_int);
    pub fn PlayerHeal(amt: u8);
    pub fn PlayerGetMusicSettings() -> ::options::Music;
    pub fn PlayerSetMusicSettings(m: ::options::Music);
    pub fn PlayerHasHammer() -> bool;
    pub fn PlayerHasLunacyKey(world: u8) -> bool;
    pub fn PlayerShield() -> u8;
    pub fn ToggleWaterwalk();

    pub fn PlayerLoadGame(which: u8);
    pub fn PlayerSaveGame(which: u8);

    pub fn PlayerKeys(w: u8) -> u8;
    pub fn PlayerKeyChain(w: u8) -> u8;
}
