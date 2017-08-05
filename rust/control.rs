bitflags! {
    /// the various control flags
    pub struct Controls: u8 {
        const EMPTY = 0;
        const CONTROL_UP = 1;
        const CONTROL_DN = 2;
        const CONTROL_LF = 4;
        const CONTROL_RT = 8;
        const CONTROL_B1 = 16;
        const CONTROL_B2 = 32;
        const CONTROL_B3 = 64;
        const CONTROL_B4 = 128;
    }
}

/// source of control
#[repr(C)]
pub enum ControlSource {
    CONTROL_KB1 = 0,
    CONTROL_KB2,
    CONTROL_JOY
}

extern {
    pub fn GetControls() -> u8;
    pub fn GetTaps() -> u8;
    pub fn GetArrows() -> u8;
}
