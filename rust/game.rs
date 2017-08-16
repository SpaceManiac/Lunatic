use libc::{FILE, c_int};
use ffi::win::timeGetTime;
use mgldraw::MGLDraw;
use world::world_t;
use map::Map;
use player::player;

#[repr(C)]
enum Q {
    CONTINUE = 0,
    QUITGAME,
}

const TIME_PER_FRAME: u64 = 1000 / 30;

#[repr(u8)]
enum GameMode {
    Play = 0,
    Menu,
    Pic,
    Rage,
}

/// these are the messages you can send to the game
#[repr(C)]
pub enum Message {
    None = 0,
    GotoMap,
    WinLevel,
    Reset,
    LoadGame,
    WinGame,
    NewFeature,
}

/// these are the possible outcomes of a level
/// if you die, the level just starts over, so that isn't included
/// playing isn't an outcome, it's just used to keep the level going
#[repr(u8)]
pub enum LevelOutcome {
    Abort = 0,
    Win,
    Playing,
    Reset,
    Loading,
}

/// these are world outcomes
#[repr(u8)]
pub enum WorldOutcome {
    Abort = 5,
    Win,
    Playing,
    Load,
    QuitGame
}

extern {
    pub static mut curWorld: world_t;
    pub static mut logFile: *mut FILE;

    // these are the major inits, just at the beginning and ending of a whole game
    pub fn LunaticInit(mgl: *mut MGLDraw);
    pub fn LunaticGame(mgl: *mut MGLDraw, load: u8);
    pub fn LunaticExit();

    pub fn SendMessageToGame(msg: Message, content: c_int);

    static mut showStats: u8;
    static mut gameStartTime: u32;
    static mut visFrameCount: u32;
    static mut updFrameCount: u32;
    static mut tickerTime: u32;
    /// how long the CD messing with took, take it out of the time budget, because
    /// it can bog the game, but it should just freeze the action
    static mut CDMessingTime: u32;
    static mut garbageTime: u32;

    static mut visFrms: c_int;
    static mut frmRate: f32;
    static mut numRunsToMakeUp: u16;

    static mut lastKey: u8;

    static mut gamemgl: *mut MGLDraw;
    #[link_name="game_curMap"]
    static mut curMap: *mut Map;
    static mut gameMode: GameMode;
    static mut mapToGoTo: u8;
    static mut worldNum: u8;
    static mut mapNum: u8;
    static mut curMapFlags: u8;

    static mut msgFromOtherModules: Message;
    static mut msgContent: u8;

    static mut windingDown: u16;
    static mut windingUp: u8;
    static mut windDownReason: u8;
    #[link_name="game_idleGame"]
    static mut idleGame: bool;
}

#[no_mangle]
pub unsafe extern fn GameCurrentMap() -> *mut Map {
    curMap
}

// LunaticInit
// LunaticExit

#[no_mangle]
pub unsafe extern fn GetCurSong() -> u8 {
    if curMap.is_null() {
        3
    } else {
        (*curMap).song
    }
}

// InitLevel
// ExitLevel

#[no_mangle]
pub unsafe extern fn SetGameIdle(b: bool) {
    idleGame = b;
}

#[no_mangle]
pub unsafe extern fn GetGameIdle() -> bool {
    idleGame
}

/// this is run whenever the game is swapped away from
#[no_mangle]
pub unsafe extern fn GameIdle() {
    let start = timeGetTime();
    while idleGame {
        HandleCDMusic();
        if !(*gamemgl).Process() {
            break
        }
    }
    AddGarbageTime(timeGetTime() - start);
    player.boredom = 0;
}

#[no_mangle]
pub unsafe extern fn EnterStatusScreen() {
    gameMode = GameMode::Menu;
}

#[no_mangle]
pub unsafe extern fn EnterPictureDisplay() {
    gameMode = GameMode::Pic;
    ::control::GetTaps(); // clear the key tap buffer
}

#[no_mangle]
pub unsafe extern fn EnterRage() {
    ::sound::make_normal_sound(::sound::Sound::SND_RAGE);
    gameMode = GameMode::Rage;
}

#[no_mangle]
pub unsafe extern fn AddGarbageTime(t: u32) {
    garbageTime += t;
}

// LunaticRun

#[no_mangle]
pub unsafe extern fn HandleCDMusic() {
    let start = timeGetTime();
    ::music::CDPlayerUpdate(match ::player::PlayerGetMusicSettings() {
        ::options::Music::Off => ::music::AudioMode::Off,
        ::options::Music::On => ::music::AudioMode::LoopTrack,
        ::options::Music::Random => ::music::AudioMode::Random,
    });
    CDMessingTime = timeGetTime() - start; // that's how long CD messing took
    CDMessingTime += garbageTime; // time wasted with such things as playing animations
}
