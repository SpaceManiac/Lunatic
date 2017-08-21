use libc::{c_int, c_char};
use ffi::win::timeGetTime;
use jamulspr::sprite_set_t;
use jamulfont::mfont_t;
use mgldraw::MGLDraw;
use player::{player, MAX_CUSTOM};
use game::TIME_PER_FRAME;
use world::{GetWorldName, GetWorldPoints};

#[repr(C)]
struct title_t {
    bouaphaX: c_int,
    doctorX: c_int,
    blueY: c_int,
    titleBright: i8,
    titleDir: i8,
    expando: c_int,
    dexpando: i8,
    cursor: u8,
    savecursor: u8,
    percent: [f32; 3],
}

extern {
    pub fn SplashScreen(mgl: *mut MGLDraw, fname: *const c_char, delay: c_int, sound: u8);
    pub fn MainMenu(mgl: *mut MGLDraw) -> u8;
    pub fn VictoryText(mgl: &mut MGLDraw);
    pub fn Credits(mgl: &mut MGLDraw);

    pub fn ScanWorldNames();
    pub fn ReScanWorldNames();

    #[link_name="title_oldc"]
    static mut oldc: ::control::Controls;
}

static mut numRunsToMakeUp: c_int = 0;
static mut pickerpos: u8 = 0;
static mut pickeroffset: i8 = 0;
static mut offsetdir: u8 = 0;
static mut curCustom: u8 = 0;

static mut keyAnim: u8 = 0;
static mut lvlName: [c_char; 32] = [0; 32];

static starColorTable: [u8; 9] = [214, 81, 63, 49, 33, 21, 32, 83, 93];

fn HandleWorldPickerKeys(mgl: &mut MGLDraw) -> u8 {
    if mgl.LastKeyPressed() == 27 {
        253 // pause menu
    } else {
        254
    }
}

unsafe fn NextLegal(mut now: u8, dir: u8) -> u8 {
    let mut tries = 0;
    if dir == 1 {
        loop {
            now += 1;
            if now > MAX_CUSTOM as u8 - 1 { now = 5; }
            tries += 1;
            if !(player.customName[now as usize][0] == 0 && tries < MAX_CUSTOM - 5) {
                break
            }
        }
    } else {
        loop {
            now -= 1;
            if now < 5 { now = MAX_CUSTOM as u8 - 1; }
            tries += 1;
            if !(player.customName[now as usize][0] == 0 && tries < MAX_CUSTOM - 5) {
                break
            }
        }
    }
    now
}

#[no_mangle] // x
pub unsafe extern fn PickerRun(lastTime: &mut u32, mgl: &mut MGLDraw) -> u8 {
    use control::*;
    use sound::*;

    static mut ticktock: u8 = 0;
    static mut flipper: u8 = 0;

    let mut movedCursor = false;
    numRunsToMakeUp = 0;
    while *lastTime >= TIME_PER_FRAME {
        if !mgl.Process() {
            return 255;
        }

        keyAnim += 1;
        if keyAnim > 63 { keyAnim = 0; }

        let c = HandleWorldPickerKeys(mgl);
        if c != 254 { return c; }

        let c = GetControls() | GetArrows();

        if ticktock > 0 { ticktock -= 1; }
        if !oldc.intersects(CONTROL_UP | CONTROL_DN) {
            ticktock = 0;
        }

        if pickeroffset == 0 { // only listen to keys when it is on a spot
            if pickerpos < 4 {
                if c.contains(CONTROL_UP) {
                    if pickerpos == 0 {
                        pickerpos = 3;
                    } else {
                        pickerpos -= 1;
                    }
                    pickeroffset = 3;
                    offsetdir = 0;
                    MakeNormalSound(Sound::SND_WORLDTURN);
                    movedCursor = true;
                }
                if c.contains(CONTROL_DN) {
                    pickerpos += 1;
                    if pickerpos == 4 {
                        pickerpos = 0;
                    }
                    pickeroffset = -3;
                    offsetdir = 0;
                    MakeNormalSound(Sound::SND_WORLDTURN);
                    movedCursor = true;
                }
                if c.contains(CONTROL_LF) {
                    pickerpos = 5;
                    pickeroffset = -3;
                    offsetdir = 0;
                    MakeNormalSound(Sound::SND_WORLDTURN);
                    movedCursor = true;
                    curCustom = NextLegal(curCustom.wrapping_sub(1), 1);
                }
                if c.contains(CONTROL_RT) {
                    pickerpos = 4;
                    pickeroffset = -3;
                    offsetdir = 0;
                    MakeNormalSound(Sound::SND_WORLDTURN);
                    movedCursor = true;
                }
            } else if pickerpos == 4 {
                if c.contains(CONTROL_LF) {
                    pickerpos = 0;
                    pickeroffset = 3;
                    offsetdir = 2;
                    MakeNormalSound(Sound::SND_WORLDTURN);
                    movedCursor = true;
                }
            } else if pickerpos == 5 {
                if c.contains(CONTROL_RT) {
                    pickerpos = 0;
                    pickeroffset = 3;
                    offsetdir = 1;
                    MakeNormalSound(Sound::SND_WORLDTURN);
                    movedCursor = true;
                }
                if c.contains(CONTROL_UP) && ticktock == 0 {
                    let next = NextLegal(curCustom, 0);
                    if curCustom != next {
                        curCustom = next;
                        MakeNormalSound(Sound::SND_FOOD);
                    }
                    ticktock = 20;
                    movedCursor = true;
                }
                if c.contains(CONTROL_DN) && ticktock == 0 {
                    let next = NextLegal(curCustom, 1);
                    if curCustom != next {
                        curCustom = next;
                        MakeNormalSound(Sound::SND_FOOD);
                    }
                    ticktock = 20;
                    movedCursor = true;
                }
            }
        } else {
            flipper += 1;
            if flipper == 2 {
                if pickeroffset > 0 {
                    pickeroffset -= 1;
                } else {
                    pickeroffset += 1;
                }
                flipper = 0;
            }
            if pickeroffset == 0 {
                offsetdir = 0;
            }
        }

        if (c - oldc).intersects(CONTROL_B1 | CONTROL_B2) {
            MakeNormalSound(Sound::SND_WORLDPICK);
            if pickerpos < 5 {
                return pickerpos;
            } else {
                if player.customName[curCustom as usize][0] != 0 {
                    return curCustom;
                } else { // can't pick a nonexistent level
                    MakeNormalSound(Sound::SND_BOUAPHAOUCH);
                }
            }
        }
        oldc = c;

        *lastTime -= TIME_PER_FRAME;
        numRunsToMakeUp += 1;
    }
    ::jamulsound::JamulSoundUpdate();

    if movedCursor {
        if pickerpos < 5 {
            GetWorldPoints(player.customName[pickerpos as usize].as_ptr());
        } else {
            GetWorldName(player.customName[curCustom as usize].as_ptr(), lvlName.as_mut_ptr());
            GetWorldPoints(player.customName[curCustom as usize].as_ptr());
        }
    }

    254
}

unsafe fn PickerDraw(mgl: &mut MGLDraw, planet: &sprite_set_t, pickerFont: &mfont_t) {
    use mgldraw::{MGL_srand, MGL_random};
    use jamulfont::FontPrintString;

    { // draw stars
        mgl.ClearScreen();
        MGL_srand(123);
        let scrn = mgl.get_screen();
        for _ in 0..221 {
            scrn[(MGL_random(640) + MGL_random(480) * 640) as usize] = starColorTable[MGL_random(9) as usize];
        }
    }

    let mut frm: c_int = 0;
    if offsetdir == 0 {
        if pickerpos < 4 {
            frm = pickerpos as c_int * 4;
            frm += pickeroffset as c_int;
            if frm < 0 {
                frm += 16;
            }
        } else if pickerpos == 4 {
            frm = 23 + pickeroffset as c_int;
        } else if pickerpos == 5 {
            frm = 19 + pickeroffset as c_int;
        }
    } else if offsetdir == 1 {
        frm = 16 + pickeroffset as c_int;
    } else if offsetdir == 2 {
        frm = 19 + pickeroffset as c_int;
    }

    planet.GetSprite(frm).Draw(320, 240, mgl);

    match pickerpos {
        0 | 1 | 2 | 3 => {
            // regular places
            planet.GetSprite(44).Draw(14, 34, mgl);
            FontPrintString(25, 2, cstr!("Previous World"), &pickerFont);
            planet.GetSprite(42).Draw(14, 54, mgl);
            FontPrintString(25, 22, cstr!("Next World"), &pickerFont);
            planet.GetSprite(43).Draw(14, 74, mgl);
            FontPrintString(25, 42, cstr!("Custom Worlds"), &pickerFont);
            planet.GetSprite(41).Draw(14, 94, mgl);
            FontPrintString(25, 62, cstr!("Asylum"), &pickerFont);
        }
        4 => {
            // asylum
            planet.GetSprite(43).Draw(14, 34, mgl);
            FontPrintString(25, 2, cstr!("Normal Worlds"), &pickerFont);
        }
        5 => {
            // custom
            planet.GetSprite(41).Draw(14, 34, mgl);
            FontPrintString(25, 2, cstr!("Normal Worlds"), &pickerFont);
            planet.GetSprite(44).Draw(14, 54, mgl);
            FontPrintString(25, 22, cstr!("Previous Custom"), &pickerFont);
            planet.GetSprite(42).Draw(14, 74, mgl);
            FontPrintString(25, 42, cstr!("Next Custom"), &pickerFont);
        }
        _ => {}
    }

    let worldname = cstr!["Cavernous Caves", "Icy Mountain", "Spooky Forest",
        "Dusty Desert", "Crazy Asylum Of Madness", "Custom World"][pickerpos as usize];
    FontPrintString(2, if pickerpos == 5 { 440 } else { 460 }, worldname, &pickerFont);

    let world = if pickerpos < 5 { pickerpos } else { curCustom };
    let f = ::player::PlayerGetPercent(pickerpos) * 100.0;
    if ::player::PlayerHasLunacyKey(world) {
        planet.GetSprite(24 + (keyAnim / 4) as c_int).Draw(570, 400, mgl);
    }
    let mut txt = [0; 32];
    if f > 99.9 {
        sprintf!(txt, "Complete: 100%",);
    } else {
        sprintf!(txt, "Complete: {:02.1}%", f);
    }
    FontPrintString(388, 460, decay!(&txt), &pickerFont);

    if pickerpos == 5 { // customs show which custom world is selected
        FontPrintString(2, 460, if player.customName[curCustom as usize][0] == 0 {
            cstr!("None Available")
        } else {
            lvlName.as_ptr()
        }, &pickerFont);
    }
    let f = ::player::PlayerGetGamePercent() * 100.0;
    if f > 99.9 {
        sprintf!(txt, "Total: 100%",);
    } else {
        sprintf!(txt, "Total: {:03.1}%", f);
    }
    FontPrintString(446, 2, decay!(&txt), &pickerFont);
    mgl.Flip();
}

#[no_mangle]
pub unsafe extern fn WorldPicker(mgl: &mut MGLDraw) -> u8 {
    use control::*;

    let mut exitcode = 254;
    let mut lastTime = TIME_PER_FRAME;

    for i in 0..5 {
        player.totalCompletion[i] = GetWorldPoints(player.customName[i].as_ptr());
    }

    if ::player::PlayerGetMusicSettings() == ::options::Music::On {
        ::music::CDPlay(3); // world picker theme
    }

    mgl.ClearScreen();
    mgl.Flip();
    let planetSpr = sprite_set_t::load("graphics/planet.jsp").unwrap();
    mgl.LoadBMP(cstr!("graphics/picker.bmp"));
    let pickerFont = ::jamulfont::load_font(cstr!("graphics/gillsans4.jft")).ok().unwrap();

    pickerpos = 0;
    pickeroffset = 0;
    offsetdir = 0;
    curCustom = 5;
    oldc = GetControls() | GetArrows();

    numRunsToMakeUp = 0;
    while exitcode == 254 {
        ::game::HandleCDMusic();

        let start = timeGetTime();
        exitcode = PickerRun(&mut lastTime, mgl);
        if numRunsToMakeUp > 0 {
            PickerDraw(mgl, &planetSpr, &pickerFont);
        }

        if !mgl.Process() {
            exitcode = 255;
        }
        lastTime += timeGetTime() - start;
    }

    if pickerpos == 5 { // custom world
        player.worldNum = curCustom;
    }

    mgl.ClearScreen();
    mgl.Flip();
    mgl.LoadBMP(cstr!("graphics/title.bmp"));
    mgl.GammaCorrect(::display::GetGamma());
    ::mgldraw::MGL_srand(timeGetTime() as i32);
    return exitcode;
}
