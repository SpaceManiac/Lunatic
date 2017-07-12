use libc::{c_char, c_int};
use mgldraw::MGLDraw;

extern {
    pub fn InitInterface();
    pub fn ExitInterface();
    pub fn ResetInterface();
    pub fn RenderRage(size: u8, mgl: *mut MGLDraw);
    pub fn ShowEnemyLife(name: *mut c_char, formerLife: u8, life: u8, alive: u8);
    pub fn RenderInterface(
        life: u8, rage: u8, hmrFlags: u8, hammers: u8,
        brains: c_int, score: c_int, wpn: u8, ammo: u8,
        hamSped: u8, mgl: *mut MGLDraw,
    );
}
