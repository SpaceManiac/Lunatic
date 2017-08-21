use mgldraw::{MGLDraw, AppdataOpen};
use libc::{fread, fwrite, fclose, c_int};

#[repr(u8)]
#[derive(FromInt, Copy, Clone, PartialEq, Eq)]
pub enum Music {
    Off = 0,
    On,
    Random,
}

impl Music {
    pub fn cycle(self) -> Self {
        use self::Music::*;
        match self {
            Off => On,
            On => Random,
            Random => Off,
        }
    }
}

#[repr(u8)]
#[derive(FromInt, Copy, Clone, PartialEq, Eq)]
pub enum PlayAs {
    Bouapha = 0,
    Lunatic,
    Happy,
}

impl PlayAs {
    pub fn cycle(self, secrets: bool) -> Self {
        use self::PlayAs::*;
        match (self, secrets) {
            (Bouapha, _) => Lunatic,
            (Happy, _) |
            (Lunatic, false) => Bouapha,
            (Lunatic, true) => Happy,
        }
    }
}

#[repr(C)]
pub struct options_t {
    /// key scancodes
    pub control: [[u8; 6]; 2],
    /// joystick 'codes' for the buttons
    pub joyCtrl: [u8; 2],
    pub sound: bool,
    pub music: Music,
    /// if you wish to play as a different character after winning
    pub playAs: PlayAs,
    pub wonGame: bool,
    pub gotAllSecrets: bool,
    pub youSuck: bool,
    // new options
    pub discoMode: bool,
    pub smoothLight: bool,
}

const DEFAULT_OPTIONS: options_t = options_t {
    sound: true,
    music: Music::On,
    playAs: PlayAs::Bouapha,
    wonGame: false,
    gotAllSecrets: false,
    smoothLight: true, // new
    discoMode: false, // new
    control: [
        [
            84, // up
            85, // down,
            82, // left
            83, // right
            117, // hammer: CTRL
            115, // special: shift
        ],
        [
            0, // up
            0, // down
            0, // left
            0, // right
            26, // hammer: Z
            24, // special: X
        ]
    ],
    joyCtrl: [0, 1],
    youSuck: false,
};

#[no_mangle]
pub static mut opt: options_t = DEFAULT_OPTIONS;
#[no_mangle]
pub static mut oldPlayAs: u8 = 0;

static mut cursor: u8 = 0;
static mut oldc: ::control::Controls = ::control::EMPTY;
static mut oldBtn: u32 = 0;
static mut controlX: u8 = 0;
static mut controlY: u8 = 0;
static mut optMode: OptMode = OptMode::Menu;

#[repr(u8)]
#[derive(PartialEq)]
enum OptMode {
    Menu,
    Controls,
    KeyPress,
    JoyPress,
}

#[no_mangle]
pub unsafe extern fn LoadOptions() {
    let f = ::mgldraw::AppdataOpen(cstr!("lunatic.cfg"), cstr!("rb"));
    if f.is_null() {
        opt = DEFAULT_OPTIONS;
    } else {
        fread(decay!(&mut opt), szof!(options_t), 1, f);
        fclose(f);
    }
    ::control::ApplyControlSettings();
}

#[no_mangle]
pub unsafe extern fn SaveOptions() {
    let f = AppdataOpen(cstr!("lunatic.cfg"), cstr!("wb"));
    assert!(!f.is_null());
    fwrite(decay!(&opt), szof!(options_t), 1, f);
    fclose(f);
}

unsafe fn UpdateOptionsMenu(mgl: &mut MGLDraw) -> u8 {
    use control::*;

    ::game::HandleCDMusic();

    match optMode {
        OptMode::Menu => { // just going through options
            let c = mgl.LastKeyPressed();
            let c2 = GetControls() | GetArrows();

            if c == 27 {
                return 1;
            } else if c == b'u' {
                if opt.gotAllSecrets {
                    opt.gotAllSecrets = false;
                    opt.wonGame = false;
                } else {
                    opt.gotAllSecrets = true;
                    opt.wonGame = true;
                }
            }

            if (c2 - oldc).contains(CONTROL_UP) {
                cursor = cursor.checked_sub(1).unwrap_or(7);
            }
            if (c2 - oldc).contains(CONTROL_DN) {
                cursor += 1;
                if cursor > 7 { cursor = 0; }
            }
            if (c2 - oldc).intersects(CONTROL_B1 | CONTROL_B2 | CONTROL_B3) {
                match cursor {
                    0 => { opt.sound = !opt.sound; }
                    1 => {
                        opt.music = opt.music.cycle();
                        ::player::PlayerSetMusicSettings(opt.music);
                        ::music::CDNeedsUpdating();
                        ::music::CDStop();
                        if opt.music == Music::On {
                            let i = ::game::GetCurSong();
                            let i = if i == 3 { 2 } else { i };
                            ::music::CDPlay(i as c_int);
                        }
                    }
                    2 if opt.wonGame => {
                        opt.playAs = opt.playAs.cycle(opt.gotAllSecrets);
                    }
                    3 if opt.wonGame => {
                        opt.discoMode = !opt.discoMode;
                    }
                    4 => {
                        opt.smoothLight = !opt.smoothLight;
                    }
                    5 => {
                        opt.youSuck = !opt.youSuck;
                    }
                    6 => {
                        optMode = OptMode::Controls;
                        controlX = 0;
                        controlY = 0;
                    }
                    7 => { return 1; }
                    _ => {}
                }
            }
            oldc = c2;
        }
        OptMode::Controls => { // selecting keys to configure
            let c = mgl.LastKeyPressed();
            let mut c2 = GetControls() | GetArrows();

            if c == 27 {
                optMode = OptMode::Menu;
                controlX = 10;
                ApplyControlSettings();
                return 0;
            } else if c == 13 {
                c2 |= CONTROL_B1;
            }

            if (c2 - oldc).contains(CONTROL_UP) {
                controlY = controlY.checked_sub(1).unwrap_or(5);
            }
            if (c2 - oldc).contains(CONTROL_DN) {
                controlY += 1;
                if controlY > 5 { controlY = 0; }
            }
            if (c2 - oldc).contains(CONTROL_LF) {
                controlX = controlX.checked_sub(1).unwrap_or(2);
            }
            if (c2 - oldc).contains(CONTROL_RT) {
                controlX += 1;
                if controlX > 2 { controlX = 0; }
            }
            if (c2 - oldc).intersects(CONTROL_B1 | CONTROL_B2 | CONTROL_B3) {
                if controlX < 2 { // keyboard
                    optMode = OptMode::KeyPress;
                    LastScanCode();
                } else if controlY > 3 {
                    // btn = 0;
                    oldBtn = !0;
                    optMode = OptMode::JoyPress;
                }
            }

            oldc = c2;
        }
        OptMode::KeyPress => { // entering a specific key
            let c2 = LastScanCode();
            if c2 == 59 { // ESC key
                optMode = OptMode::Controls;
                mgl.LastKeyPressed();
                oldc = Controls::all();
                return 0;
            } else if c2 != 0 && c2 != 67 { // 67 = enter
                opt.control[controlX as usize][controlY as usize] = c2;
                optMode = OptMode::Controls;
                mgl.LastKeyPressed();
                oldc = Controls::all();
            }
        }
        OptMode::JoyPress => { // pressing a joystick button
            let c = mgl.LastKeyPressed();
            if c == 27 {
                optMode = OptMode::Controls;
                oldc = Controls::all();
                return 0;
            }

            let btn = GetJoyButtons();
            let mut j = 1;
            for i in 0..16 {
                if (btn & j) != 0 && (oldBtn & j) == 0 {
                    opt.joyCtrl[controlY as usize - 4] = i;
                    optMode = OptMode::Controls;
                    oldc = Controls::all();
                }
                j <<= 1;
            }
            oldBtn = btn;
        }
    }

    0
}

unsafe fn RenderOptionsMenu(mgl: &mut MGLDraw) {
    use display::{CenterPrint, Print};

    let onoff = cstr!["Off", "On", "Random"];
    let playAs = cstr!["Bouapha", "Dr. Lunatic", "Happy Stick Man"];
    let youSuck = cstr!["Sanitized", "Classic"];

    mgl.ClearScreen();
    CenterPrint(320, 2, cstr!("Game Options"), 0, 0);

    let dy = 18_i32;
    let mut y = 80 - dy;
    macro_rules! option {
        ($name:expr, $value:expr) => {
            y += dy;
            CenterPrint(320, y, cstr!($name), 0, 1);
            Print(392, y, $value, 0, 1);
        }
    }

    mgl.FillBox(250, 80 - 1 + dy * cursor as c_int, 390, 80 + 12 + dy * cursor as c_int, 10);
    option!("Sound", onoff[opt.sound as usize]);
    option!("Music", onoff[opt.music as usize]);
    if !opt.wonGame {
        option!("?????", cstr!());
        option!("?????", cstr!());
    } else {
        option!("Play As", playAs[opt.playAs as usize]);
        option!("Disco Mode", onoff[opt.discoMode as usize]);
    }
    option!("Smooth Lighting", onoff[opt.smoothLight as usize]);
    option!("Game Over Msg.", youSuck[opt.youSuck as usize]);

    option!("Configure Controls", cstr!());
    option!("Exit To Main Menu", cstr!());

    RenderControls(120, 230, mgl);
}

unsafe fn RenderControls(x: c_int, y: c_int, mgl: &mut MGLDraw) {
    use display::CenterPrint;
    use control::ScanCodeText;

    let dirName = cstr!["Up", "Down", "Left", "Right", "Hammer", "Weapon"];
    let mut btnTxt = [0; 64];

    mgl.FillBox(x, y - 2, x + 398, y + 20, 16);
    CenterPrint(x + 50, y + 2, cstr!("Control"), 0, 1);
    CenterPrint(x + 150, y + 2, cstr!("Keyboard1"), 0, 1);
    CenterPrint(x + 250, y + 2, cstr!("Keyboard2"), 0, 1);
    CenterPrint(x + 350, y + 2, cstr!("Joystick"), 0, 1);
    mgl.Box(x + 98, y - 2, x + 198, y + 200, 16);
    mgl.Box(x + 198, y - 2, x + 298, y + 200, 16);
    mgl.Box(x + 298, y - 2, x + 398, y + 200, 16);

    for i in 0..6 {
        if controlY as c_int == i && controlX < 3 {
            if optMode == OptMode::Controls {
                mgl.FillBox(
                    x + 99 + 100 * controlX as c_int,
                    y + 20 + 1 + i * 30,
                    x + 198 + 100 * controlX as c_int,
                    y + 20 + 29 + i * 30,
                    20);
            } else {
                mgl.FillBox(
                    x + 99 + 100 * controlX as c_int,
                    y + 20 + 1 + i * 30,
                    x + 198 + 100 * controlX as c_int,
                    y + 20 + 29 + i * 30,
                    31);
                CenterPrint(x + 150 + controlX as c_int * 100, y + 27 + i * 30, cstr!("???"), 0, 1);
            }
        }

        mgl.FillBox(x, y + 20 + 1 + i * 30, x + 98, y + 20 + 29 + i * 30, 10);
        mgl.Box(x, y + 20 + i * 30, x + 398, y + 20 + 30 + i * 30, 16);
        CenterPrint(x + 50, y + 27 + i * 30, dirName[i as usize], 0, 1);
        if optMode == OptMode::Controls || controlX != 0 || controlY as i32 != i {
            CenterPrint(x + 150, y + 27 + i * 30, ScanCodeText(opt.control[0][i as usize]), 0, 1);
        }
        if optMode == OptMode::Controls || controlX != 1 || controlY as i32 != i {
            CenterPrint(x + 250, y + 27 + i * 30, ScanCodeText(opt.control[1][i as usize]), 0, 1);
        }

        if i > 3 {
            if optMode == OptMode::Controls || controlX != 2 || controlY as i32 != i {
                sprintf!(btnTxt, "Button {}", opt.joyCtrl[i as usize - 4] + 1);
                CenterPrint(x + 350, y + 27 + i * 30, decay!(&btnTxt), 0, 1);
            }
        } else {
            CenterPrint(x + 350, y + 27 + i * 30, dirName[i as usize], 16, 1);
        }
    }

    match optMode {
        OptMode::Menu => {
            CenterPrint(x + 200, 210 + 232, cstr!("Move with arrow keys, ENTER to select"), 0, 1);
            CenterPrint(x + 200, 210 + 252, cstr!("ESC to return to main menu"), 0, 1);
        }
        OptMode::Controls => {
            CenterPrint(x + 200, 210 + 232, cstr!("Select with arrow keys, ENTER to set new control"), 0, 1);
            CenterPrint(x + 200, 210 + 252, cstr!("ESC to return to options"), 0, 1);
        }
        OptMode::KeyPress => {
            sprintf!(btnTxt, "Press a key for {}", ::PctS(dirName[controlY as usize]));
            CenterPrint(x + 200, 210 + 232, decay!(&btnTxt), 0, 1);
            CenterPrint(x + 200, 210 + 252, cstr!("ESC to cancel"), 0, 1);
        }
        OptMode::JoyPress => {
            sprintf!(btnTxt, "Press a joystick button for {}", ::PctS(dirName[controlY as usize]));
            CenterPrint(x + 200, 210 + 232, decay!(&btnTxt), 0, 1);
            CenterPrint(x + 200, 210 + 252, cstr!("ESC to cancel"), 0, 1);
        }
    }
}

#[no_mangle]
pub unsafe extern fn OptionsMenu(mgl: &mut MGLDraw) {
    // InitOptionsMenu
    oldc = ::control::Controls::all();
    controlX = 10;
    cursor = 0;
    optMode = OptMode::Menu;
    // end InitOptionsMenu

    let mut done = 0;
    while done == 0 {
        done = UpdateOptionsMenu(mgl);
        RenderOptionsMenu(mgl);
        mgl.Flip();

        if !mgl.Process() {
            done = 1;
        }
    }

    // ExitOptionsMenu
    SaveOptions();
    // end ExitOptionsMenu
}
