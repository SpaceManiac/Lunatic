use libc::{FILE, c_int};
use ffi::win::timeGetTime;
use mgldraw::MGLDraw;
use world::world_t;
use map::Map;
use player::player;

#[repr(C)]
pub enum GameOutcome {
    CONTINUE = 0,
    QUITGAME,
}

pub const TIME_PER_FRAME: u64 = 1000 / 30;

#[repr(u8)]
pub enum GameMode {
    Play = 0,
    Menu,
    Pic,
    Rage,
}

/// these are the messages you can send to the game
#[repr(C)]
#[derive(PartialEq)]
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

#[allow(dead_code)]
extern {
    pub static mut curWorld: world_t;
    pub static mut logFile: *mut FILE;

    pub fn LunaticGame(mgl: *mut MGLDraw, load: u8);

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

/// replaces extern Map *curMap
#[no_mangle]
pub unsafe extern fn GameCurrentMap() -> *mut Map {
    curMap
}

// these are the major inits, just at the beginning and ending of a whole game
#[no_mangle]
pub unsafe extern fn LunaticInit(mgl: *mut MGLDraw) {
    gamemgl = mgl;

    logFile = ::mgldraw::AppdataOpen(cstr!("loonylog.txt"), cstr!("wt"));
    ::cossin::InitCosSin();
    ::display::InitDisplay(gamemgl);
    ::sound::InitSound();
    ::monster::InitMonsters();
    ::tile::InitTiles(mgl);
    ::intface::InitInterface();
    ::options::LoadOptions();
    ::music::MusicInit();
    (*mgl).SetLastKey(0);
    ::mgldraw::MGL_srand(timeGetTime() as i32);
    ::control::InitControls();
    ::player::InitPlayer(::player::Init::Game, 0, 0);
    msgFromOtherModules = Message::None;
}

#[no_mangle]
pub unsafe extern fn LunaticExit() {
    ::music::MusicExit();
    ::items::ExitItems();
    ::sound::ExitSound();
    ::display::ExitDisplay();
    ::tile::ExitTiles();
    ::monster::ExitMonsters();
    ::player::ExitPlayer();
    ::intface::ExitInterface();
    ::libc::fclose(logFile);
}

#[no_mangle]
pub unsafe extern fn GetCurSong() -> u8 {
    if curMap.is_null() {
        3
    } else {
        (*curMap).song
    }
}

/// these are the minor inits, called every time you enter a new map
#[no_mangle]
pub unsafe fn InitLevel(map: u8) -> bool {
    ::jamulsound::JamulSoundPurge(); // each level, that should be good

    if curWorld.numMaps <= map {
        return false; // can't go to illegal map
    }

    // make a copy of the map to be played
    curMap = Map::from_map(curWorld.map[map as usize]);
    curMapFlags = (*curMap).flags;

    match ::player::PlayerGetMusicSettings() {
        ::options::Music::Off => ::music::CDStop(), // in case it's playing for some reason
        ::options::Music::On => ::music::CDPlay((*curMap).song as i32),
        ::options::Music::Random => {} // do nothing- if there is a song currently playing,
            // let it finish, else a new one will automatically start at the next call to CDPlayerUpdate
    }

    gameStartTime = timeGetTime();
    tickerTime = timeGetTime();
    updFrameCount = 0;
    visFrameCount = 0;
    numRunsToMakeUp = 0;
    frmRate = 30.0;
    visFrms = 0;
    if msgFromOtherModules != Message::NewFeature {
        msgFromOtherModules = Message::None;
    }

    ::guy::InitGuys(256);
    ::bullet::InitBullets();
    ::player::InitPlayer(::player::Init::Level, 0, map);
    ::message::InitMessage();
    ::message::NewBigMessage((*curMap).name.as_ptr(), 100);
    ::particle::InitParticles(512);
    lastKey = 0;
    (*curMap).Init(&mut curWorld);

    windingDown = 0;
    windingUp = 30;
    ::intface::ResetInterface();
    ::cheat::InitCheater();

    ::pause::SetGiveUpText(match map {
        0 => ::pause::GiveUp::WorldSelect,
        _ => ::pause::GiveUp::GiveUp,
    });

    true
}

#[no_mangle]
pub unsafe extern fn ExitLevel() {
    // exit everything
    ::guy::ExitGuys();
    ::bullet::ExitBullets();
    ::particle::ExitParticles();

    if ::player::PlayerGetMusicSettings() == ::options::Music::On {
        ::music::CDStop(); // don't stop if it's on random
    }

    Map::delete(curMap);
    curMap = ::std::ptr::null_mut();
    ::monster::PurgeMonsterSprites();
}

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

// LunaticDraw
// WorldPauseRun
// WorldPauseDraw
// WorldPickerPause

#[no_mangle]
pub unsafe extern fn SendMessageToGame(msg: Message, content: c_int) {
    msgFromOtherModules = msg;
    msgContent = content as u8;
}

// HandleKeyPresses
// PlayALevel
// LunaticWorld
// LunaticGame
