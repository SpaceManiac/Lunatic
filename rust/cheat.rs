use libc::{tolower};

const CHEAT_CODES: &[&[u8]] = &[
    b"zappo" as &[u8], // auto-win the level
    b"hammersplease", // max hammer up
    b"kablooie", // super nuke attack
    b"zool", // all keys
    b"medic", // full health
    b"barrier", // shield
    b"agua", // waterwalk
    b"stampy", // keychains!
    b"brainz", // get all brains
    b"itembright", // light up items
    b"whereami", // cloak d'invisible
    b"conwiz", // conspiracies!
    b"arglebargle", // fill with rage
];

static mut lastKeys: [u8; 16] = [0; 16]; // the last 16 letter keys pressed

pub unsafe fn InitCheater() {
    for ch in lastKeys.iter_mut() {
        *ch = 0;
    }
}

pub unsafe fn CheatKey(c: u8) {
    // scoot the existing letters over 1
    for i in 0..15 {
        lastKeys[i] = lastKeys[i + 1];
    }
    // and stick the new one on the end
    lastKeys[15] = tolower(c as i32) as u8;

    for i in 0..CHEAT_CODES.len() {
        if lastKeys.ends_with(CHEAT_CODES[i]) {
            DoCheat(i as u8);
            lastKeys[15] = 0;
            break;
        }
    }
}

pub unsafe fn DoCheat(w: u8) {
    use sound::make_normal_sound;
    use mgldraw::MGL_random;
    use message::NewMessage;
    use sound::Sound::*;
    use items::Item::*;
    use player::*;

    match w {
        0 => { // auto-win level
            make_normal_sound(SND_CHEATWIN);
            NewMessage(cstr!("Lemme out!"), 30, 0);
            ::game::SendMessageToGame(::game::Message::WinLevel, 0);
        }
        1 => { // max hammer up
            for _ in 0..5 {
                PlayerGetItem(ITM_HAMMERUP, 0, 0);
            }
            for _ in 0..4 {
                PlayerGetItem(ITM_PANTS, 0, 0);
            }
            PlayerGetItem(ITM_REVERSE, 0, 0);
            PlayerGetItem(ITM_REFLECT, 0, 0);
            NewMessage(cstr!("ULTRA HAMMER UP!!"), 30, 0);
        }
        2 => { // meganuke
            let (mut cx, mut cy) = ::display::get_camera();
            cx -= 320;
            cy -= 240;
            for _ in 0..60 {
                ::bullet::FireBullet(
                    (cx + MGL_random(640)) << ::FIXSHIFT,
                    (cy + MGL_random(480)) << ::FIXSHIFT,
                    0, ::bullet::Bullet::BLT_BOOM, 1);
            }
            ::display::ShakeScreen(10); // make the screen shake!
        }
        3 => { // all keys
            for _ in 0..3 {
                PlayerGetItem(ITM_KEY, 0, 0);
            }
            PlayerGetItem(ITM_KEYR, 0, 0);
            PlayerGetItem(ITM_KEYG, 0, 0);
            PlayerGetItem(ITM_KEYB, 0, 0);
            NewMessage(cstr!("I am the keymaster!"), 30, 0);
        }
        4 => { // restore health
            PlayerHeal(128);
            NewMessage(cstr!("Aaaaah"), 30, 0);
            make_normal_sound(SND_HEALTHCHEAT);
        }
        5 => { // shield
            PlayerGetItem(ITM_SHIELD, 0, 0);
        }
        6 => { // water-walk
            ToggleWaterwalk();
            make_normal_sound(SND_CHEATWIN);
        }
        7 => { // keychains
            PlayerGetItem(ITM_KEYCH1, 0, 0);
            PlayerGetItem(ITM_KEYCH2, 0, 0);
            PlayerGetItem(ITM_KEYCH3, 0, 0);
            PlayerGetItem(ITM_KEYCH4, 0, 0);
        }
        8 => { // brains
            for _ in 0..20 {
                PlayerGetItem(ITM_BRAIN, 0, 0);
            }
        }
        9 => { // itemlight
            ::items::ItemLightUp();
        }
        10 => { // clock invisible
            PlayerGetItem(ITM_INVIS, 0, 0);
        }
        11 => { // conwiz!
            //NewBigMessage("Conspiracies", 30);
        }
        12 => { // fill with rage
            NewMessage(cstr!("Filling... with... RAGE!!"), 75, 0);
            ::player::player.rage = 127 * 256;
        }
        _ => {}
    }
}
