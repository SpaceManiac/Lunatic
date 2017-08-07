use world::world_t;
use map::{self, special_t};
use display::{Print, GetDisplayMGL};
use mgldraw::MGLDraw;
use libc::{c_int, c_char, isprint, strlen};

static mut mapNum: u8 = 0;
static mut world: *mut world_t = 0 as *mut _;
static mut spcl: *mut special_t = 0 as *mut _;
static mut spclnum: c_int = 0;

#[no_mangle]
pub unsafe extern fn InitSpclDialog(theSpcl: *mut special_t, spnum: u8, wrld: *mut world_t, currentMap: u8) {
    world = wrld;
    mapNum = currentMap;
    spcl = theSpcl;
    spclnum = spnum as c_int;
}

#[no_mangle]
pub extern fn ExitSpclDialog() {}

#[no_mangle]
pub unsafe extern fn SpclDialogKey(key: c_char) -> u8 {
    if key == 27 { return 0; } // esc
    if key == 10 { // enter
        return 1;
    }

    let len = strlen((*spcl).msg.as_ptr()) as usize;
    if key == 8 { // backspace
        if len > 0 {
            (*spcl).msg[len - 1] = 0;
        }
        return 1;
    }

    if isprint(key as c_int) != 0 && len < 31 {
        (*spcl).msg[len] = key;
        (*spcl).msg[len + 1] = 0;
    }
    1
}

unsafe fn RenderCheckbox(x: c_int, y: c_int, v: bool, txt: *const c_char) {
    if v {
        GetDisplayMGL().FillBox(x, y, x + 11, y + 11, 16);
    } else {
        GetDisplayMGL().Box(x, y, x + 11, y + 11, 16);
    }
    Print(x + 13, y, txt, 0, 1);
}

unsafe fn RenderValueAdjuster(name: *const c_char, value: c_int, x: c_int, y: c_int) {
    let mgl = GetDisplayMGL();
    Print(x, y, name, 0, 1);
    mgl.Box(x, y + 14, x + 20, y + 28, 16);
    Print(x + 5, y + 16, cstr!("--"), 0, 1);
    mgl.Box(x + 22, y + 14, x + 32, y + 28, 16);
    Print(x + 25, y + 16, cstr!("-"), 0, 1);

    let mut s = [0; 8];
    sprintf!(s, "{:03}", value);
    Print(x + 38, y + 16, decay!(&s), 0, 1);

    mgl.Box(x + 68, y + 14, x + 78, y + 28, 16);
    Print(x + 70, y + 16, cstr!("+"), 0, 1);
    mgl.Box(x + 80, y + 14, x + 100, y + 28, 16);
    Print(x + 83, y + 16, cstr!("++"), 0, 1);
}

#[no_mangle]
pub unsafe extern fn RenderSpclDialog(_msx: c_int, _msy: c_int, mgl: &mut MGLDraw) {
    // box for the whole dialog
    mgl.FillBox(60, 10, 480, 370, 8);
    mgl.Box(60, 10, 480, 370, 16);

    let mut s = [0; 6];
    sprintf!(s, "#{:03}", spclnum);
    Print(445, 12, decay!(&s), 0, 1);

    // the trigger checkboxes
    Print(62, 12, cstr!("Trigger"), 0, 1);
    macro_rules! trigger {
        ([$x:expr, $y:expr] $trg:ident, $text:expr) => {
            RenderCheckbox($x, $y, (*spcl).trigger.contains(map::$trg), cstr!($text))
        }
    }
    trigger!([62, 26] TRG_STEP, "Player step");
    trigger!([62, 40] TRG_ENEMYSTEP, "Enemy step");
    trigger!([62, 54] TRG_NEAR, "Player within N tiles");
    trigger!([62, 68] TRG_PASSLEVELS, "Passed N levels");
    trigger!([62, 82] TRG_KEYCHAINS, "Have all keychains");
    trigger!([62, 96] TRG_KILLALL, "Kill all monster N");
    trigger!([62, 110] TRG_HAVEBRAINS, "Have N brains");
    trigger!([62, 124] TRG_SHOOT, "Shoot item/wall");
    trigger!([220, 54] TRG_CHAIN, "Chain off neighbor Special");
    trigger!([220, 68] TRG_TIMED, "Once every N seconds");
    trigger!([220, 82] TRG_RANDOM, "Random (N% per second)");
    trigger!([220, 96] TRG_HAVELOONY, "Have Lunacy Key");
    trigger!([220, 110] TRG_KILLONE, "Kill one monster N");
    trigger!([220, 124] TRG_FLOORHERE, "Floor N is here");

    trigger!([62, 148] TRG_REPEATABLE, "Multiple use");
    trigger!([62, 162] TRG_MESSAGE, "Show message");

    // the box where the text is entered
    Print(62, 341, cstr!("Message"), 0, 1);
    mgl.Box(62, 355, 362, 368, 16);
    mgl.FillBox(63, 356, 361, 367, 0);
    Print(64, 357, decay!(&(*spcl).msg), 0, 1);

    // the effect choices
    Print(62, 180, cstr!("Effect"), 0, 1);
    macro_rules! effect {
        ([$xr:expr, $y:expr] $ef:ident, $text:expr) => {
            mgl.Box($xr.start, $y, $xr.end, $y + 14, 16 + if (*spcl).effect == map::Effect::$ef { 15 } else { 0 });
            Print($xr.start + 2, $y + 2, cstr!($text), 0, 1);
        }
    }
    effect!([62..132, 194] SPC_NONE, "None");
    effect!([62..132, 210] SPC_SUMMON, "Summon");
    effect!([62..132, 226] SPC_ZAPWALL, "Zap Wall");
    effect!([62..132, 242] SPC_RAISEWALL, "MakeWall");
    effect!([62..132, 258] SPC_TOGGLEWALL, "TogglWall");
    effect!([62..132, 274] SPC_PLAYSONG, "PlaySong");
    effect!([62..132, 290] SPC_DROPITEM, "DropItem");
    effect!([62..132, 306] SPC_SWAPMAP, "SwapMap");

    effect!([134..204, 194] SPC_TELEPORT, "Teleport");
    effect!([134..204, 210] SPC_LIGHT, "Light");
    effect!([134..204, 226] SPC_GOTOMAP, "Goto Map");
    effect!([134..204, 242] SPC_EXIT, "WinLevel");
    effect!([134..204, 258] SPC_PICTURE, "Show Pic");
    effect!([134..204, 274] SPC_PLAYSOUND, "Play Snd");
    effect!([134..204, 290] SPC_TEMPLIGHT, "TmpLght");
    effect!([134..204, 306] SPC_CHGTILE, "ChgFloor");

    effect!([206..296, 210] SPC_PLAYSOUND2, "PlaySnd2");
    effect!([206..296, 226] SPC_WINANDGO, "Win & Go");
    effect!([206..296, 242] SPC_COPYMAP, "CopyMap");
    effect!([206..296, 258] SPC_KILLMONS, "KillMons");
    effect!([206..296, 274] SPC_CHGMONS, "ChngMons");
    effect!([206..296, 290] SPC_RMVSPCL, "Del Spcl");
    effect!([206..296, 306] SPC_TOGGLEITEM, "TogglItm");

    // trigger value
    RenderValueAdjuster(cstr!("Trigger Value"), (*spcl).trigValue as c_int, 210, 12);
    // value
    RenderValueAdjuster(cstr!("Effect Value"), (*spcl).value, 210, 180);
    // targetX
    RenderValueAdjuster(cstr!("Target X"), (*spcl).effectX as c_int, 370, 300);
    // targetY
    RenderValueAdjuster(cstr!("Target Y"), (*spcl).effectY as c_int, 370, 330);
}

fn CheckValueAdjustClick(msx: c_int, msy: c_int, x: c_int, y: c_int, value: &mut u8, min: u8, max: u8) {
    if msy > y + 13 && msy < y + 29 {
        if msx > x - 1 && msx < x + 21 {
            *value = value.checked_sub(10).unwrap_or(max);
        } else if msx > x + 21 && msx < x + 33 {
            *value = value.checked_sub(1).unwrap_or(max);
        } else if msx > x + 67 && msx < x + 79 {
            *value = value.checked_add(1).unwrap_or(min);
        } else if msx > x + 79 && msx < x + 101 {
            *value = value.checked_add(10).unwrap_or(min);
        }
    }
    if *value < min { *value = max }
    if *value > max { *value = min }
}

fn CheckValueAdjustClick2(msx: c_int, msy: c_int, x: c_int, y: c_int, value: &mut i32, min: i32, max: i32) {
    if msy > y + 13 && msy < y + 29 {
        if msx > x - 1 && msx < x + 21 {
            *value = value.checked_sub(10).unwrap_or(max);
        } else if msx > x + 21 && msx < x + 33 {
            *value = value.checked_sub(1).unwrap_or(max);
        } else if msx > x + 67 && msx < x + 79 {
            *value = value.checked_add(1).unwrap_or(min);
        } else if msx > x + 79 && msx < x + 101 {
            *value = value.checked_add(10).unwrap_or(min);
        }
    }
    if *value < min { *value = max }
    if *value > max { *value = min }
}

#[no_mangle]
pub unsafe extern fn SpclDialogClick(msx: c_int, msy: c_int) -> u8 {
    // checkbox toggles
    macro_rules! trigger {
        ([$xr:expr, $yr:expr] $trg:ident) => {
            if msx > $xr.start && msx < $xr.end && msy > $yr.start && msy < $yr.end {
                (*spcl).trigger ^= map::$trg;
            }
        }
    }

    trigger!([61..72, 25..26+14] TRG_STEP);
    trigger!([61..72, 39..40+14] TRG_ENEMYSTEP);
    trigger!([61..72, 53..54+14] TRG_NEAR);
    trigger!([61..72, 67..68+14] TRG_PASSLEVELS);
    trigger!([61..72, 81..82+14] TRG_KEYCHAINS);
    trigger!([61..72, 95..96+14] TRG_KILLALL);
    trigger!([61..72, 109..110+14] TRG_HAVEBRAINS);
    trigger!([61..72, 123..124+14] TRG_SHOOT);
    trigger!([61..72, 147..148+14] TRG_REPEATABLE);
    trigger!([61..72, 161..162+14] TRG_MESSAGE);

    trigger!([219..230, 53..54+14] TRG_CHAIN);
    trigger!([219..230, 67..68+14] TRG_TIMED);
    trigger!([219..230, 81..82+14] TRG_RANDOM);
    trigger!([219..230, 95..96+14] TRG_HAVELOONY);
    trigger!([219..230, 109..110+14] TRG_KILLONE);
    trigger!([219..230, 123..124+14] TRG_FLOORHERE);

    // effect choices
    macro_rules! effect {
        ([$xr:expr, $yr:expr] $ef:ident) => {
            if msx > $xr.start && msx < $xr.end && msy > $yr.start && msy < $yr.end {
                (*spcl).effect = map::Effect::$ef;
            }
        }
    }

    effect!([61..133, 193..209] SPC_NONE);
    effect!([61..133, 209..227] SPC_SUMMON);
    effect!([61..133, 227..243] SPC_ZAPWALL);
    effect!([61..133, 243..257] SPC_RAISEWALL);
    effect!([61..133, 257..271] SPC_TOGGLEWALL);
    effect!([61..133, 273..287] SPC_PLAYSONG);
    effect!([61..133, 289..303] SPC_DROPITEM);
    effect!([61..133, 305..319] SPC_SWAPMAP);

    effect!([133..205, 193..209] SPC_TELEPORT);
    effect!([133..205, 209..227] SPC_LIGHT);
    effect!([133..205, 227..243] SPC_GOTOMAP);
    effect!([133..205, 243..257] SPC_EXIT);
    effect!([133..205, 257..271] SPC_PICTURE);
    effect!([133..205, 273..287] SPC_PLAYSOUND);
    effect!([133..205, 289..303] SPC_TEMPLIGHT);
    effect!([133..205, 305..319] SPC_CHGTILE);

    effect!([205..297, 209..227] SPC_PLAYSOUND2);
    effect!([205..297, 227..243] SPC_WINANDGO);
    effect!([205..297, 243..257] SPC_COPYMAP);
    effect!([205..297, 257..271] SPC_KILLMONS);
    effect!([205..297, 273..287] SPC_CHGMONS);
    effect!([205..297, 289..303] SPC_RMVSPCL);
    effect!([205..297, 305..319] SPC_TOGGLEITEM);

    // trigger value
    CheckValueAdjustClick(msx, msy, 210, 12, &mut (*spcl).trigValue, 0, 255);
    // value
    CheckValueAdjustClick2(msx, msy, 210, 180, &mut (*spcl).value, -255, 255);
    CheckValueAdjustClick(msx, msy, 370, 300, &mut (*spcl).effectX, 0, 127);
    CheckValueAdjustClick(msx, msy, 370, 330, &mut (*spcl).effectY, 0, 127);
    1
}
