#[repr(C)]
pub enum Action {
    ACTION_IDLE = 0,
    ACTION_BUSY
}

opaque!(Guy);

extern {
    pub static mut goodguy: *mut Guy;
}
