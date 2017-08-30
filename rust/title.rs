use libc::{c_int, c_char};
use ffi::win::timeGetTime;
use jamulspr::sprite_set_t;
use jamulfont::mfont_t;
use mgldraw::MGLDraw;
use player::{player, MAX_CUSTOM};
use game::TIME_PER_FRAME;
use world::{GetWorldName, GetWorldPoints};

const TAGLINE: *const c_char = cstr!("Version 3.1");
const COPYRIGHT: *const c_char = cstr!("Copyright 1998-2011, Hamumu Software");

#[repr(C)]
pub struct title_t {
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

static mut numRunsToMakeUp: c_int = 0;
static mut pickerpos: u8 = 0;
static mut pickeroffset: i8 = 0;
static mut offsetdir: u8 = 0;
static mut curCustom: u8 = 0;

static mut keyAnim: u8 = 0;
static mut lvlName: [c_char; 32] = [0; 32];

static mut oldc: ::control::Controls = ::control::EMPTY;

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
    let f = ::player::PlayerGetPercent(world) * 100.0;
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
    let pickerFont = mfont_t::load("graphics/gillsans4.jft").unwrap();

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

// rule out the regular game worlds, so they don't show up as custom worlds
unsafe fn custom_world(fname: *const c_char) -> bool {
    use libc::strcmp;
    strcmp(fname, cstr!("forest.dlw")) != 0 &&
        strcmp(fname, cstr!("desert.dlw")) != 0 &&
        strcmp(fname, cstr!("icymount.dlw")) != 0 &&
        strcmp(fname, cstr!("caverns.dlw")) != 0 &&
        strcmp(fname, cstr!("asylum.dlw")) != 0 &&
        strcmp(fname, cstr!("backup_load.dlw")) != 0 &&
        strcmp(fname, cstr!("backup_exit.dlw")) != 0
}

#[no_mangle]
pub unsafe extern fn ScanWorldNames() {
    use ffi::win::{_finddata_t, _findfirst, _findnext, _findclose};
    use libc::strncpy;

    for i in 5..MAX_CUSTOM {
        player.customName[i][0] = 0;
    }

    let mut filedata: _finddata_t = ::std::mem::zeroed();
    let hFile = _findfirst(cstr!("worlds/*.dlw"), &mut filedata);
    if hFile == -1 {
        return;
    }

    // there's at least one
    let mut index = 5;
    if custom_world(filedata.name.as_ptr()) {
        strncpy(player.customName[index].as_mut_ptr(), filedata.name.as_ptr(), 32);
        index += 1;
    }

    while index < MAX_CUSTOM {
        if _findnext(hFile, &mut filedata) != 0 {
            break; // no more files
        }
        if custom_world(filedata.name.as_ptr()) {
            strncpy(player.customName[index].as_mut_ptr(), filedata.name.as_ptr(), 32);
            index += 1;
        }
    }

    _findclose(hFile);
}

#[no_mangle]
pub unsafe extern fn ReScanWorldNames() {
    use ffi::win::{_finddata_t, _findfirst, _findnext, _findclose};
    use libc::{strncpy, strcmp};

    let mut okay = [false; MAX_CUSTOM];

    for i in 5..MAX_CUSTOM {
        okay[i] = player.customName[i][0] == 0;
    }

    let mut filedata: _finddata_t = ::std::mem::zeroed();
    let hFile = _findfirst(cstr!("worlds/*.dlw"), &mut filedata);
    while hFile != -1 { // there's at least one
        if custom_world(filedata.name.as_ptr()) {
            let mut found = false;
            for i in 5..MAX_CUSTOM {
                if strcmp(filedata.name.as_ptr(), player.customName[i].as_ptr()) == 0 {
                    okay[i] = true;
                    found = true;
                    break;
                }
            }
            if !found { // none of the files matched, this is a new one
                // add it in, if there's room
                for i in 5..MAX_CUSTOM {
                    if player.customName[i][0] == 0 {
                        strncpy(player.customName[i].as_mut_ptr(), filedata.name.as_ptr(), 32);
                        break;
                    }
                }
            }
        }

        if _findnext(hFile, &mut filedata) != 0 {
            break
        }
    }
    _findclose(hFile);

    // remove any that aren't valid
    for i in 5..MAX_CUSTOM {
        if !okay[i] {
            player.customName[i][0] = 0;
        }
    }
}

unsafe fn CommonMenuUpdate(title: &mut title_t, bouapha: c_int, doctor: c_int) {
	title.titleBright += title.titleDir;
	if title.titleBright > 31 {
		title.titleDir = -1;
		title.titleBright = 31;
	}
	if title.titleDir < 0 && title.titleBright == 0 {
		title.titleDir = 0;
    }

    move_towards!(title.bouaphaX, bouapha, 8);
    move_towards!(title.doctorX, doctor, 8);
    move_towards!(title.blueY, 0, 8);

	title.expando += title.dexpando as c_int;
	if title.expando > 79 {
		title.dexpando = (-title.dexpando as c_int * 13 / 16) as i8;
		title.expando = 79;
	} else {
		title.dexpando += 1;
    }
}

unsafe fn CommonMenuDisplay(mgl: &mut MGLDraw, title: &title_t, titleSpr: &sprite_set_t) {
    let mut color = 0;
    {
        let deltaColor = (12 * 65536) / (480 - title.blueY);
        let mut scrn = mgl.get_screen();
        if title.blueY > 0 {
            let (init, rest) = {scrn}.split_at_mut(640 * title.blueY as usize);
            for p in init.iter_mut() {
                *p = 0;
            }
            scrn = rest;
        }
        for _ in title.blueY..480 {
            let (init, rest) = {scrn}.split_at_mut(640);
            for p in init.iter_mut() {
                *p = (color / 65536 + 96) as u8;
            }
            scrn = rest;
            color += deltaColor;
        }
    }

    // draw Dr. L & Bouapha
    titleSpr.GetSprite(0).Draw(640 - title.doctorX, 480, mgl);
    titleSpr.GetSprite(1).Draw(title.bouaphaX, 480, mgl);

    // draw the title parts
    titleSpr.GetSprite(2).DrawBright(240, 30, mgl, title.titleBright); // SPISPOPD II:
    titleSpr.GetSprite(3).DrawBright(290, 125, mgl, title.titleBright); // DR. LUNATIC

    // LoonyMod and version number
    ::display::CenterPrint(320, 120, cstr!("LoonyMod"), 0, 0);
    ::display::CenterPrint(321, 171, TAGLINE, 1, 1);
    ::display::CenterPrint(320, 170, TAGLINE, 0, 1);

    // Copyright
    ::display::Print(3, 467, COPYRIGHT, 1, 1);
    ::display::Print(2, 466, COPYRIGHT, 0, 1);
}

unsafe fn MainMenuDisplay(mgl: &mut MGLDraw, title: &title_t, titleSpr: &sprite_set_t) {
    CommonMenuDisplay(mgl, title, titleSpr);

    // now the menu options
    macro_rules! one {
        ($i:expr, $s:expr, $x:expr, $y:expr) => {
            titleSpr.GetSprite($s + if title.cursor == $i { 1 } else { 0 }).Draw($x, $y, mgl);
        }
    }
    one!(0, 9, 240, 270);
    one!(1, 11, 260, 300);
    one!(2, 13, 280, 330);
    one!(3, 15, 300, 360);
    one!(5, 19, 340, 420);
    one!(6, 21, 360, 450);
}

unsafe fn MainMenuUpdate(mgl: &mut MGLDraw, title: &mut title_t) -> u8 {
    use control::*;
    use sound::*;

    // update graphics
    CommonMenuUpdate(title, 0, 0);

    // now real updating
    let c = GetControls() | GetArrows();

    static mut reptCounter: u8 = 0;
    reptCounter += 1;
    if oldc.is_empty() || reptCounter > 10 {
        reptCounter = 0;
    }

    if reptCounter == 0 {
        if c.contains(CONTROL_UP) {
            if title.cursor == 0 {
                title.cursor = 6;
            } else {
                title.cursor -= 1;
                #[cfg(not(feature="demo"))] {
                    // ordering is not a viable option in the non-shareware
                    if title.cursor == 4 {
                        title.cursor = 3;
                    }
                }
            }
            MakeNormalSound(Sound::SND_MENUCLICK);
        }
        if c.contains(CONTROL_DN) {
            title.cursor += 1;
            if title.cursor == 7 {
                title.cursor = 0;
            }
            #[cfg(not(feature="demo"))] {
                // ordering is not a viable option in the non-shareware
                if title.cursor == 4 {
                    title.cursor = 5;
                }
            }
            MakeNormalSound(Sound::SND_MENUCLICK);
        }
    }

    if (c - oldc).intersects(CONTROL_B1 | CONTROL_B2) {
        MakeNormalSound(Sound::SND_MENUSELECT);
        return 1;
    }
    oldc = c;

    if mgl.LastKeyPressed() == 27 {
        MakeNormalSound(Sound::SND_MENUSELECT);
        return 2;
    }

    ::game::HandleCDMusic();
    0
}

pub unsafe fn MainMenu(mgl: &mut MGLDraw) -> u8 {
    use ffi::win::{timeGetTime, Sleep};

    if ::options::opt.music == ::options::Music::On {
        ::music::CDPlay(2); // the title theme
    }
    ::music::CDNeedsUpdating();

    mgl.LoadBMP(cstr!("graphics/title.bmp"));
    mgl.LastKeyPressed();
    mgl.ClearScreen();
    oldc = ::control::CONTROL_B1 | ::control::CONTROL_B2;
    let titleSpr = sprite_set_t::load("graphics/titlespr.jsp").unwrap();

    let mut title = title_t {
        bouaphaX: -320,
        doctorX: -320,
        titleBright: -32,
        titleDir: 4,
        cursor: 0,
        blueY: 479,
        expando: 0,
        dexpando: 0,
        savecursor: 0,
        percent: [0.0; 3],
    };

    let mut startTime = timeGetTime();
    let mut b = 0;
    while b == 0 {
        let runStart = timeGetTime();
        b = MainMenuUpdate(mgl, &mut title);
        MainMenuDisplay(mgl, &title, &titleSpr);
        mgl.Flip();
        let diff = timeGetTime() - runStart;

        if diff < 1000 / 50 {
            Sleep(1000 / 50 - diff);
        }

        if !mgl.Process() {
            ::music::CDStop();
            return 255;
        }
        if b == 1 && title.cursor == 1 { // selected Load Game
            if GameSlotPicker(mgl, &mut title, &titleSpr) == 0 { // Pressed ESC on the slot picker
                b = 0;
            }
            startTime = timeGetTime();
        }
        if b == 1 && title.cursor == 2 { // options
            ::options::OptionsMenu(mgl);
            startTime = timeGetTime();
        }
        if b == 1 && title.cursor == 5 { // help
            HelpScreens(mgl);
            startTime = timeGetTime();
        }
        if timeGetTime() - startTime > 1000 * 20 {
            Credits(mgl);
            startTime = timeGetTime();
        }
    }
    if b == 1 { // something was selected
        if title.cursor == 6 { // exit
            255
        } else {
            title.cursor
        }
    } else {
        255 // ESC was pressed
    }
}

unsafe fn GameSlotPickerDisplay(mgl: &mut MGLDraw, title: &title_t, titleSpr: &sprite_set_t) {
    CommonMenuDisplay(mgl, title, titleSpr);

    // now the game slots
    let mut txt = [0; 18];
    for i in 0..3 {
        if title.percent[i] > 99.9 {
            sprintf!(txt, "Slot {} - 100%", i + 1);
        } else {
            sprintf!(txt, "Slot {} - {:03.1}%", i + 1, title.percent[i]);
        }

        ::display::Print(180 + 30 * i as c_int, 220 + 70 * i as c_int, decay!(&txt),
            -6 + if title.savecursor == i as u8 { 12 } else { 0 }, 0);
    }
}

unsafe fn GameSlotPickerUpdate(mgl: &mut MGLDraw, title: &mut title_t) -> u8 {
    use control::*;
    use sound::*;

    // update graphics
    CommonMenuUpdate(title, -60, -40);

    // now real updating
    let c = GetControls() | GetArrows();

    static mut reptCounter: u8 = 0;
    reptCounter += 1;
    if oldc.is_empty() || reptCounter > 10 {
        reptCounter = 0;
    }

    if reptCounter == 0 {
        if c.contains(CONTROL_UP) {
            if title.savecursor == 0 {
                title.savecursor = 2;
            } else {
                title.savecursor -= 1;
            }
            MakeNormalSound(Sound::SND_MENUCLICK);
        }
        if c.contains(CONTROL_DN) {
            title.savecursor += 1;
            if title.savecursor > 2 {
                title.savecursor = 0;
            }
            MakeNormalSound(Sound::SND_MENUCLICK);
        }
    }
    if (c - oldc).intersects(CONTROL_B1 | CONTROL_B2) {
        MakeNormalSound(Sound::SND_MENUSELECT);
        return 1;
    }
    oldc = c;

    if mgl.LastKeyPressed() == 27 {
        MakeNormalSound(Sound::SND_MENUSELECT);
        return 2;
    }

    ::game::HandleCDMusic();
    0
}

unsafe fn InitGameSlotPicker(mgl: &mut MGLDraw, title: &mut title_t) {
    let f = ::mgldraw::AppdataOpen(cstr!("loony.sav"), cstr!("rb"));
    if f.is_null() {
        for pct in title.percent.iter_mut() {
            *pct = 0.0;
        }
    } else {
        let mut p: ::player::player_t = ::std::mem::zeroed();
        for pct in title.percent.iter_mut() {
            ::libc::fread(decay!(&mut p), szof!(::player::player_t), 1, f);
            *pct = ::pause::CalcTotalPercent(&p) * 100.0;
        }
    }
    mgl.LastKeyPressed();
    oldc = ::control::CONTROL_B1 | ::control::CONTROL_B2;
}

#[no_mangle] // x
pub unsafe extern fn GameSlotPicker(mgl: &mut MGLDraw, title: &mut title_t, titleSpr: &sprite_set_t) -> u8 {
    use ffi::win::{timeGetTime, Sleep};

    title.savecursor = 0;
    InitGameSlotPicker(mgl, title);

    let mut b = 0;
    while b == 0 {
        let start = timeGetTime();
        b = GameSlotPickerUpdate(mgl, title);
        GameSlotPickerDisplay(mgl, title, titleSpr);
        mgl.Flip();
        let diff = timeGetTime() - start;

        if diff < 1000 / 50 {
            Sleep((1000 / 50) - diff);
        }

        if !mgl.Process() {
            return 0;
        }
    }

    if b == 1 { // something was selected
        ::player::InitPlayer(::player::Init::Game, 0, 0);
        ::player::PlayerLoadGame(title.savecursor);
		// make it remember which was picked so the pause menu will start on the same
		::pause::SetSubCursor(title.savecursor);
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern fn CreditsRender(mgl: &mut MGLDraw, y: c_int, document: &[*const c_char]) {
    let mut i = 0;
    let mut ypos = 0;

    while i < document.len() && ypos - y < 480 {
        let s = document[i];
        if ypos - y > -60 {
            if *s == b'@' as i8 {
                ::display::CenterPrint(320, ypos - y, s.offset(1), 0, 0);
            } else if *s == b'#' as i8 {
                mgl.FillBox(320 - 200, ypos - y + 8, 320 + 200, ypos - y + 11, 255);
            } else if *s == b'%' as i8 {
                mgl.FillBox(320 - 70, ypos - y + 8, 320 + 70, ypos - y + 9, 255);
            } else {
                ::display::CenterPrint(320, ypos - y, s, 0, 1);
            }
        }
        ypos += 20;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern fn Credits(mgl: &mut MGLDraw) {
    let mut y = -470;
    let mut flip = false;

    mgl.LastKeyPressed();
    mgl.LoadBMP(cstr!("graphics/title.bmp"));
    loop {
        mgl.ClearScreen();
        CreditsRender(mgl, y, CREDITS);

        ::game::HandleCDMusic();

        // only scroll every other frame
        flip = !flip;
        if flip { y += 1; }

        mgl.Flip();
        if !mgl.Process() || mgl.LastKeyPressed() != 0 || y >= END_OF_CREDITS {
            break
        }
    }
}

#[no_mangle]
pub unsafe extern fn VictoryText(mgl: &mut MGLDraw) {
    let mut y = -470;

    mgl.LastKeyPressed();
    mgl.LoadBMP(cstr!("graphics/title.bmp"));
    loop {
        mgl.ClearScreen();
        CreditsRender(mgl, y, VICTORY_TEXT);

        ::game::HandleCDMusic();

        y += 1;

        mgl.Flip();
        if !mgl.Process() || mgl.LastKeyPressed() == 27 || y >= END_OF_VICTORY {
            break
        }
    }
}

unsafe fn SpeedSplash(mgl: &mut MGLDraw, fname: *const c_char) -> bool {
    use control::*;
    use mgldraw::palette_t;

    mgl.LastKeyPressed();
    oldc = GetControls() | GetArrows();

    let mut curpal = [palette_t { alpha: 0, red: 0, green: 0, blue: 0 }; 256];
    let mut desiredpal = curpal;
    mgl.LoadBMP(fname);
    mgl.get_palette(&mut desiredpal);
    mgl.set_palette(&curpal);

    let mut mode = 0;
    let mut clock = 0;

    loop {
        mgl.Flip();
        if !mgl.Process() {
            return false;
        }
        let c = mgl.LastKeyPressed();
        if c == 27 {
            return false;
        } else if c != 0 {
            mode = 2;
        }

        ::game::HandleCDMusic();

        let c = GetControls() | GetArrows();
        if (c - oldc).intersects(CONTROL_B1 | CONTROL_B2) {
            mode = 2;
        }
        oldc = c;

        clock += 1;
        match mode {
            0 => { // fading in
                for _ in 0..16 {
                    for (cur, des) in curpal.iter_mut().zip(desiredpal.iter()) {
                        if cur.red < des.red {
                            cur.red += 1;
                        }
                        if cur.green < des.green {
                            cur.green += 1;
                        }
                        if cur.blue < des.blue {
                            cur.blue += 1;
                        }
                    }
                }
                mgl.set_palette(&curpal);
                if clock > 16 {
                    mode = 1;
                    clock = 0;
                }
            }
            1 => {} // sit around
            2 => { // fading out
                clock = 0;
                for _ in 0..16 {
                    for cur in curpal.iter_mut() {
                        if cur.red > 0 {
                            cur.red -= 1;
                        } else {
                            clock += 1;
                        }
                        if cur.green > 0 {
                            cur.green -= 1;
                        } else {
                            clock += 1;
                        }
                        if cur.blue > 0 {
                            cur.blue -= 1;
                        } else {
                            clock += 1;
                        }
                    }
                }
                mgl.set_palette(&curpal);
                if clock == 256 * 3 * 16 {
                    break;
                }
            }
            _ => {}
        }
    }
    mgl.ClearScreen();
    mgl.Flip();
    true
}

#[no_mangle]
pub unsafe extern fn HelpScreens(mgl: &mut MGLDraw) {
    let mut name = [0; 32];
    for i in 1..6 {
        sprintf!(name, "docs/help{}.bmp", i);
        if !SpeedSplash(mgl, decay!(&name)) {
            return;
        }
    }
}

#[no_mangle]
pub unsafe extern fn SplashScreen(mgl: &mut MGLDraw, fname: *const c_char, delay: c_int, sound: u8) {
    use control::*;
    use mgldraw::palette_t;

    mgl.LastKeyPressed();
    oldc = GetControls() | GetArrows();

    let mut curpal = [palette_t { alpha: 0, red: 0, green: 0, blue: 0 }; 256];
    let mut desiredpal = curpal;
    mgl.LoadBMP(fname);
    mgl.get_palette(&mut desiredpal);
    mgl.set_palette(&curpal);

    let mut mode = 0;
    let mut clock = 0;

    loop {
        mgl.Flip();
        if !mgl.Process() {
            return;
        }
        if mgl.LastKeyPressed() != 0 {
            mode = 2;
        }

        clock += 1;
        match mode {
            0 => { // fading in
                for _ in 0..8 {
                    for (cur, des) in curpal.iter_mut().zip(desiredpal.iter()) {
                        if cur.red < des.red {
                            cur.red += 1;
                        }
                        if cur.green < des.green {
                            cur.green += 1;
                        }
                        if cur.blue < des.blue {
                            cur.blue += 1;
                        }
                    }
                }
                mgl.set_palette(&curpal);
                if clock == 32 && sound == 2 {
                    ::sound::MakeNormalSound(::sound::Sound::SND_HAMUMU);
                }
                if clock > 64 {
                    mode = 1;
                    clock = 0;
                }
            }
            1 => {
                if clock > delay {
                    mode = 2;
                    clock = 0;
                }
            }
            2 => { // fading out
                clock = 0;
                for _ in 0..8 {
                    for cur in curpal.iter_mut() {
                        if cur.red > 0 {
                            cur.red -= 1;
                        } else {
                            clock += 1;
                        }
                        if cur.green > 0 {
                            cur.green -= 1;
                        } else {
                            clock += 1;
                        }
                        if cur.blue > 0 {
                            cur.blue -= 1;
                        } else {
                            clock += 1;
                        }
                    }
                }
                mgl.set_palette(&curpal);
                if clock == 256 * 3 * 8 {
                    break;
                }
            }
            _ => {}
        }
    }
    mgl.ClearScreen();
    mgl.Flip();
}

// once the credits have scrolled to END_OF_CREDITS pixels, they end
const END_OF_CREDITS: c_int = 480 * 4 + 180;
const END_OF_VICTORY: c_int = 480 * 2;

// special codes in the credits:
// @ = use GirlsRWeird font
// # = draw a major horizontal line
// % = draw a minor horizontal line
// $ = last line of the whole deal

const CREDITS: &[*const c_char] = &cstr![
    "SPISPOPD II",
    "@DR. LUNATIC",
    "",
    "",
    "Copyright 1998-2011, Hamumu Software",
    "#",
    "Original Concept",
    "Mike Hommel",
    "%",
    "Programming",
    "Mike Hommel",
    "Tad Hardesty",
    "%",
    "Character Design",
    "Mike Hommel",
    "%",
    "Level Design",
    "Mike Hommel",
    "%",
    "3D Graphics",
    "Mike Hommel",
    "%",
    "2D Graphics",
    "Mike Hommel",
    "%",
    "Sound Effects",
    "Brent Christian",
    "Mike Hommel",
    "(Surprise!)",
    "%",
    "Music",
    "Brent Christian",
    "%",
    "Producer/Designer/Director",
    "Mike Hommel",
    "%",
    "Gaffer",
    "Mike Hommel",
    "%",
    "QA Director",
    "Angela Finer",
    "%",
    "Testing",
    "Baba",
    "Brent Christian",
    "Jim Crawford",
    "Chris Dillman",
    "Angela Finer",
    "Tim Finer",
    "Dawn Genge",
    "Mattie Goodman",
    "Matt Guest",
    "Suzanne Hommel",
    "Solange Hunt",
    "Brad Kasten",
    "Geoff Michell",
    "Britt Morris",
    "Trevor Strohman",
    "Peter Young",
    "%",
    "Ideas & Hamumu Theme",
    "Mattie Goodman",
    "%",
    "Technical Assistance",
    "Trevor Strohman",
    "%",
    "Special Thanks",
    "Junebug Superspy &",
    "Too Much Hot Sauce",
    "All the SpisFans",
    "(both of them that is)",
    "Ketmany Bouapha (NO, Bouapha!)",
    "Rinley \"Dirty\" Deeds",
    "%",
    "Henry G., wherefore art thou?",
    "%",
    "\"it's certainly pointless",
    "and annoying, but i say you",
    "should keep it anyways.\"",
    "The words of a true SpisFan",
    "%",
    "#",
    "Stop by www.hamumu.com!",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "@THE END",
];

const VICTORY_TEXT: &[*const c_char] = &cstr![
    "@With Dr. Lunatic vanquished, the",
    "",
    "",
    "@zombie menace was ended. Never again",
    "",
    "",
    "@would someone take the brains out of",
    "",
    "",
    "@zombies and put them into other",
    "",
    "",
    "@zombies to create a race of super",
    "",
    "",
    "@zombies.",
    "",
    "",
    "#",
    "",
    "@Bouapha was the hero of the hour,",
    "",
    "",
    "@loved and respected by all. There",
    "",
    "",
    "@were parades and parties for days.",
    "",
    "",
    "@Until the president got himself into",
    "",
    "",
    "@another madcap scandal, and everyone",
    "",
    "",
    "@forgot about Bouapha and the zombies",
    "",
    "",
    "@altogether.",
    "",
    "",
    "#",
];
