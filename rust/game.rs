use mgldraw::MGLDraw;
extern {
    pub fn LunaticInit(mgl: *mut MGLDraw);
    pub fn LunaticGame(mgl: *mut MGLDraw, load: u8);
    pub fn LunaticExit();
}
