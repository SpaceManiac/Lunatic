use libc::c_int;
use mgldraw::MGLDraw;

/// these are the messages you can send to the game
#[repr(C)]
pub enum Message {
    MSG_NONE = 0,
    MSG_GOTOMAP,
    MSG_WINLEVEL,
    MSG_RESET,
    MSG_LOADGAME,
    MSG_WINGAME,
    MSG_NEWFEATURE
}

extern {
    pub fn LunaticInit(mgl: *mut MGLDraw);
    pub fn LunaticGame(mgl: *mut MGLDraw, load: u8);
    pub fn LunaticExit();
    pub fn SendMessageToGame(msg: u8, content: c_int);
}