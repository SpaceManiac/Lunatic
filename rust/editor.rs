use libc::c_char;
use mgldraw::MGLDraw;
extern {
    pub fn LunaticEditor(mgl: *mut MGLDraw) -> u8;
    pub fn EditorLoadTiles(fname: *const c_char);
}
