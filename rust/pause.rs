use player::{self, player_t};
use options::Music;
use display::{GetDisplayMGL, CenterPrint};
use libc::c_float;

#[repr(C)]
pub enum Submode {
    SUBMODE_NONE = 0,
    SUBMODE_SLOTPICK,
}

#[repr(C)]
#[derive(FromInt)]
pub enum GiveUp {
    WorldSelect = 0,
    GiveUp = 1,
    None = 2,
}

extern {
    static mut cursor: u8;
    static mut subcursor: u8;
    static mut subMode: u8;
    /// the percentages in each save slot
    static mut percent: [f32; 3];
    /// which text should be shown for "Give Up"
    static mut giveUp: u8;
}

#[no_mangle]
pub unsafe extern fn SetSubCursor(s: u8) {
    subcursor = s;
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
    if subMode == Submode::SUBMODE_SLOTPICK as u8 {
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
pub unsafe extern fn SetGiveUpText(gu: u8) {
    giveUp = gu;
}
