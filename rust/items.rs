use libc::{c_int, c_char};
use jamulspr::sprite_set_t;
use mgldraw::MGLDraw;

pub const MAX_ITMS: c_int = 128;

// these defines are 1 greater than the highest item of that type
// (that is, items 1 - X-1 are things the player can pick up, and items
// X - Y-1 are walkable things that don't obstruct movement, and items
// Y - Z-1 are not walkable but don't stop projectiles, and the remaining
// items are obstacles.
pub const MAX_PICKUP_ITMS: c_int = 30;
pub const MAX_WALKABLE_ITMS: c_int = 50;
pub const MAX_SHOOTABLE_ITMS: c_int = 69;
pub const NEW_PICKUP_ITMS: c_int = 92;

static mut itmSpr: *mut sprite_set_t = 0 as *mut sprite_set_t;
static mut glowism: u8 = 0;
static mut itmLight: bool = false;

#[no_mangle]
pub unsafe extern fn InitItems() {
    itmSpr = sprite_set_t::load_boxed("graphics/items.jsp");
    glowism = 0;
}

#[no_mangle]
pub unsafe extern fn ExitItems() {
    sprite_set_t::delete(itmSpr);
}

#[no_mangle]
pub unsafe extern fn DrawRedX(x: c_int, y: c_int, mgl: &mut MGLDraw) {
    (*itmSpr).GetSprite(8).Draw(x, y, mgl);
}

#[no_mangle]
pub unsafe extern fn ItemLightUp() {
    itmLight = !itmLight;
}

#[no_mangle]
pub unsafe extern fn RenderItem(x: c_int, y: c_int, type_: u8, bright: i8) {
    use display::*;

    let x = x - 3;
    let bright = if itmLight { 10 } else { bright };

    if let Some(item) = Item::from_int(type_ as usize) {
        let info = info(item);
        if info.spr < 0 { return; }

        let sprite = (*itmSpr).GetSprite(info.spr);
        if info.shadow {
            SprDraw(x + info.ofsx, y + info.ofsy, 0, 255, bright, sprite, DISPLAY_DRAWME | DISPLAY_SHADOW);
        }
        if info.loony {
            glowism = glowism.wrapping_add(1);
            let b = (16 - (glowism as i8 & 31)).abs();
            SprDraw(x, y, 0, glowism / 32, bright + b, sprite, DISPLAY_DRAWME);
        } else if info.glow {
            SprDraw(x + info.ofsx, y + info.ofsy, 0, 255, bright, sprite, DISPLAY_DRAWME | DISPLAY_GLOW);
        } else if info.recolor != NO_RECOLOR {
            SprDrawOff(x + info.ofsx, y + info.ofsy, 0, info.recolor.0, info.recolor.1, bright, sprite, DISPLAY_DRAWME);
        } else {
            SprDraw(x + info.ofsx, y + info.ofsy, 0, 255, bright, sprite, DISPLAY_DRAWME);
        }
    }
}

#[no_mangle]
pub unsafe extern fn InstaRenderItem(
    x: c_int, y: c_int, type_: u8,
    bright: c_char, mgl: &mut ::mgldraw::MGLDraw
) {
    let x = x - 3;

    if let Some(item) = Item::from_int(type_ as usize) {
        let info = info(item);
        if info.spr < 0 { return; }

        let sprite = (*itmSpr).GetSprite(info.spr);
        if info.glow {
            sprite.DrawGlow(x + info.ofsx, y + info.ofsy, mgl, bright);
        } else if info.recolor != NO_RECOLOR {
            sprite.DrawOffColor(x + info.ofsx, y + info.ofsy, mgl, info.recolor.0, info.recolor.1, bright);
        } else {
            sprite.DrawBright(x + info.ofsx, y + info.ofsy, mgl, bright);
        }
    }
}

struct ItemInfo {
    spr: c_int,
    ofsx: c_int,
    ofsy: c_int,
    shadow: bool,
    loony: bool,
    glow: bool,
    recolor: (u8, u8),
}

const NO_RECOLOR: (u8, u8) = (255, 255);
const DEFAULT_ITEM: ItemInfo = ItemInfo {
    spr: -1,
    ofsx: 0,
    ofsy: 0,
    shadow: false,
    loony: false,
    glow: false,
    recolor: NO_RECOLOR,
};

macro_rules! items {
    ( $( $itm:ident = $num:expr, { $($field:ident: $value:expr),* } ,)* ) => {
        #[repr(u8)]
        #[derive(FromInt)]
        pub enum Item {
            $($itm = $num,)*
        }

        fn info(item: Item) -> ItemInfo {
            match item {
                $(Item::$itm => ItemInfo { $($field: $value,)* ..DEFAULT_ITEM },)*
            }
        }
    }
}

items! {
    ITM_NONE = 0,         {},
    ITM_HAMMERUP = 1,     { spr: 0 },
    ITM_PANTS = 2,        { spr: 3, ofsy: -2 },
    ITM_REVERSE = 3,      { spr: 4 },
    ITM_REFLECT = 4,      { spr: 5 },
    ITM_MISSILES = 5,     { spr: 6 },
    ITM_AK8087 = 6,       { spr: 33 },
    ITM_TAKEOUT = 7,      { spr: 38 },
    ITM_SHIELD = 8,       { spr: 39, glow: true },
    ITM_BOMBS = 9,        { spr: 35 },
    ITM_FLAME = 10,       { spr: 34 },
    ITM_BRAIN = 11,       { spr: 7, ofsy: -2 },
    ITM_KEYCH1 = 12,      { spr: 24, ofsy: -1 },
    ITM_KEYCH2 = 13,      { spr: 21, ofsy: -1 },
    ITM_KEYCH3 = 14,      { spr: 22, ofsy: -1 },
    ITM_KEYCH4 = 15,      { spr: 23, ofsy: -1 },
    ITM_KEY = 16,         { spr: 17, ofsy: -1 },
    ITM_KEYR = 17,        { spr: 18, ofsy: -1 },
    ITM_KEYG = 18,        { spr: 19, ofsy: -1 },
    ITM_KEYB = 19,        { spr: 20, ofsy: -1 },
    ITM_LOONYKEY = 20,    { spr: 25, loony: true },
    ITM_BIGAXE = 21,      { spr: 44 },
    ITM_PWRARMOR = 22,    { spr: 43 },
    ITM_LIGHTNING = 23,   { spr: 46 },
    ITM_SPEAR = 24,       { spr: 45 },
    ITM_MACHETE = 25,     { spr: 47 },
    ITM_MINES = 26,       { spr: 48 },
    ITM_GARLIC = 27,      { spr: 49 },
    ITM_ORBITER = 28,     { spr: 58 },
    ITM_ACCEL = 29,       { spr: 59 },

    ITM_SMLROCKS = 30,    { spr: 32 },
    ITM_HOLETREE = 31,    { spr: 26 },
    ITM_IGLOO = 32,       { spr: 42 },
    ITM_WEB = 33,         { spr: 54, ofsx: 5, ofsy: -6 },
    ITM_WEB2 = 34,        { spr: 55, ofsy: -10 },
    ITM_WEB3 = 35,        { spr: 56, ofsx: -2, ofsy: -8 },
    ITM_WEB4 = 36,        { spr: 57, ofsy: -9 },
    ITM_GRASS = 37,       { spr: 61 },
    ITM_GRASS2 = 38,      { spr: 62 },
    ITM_VILLAGE = 39,     { spr: 65 },

    ITM_BOX = 50,         { spr: 1 },
    ITM_STUMP = 51,       { spr: 27 },
    ITM_BUSH = 52,        { spr: 30 },
    ITM_BIGROCKS = 53,    { spr: 31 },
    ITM_POST = 54,        { spr: 28, shadow: true },
    ITM_CHAIR1 = 55,      { spr: 36, shadow: true },
    ITM_CHAIR2 = 56,      { spr: 37, shadow: true },
    ITM_WALLGRASS = 57,   { spr: 66 },
    ITM_BARREL = 58,      { spr: 73, ofsy: -4, shadow: true },
    ITM_BARREL2 = 59,     { spr: 73, ofsy: -4, shadow: true },
    ITM_BARREL3 = 60,     { spr: 74, ofsy: -4, shadow: true },
    ITM_TRASHCAN = 61,    { spr: 75, ofsx: -1, ofsy: -2, shadow: true },
    ITM_TRASHCAN2 = 62,   { spr: 76, ofsx: -1, ofsy: -2, shadow: true },
    ITM_CRATE = 63,       { spr: 77, ofsx: -3, ofsy: -2 },
    ITM_CRATE2 = 64,      { spr: 78, ofsx: -3, ofsy: -2 },
    ITM_BUSH2 = 65,       { spr: 79 },
    ITM_BUSH3 = 66,       { spr: 80, ofsx: -2, ofsy: -3, shadow: true },
    ITM_BRWNROCK = 67,    { spr: 87, ofsy: -4 },

    ITM_PALM = 69,        { spr: 41, shadow: true },
    ITM_TREE = 70,        { spr: 2 },
    ITM_DOOR1 = 71,       { spr: 9, ofsx: -13, ofsy: 9 },
    ITM_DOOR1R = 72,      { spr: 10, ofsx: -13, ofsy: 9 },
    ITM_DOOR1G = 73,      { spr: 11, ofsx: -13, ofsy: 9 },
    ITM_DOOR1B = 74,      { spr: 12, ofsx: -13, ofsy: 9 },
    ITM_DOOR2 = 75,       { spr: 13, ofsx: -13, ofsy: 9 },
    ITM_DOOR2R = 76,      { spr: 14, ofsx: -13, ofsy: 9 },
    ITM_DOOR2G = 77,      { spr: 15, ofsx: -13, ofsy: 9 },
    ITM_DOOR2B = 78,      { spr: 16, ofsx: -13, ofsy: 9 },
    ITM_SIGN = 79,        { spr: 29, shadow: true },
    ITM_PINE = 80,        { spr: 40 },
    ITM_DEADTREE = 81,    { spr: 52 },
    ITM_DEADTREE2 = 82,   { spr: 53 },
    ITM_FATPALM = 83,     { spr: 60, shadow: true },
    ITM_TREE2 = 84,       { spr: 63 },
    ITM_MINEBLOCK = 85,   { spr: 64 },
    ITM_SIGN2 = 86,       { spr: 81, shadow: true },
    ITM_SIGN3 = 87,       { spr: 82, shadow: true },
    ITM_SIGN4 = 88,       { spr: 83, shadow: true },
    ITM_SIGN5 = 89,       { spr: 84, shadow: true },
    ITM_SIGN6 = 90,       { spr: 85, shadow: true },
    ITM_BIGROCK = 91,     { spr: 86, shadow: true },

    ITM_TURRETWPN = 92,   { spr: 68, ofsx: -1, ofsy: -2 },
    ITM_MINDCONTROL = 93, { spr: 69 },
    ITM_REFLECTOR = 94,   { spr: 67, ofsx: -1 },
    ITM_INVIS = 95,       { spr: 88, ofsx: -1, glow: true },
    ITM_JETPACK = 96,     { spr: 71 },
    ITM_UNHAMMER = 97,    { spr: 0, recolor: (4, 0) },
    ITM_UNPANTS = 98,     { spr: 3, recolor: (3, 0) },
    ITM_SWAPGUN = 99,     { spr: 70 },
    ITM_BADCHINESE = 100, { spr: 38, recolor: (4, 1) },
}
