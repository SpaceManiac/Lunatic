use libc::{FILE, c_int, c_char};
use ffi::win::timeGetTime;
use mgldraw::MGLDraw;
use world::world_t;
use map::{Map, MapFlags};
use player::player;

#[repr(C)]
pub enum GameOutcome {
    CONTINUE = 0,
    QUITGAME,
}

pub const TIME_PER_FRAME: u32 = 1000 / 30;

#[repr(u8)]
#[derive(PartialEq)]
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
#[derive(PartialEq, Copy, Clone)]
pub enum LevelOutcome {
    Abort = 0,
    Win,
    Playing,
    Reset,
    Loading,
    QuitGame = WorldOutcome::QuitGame as u8,
}

/// these are world outcomes
#[repr(u8)]
#[derive(PartialEq)]
pub enum WorldOutcome {
    None = 0,
    Abort = 5,
    Win,
    Playing,
    Load,
    QuitGame
}

#[no_mangle]
pub static mut curWorld: world_t = ::world::ZERO_WORLD;
#[no_mangle]
pub static mut logFile: *mut FILE = 0 as *mut FILE;

static mut showStats: bool = false;
static mut gameStartTime: u32 = 0;
static mut visFrameCount: u32 = 0;
static mut updFrameCount: u32 = 0;
static mut tickerTime: u32 = 0;
/// how long the CD messing with took, take it out of the time budget, because
/// it can bog the game, but it should just freeze the action
static mut CDMessingTime: u32 = 0;
static mut garbageTime: u32 = 0;

static mut visFrms: c_int = 0;
static mut frmRate: f32 = 0.0;
static mut numRunsToMakeUp: u16 = 0;

static mut lastKey: u8 = 0;

static mut gamemgl: *mut MGLDraw = 0 as *mut MGLDraw;
static mut curMap: *mut Map = 0 as *mut Map;
static mut gameMode: GameMode = GameMode::Play;
static mut mapToGoTo: u8 = 0;
static mut worldNum: u8 = 0;
static mut mapNum: u8 = 0;
static mut curMapFlags: MapFlags = ::map::MAP_EMPTY;

static mut msgFromOtherModules: Message = Message::None;
static mut msgContent: u8 = 0;

static mut windingDown: u16 = 0;
static mut windingUp: u8 = 0;
static mut windDownReason: LevelOutcome = LevelOutcome::Playing;
static mut idleGame: bool = false;

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
    ::items::InitItems();
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
unsafe fn InitLevel(map: u8) -> bool {
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

unsafe fn ExitLevel() {
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

unsafe fn LunaticRun(lastTime: &mut u32) -> LevelOutcome {
    numRunsToMakeUp = 0;
    if *lastTime > TIME_PER_FRAME * 30 {
        *lastTime = TIME_PER_FRAME * 30;
    }

    while *lastTime >= TIME_PER_FRAME {
        if !(*gamemgl).Process() {
            mapToGoTo = 255;
            return LevelOutcome::Abort;
        }

        match gameMode {
            GameMode::Play => {
                // update everything here
                if windingDown != 0 {
                    (*curMap).Update(::map::UpdateMode::FadeOut, &mut curWorld);
                    ::guy::EditorUpdateGuys(&mut *curMap);
                } else if windingUp != 0 {
                    (*curMap).Update(::map::UpdateMode::FadeIn, &mut curWorld);
                    ::guy::EditorUpdateGuys(&mut *curMap);
                    windingUp -= 1;
                } else {
                    (*curMap).Update(::map::UpdateMode::Game, &mut curWorld);
                    ::guy::UpdateGuys(&mut *curMap, &mut curWorld);
                    ::bullet::UpdateBullets(&mut *curMap, &mut curWorld);
                    ::map::SpecialAnytimeCheck(&mut *curMap);
                }

                ::particle::UpdateParticles(&mut *curMap);
                ::message::UpdateMessage();

                if (*curMap).flags.contains(::map::MAP_SNOWING) {
                    ::particle::MakeItSnow(&mut *curMap);
                }

                if windingDown > 0 {
                    windingDown -= 1;
                    if windingDown == 0 {
                        return windDownReason;
                    }
                }
            },
            GameMode::Menu => match ::pause::UpdatePauseMenu(&mut *gamemgl) {
                0 => {
                    lastKey = 0;
                    gameMode = GameMode::Play;
                },
                1 => {},
                2 => {
                    if mapNum > 0 {
                        mapToGoTo = 0;
                    } else {
                        mapToGoTo = 255;
                    }
                    lastKey = 0;
                    return LevelOutcome::Abort;
                }
                3 => {
                    mapToGoTo = 255;
                    lastKey = 0;
                    return LevelOutcome::QuitGame; // dump out altogether
                }
                _ => {},
            },
            GameMode::Pic => {
                if ::control::GetTaps().intersects(::control::CONTROL_B1 | ::control::CONTROL_B2) {
                    gameMode = GameMode::Play;
                    // restore the palette
                    (*gamemgl).LoadBMP(cstr!("graphics/title.bmp"));
                }
            },
            GameMode::Rage => {
                use guy::goodguy;

                ::rage::UpdateRage(&mut *gamemgl);
                if player.rageClock > 0 {
                    player.rageClock -= 1;
                } else {
                    gameMode = GameMode::Play;
                    ::rage::StartRaging();
                }
                if !goodguy.is_null() {
                    (*goodguy).facing = ((*goodguy).facing + 1) & 7;
                }
            }
        }

        match ::std::mem::replace(&mut msgFromOtherModules, Message::None) {
            Message::None => {}
            Message::NewFeature => {
                ::message::NewMessage(cstr!("** NEW FEATURE ADDED!! **"), 120, 1);
            },
            Message::GotoMap => {
                mapToGoTo = msgContent;
                windingDown = 30;
                windDownReason = LevelOutcome::Abort;
            },
            Message::WinLevel => {
                mapToGoTo = msgContent;
                windingDown = 40;
                windDownReason = LevelOutcome::Win;
                if player.worldNum == 4 && player.levelNum == 6 {
                    ::display::ShowVictoryAnim(4); // you killed him.
                    SendMessageToGame(Message::WinGame, 0);
                }
                player.boredom = 0;
            },
            Message::Reset => {
                ::message::NewBigMessage(if ::options::opt.youSuck {
                    cstr!("You Suck")
                } else {
                    cstr!("Try Again!")
                }, 30);
                windingDown = 30;
                windDownReason = LevelOutcome::Reset;
            },
            Message::LoadGame => {
                ::message::NewBigMessage(cstr!("Loading Game"), 30);
                windingDown = 30;
                windDownReason = LevelOutcome::Loading;
            },
            Message::WinGame => {
                mapToGoTo = 0;
                windingDown = 1;
                windDownReason = LevelOutcome::Win;
                let CDtime = timeGetTime();
                ::title::VictoryText(&mut *gamemgl);
                ::title::Credits(&mut *gamemgl);
                garbageTime += timeGetTime() - CDtime;
                player.boredom = 0;
            }
        }

        *lastTime -= TIME_PER_FRAME;
        numRunsToMakeUp += 1;
        updFrameCount += 1;
    }
    HandleCDMusic();
    garbageTime = 0;
    ::jamulsound::JamulSoundUpdate();

    LevelOutcome::Playing
}

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

unsafe fn LunaticDraw() {
    // add all the sprites to the list
    if gameMode != GameMode::Pic {
        ::guy::RenderGuys(true);
        ::bullet::RenderBullets();
        ::particle::RenderParticles();
        ::display::RenderItAll(&mut curWorld, &mut *curMap,
            ::map::MAP_SHOWLIGHTS | ::map::MAP_SHOWITEMS | ::map::MAP_SHOWWALLS);
        ::map::RenderSpecialXes(&mut *gamemgl, &mut *curMap, worldNum);
        ::message::RenderMessage();
        ::player::PlayerRenderInterface(&mut *gamemgl);
        if gameMode == GameMode::Menu {
            ::pause::RenderPauseMenu();
        } else if gameMode == GameMode::Rage {
            ::rage::ShowRage(&mut *gamemgl);
        }
    } // else nothing to do

    if showStats {
        use display::Print;
        let seconds = (timeGetTime() - gameStartTime) as f32 / 1000.0;

        let mut s = [0; 32];
        sprintf!(s, "QFPS {:02.2}", frmRate);
        Print(0, 180, decay!(&s), 6, 0);
        sprintf!(s, "VFPS {:02.2}", visFrameCount as f32 / seconds);
        Print(0, 10, decay!(&s), 6, 0);
        sprintf!(s, "GFPS {:02.2}", updFrameCount as f32 / seconds);
        Print(0, 50, decay!(&s), 5, 0);
        sprintf!(s, "Runs {}", numRunsToMakeUp);
        Print(0, 100, decay!(&s), 6, 0);
    }

    draw_common();
}

unsafe fn WorldPauseRun(lastTime: &mut u32) -> LevelOutcome {
    numRunsToMakeUp = 0;
    while *lastTime >= TIME_PER_FRAME {
        if !(*gamemgl).Process() {
            mapToGoTo = 255;
            return LevelOutcome::QuitGame;
        }

        match ::pause::UpdatePauseMenu(&mut *gamemgl) {
            0 => {
                lastKey = 0;
                return LevelOutcome::Win;
            }
            1 => {}
            2 => {
                if mapNum > 0 {
                    mapToGoTo = 0;
                } else {
                    mapToGoTo = 255;
                }
                lastKey = 0;
                return LevelOutcome::Abort;
            }
            3 => {
                mapToGoTo = 255;
                lastKey = 0;
                return LevelOutcome::QuitGame; // dump out altogether
            }
            _ => {}
        }

        *lastTime -= TIME_PER_FRAME;
        numRunsToMakeUp += 1;
        updFrameCount += 1;
    }

    HandleCDMusic();
    garbageTime = 0;
    ::jamulsound::JamulSoundUpdate();

    LevelOutcome::Playing
}

unsafe fn WorldPauseDraw() {
    (*gamemgl).ClearScreen();
    ::pause::RenderPauseMenu();
    draw_common();
}

unsafe fn draw_common() {
    // update statistics
    let d = timeGetTime();
    if d - tickerTime > 999 {
        frmRate = (frmRate * 3.0 + (visFrms as f32 / ((d - tickerTime) as f32 / 1000.0))) / 4.0;
        visFrms = 0;
        tickerTime = d;
    }

    (*gamemgl).Flip();
    CDMessingTime += garbageTime;
    garbageTime = 0;

    visFrameCount += 1;
    visFrms += 1;
}

unsafe fn WorldPickerPause() -> LevelOutcome {
    let mut lastTime = 1;
    let mut exitCode = LevelOutcome::Playing;

    ::pause::InitPauseMenu();
    ::pause::SetGiveUpText(::pause::GiveUp::None);
    while exitCode == LevelOutcome::Playing {
        let start = timeGetTime();
        exitCode = WorldPauseRun(&mut lastTime);
        WorldPauseDraw();
        if !(*gamemgl).Process() {
            exitCode = LevelOutcome::QuitGame;
            mapToGoTo = 255;
        }
        lastTime += timeGetTime() - start;
    }
    exitCode
}

#[no_mangle]
pub unsafe extern fn SendMessageToGame(msg: Message, content: c_int) {
    msgFromOtherModules = msg;
    msgContent = content as u8;
}

unsafe fn HandleKeyPresses() {
    let k = (*gamemgl).LastKeyPressed();
    if k != 0 {
        lastKey = k;
        if (k >= b'a' && k <= b'z') || (k >= b'A' && k <= b'Z') {
            ::cheat::CheatKey(lastKey);
        }
    }

    #[cfg(debug_assertions)] {
        // can't show stats unless in debug mode
        if lastKey == b's' {
            showStats = !showStats;
            lastKey = 0;
        }
    }

    if lastKey == b'g' {
        let gamma = (::display::GetGamma() + 1) % 4;
        (*gamemgl).GammaCorrect(gamma);
        ::display::SetGamma(gamma);
        lastKey = 0;
    }
}

#[no_mangle]
pub unsafe fn PlayALevel(map: u8) -> LevelOutcome {
    if !InitLevel(map) {
        mapToGoTo = 255;
        return LevelOutcome::Abort;
    }

    let mut exitcode = LevelOutcome::Playing;
    gameMode = GameMode::Play;
    CDMessingTime = 0;
    garbageTime = 0;

    // this will force the camera into the right position
    // it also makes everybody animate by one frame, but no one will
    // ever notice
    ::guy::UpdateGuys(&mut *curMap, &mut curWorld);

    let mut lastTime = 0;
    while exitcode == LevelOutcome::Playing {
        let start = timeGetTime();
        if gameMode == GameMode::Play {
            HandleKeyPresses();
        }

        exitcode = LunaticRun(&mut lastTime);
        LunaticDraw();

        if lastKey == 27 && gameMode == GameMode::Play {
            ::pause::InitPauseMenu();
            gameMode = GameMode::Menu;
        }

        if !(*gamemgl).Process() {
            mapToGoTo = 255;
            exitcode = LevelOutcome::Abort;
        }
        lastTime += timeGetTime() - start - CDMessingTime;
    }
    ExitLevel();
    exitcode
}

#[no_mangle]
pub unsafe extern fn LunaticWorld(world: u8, worldName: *const c_char) -> WorldOutcome {
    ::player::InitPlayer(::player::Init::World, world, 0);
    if ::world::LoadWorld(&mut curWorld, worldName) == 0 {
        return WorldOutcome::Abort;
    }

    worldNum = world;
    ::world::InitWorld(&mut curWorld, worldNum);

    mapNum = 0;
    loop {
        match PlayALevel(mapNum) {
            LevelOutcome::Playing => {},
            LevelOutcome::Abort => {
                ::player::PlayerResetScore();
                if mapToGoTo < 255 {
                    mapNum = mapToGoTo;
                } else {
                    break;
                }
            },
            LevelOutcome::Reset => {
                ::player::PlayerResetScore();
                // don't do anything, play the same level
            },
            LevelOutcome::Win => {
                ::player::PlayerWinLevel(world, mapNum, curMapFlags.contains(::map::MAP_SECRET));
                mapNum = mapToGoTo;
            },
            LevelOutcome::Loading => {
                ::world::FreeWorld(&mut curWorld);
                ::player::PlayerResetScore();
                return WorldOutcome::Load;
            },
            LevelOutcome::QuitGame => {
                ::world::FreeWorld(&mut curWorld);
                ::player::PlayerResetScore();
                return WorldOutcome::QuitGame;
            },
        }
    }
    ::world::FreeWorld(&mut curWorld);
    WorldOutcome::Abort
}

#[no_mangle]
pub unsafe extern fn LunaticGame(mgl: &mut MGLDraw, load: bool) {
    let mut custName = [0; 64];
    let mut worldResult = if load { // continuing a saved game
        WorldOutcome::Load
    } else {
        // don't do this if loading a game, it was already done and the player was filled with values
        ::player::InitPlayer(::player::Init::Game, 0, 0);
        WorldOutcome::None
    };

    'outer: loop {
        let mut b;

        if worldResult == WorldOutcome::Load {
            ::title::ReScanWorldNames();
            b = player.worldNum;
        } else {
            loop {
                msgFromOtherModules = Message::None;
                b = ::title::WorldPicker(mgl);
                if b != 253 { break }

                if WorldPickerPause() == LevelOutcome::QuitGame {
                    break 'outer;
                }
                if msgFromOtherModules == Message::LoadGame { // load a game
                    ::title::ReScanWorldNames();
                    b = player.worldNum;
                    break;
                }
            }
            if b == 255 { break }
            player.worldNum = b;
        }

        if b == 4 && worldResult != WorldOutcome::Load {
            use player::PlayerHasLunacyKey;
            // the asylum requires that you have all four loony keys to enter
            // unless you're loading a saved game saved in there, in which case watching
            // the animation would be a waste
            if PlayerHasLunacyKey(0) && PlayerHasLunacyKey(1) &&
                PlayerHasLunacyKey(2) && PlayerHasLunacyKey(3)
            {
                ::display::ShowVictoryAnim(12);
                garbageTime = 0;
                sprintf!(custName, "worlds/{}", ::PctS(::player::GetCustomName()));
                worldResult = LunaticWorld(b, decay!(&custName));
            } else {
                ::display::ShowVictoryAnim(11);
                garbageTime = 0;
                worldResult = WorldOutcome::None; // not allowed to enter
            }
        } else {
            sprintf!(custName, "worlds/{}", ::PctS(::player::GetCustomName()));
            worldResult = LunaticWorld(b, decay!(&custName));
        }

        if worldResult == WorldOutcome::QuitGame {
            mgl.LastKeyPressed(); // just to clear the buffer
            break;
        }
    }

    ::player::ExitPlayer();
}
