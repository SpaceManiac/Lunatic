use libc::{c_int, c_char};
use world::world_t;
use mgldraw::MGLDraw;

extern {
    pub fn InitMapDialog(wrld: *mut world_t, currentMap: u8);
    pub fn ExitMapDialog();
    pub fn RenderMapDialog(msx: c_int, msy: c_int, mgl: *mut MGLDraw);
    pub fn MapDialogKey(key: c_char) -> u8;
    pub fn MapDialogClick(msx: c_int, msy: c_int) -> u8;
}
