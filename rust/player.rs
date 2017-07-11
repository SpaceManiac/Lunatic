use libc::c_int;
extern {
    pub fn PlayerGetItem(itm: u8, x: c_int, y: c_int) -> u8;
    pub fn PlayerHeal(amt: u8);
    pub fn ToggleWaterwalk();
}

cpp! {{
    #include "player.h"
}}

pub unsafe fn fill_rage() {
    cpp!([] { player.rage = 127 * 256; });
}
