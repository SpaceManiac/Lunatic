use libc::{c_char, c_int};
use mgldraw::MGLDraw;
use jamulspr::sprite_set_t;

const SPR_RAGE: c_int = 42;

extern {
    pub fn ResetInterface();
    pub fn ShowEnemyLife(name: *mut c_char, formerLife: u8, life: u8, alive: u8);
    pub fn RenderInterface(
        life: u8, rage: u8, hmrFlags: u8, hammers: u8,
        brains: c_int, score: c_int, wpn: u8, ammo: u8,
        hamSped: u8, mgl: *mut MGLDraw,
    );

    static mut intfaceSpr: *mut sprite_set_t;
}

#[no_mangle]
pub unsafe extern fn InitInterface() {
    intfaceSpr = sprite_set_t::from_fname(cstr!("graphics/intface.jsp"));
}

#[no_mangle]
pub unsafe extern fn ExitInterface() {
    sprite_set_t::delete(intfaceSpr);
}

#[no_mangle]
pub unsafe extern fn RenderRage(size: u8, mgl: *mut MGLDraw) {
    let mgl = &mut *mgl;
    (*intfaceSpr).GetSprite(SPR_RAGE + size as c_int).Draw(320, 240, mgl)
}
