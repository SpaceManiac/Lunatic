use player::{self, player_t};
use options::Music;
use display::{GetDisplayMGL, CenterPrint};
use mgldraw::MGLDraw;
use libc::{c_int, c_float};

#[repr(C)]
#[derive(PartialEq, Copy, Clone)]
pub enum Submode {
    None = 0,
    SlotPick,
}

#[repr(C)]
#[derive(FromInt, PartialEq, Copy, Clone)]
pub enum GiveUp {
    WorldSelect = 0,
    GiveUp = 1,
    None = 2,
}

static mut cursor: u8 = 0;
static mut subcursor: u8 = 0;
static mut subMode: Submode = Submode::None;
/// the percentages in each save slot
static mut percent: [f32; 3] = [0.; 3];
/// which text should be shown for "Give Up"
static mut giveUp: GiveUp = GiveUp::WorldSelect;

static mut lastKey: u8 = 0;

#[no_mangle]
pub unsafe extern fn InitPauseMenu() {
    lastKey = 0;
    subMode = Submode::None;

    let f = ::mgldraw::AppdataOpen(cstr!("loony.sav"), cstr!("rb"));
    if f.is_null() {
        percent = [0., 0., 0.];
    } else {
        let mut p = ::std::mem::zeroed();
        for i in 0..3 {
            ::libc::fread(decay!(&mut p), szof!(player_t), 1, f);
            percent[i] = 100. * CalcTotalPercent(&p);
        }
    }

    ::sound::make_normal_sound(::sound::Sound::SND_PAUSE);
}

#[no_mangle]
pub unsafe extern fn SetSubCursor(s: u8) {
    subcursor = s;
}

#[no_mangle]
pub unsafe extern fn HandlePauseKeyPresses(mgl: &mut MGLDraw) {
    let k = mgl.LastKeyPressed();
    if k != 0 { lastKey = k }
}

#[no_mangle]
pub extern fn CalcTotalPercent(p: &player_t) -> c_float {
    let (mut amt, mut total) = (0, 0);
    for i in 0..5 { // only the default five worlds
        total += p.totalCompletion[i];
        amt += p.complete[i];
    }
    (amt as f32) / (total as f32)
}

#[no_mangle]
pub unsafe extern fn RenderPauseMenu() {
    let mgl = GetDisplayMGL();
    mgl.Box(208, 44, 432, 422, 128 + 10);
    mgl.Box(209, 45, 431, 421, 128 + 16);
    mgl.FillBox(210, 46, 430, 420, 0);
    mgl.Box(211, 47, 429, 419, 128 + 10);

    CenterPrint(320, 50, cstr!("Cancel"), if cursor == 0 { 16 } else { 0 }, 0);
    CenterPrint(320, 110, cstr!("Load Game"), if cursor == 1 { 16 } else { 0 }, 0);
    CenterPrint(320, 170, cstr!("Save Game"), if cursor == 2 { 16 } else { 0 }, 0);

    let message = match Music::from_int(player::PlayerGetMusicSettings() as usize) {
        Some(Music::MUSIC_OFF) => cstr!("Music: Off"),
        Some(Music::MUSIC_ON) => cstr!("Music: On"),
        _ => cstr!("Music: Rnd"),
    };
    CenterPrint(320, 230, message, if cursor == 3 { 16 } else { 0 }, 0);

    // giveUp==2 means don't draw the give up option at all (for world picker pause)
    match GiveUp::from_int(giveUp as usize) {
        Some(GiveUp::GiveUp) => CenterPrint(320, 290, cstr!("Give Up"), if cursor == 4 { 16 } else { 0 }, 0),
        Some(GiveUp::WorldSelect) => CenterPrint(320, 290, cstr!("World Select"), if cursor == 4 { 16 } else { 0 }, 0),
        _ => {}
    }
    CenterPrint(320, 350, cstr!("Quit Game"), if cursor == 5 { 16 } else { 0 }, 0);
    if subMode == Submode::SlotPick {
        RenderSlotPickMenu();
    }
}

#[no_mangle]
pub unsafe extern fn RenderSlotPickMenu() {
    let mgl = GetDisplayMGL();
    mgl.Box(258, 104, 492, 294, 128 + 10);
    mgl.Box(259, 105, 491, 293, 128 + 16);
    mgl.FillBox(260, 106, 490, 292, 0);
    mgl.Box(261, 107, 489, 291, 128 + 10);

    let mut txt = [0; 20];
    for i in 0..3 {
        if percent[i] > 99.9 {
            sprintf!(txt, "Slot {} - 100%", i + 1);
        } else {
            sprintf!(txt, "Slot {} - {:3.1}%", i + 1, percent[i]);
        }
        CenterPrint(375, 110 + (60 * i as i32), decay!(&txt), if subcursor == i as u8 { 16 } else { 0 }, 0);
    }
}

#[no_mangle]
pub unsafe extern fn SetGiveUpText(gu: GiveUp) {
    giveUp = gu;
}

#[no_mangle]
pub unsafe extern fn UpdatePauseMenu(mgl: &mut MGLDraw) -> u8 {
    use control::*;
    use sound::*;

    static mut oldc: Controls = EMPTY;
    static mut reptCounter: u8 = 0;

    if giveUp == GiveUp::None && cursor == 4 { // not allowed in world picker pause
        cursor = 0;
    }

    let c = Controls::from_bits_truncate(GetControls() | GetArrows());

    reptCounter += 1;
    if oldc.is_empty() || reptCounter > 10 {
        reptCounter = 0;
    }

    if subMode == Submode::None { // not in any submenu
        if c.contains(CONTROL_UP) && reptCounter == 0 {
            if cursor == 0 {
                cursor = 5;
            } else {
                cursor -= 1;
            }

            if giveUp == GiveUp::None && cursor == 4 {
                // world picker pause has no option 4
                cursor = 3;
            }

            make_normal_sound(Sound::SND_MENUCLICK);
        }
        if c.contains(CONTROL_DN) && reptCounter == 0 {
            cursor += 1;
            if cursor == 6 {
                cursor = 0;
            }
            if giveUp == GiveUp::None && cursor == 4 {
                // world picker pause has no option 4
                cursor = 5;
            }

            make_normal_sound(Sound::SND_MENUCLICK);
        }
        if (c - oldc).intersects(CONTROL_B1 | CONTROL_B2) {
            make_normal_sound(Sound::SND_MENUCLICK);
            match cursor {
                0 => return 0, // cancel
                1 | 2 => subMode = Submode::SlotPick, // Load, Save
                3 => { // music
                    ::music::CDNeedsUpdating();
                    ::music::CDStop();
                    let mus = (::player::PlayerGetMusicSettings() + 1) % 3;
                    ::player::PlayerSetMusicSettings(mus);
                    if mus == ::options::Music::MUSIC_ON as u8 {
                        ::music::CDPlay(::game::GetCurSong() as c_int);
                    }
                    ::options::opt.music = mus;
                }
                4 => return 2, // give up
                5 => return 3, // quit game
                _ => {},
            }
        }
    } else if subMode == Submode::SlotPick {
        if c.contains(CONTROL_UP) && reptCounter == 0 {
            make_normal_sound(Sound::SND_MENUCLICK);
            if subcursor == 0 {
                subcursor = 2;
            } else {
                subcursor -= 1;
            }
        }
        if c.contains(CONTROL_DN) && reptCounter == 0 {
            make_normal_sound(Sound::SND_MENUCLICK);
            subcursor += 1;
            if subcursor == 3 {
                subcursor = 0;
            }
        }
        if (c - oldc).intersects(CONTROL_B1 | CONTROL_B2) {
            make_normal_sound(Sound::SND_MENUCLICK);
            match cursor {
                1 => { // Load
                    ::game::SendMessageToGame(::game::Message::MSG_LOADGAME, 0);
                    ::player::PlayerLoadGame(subcursor);
                    make_normal_sound(Sound::SND_LOADGAME);
                    return 0;
                }
                2 => { // Save
                    ::player::PlayerSaveGame(subcursor);
                    make_normal_sound(Sound::SND_SAVEGAME);
                    return 0;
                }
                _ => {
                    subMode = Submode::None;
                }
            }
        }
    }
    oldc = c;

    HandlePauseKeyPresses(mgl);
    if lastKey == 27 { // hit ESC to exit pause menu
        make_normal_sound(Sound::SND_MENUSELECT);
        if subMode == Submode::None {
            return 0;
        } else {
            subMode = Submode::None;
        }
        lastKey = 0;
    }
    return 1;
}
